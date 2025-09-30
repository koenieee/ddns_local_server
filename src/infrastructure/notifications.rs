use std::net::IpAddr;
use async_trait::async_trait;

use crate::domain::ports::NotificationService;

/// Console-based notification service
pub struct ConsoleNotificationService {
    verbose: bool,
}

impl ConsoleNotificationService {
    pub fn new(verbose: bool) -> Self {
        Self { verbose }
    }
}

#[async_trait]
impl NotificationService for ConsoleNotificationService {
    async fn notify_ip_change(
        &self,
        hostname: &str,
        old_ip: Option<IpAddr>,
        new_ip: IpAddr,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        match old_ip {
            Some(old) => {
                println!("✅ IP updated for {}: {} → {}", hostname, old, new_ip);
                if self.verbose {
                    println!("   Changed at: {}", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"));
                }
            }
            None => {
                println!("✅ New IP registered for {}: {}", hostname, new_ip);
                if self.verbose {
                    println!("   Registered at: {}", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"));
                }
            }
        }
        Ok(())
    }

    async fn notify_error(
        &self,
        error: &str,
        context: Option<&str>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        match context {
            Some(ctx) => eprintln!("❌ Error in {}: {}", ctx, error),
            None => eprintln!("❌ Error: {}", error),
        }
        Ok(())
    }
}

/// Log-based notification service (writes to system log)
pub struct LogNotificationService;

impl LogNotificationService {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl NotificationService for LogNotificationService {
    async fn notify_ip_change(
        &self,
        hostname: &str,
        old_ip: Option<IpAddr>,
        new_ip: IpAddr,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let message = match old_ip {
            Some(old) => format!("DDNS IP updated for {}: {} -> {}", hostname, old, new_ip),
            None => format!("DDNS new IP registered for {}: {}", hostname, new_ip),
        };
        
        // In a real implementation, this would use a proper logging library
        // For now, we'll write to stderr with a timestamp
        eprintln!("[{}] INFO: {}", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"), message);
        Ok(())
    }

    async fn notify_error(
        &self,
        error: &str,
        context: Option<&str>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let message = match context {
            Some(ctx) => format!("DDNS error in {}: {}", ctx, error),
            None => format!("DDNS error: {}", error),
        };
        
        eprintln!("[{}] ERROR: {}", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"), message);
        Ok(())
    }
}

impl Default for LogNotificationService {
    fn default() -> Self {
        Self::new()
    }
}

/// Composite notification service that can send to multiple services
pub struct CompositeNotificationService {
    services: Vec<Box<dyn NotificationService>>,
}

impl CompositeNotificationService {
    pub fn new() -> Self {
        Self {
            services: Vec::new(),
        }
    }

    pub fn add_service(mut self, service: Box<dyn NotificationService>) -> Self {
        self.services.push(service);
        self
    }
}

#[async_trait]
impl NotificationService for CompositeNotificationService {
    async fn notify_ip_change(
        &self,
        hostname: &str,
        old_ip: Option<IpAddr>,
        new_ip: IpAddr,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        for service in &self.services {
            if let Err(e) = service.notify_ip_change(hostname, old_ip, new_ip).await {
                eprintln!("Warning: Notification service failed: {}", e);
            }
        }
        Ok(())
    }

    async fn notify_error(
        &self,
        error: &str,
        context: Option<&str>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        for service in &self.services {
            if let Err(e) = service.notify_error(error, context).await {
                eprintln!("Warning: Notification service failed: {}", e);
            }
        }
        Ok(())
    }
}

impl Default for CompositeNotificationService {
    fn default() -> Self {
        Self::new()
    }
}