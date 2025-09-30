use crate::domain::entities::{DomainError, IpEntry, WebServerConfig};
use async_trait::async_trait;
use std::net::IpAddr;

/// Repository trait for IP storage operations
#[async_trait]
pub trait IpRepository: Send + Sync {
    async fn store_ip(
        &self,
        hostname: &str,
        ip: IpAddr,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn load_ip(
        &self,
        hostname: &str,
    ) -> Result<Option<IpAddr>, Box<dyn std::error::Error + Send + Sync>>;
    async fn get_ip_entry(
        &self,
        hostname: &str,
    ) -> Result<Option<IpEntry>, Box<dyn std::error::Error + Send + Sync>>;
    async fn list_all_entries(
        &self,
    ) -> Result<Vec<IpEntry>, Box<dyn std::error::Error + Send + Sync>>;
    async fn delete_entry(
        &self,
        hostname: &str,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>>;
}

/// Web server configuration handler trait
#[async_trait]
pub trait WebServerHandler: Send + Sync {
    async fn update_allow_list(
        &self,
        config: &WebServerConfig,
        hostname: &str,
        old_ip: Option<IpAddr>,
        new_ip: IpAddr,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>>;

    async fn validate_config(
        &self,
        config: &WebServerConfig,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>>;

    async fn reload_server(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    async fn create_backup(
        &self,
        config: &WebServerConfig,
    ) -> Result<std::path::PathBuf, Box<dyn std::error::Error + Send + Sync>>;

    async fn test_configuration(
        &self,
        config: &WebServerConfig,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>>;

    fn server_type(&self) -> crate::domain::entities::WebServerType;
}

/// Network service for retrieving public IP addresses
#[async_trait]
pub trait NetworkService: Send + Sync {
    async fn get_public_ip(&self) -> Result<IpAddr, Box<dyn std::error::Error + Send + Sync>>;
    async fn resolve_hostname(
        &self,
        hostname: &str,
    ) -> Result<Vec<IpAddr>, Box<dyn std::error::Error + Send + Sync>>;
    async fn is_reachable(
        &self,
        ip: IpAddr,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>>;
}

/// Configuration discovery service
#[async_trait]
pub trait ConfigDiscoveryService: Send + Sync {
    async fn discover_configs(
        &self,
        pattern: Option<&str>,
    ) -> Result<Vec<WebServerConfig>, Box<dyn std::error::Error + Send + Sync>>;
    async fn detect_server_type(
        &self,
        config_path: &std::path::Path,
    ) -> Result<crate::domain::entities::WebServerType, DomainError>;
}

/// Notification service for alerting on changes
#[async_trait]
pub trait NotificationService: Send + Sync {
    async fn notify_ip_change(
        &self,
        hostname: &str,
        old_ip: Option<IpAddr>,
        new_ip: IpAddr,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    async fn notify_error(
        &self,
        error: &str,
        context: Option<&str>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}
