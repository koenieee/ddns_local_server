use crate::application::services::{AppConfig, ServiceFactory};
use crate::domain::entities::{IpEntry, WebServerConfig};
use crate::domain::ports::{
    ConfigDiscoveryService, IpRepository, NetworkService, NotificationService, WebServerHandler,
};
use crate::domain::services::{DdnsUpdateService, UpdateResult, ValidationResult};
use std::sync::Arc;

/// Result of processing multiple configurations
#[derive(Debug)]
pub struct MultiConfigResult {
    pub successes: Vec<UpdateResult>,
    pub errors: Vec<(std::path::PathBuf, String)>,
}

impl MultiConfigResult {
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn total_processed(&self) -> usize {
        self.successes.len() + self.errors.len()
    }
}

/// Use case for updating DDNS entries
pub struct UpdateDdnsUseCase {
    service: DdnsUpdateService,
}

impl UpdateDdnsUseCase {
    pub fn new(
        ip_repository: Arc<dyn IpRepository>,
        web_server_handler: Arc<dyn WebServerHandler>,
        network_service: Arc<dyn NetworkService>,
        notification_service: Arc<dyn NotificationService>,
    ) -> Self {
        let service = DdnsUpdateService::new(
            ip_repository,
            web_server_handler,
            network_service,
            notification_service,
        );

        Self { service }
    }

    /// Execute the DDNS update for a hostname and configuration
    pub async fn execute(
        &self,
        hostname: &str,
        config: &WebServerConfig,
    ) -> Result<UpdateResult, Box<dyn std::error::Error + Send + Sync>> {
        self.service.update_ddns(hostname, config).await
    }

    /// Execute the DDNS update with options (like no-reload)
    pub async fn execute_with_options(
        &self,
        hostname: &str,
        config: &WebServerConfig,
        no_reload: bool,
    ) -> Result<UpdateResult, Box<dyn std::error::Error + Send + Sync>> {
        self.service
            .update_ddns_with_options(hostname, config, no_reload)
            .await
    }

    /// List all stored IP entries
    pub async fn list_entries(
        &self,
    ) -> Result<Vec<IpEntry>, Box<dyn std::error::Error + Send + Sync>> {
        self.service.list_entries().await
    }

    /// Remove an IP entry
    pub async fn remove_entry(
        &self,
        hostname: &str,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        self.service.remove_entry(hostname).await
    }
}

/// Use case for discovering and validating configurations
pub struct ConfigValidationUseCase {
    config_discovery: Arc<dyn ConfigDiscoveryService>,
}

impl ConfigValidationUseCase {
    pub fn new(config_discovery: Arc<dyn ConfigDiscoveryService>) -> Self {
        Self { config_discovery }
    }

    /// Discover configuration files
    pub async fn discover_configs(
        &self,
        pattern: Option<&str>,
    ) -> Result<Vec<WebServerConfig>, Box<dyn std::error::Error + Send + Sync>> {
        self.config_discovery.discover_configs(pattern).await
    }

    /// Validate multiple configurations
    pub async fn validate_configs(
        &self,
        configs: &[WebServerConfig],
        web_server_handler: Arc<dyn WebServerHandler>,
    ) -> Result<Vec<ValidationResult>, Box<dyn std::error::Error + Send + Sync>> {
        let service = DdnsUpdateService::new(
            ServiceFactory::create_ip_repository(std::path::PathBuf::from("/tmp"))?,
            web_server_handler,
            ServiceFactory::create_network_service(),
            ServiceFactory::create_notification_service(false),
        );

        service.validate_configs(configs).await
    }
}

/// Application facade that provides a high-level interface
pub struct DdnsApplication {
    config: AppConfig,
    ip_repository: Arc<dyn IpRepository>,
    network_service: Arc<dyn NetworkService>,
    notification_service: Arc<dyn NotificationService>,
    config_discovery: Arc<dyn ConfigDiscoveryService>,
}

impl DdnsApplication {
    /// Create a new application instance with the given configuration
    pub fn new(config: AppConfig) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let ip_repository = ServiceFactory::create_ip_repository(config.storage_dir.clone())?;
        let network_service = ServiceFactory::create_network_service();
        let notification_service = ServiceFactory::create_notification_service(config.verbose);
        let config_discovery = ServiceFactory::create_config_discovery_service();

        Ok(Self {
            config,
            ip_repository,
            network_service,
            notification_service,
            config_discovery,
        })
    }

    /// Update DDNS for a specific hostname and configuration file
    pub async fn update_ddns(
        &self,
        hostname: &str,
        config_path: std::path::PathBuf,
    ) -> Result<UpdateResult, Box<dyn std::error::Error + Send + Sync>> {
        // Detect server type
        eprintln!(
            "DEBUG: About to detect server type for: {}",
            config_path.display()
        );
        let server_type = self
            .config_discovery
            .detect_server_type(&config_path)
            .await?;
        eprintln!("DEBUG: Server type detected: {:?}", server_type);

        let config = WebServerConfig::new(config_path, server_type.clone());
        eprintln!("DEBUG: WebServerConfig created");

        // Create appropriate web server handler
        let web_server_handler =
            ServiceFactory::create_web_server_handler(server_type, self.config.backup_dir.clone());
        eprintln!("DEBUG: Web server handler created");

        // Create and execute the use case
        let use_case = UpdateDdnsUseCase::new(
            self.ip_repository.clone(),
            web_server_handler,
            self.network_service.clone(),
            self.notification_service.clone(),
        );
        eprintln!("DEBUG: Use case created, about to execute");

        let result = use_case
            .execute_with_options(hostname, &config, self.config.no_reload)
            .await;
        eprintln!(
            "DEBUG: Use case execution result: {:?}",
            result
                .as_ref()
                .map(|_| "Ok")
                .map_err(|e| format!("Err: {}", e))
        );
        result
    }

    /// Update DDNS for multiple configuration files with consistent IP storage
    pub async fn update_ddns_multiple(
        &self,
        hostname: &str,
        config_paths: Vec<std::path::PathBuf>,
    ) -> Result<MultiConfigResult, Box<dyn std::error::Error + Send + Sync>> {
        let mut successes = Vec::new();
        let mut errors = Vec::new();

        // Check IP change first - if no change, skip processing all files
        let current_ip = match self.network_service.resolve_hostname(hostname).await {
            Ok(ips) if !ips.is_empty() => ips[0],
            Ok(_) => {
                let error_msg = format!("Could not resolve hostname: {}", hostname);
                for config_path in config_paths {
                    errors.push((config_path, error_msg.clone()));
                }
                return Ok(MultiConfigResult { successes, errors });
            }
            Err(e) => {
                let error_msg = e.to_string();
                for config_path in config_paths {
                    errors.push((config_path, error_msg.clone()));
                }
                return Ok(MultiConfigResult { successes, errors });
            }
        };

        let stored_ip = match self.ip_repository.load_ip(hostname).await {
            Ok(ip) => ip,
            Err(e) => {
                let error_msg = format!("Failed to load stored IP: {}", e);
                for config_path in config_paths {
                    errors.push((config_path, error_msg.clone()));
                }
                return Ok(MultiConfigResult { successes, errors });
            }
        };

        // If IP hasn't changed, return NoChange for all configs without processing them
        if let Some(old_ip) = stored_ip {
            if old_ip == current_ip {
                if self.config.verbose {
                    println!(
                        "ℹ️  No IP change detected ({}), skipping all config file processing",
                        current_ip
                    );
                }
                for _config_path in &config_paths {
                    successes.push(UpdateResult::NoChange { ip: current_ip });
                }
                return Ok(MultiConfigResult { successes, errors });
            }
        }

        // IP has changed (or no stored IP), process all configs that need actual updates
        if self.config.verbose {
            println!(
                "🔄 IP change detected, processing {} config files",
                config_paths.len()
            );
        }

        // Process all config files without storing IP yet
        let mut any_updated = false;
        for config_path in config_paths {
            match self
                .update_ddns_file_only(hostname, config_path.clone(), stored_ip, current_ip)
                .await
            {
                Ok(result) => {
                    if matches!(result, UpdateResult::Updated { .. }) {
                        any_updated = true;
                    }
                    successes.push(result);
                }
                Err(e) => {
                    let error_msg = e.to_string();
                    errors.push((config_path, error_msg));
                }
            }
        } // Only store the new IP and send notification if at least one file was actually updated
        if any_updated {
            if let Err(e) = self.ip_repository.store_ip(hostname, current_ip).await {
                // If we can't store the IP, treat it as an error but don't fail the whole operation
                eprintln!(
                    "Warning: Failed to store IP after successful updates: {}",
                    e
                );
            }

            // Send notification for the IP change (once for all files)
            if let Err(e) = self
                .notification_service
                .notify_ip_change(hostname, stored_ip, current_ip)
                .await
            {
                eprintln!("Warning: Failed to send notification: {}", e);
            }
        }

        Ok(MultiConfigResult { successes, errors })
    }

    /// Update DDNS for a specific config file without storing IP (used by multi-config processing)
    async fn update_ddns_file_only(
        &self,
        hostname: &str,
        config_path: std::path::PathBuf,
        stored_ip: Option<std::net::IpAddr>,
        current_ip: std::net::IpAddr,
    ) -> Result<UpdateResult, Box<dyn std::error::Error + Send + Sync>> {
        // Detect server type
        let server_type = self
            .config_discovery
            .detect_server_type(&config_path)
            .await?;

        let config = WebServerConfig::new(config_path, server_type.clone());

        // Create appropriate web server handler
        let web_server_handler =
            ServiceFactory::create_web_server_handler(server_type, self.config.backup_dir.clone());

        // Create the service but use it directly without storing IP
        let service = DdnsUpdateService::new(
            self.ip_repository.clone(),
            web_server_handler,
            self.network_service.clone(),
            self.notification_service.clone(),
        );

        // Process this specific file without storing IP
        service
            .update_file_only(
                &config,
                hostname,
                stored_ip,
                current_ip,
                self.config.no_reload,
            )
            .await
    }

    /// Discover configuration files using pattern
    pub async fn discover_configs(
        &self,
        pattern: Option<&str>,
    ) -> Result<Vec<WebServerConfig>, Box<dyn std::error::Error + Send + Sync>> {
        let use_case = ConfigValidationUseCase::new(self.config_discovery.clone());
        use_case.discover_configs(pattern).await
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

    /// Get current public IP without updating anything
    pub async fn get_current_ip(
        &self,
    ) -> Result<std::net::IpAddr, Box<dyn std::error::Error + Send + Sync>> {
        self.network_service.get_public_ip().await
    }

    /// Initialize DNS host file if it doesn't exist yet
    /// This tries to resolve the hostname's IP and creates a JSON file with the real IP
    /// Falls back to placeholder IP if resolution fails (e.g., in CI/CD environments)
    pub async fn initialize_host_file(
        &self,
        hostname: &str,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        // Try to resolve the hostname to get the current IP
        let current_ip = match self.network_service.resolve_hostname(hostname).await {
            Ok(resolved_ips) => match resolved_ips.first() {
                Some(ip) => *ip,
                None => {
                    eprintln!(
                        "Warning: No IPs resolved for {}, using placeholder",
                        hostname
                    );
                    "0.0.0.0".parse()?
                }
            },
            Err(e) => {
                eprintln!(
                    "Warning: Failed to resolve {}: {}, using placeholder",
                    hostname, e
                );
                "0.0.0.0".parse()?
            }
        };

        // Initialize the host file with the resolved IP (or placeholder if resolution failed)
        self.ip_repository
            .initialize_host_file(hostname, current_ip)
            .await
    }
}
