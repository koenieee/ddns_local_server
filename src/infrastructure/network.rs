use std::net::IpAddr;
use async_trait::async_trait;
use crate::domain::ports::NetworkService;

/// HTTP-based network service implementation
pub struct HttpNetworkService {
    client: reqwest::Client,
}

impl HttpNetworkService {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl NetworkService for HttpNetworkService {
    async fn get_public_ip(&self) -> Result<IpAddr, Box<dyn std::error::Error + Send + Sync>> {
        let endpoints = [
            "https://api.ipify.org",
            "https://ipinfo.io/ip", 
            "https://icanhazip.com",
        ];

        let mut last_error: Option<Box<dyn std::error::Error + Send + Sync>> = None;
        
        for endpoint in &endpoints {
            match self.client
                .get(*endpoint)
                .timeout(std::time::Duration::from_secs(10))
                .send()
                .await
            {
                Ok(response) => {
                    match response.text().await {
                        Ok(text) => {
                            let ip_str = text.trim();
                            match ip_str.parse::<IpAddr>() {
                                Ok(ip) => return Ok(ip),
                                Err(e) => {
                                    last_error = Some(Box::new(e));
                                }
                            }
                        }
                        Err(e) => {
                            last_error = Some(Box::new(e));
                        }
                    }
                }
                Err(e) => {
                    last_error = Some(Box::new(e));
                }
            }
        }

        Err(last_error.unwrap_or_else(|| {
            Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to get public IP from all endpoints",
            ))
        }))
    }

    async fn resolve_hostname(&self, hostname: &str) -> Result<Vec<IpAddr>, Box<dyn std::error::Error + Send + Sync>> {
        let addrs = tokio::net::lookup_host(format!("{}:80", hostname)).await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        let ips: Vec<IpAddr> = addrs.map(|addr| addr.ip()).collect();
        Ok(ips)
    }

    async fn is_reachable(&self, ip: IpAddr) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        // Simple HTTP connectivity check
        let url = format!("http://{}:80", ip);
        match self.client.get(&url)
            .timeout(std::time::Duration::from_secs(5))
            .send()
            .await
        {
            Ok(_) => Ok(true),
            Err(_) => Ok(false), // Not reachable or no HTTP service
        }
    }
}

impl Default for HttpNetworkService {
    fn default() -> Self {
        Self::new()
    }
}