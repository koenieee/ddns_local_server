use crate::domain::entities::{IpEntry, WebServerConfig};
use crate::domain::ports::{IpRepository, NetworkService, NotificationService, WebServerHandler};
use std::net::IpAddr;
use std::sync::Arc;

/// Core DDNS update service - implements the main business logic
pub struct DdnsUpdateService {
    ip_repository: Arc<dyn IpRepository>,
    web_server_handler: Arc<dyn WebServerHandler>,
    network_service: Arc<dyn NetworkService>,
    notification_service: Arc<dyn NotificationService>,
}

impl DdnsUpdateService {
    pub fn new(
        ip_repository: Arc<dyn IpRepository>,
        web_server_handler: Arc<dyn WebServerHandler>,
        network_service: Arc<dyn NetworkService>,
        notification_service: Arc<dyn NotificationService>,
    ) -> Self {
        Self {
            ip_repository,
            web_server_handler,
            network_service,
            notification_service,
        }
    }

    /// Main update operation - checks current IP and updates configuration if changed
    pub async fn update_ddns(
        &self,
        hostname: &str,
        config: &WebServerConfig,
    ) -> Result<UpdateResult, Box<dyn std::error::Error + Send + Sync>> {
        self.update_ddns_with_options(hostname, config, false).await
    }

    pub async fn update_ddns_with_options(
        &self,
        hostname: &str,
        config: &WebServerConfig,
        no_reload: bool,
    ) -> Result<UpdateResult, Box<dyn std::error::Error + Send + Sync>> {
        eprintln!("DEBUG: Starting update_ddns for hostname: {}", hostname);
        eprintln!("DEBUG: Config path: {}", config.path.display());

        // Validate web server configuration first, before any other operations
        eprintln!("DEBUG: Validating web server configuration...");
        let is_valid = self.web_server_handler.validate_config(config).await?;
        eprintln!("DEBUG: Configuration validation result: {}", is_valid);
        if !is_valid {
            return Err("Invalid web server configuration".into());
        }

        // Resolve the hostname to get its current IP address
        eprintln!("DEBUG: Resolving hostname: {}", hostname);
        let resolved_ips = self.network_service.resolve_hostname(hostname).await?;
        eprintln!("DEBUG: Resolved IPs: {:?}", resolved_ips);

        if resolved_ips.is_empty() {
            return Err(format!("Could not resolve hostname: {}", hostname).into());
        }

        // Use the first resolved IP (typically the primary A record)
        let current_ip = resolved_ips[0];
        eprintln!("DEBUG: Using IP: {}", current_ip);

        // Get stored IP for this hostname
        eprintln!("DEBUG: Loading stored IP for hostname: {}", hostname);
        let stored_ip = self.ip_repository.load_ip(hostname).await?;
        eprintln!("DEBUG: Stored IP: {:?}", stored_ip);

        // If no JSON file exists, create it with current IP and check config
        if stored_ip.is_none() {
            eprintln!("DEBUG: No stored IP found, creating JSON file and checking config");

            // Store the current IP in JSON file
            self.ip_repository.store_ip(hostname, current_ip).await?;
            eprintln!("DEBUG: Created new JSON file with IP: {}", current_ip);

            // Check if the current IP is already in the config file
            let ip_in_config = self
                .web_server_handler
                .check_ip_in_config(config, current_ip)
                .await?;

            if ip_in_config {
                eprintln!(
                    "DEBUG: IP {} already exists in config, no update needed",
                    current_ip
                );
                return Ok(UpdateResult::NoChange { ip: current_ip });
            } else {
                eprintln!(
                    "DEBUG: IP {} not found in config, but not adding new entries",
                    current_ip
                );
                return Ok(UpdateResult::NoChange { ip: current_ip });
            }
        } else {
            // Check if IP has changed
            if let Some(old_ip) = stored_ip {
                if old_ip == current_ip {
                    eprintln!("DEBUG: IP unchanged, returning NoChange");
                    return Ok(UpdateResult::NoChange { ip: current_ip });
                }
            }
        }

        // Only proceed with backup and update if we have an IP change
        eprintln!("DEBUG: IP has changed, proceeding with backup and update");

        // First, check if this specific config file actually needs updating
        // by checking if the old IP exists in this file
        let needs_update = if let Some(old_ip) = stored_ip {
            self.web_server_handler
                .check_ip_in_config(config, old_ip)
                .await?
        } else {
            // No stored IP, check if current IP is already in config
            !self
                .web_server_handler
                .check_ip_in_config(config, current_ip)
                .await?
        };

        if !needs_update {
            eprintln!("DEBUG: Config file doesn't contain old IP, no update needed for this file");
            return Ok(UpdateResult::NoChange { ip: current_ip });
        }

        // Create backup only when we're actually going to modify the file
        eprintln!("DEBUG: Creating backup before modifying config file");
        let backup_path = self.web_server_handler.create_backup(config).await?;

        // Update the web server configuration
        let updated = self
            .web_server_handler
            .update_allow_list(config, hostname, stored_ip, current_ip)
            .await?;

        if updated {
            // Test the new configuration
            if !self.web_server_handler.test_configuration(config).await? {
                return Err("Configuration test failed after update".into());
            }

            // Reload the web server (unless --no-reload is specified)
            if !no_reload {
                eprintln!("DEBUG: About to reload server");
                match self.web_server_handler.reload_server().await {
                    Ok(()) => eprintln!("DEBUG: Server reload completed successfully"),
                    Err(e) => {
                        eprintln!("DEBUG: Server reload failed: {}", e);
                        return Err(e);
                    }
                }
            } else {
                eprintln!("DEBUG: Skipping server reload (--no-reload specified)");
            }

            // Store the new IP
            eprintln!("DEBUG: About to store IP");
            match self.ip_repository.store_ip(hostname, current_ip).await {
                Ok(()) => eprintln!("DEBUG: IP stored successfully"),
                Err(e) => {
                    eprintln!("DEBUG: Failed to store IP: {}", e);
                    return Err(e);
                }
            }

            // Send notification
            self.notification_service
                .notify_ip_change(hostname, stored_ip, current_ip)
                .await?;

            Ok(UpdateResult::Updated {
                hostname: hostname.to_string(),
                old_ip: stored_ip,
                new_ip: current_ip,
                backup_path,
            })
        } else {
            Ok(UpdateResult::NoChange { ip: current_ip })
        }
    }

    /// List all stored IP entries
    pub async fn list_entries(
        &self,
    ) -> Result<Vec<IpEntry>, Box<dyn std::error::Error + Send + Sync>> {
        self.ip_repository.list_all_entries().await
    }

    /// Remove an IP entry
    pub async fn remove_entry(
        &self,
        hostname: &str,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        self.ip_repository.delete_entry(hostname).await
    }

    /// Validate multiple configurations
    pub async fn validate_configs(
        &self,
        configs: &[WebServerConfig],
    ) -> Result<Vec<ValidationResult>, Box<dyn std::error::Error + Send + Sync>> {
        let mut results = Vec::new();

        for config in configs {
            let result = match self.web_server_handler.validate_config(config).await {
                Ok(valid) => ValidationResult {
                    config_path: config.path.clone(),
                    valid,
                    error: None,
                },
                Err(e) => ValidationResult {
                    config_path: config.path.clone(),
                    valid: false,
                    error: Some(e.to_string()),
                },
            };
            results.push(result);
        }

        Ok(results)
    }
}

/// Result of a DDNS update operation
#[derive(Debug, Clone)]
pub enum UpdateResult {
    Updated {
        hostname: String,
        old_ip: Option<IpAddr>,
        new_ip: IpAddr,
        backup_path: std::path::PathBuf,
    },
    NoChange {
        ip: IpAddr,
    },
}

/// Result of configuration validation
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub config_path: std::path::PathBuf,
    pub valid: bool,
    pub error: Option<String>,
}
