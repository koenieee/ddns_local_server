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
        // Get current public IP
        let current_ip = self.network_service.get_public_ip().await?;

        // Get stored IP for this hostname
        let stored_ip = self.ip_repository.load_ip(hostname).await?;

        // Check if IP has changed
        if let Some(old_ip) = stored_ip {
            if old_ip == current_ip {
                return Ok(UpdateResult::NoChange { ip: current_ip });
            }
        }

        // Validate web server configuration before making changes
        self.web_server_handler.validate_config(config).await?;

        // Create backup before modification
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

            // Reload the web server
            self.web_server_handler.reload_server().await?;

            // Store the new IP
            self.ip_repository.store_ip(hostname, current_ip).await?;

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
