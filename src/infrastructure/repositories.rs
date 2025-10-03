use std::fs;
use std::net::IpAddr;
use std::path::PathBuf;

use async_trait::async_trait;
use tokio::fs as async_fs;

use crate::domain::entities::IpEntry;
use crate::domain::ports::IpRepository;

/// File-based IP repository implementation
pub struct FileIpRepository {
    storage_dir: PathBuf,
}

impl FileIpRepository {
    pub fn new(storage_dir: PathBuf) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        eprintln!(
            "DEBUG: FileIpRepository attempting to use storage_dir: {:?}",
            storage_dir
        );
        if !storage_dir.exists() {
            eprintln!(
                "DEBUG: Storage directory doesn't exist, creating: {:?}",
                storage_dir
            );
            match fs::create_dir_all(&storage_dir) {
                Ok(()) => eprintln!("DEBUG: Successfully created storage directory"),
                Err(e) => {
                    eprintln!("DEBUG: Failed to create storage directory: {}", e);
                    return Err(e.into());
                }
            }
        }

        Ok(Self { storage_dir })
    }

    fn get_file_path(&self, hostname: &str) -> PathBuf {
        self.storage_dir.join(format!("{}.json", hostname))
    }
}

#[async_trait]
impl IpRepository for FileIpRepository {
    async fn store_ip(
        &self,
        hostname: &str,
        ip: IpAddr,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let entry = IpEntry::new(ip, hostname.to_string(), None);
        let file_path = self.get_file_path(hostname);
        let absolute_path = std::fs::canonicalize(&file_path).unwrap_or_else(|_| {
            // If file doesn't exist, construct absolute path manually
            std::env::current_dir()
                .map(|cwd| cwd.join(&file_path))
                .unwrap_or_else(|_| file_path.clone())
        });
        eprintln!(
            "DEBUG: Storing IP {} for {} in JSON file: {}",
            ip,
            hostname,
            absolute_path.display()
        );
        let json = serde_json::to_string_pretty(&entry)?;
        async_fs::write(&file_path, json).await?;
        let final_absolute_path =
            std::fs::canonicalize(&file_path).unwrap_or_else(|_| file_path.clone());
        eprintln!(
            "DEBUG: Successfully stored JSON file at: {}",
            final_absolute_path.display()
        );
        Ok(())
    }

    async fn load_ip(
        &self,
        hostname: &str,
    ) -> Result<Option<IpAddr>, Box<dyn std::error::Error + Send + Sync>> {
        let file_path = self.get_file_path(hostname);
        let absolute_path = std::fs::canonicalize(&file_path).unwrap_or_else(|_| {
            // If file doesn't exist, construct absolute path manually
            std::env::current_dir()
                .map(|cwd| cwd.join(&file_path))
                .unwrap_or_else(|_| file_path.clone())
        });
        eprintln!(
            "DEBUG: Checking for existing JSON file: {}",
            absolute_path.display()
        );

        if !file_path.exists() {
            eprintln!(
                "DEBUG: JSON file does not exist: {}",
                absolute_path.display()
            );
            return Ok(None);
        }

        eprintln!(
            "DEBUG: Loading IP from JSON file: {}",
            absolute_path.display()
        );
        let content = async_fs::read_to_string(&file_path).await?;
        let entry: IpEntry = serde_json::from_str(&content)?;
        eprintln!(
            "DEBUG: Loaded IP {} from JSON file: {}",
            entry.ip,
            absolute_path.display()
        );
        Ok(Some(entry.ip))
    }

    async fn get_ip_entry(
        &self,
        hostname: &str,
    ) -> Result<Option<IpEntry>, Box<dyn std::error::Error + Send + Sync>> {
        let file_path = self.get_file_path(hostname);

        if !file_path.exists() {
            return Ok(None);
        }

        let content = async_fs::read_to_string(file_path).await?;
        let entry: IpEntry = serde_json::from_str(&content)?;
        Ok(Some(entry))
    }

    async fn list_all_entries(
        &self,
    ) -> Result<Vec<IpEntry>, Box<dyn std::error::Error + Send + Sync>> {
        let mut entries = Vec::new();
        let mut dir = async_fs::read_dir(&self.storage_dir).await?;

        while let Some(entry) = dir.next_entry().await? {
            if let Some(ext) = entry.path().extension() {
                if ext == "json" {
                    let content = async_fs::read_to_string(entry.path()).await?;
                    if let Ok(ip_entry) = serde_json::from_str::<IpEntry>(&content) {
                        entries.push(ip_entry);
                    }
                }
            }
        }

        entries.sort_by(|a, b| a.hostname.cmp(&b.hostname));
        Ok(entries)
    }

    async fn delete_entry(
        &self,
        hostname: &str,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let file_path = self.get_file_path(hostname);

        if file_path.exists() {
            async_fs::remove_file(file_path).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Initialize DNS host file if it doesn't exist yet (FileIpRepository implementation)
    async fn initialize_host_file(
        &self,
        hostname: &str,
        resolved_ip: IpAddr,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let file_path = self.get_file_path(hostname);

        // Only create if file doesn't exist
        if file_path.exists() {
            return Ok(false);
        }

        // Create an initial entry with the resolved IP (or placeholder if resolution failed)
        let comment = if resolved_ip.to_string() == "0.0.0.0" {
            "Initial DNS host file created at first startup (will be updated with real IP)"
                .to_string()
        } else {
            "Initial DNS host file created at first startup with resolved IP".to_string()
        };

        let initial_entry = IpEntry::new(
            resolved_ip, // Use the resolved IP (real or placeholder)
            hostname.to_string(),
            Some(comment),
        );

        let json = serde_json::to_string_pretty(&initial_entry)?;
        async_fs::write(&file_path, json).await?;

        Ok(true)
    }
}

/// In-memory IP repository for testing
pub struct InMemoryIpRepository {
    entries: std::sync::Arc<tokio::sync::RwLock<std::collections::HashMap<String, IpEntry>>>,
}

impl InMemoryIpRepository {
    pub fn new() -> Self {
        Self {
            entries: std::sync::Arc::new(
                tokio::sync::RwLock::new(std::collections::HashMap::new()),
            ),
        }
    }
}

#[async_trait]
impl IpRepository for InMemoryIpRepository {
    async fn store_ip(
        &self,
        hostname: &str,
        ip: IpAddr,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut entries = self.entries.write().await;
        let entry = IpEntry::new(ip, hostname.to_string(), None);
        entries.insert(hostname.to_string(), entry);
        Ok(())
    }

    async fn load_ip(
        &self,
        hostname: &str,
    ) -> Result<Option<IpAddr>, Box<dyn std::error::Error + Send + Sync>> {
        let entries = self.entries.read().await;
        Ok(entries.get(hostname).map(|entry| entry.ip))
    }

    async fn get_ip_entry(
        &self,
        hostname: &str,
    ) -> Result<Option<IpEntry>, Box<dyn std::error::Error + Send + Sync>> {
        let entries = self.entries.read().await;
        Ok(entries.get(hostname).cloned())
    }

    async fn list_all_entries(
        &self,
    ) -> Result<Vec<IpEntry>, Box<dyn std::error::Error + Send + Sync>> {
        let entries = self.entries.read().await;
        let mut result: Vec<IpEntry> = entries.values().cloned().collect();
        result.sort_by(|a, b| a.hostname.cmp(&b.hostname));
        Ok(result)
    }

    async fn delete_entry(
        &self,
        hostname: &str,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let mut entries = self.entries.write().await;
        Ok(entries.remove(hostname).is_some())
    }
}

impl Default for InMemoryIpRepository {
    fn default() -> Self {
        Self::new()
    }
}
