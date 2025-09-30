use std::sync::Arc;
use crate::domain::services::{DdnsUpdateService, UpdateResult, ValidationResult};
use crate::domain::entities::{WebServerConfig, IpEntry};
use crate::domain::ports::{IpRepository, WebServerHandler, NetworkService, NotificationService, ConfigDiscoveryService};
use crate::application::services::{ServiceFactory, AppConfig};

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

    /// List all stored IP entries
    pub async fn list_entries(&self) -> Result<Vec<IpEntry>, Box<dyn std::error::Error + Send + Sync>> {
        self.service.list_entries().await
    }

    /// Remove an IP entry
    pub async fn remove_entry(&self, hostname: &str) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
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
        let server_type = self.config_discovery.detect_server_type(&config_path).await?;
        let config = WebServerConfig::new(config_path, server_type.clone());
        
        // Create appropriate web server handler
        let web_server_handler = ServiceFactory::create_web_server_handler(server_type);
        
        // Create and execute the use case
        let use_case = UpdateDdnsUseCase::new(
            self.ip_repository.clone(),
            web_server_handler,
            self.network_service.clone(),
            self.notification_service.clone(),
        );

        use_case.execute(hostname, &config).await
    }

    /// Update DDNS for multiple configuration files
    pub async fn update_ddns_multiple(
        &self,
        hostname: &str,
        config_paths: Vec<std::path::PathBuf>,
    ) -> Result<Vec<UpdateResult>, Box<dyn std::error::Error + Send + Sync>> {
        let mut results = Vec::new();

        for config_path in config_paths {
            match self.update_ddns(hostname, config_path.clone()).await {
                Ok(result) => results.push(result),
                Err(e) => {
                    self.notification_service
                        .notify_error(&e.to_string(), Some(&format!("config: {}", config_path.display())))
                        .await?;
                    // Continue with other configs
                }
            }
        }

        Ok(results)
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
    pub async fn list_entries(&self) -> Result<Vec<IpEntry>, Box<dyn std::error::Error + Send + Sync>> {
        self.ip_repository.list_all_entries().await
    }

    /// Remove an IP entry
    pub async fn remove_entry(&self, hostname: &str) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        self.ip_repository.delete_entry(hostname).await
    }

    /// Get current public IP without updating anything
    pub async fn get_current_ip(&self) -> Result<std::net::IpAddr, Box<dyn std::error::Error + Send + Sync>> {
        self.network_service.get_public_ip().await
    }
}