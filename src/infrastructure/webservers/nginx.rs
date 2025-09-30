use std::net::IpAddr;
use std::path::PathBuf;
use std::process::Command;
use async_trait::async_trait;
use tokio::fs;
use regex::Regex;

use crate::domain::entities::{WebServerConfig, WebServerType};
use crate::domain::ports::WebServerHandler;

/// Nginx web server handler
pub struct NginxHandler;

impl NginxHandler {
    pub fn new() -> Self {
        Self
    }

    async fn backup_file(&self, config_path: &std::path::Path) -> Result<PathBuf, Box<dyn std::error::Error + Send + Sync>> {
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let backup_path = config_path.with_extension(format!("bak.{}", timestamp));
        fs::copy(config_path, &backup_path).await?;
        Ok(backup_path)
    }

    async fn update_nginx_config(
        &self,
        config_path: &std::path::Path,
        hostname: &str,
        old_ip: Option<IpAddr>,
        new_ip: IpAddr,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let content = fs::read_to_string(config_path).await?;
        let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        let mut updated = false;

        // Comment pattern for hostname identification
        let hostname_comment = format!("# DDNS: {}", hostname);
        
        // Remove old entries for this hostname
        if let Some(old_ip) = old_ip {
            let old_allow_pattern = format!("allow {};", old_ip);
            let has_hostname_comment = lines.iter().any(|l| l.contains(&hostname_comment));
            lines.retain(|line| {
                !line.trim().starts_with(&old_allow_pattern) || !has_hostname_comment
            });
        }

        // Find location blocks and add new allow rule
        let location_regex = Regex::new(r"^\s*location\s+.*\{\s*$")?;
        let server_regex = Regex::new(r"^\s*server\s*\{\s*$")?;
        
        for i in 0..lines.len() {
            if location_regex.is_match(&lines[i]) || server_regex.is_match(&lines[i]) {
                // Look for the next closing brace to find insertion point
                let mut brace_count = 1;
                for j in (i + 1)..lines.len() {
                    if lines[j].contains('{') {
                        brace_count += lines[j].matches('{').count();
                    }
                    if lines[j].contains('}') {
                        brace_count -= lines[j].matches('}').count();
                        if brace_count == 0 {
                            // Insert allow rule before closing brace
                            let indent = "    "; // Standard nginx indentation
                            lines.insert(j, format!("{}allow {}; {}", indent, new_ip, hostname_comment));
                            updated = true;
                            break;
                        }
                    }
                }
                break;
            }
        }

        if updated {
            let new_content = lines.join("\n");
            fs::write(config_path, new_content).await?;
        }

        Ok(updated)
    }
}

#[async_trait]
impl WebServerHandler for NginxHandler {
    async fn update_allow_list(
        &self,
        config: &WebServerConfig,
        hostname: &str,
        old_ip: Option<IpAddr>,
        new_ip: IpAddr,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        self.update_nginx_config(&config.path, hostname, old_ip, new_ip).await
    }

    async fn validate_config(&self, config: &WebServerConfig) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        if !config.path.exists() {
            return Ok(false);
        }

        let output = Command::new("nginx")
            .arg("-t")
            .arg("-c")
            .arg(&config.path)
            .output()?;

        Ok(output.status.success())
    }

    async fn reload_server(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let output = Command::new("systemctl")
            .arg("reload")
            .arg("nginx")
            .output()?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to reload nginx: {}", error).into());
        }

        Ok(())
    }

    async fn create_backup(&self, config: &WebServerConfig) -> Result<PathBuf, Box<dyn std::error::Error + Send + Sync>> {
        self.backup_file(&config.path).await
    }

    async fn test_configuration(&self, config: &WebServerConfig) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        self.validate_config(config).await
    }

    fn server_type(&self) -> WebServerType {
        WebServerType::Nginx
    }
}

impl Default for NginxHandler {
    fn default() -> Self {
        Self::new()
    }
}