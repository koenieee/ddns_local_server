use std::fmt;
use std::path::PathBuf;

/// Value object for configuration paths with validation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigPath {
    path: PathBuf,
}

impl ConfigPath {
    pub fn new(path: PathBuf) -> Result<Self, ConfigPathError> {
        if !path.exists() {
            return Err(ConfigPathError::NotFound(path));
        }

        if !path.is_file() {
            return Err(ConfigPathError::NotAFile(path));
        }

        Ok(Self { path })
    }

    pub fn as_path(&self) -> &std::path::Path {
        &self.path
    }

    pub fn into_path_buf(self) -> PathBuf {
        self.path
    }
}

impl fmt::Display for ConfigPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.path.display())
    }
}

/// Hostname value object with validation
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Hostname {
    value: String,
}

impl Hostname {
    pub fn new(value: String) -> Result<Self, HostnameError> {
        if value.is_empty() {
            return Err(HostnameError::Empty);
        }

        if value.len() > 253 {
            return Err(HostnameError::TooLong(value.len()));
        }

        // Basic hostname validation (RFC compliant validation would be more complex)
        if !value
            .chars()
            .all(|c| c.is_alphanumeric() || c == '.' || c == '-' || c == '_')
        {
            return Err(HostnameError::InvalidCharacters(value));
        }

        Ok(Self { value })
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }

    pub fn into_string(self) -> String {
        self.value
    }
}

impl fmt::Display for Hostname {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl std::str::FromStr for Hostname {
    type Err = HostnameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_string())
    }
}

/// Backup retention policy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackupRetention {
    pub max_backups: u16,
    pub max_age_days: u16,
}

impl Default for BackupRetention {
    fn default() -> Self {
        Self {
            max_backups: 10,
            max_age_days: 30,
        }
    }
}

impl BackupRetention {
    pub fn new(max_backups: u16, max_age_days: u16) -> Result<Self, BackupRetentionError> {
        if max_backups == 0 {
            return Err(BackupRetentionError::InvalidBackupCount);
        }

        if max_age_days == 0 {
            return Err(BackupRetentionError::InvalidAgeDays);
        }

        Ok(Self {
            max_backups,
            max_age_days,
        })
    }
}

/// Configuration path errors
#[derive(Debug, Clone)]
pub enum ConfigPathError {
    NotFound(PathBuf),
    NotAFile(PathBuf),
    PermissionDenied(PathBuf),
}

impl fmt::Display for ConfigPathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigPathError::NotFound(path) => {
                write!(f, "Configuration file not found: {}", path.display())
            }
            ConfigPathError::NotAFile(path) => {
                write!(f, "Path is not a regular file: {}", path.display())
            }
            ConfigPathError::PermissionDenied(path) => {
                write!(f, "Permission denied accessing: {}", path.display())
            }
        }
    }
}

impl std::error::Error for ConfigPathError {}

/// Hostname validation errors
#[derive(Debug, Clone)]
pub enum HostnameError {
    Empty,
    TooLong(usize),
    InvalidCharacters(String),
}

impl fmt::Display for HostnameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HostnameError::Empty => write!(f, "Hostname cannot be empty"),
            HostnameError::TooLong(len) => {
                write!(f, "Hostname too long: {} characters (max 253)", len)
            }
            HostnameError::InvalidCharacters(hostname) => {
                write!(f, "Invalid characters in hostname: {}", hostname)
            }
        }
    }
}

impl std::error::Error for HostnameError {}

/// Backup retention configuration errors
#[derive(Debug, Clone)]
pub enum BackupRetentionError {
    InvalidBackupCount,
    InvalidAgeDays,
}

impl fmt::Display for BackupRetentionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BackupRetentionError::InvalidBackupCount => {
                write!(f, "Backup count must be greater than 0")
            }
            BackupRetentionError::InvalidAgeDays => write!(f, "Age in days must be greater than 0"),
        }
    }
}

impl std::error::Error for BackupRetentionError {}
