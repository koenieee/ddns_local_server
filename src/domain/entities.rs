use std::net::IpAddr;
use std::error::Error;
use std::fmt;
use serde::{Deserialize, Serialize};

/// Domain entity representing an IP address entry in a configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IpEntry {
    pub ip: IpAddr,
    pub hostname: String,
    pub comment: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl IpEntry {
    pub fn new(ip: IpAddr, hostname: String, comment: Option<String>) -> Self {
        let now = chrono::Utc::now();
        Self {
            ip,
            hostname,
            comment,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update_ip(&mut self, new_ip: IpAddr) {
        self.ip = new_ip;
        self.updated_at = chrono::Utc::now();
    }

    pub fn update_comment(&mut self, comment: Option<String>) {
        self.comment = comment;
        self.updated_at = chrono::Utc::now();
    }
}

/// Configuration entry for a web server
#[derive(Debug, Clone)]
pub struct WebServerConfig {
    pub path: std::path::PathBuf,
    pub server_type: WebServerType,
    pub backup_path: Option<std::path::PathBuf>,
}

impl WebServerConfig {
    pub fn new(path: std::path::PathBuf, server_type: WebServerType) -> Self {
        Self {
            path,
            server_type,
            backup_path: None,
        }
    }

    pub fn with_backup(mut self, backup_path: std::path::PathBuf) -> Self {
        self.backup_path = Some(backup_path);
        self
    }
}

/// Supported web server types
#[derive(Debug, Clone, PartialEq)]
pub enum WebServerType {
    Nginx,
    Apache,
    Caddy,
    Traefik,
}

impl fmt::Display for WebServerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WebServerType::Nginx => write!(f, "nginx"),
            WebServerType::Apache => write!(f, "apache"),
            WebServerType::Caddy => write!(f, "caddy"),
            WebServerType::Traefik => write!(f, "traefik"),
        }
    }
}

impl std::str::FromStr for WebServerType {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "nginx" => Ok(WebServerType::Nginx),
            "apache" | "apache2" | "httpd" => Ok(WebServerType::Apache),
            "caddy" => Ok(WebServerType::Caddy),
            "traefik" => Ok(WebServerType::Traefik),
            _ => Err(DomainError::InvalidWebServerType(s.to_string())),
        }
    }
}

/// Domain-specific errors
#[derive(Debug, Clone)]
pub enum DomainError {
    InvalidIpAddress(String),
    InvalidHostname(String),
    InvalidWebServerType(String),
    ConfigurationNotFound(String),
    IpEntryNotFound(String),
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DomainError::InvalidIpAddress(ip) => write!(f, "Invalid IP address: {}", ip),
            DomainError::InvalidHostname(hostname) => write!(f, "Invalid hostname: {}", hostname),
            DomainError::InvalidWebServerType(server_type) => {
                write!(f, "Unsupported web server type: {}", server_type)
            }
            DomainError::ConfigurationNotFound(path) => {
                write!(f, "Configuration not found: {}", path)
            }
            DomainError::IpEntryNotFound(hostname) => {
                write!(f, "IP entry not found for hostname: {}", hostname)
            }
        }
    }
}

impl Error for DomainError {}