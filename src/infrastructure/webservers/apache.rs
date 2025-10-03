use async_trait::async_trait;
use regex::Regex;
use std::net::IpAddr;
use std::path::PathBuf;
use std::process::Command;
use tokio::fs;

use crate::domain::entities::{WebServerConfig, WebServerType};
use crate::domain::ports::WebServerHandler;

/// Apache web server handler
pub struct ApacheHandler;

impl ApacheHandler {
    pub fn new() -> Self {
        Self
    }

    async fn backup_file(
        &self,
        config_path: &std::path::Path,
    ) -> Result<PathBuf, Box<dyn std::error::Error + Send + Sync>> {
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let backup_path = config_path.with_extension(format!("bak.{}", timestamp));
        fs::copy(config_path, &backup_path).await?;
        Ok(backup_path)
    }

    async fn update_apache_config(
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
            let old_require_pattern = format!("Require ip {}", old_ip);
            let has_hostname_comment = lines.iter().any(|l| l.contains(&hostname_comment));
            lines.retain(|line| {
                !line.trim().starts_with(&old_require_pattern) || !has_hostname_comment
            });
        }

        // Find Directory or Location blocks and add new Require rule
        let directory_regex = Regex::new(r"^\s*<Directory\s+.*>\s*$")?;
        let location_regex = Regex::new(r"^\s*<Location\s+.*>\s*$")?;
        let virtualhost_regex = Regex::new(r"^\s*<VirtualHost\s+.*>\s*$")?;

        for i in 0..lines.len() {
            if directory_regex.is_match(&lines[i])
                || location_regex.is_match(&lines[i])
                || virtualhost_regex.is_match(&lines[i])
            {
                // Find the corresponding closing tag
                let tag_name = if lines[i].contains("<Directory") {
                    "Directory"
                } else if lines[i].contains("<Location") {
                    "Location"
                } else {
                    "VirtualHost"
                };

                let closing_tag = format!("</{}>", tag_name);

                for j in (i + 1)..lines.len() {
                    if lines[j].contains(&closing_tag) {
                        // Insert Require rule before closing tag
                        let indent = "    "; // Standard Apache indentation
                        lines.insert(
                            j,
                            format!("{}Require ip {} {}", indent, new_ip, hostname_comment),
                        );
                        updated = true;
                        break;
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
impl WebServerHandler for ApacheHandler {
    async fn update_allow_list(
        &self,
        config: &WebServerConfig,
        hostname: &str,
        old_ip: Option<IpAddr>,
        new_ip: IpAddr,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        self.update_apache_config(&config.path, hostname, old_ip, new_ip)
            .await
    }

    async fn validate_config(
        &self,
        config: &WebServerConfig,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        if !config.path.exists() {
            return Ok(false);
        }

        // Try both common Apache command names
        let commands = ["apache2ctl", "apachectl", "httpd"];

        for cmd in &commands {
            if let Ok(output) = Command::new(cmd)
                .arg("-t")
                .arg("-f")
                .arg(&config.path)
                .output()
            {
                return Ok(output.status.success());
            }
        }

        Err("Apache command not found (tried apache2ctl, apachectl, httpd)".into())
    }

    async fn reload_server(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Try different service names for Apache
        let services = ["apache2", "httpd"];

        for service in &services {
            if let Ok(output) = Command::new("systemctl")
                .arg("reload")
                .arg(service)
                .output()
            {
                if output.status.success() {
                    return Ok(());
                }
            }
        }

        Err("Failed to reload Apache (tried apache2, httpd services)".into())
    }

    async fn create_backup(
        &self,
        config: &WebServerConfig,
    ) -> Result<PathBuf, Box<dyn std::error::Error + Send + Sync>> {
        self.backup_file(&config.path).await
    }

    async fn test_configuration(
        &self,
        config: &WebServerConfig,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        self.validate_config(config).await
    }

    async fn check_ip_in_config(
        &self,
        config: &WebServerConfig,
        ip: IpAddr,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let content = fs::read_to_string(&config.path).await?;
        let ip_str = ip.to_string();

        // Check if the IP exists in any Allow directive (Apache format)
        for line in content.lines() {
            let trimmed = line.trim();
            // Apache uses "Allow from" or "Require ip" directives
            if (trimmed.starts_with("Allow from ") || trimmed.starts_with("Require ip "))
                && trimmed.contains(&ip_str)
            {
                eprintln!(
                    "DEBUG: Found IP {} in Apache config line: {}",
                    ip_str, trimmed
                );
                return Ok(true);
            }
        }

        eprintln!("DEBUG: IP {} not found in Apache config file", ip_str);
        Ok(false)
    }

    fn server_type(&self) -> WebServerType {
        WebServerType::Apache
    }
}

impl Default for ApacheHandler {
    fn default() -> Self {
        Self::new()
    }
}
