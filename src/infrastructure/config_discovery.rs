use std::path::{Path, PathBuf};
use async_trait::async_trait;
use tokio::fs;

use crate::domain::entities::{WebServerConfig, WebServerType, DomainError};
use crate::domain::ports::ConfigDiscoveryService;

/// File system-based configuration discovery service
pub struct FileSystemConfigDiscovery;

impl FileSystemConfigDiscovery {
    pub fn new() -> Self {
        Self
    }

    /// Detect server type based on file content and location
    async fn detect_server_type_from_content(&self, path: &Path) -> Result<WebServerType, DomainError> {
        if !path.exists() {
            return Err(DomainError::ConfigurationNotFound(path.display().to_string()));
        }

        // First, try to detect by common path patterns
        let path_str = path.to_string_lossy().to_lowercase();
        
        if path_str.contains("nginx") || path_str.contains("/etc/nginx/") {
            return Ok(WebServerType::Nginx);
        }
        
        if path_str.contains("apache") || path_str.contains("httpd") || 
           path_str.contains("/etc/apache2/") || path_str.contains("/etc/httpd/") {
            return Ok(WebServerType::Apache);
        }

        // If path doesn't give us a clue, examine file content
        match fs::read_to_string(path).await {
            Ok(content) => {
                let content_lower = content.to_lowercase();
                
                // Look for Nginx-specific directives
                if content_lower.contains("server_name") && 
                   (content_lower.contains("location") || content_lower.contains("listen")) {
                    return Ok(WebServerType::Nginx);
                }
                
                // Look for Apache-specific directives
                if content_lower.contains("<virtualhost") || 
                   content_lower.contains("<directory") ||
                   content_lower.contains("documentroot") {
                    return Ok(WebServerType::Apache);
                }
                
                // Look for Caddy-specific syntax
                if content_lower.contains("caddyfile") || 
                   (content_lower.contains("{") && content_lower.contains("reverse_proxy")) {
                    return Ok(WebServerType::Caddy);
                }
                
                // Default to Nginx if we can't determine
                Ok(WebServerType::Nginx)
            }
            Err(_) => Err(DomainError::ConfigurationNotFound(path.display().to_string())),
        }
    }

    /// Common configuration directory patterns
    fn get_common_config_patterns(&self) -> Vec<&'static str> {
        vec![
            "/etc/nginx/sites-available/*",
            "/etc/nginx/sites-enabled/*",
            "/etc/nginx/conf.d/*.conf",
            "/etc/apache2/sites-available/*",
            "/etc/apache2/sites-enabled/*",
            "/etc/apache2/conf.d/*.conf",
            "/etc/httpd/conf.d/*.conf",
            "/etc/httpd/sites-available/*",
            "/usr/local/etc/nginx/*",
            "/usr/local/etc/apache2*/*",
        ]
    }

    /// Expand glob pattern to actual file paths
    async fn expand_glob_pattern(&self, pattern: &str) -> Result<Vec<PathBuf>, Box<dyn std::error::Error + Send + Sync>> {
        let mut paths = Vec::new();
        
        // Simple glob expansion - in a real implementation you'd use a glob library
        if pattern.ends_with("/*") {
            let dir_path = &pattern[..pattern.len() - 2];
            let dir = Path::new(dir_path);
            
            if dir.exists() && dir.is_dir() {
                let mut entries = fs::read_dir(dir).await?;
                while let Some(entry) = entries.next_entry().await? {
                    if entry.file_type().await?.is_file() {
                        paths.push(entry.path());
                    }
                }
            }
        } else if pattern.ends_with("/*.conf") {
            let dir_path = &pattern[..pattern.len() - 7];
            let dir = Path::new(dir_path);
            
            if dir.exists() && dir.is_dir() {
                let mut entries = fs::read_dir(dir).await?;
                while let Some(entry) = entries.next_entry().await? {
                    let path = entry.path();
                    if path.extension().map_or(false, |ext| ext == "conf") {
                        paths.push(path);
                    }
                }
            }
        } else {
            // Direct file path
            let path = Path::new(pattern);
            if path.exists() {
                paths.push(path.to_path_buf());
            }
        }
        
        Ok(paths)
    }
}

#[async_trait]
impl ConfigDiscoveryService for FileSystemConfigDiscovery {
    async fn discover_configs(&self, pattern: Option<&str>) -> Result<Vec<WebServerConfig>, Box<dyn std::error::Error + Send + Sync>> {
        let mut configs = Vec::new();
        
        let patterns = if let Some(custom_pattern) = pattern {
            vec![custom_pattern]
        } else {
            self.get_common_config_patterns()
        };

        for pattern in patterns {
            let paths = self.expand_glob_pattern(pattern).await?;
            
            for path in paths {
                match self.detect_server_type_from_content(&path).await {
                    Ok(server_type) => {
                        let config = WebServerConfig::new(path, server_type);
                        configs.push(config);
                    }
                    Err(e) => {
                        eprintln!("Warning: Could not detect server type for {}: {}", path.display(), e);
                    }
                }
            }
        }
        
        Ok(configs)
    }

    async fn detect_server_type(&self, config_path: &Path) -> Result<WebServerType, DomainError> {
        self.detect_server_type_from_content(config_path).await
    }
}

impl Default for FileSystemConfigDiscovery {
    fn default() -> Self {
        Self::new()
    }
}