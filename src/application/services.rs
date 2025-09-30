use std::sync::Arc;
use crate::domain::entities::WebServerType;
use crate::domain::ports::{IpRepository, WebServerHandler, NetworkService, NotificationService, ConfigDiscoveryService};
use crate::infrastructure::{
    FileIpRepository, HttpNetworkService, ConsoleNotificationService, 
    FileSystemConfigDiscovery,
};
use crate::infrastructure::webservers::{NginxHandler, ApacheHandler};

/// Application service factory for creating configured services
pub struct ServiceFactory;

impl ServiceFactory {
    /// Create an IP repository with the given storage directory
    pub fn create_ip_repository(storage_dir: std::path::PathBuf) -> Result<Arc<dyn IpRepository>, Box<dyn std::error::Error + Send + Sync>> {
        let repo = FileIpRepository::new(storage_dir)?;
        Ok(Arc::new(repo))
    }

    /// Create a web server handler for the given server type
    pub fn create_web_server_handler(server_type: WebServerType) -> Arc<dyn WebServerHandler> {
        match server_type {
            WebServerType::Nginx => Arc::new(NginxHandler::new()),
            WebServerType::Apache => Arc::new(ApacheHandler::new()),
            WebServerType::Caddy => {
                // TODO: Implement Caddy handler
                Arc::new(NginxHandler::new()) // Fallback to Nginx for now
            }
            WebServerType::Traefik => {
                // TODO: Implement Traefik handler
                Arc::new(NginxHandler::new()) // Fallback to Nginx for now
            }
        }
    }

    /// Create a network service for retrieving public IP addresses
    pub fn create_network_service() -> Arc<dyn NetworkService> {
        Arc::new(HttpNetworkService::new())
    }

    /// Create a notification service based on configuration
    pub fn create_notification_service(verbose: bool) -> Arc<dyn NotificationService> {
        Arc::new(ConsoleNotificationService::new(verbose))
    }

    /// Create a configuration discovery service
    pub fn create_config_discovery_service() -> Arc<dyn ConfigDiscoveryService> {
        Arc::new(FileSystemConfigDiscovery::new())
    }
}

/// Application configuration
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub storage_dir: std::path::PathBuf,
    pub verbose: bool,
    pub backup_retention_days: u16,
    pub max_backups: u16,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            storage_dir: std::path::PathBuf::from("/var/lib/ddns-updater"),
            verbose: false,
            backup_retention_days: 30,
            max_backups: 10,
        }
    }
}

impl AppConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_storage_dir(mut self, dir: std::path::PathBuf) -> Self {
        self.storage_dir = dir;
        self
    }

    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    pub fn with_backup_retention(mut self, days: u16, max_backups: u16) -> Self {
        self.backup_retention_days = days;
        self.max_backups = max_backups;
        self
    }
}