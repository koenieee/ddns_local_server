use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "ddns_updater")]
#[command(about = "A DDNS updater that manages nginx allow lists")]
#[command(version = "0.1.0")]
pub struct Args {
    /// Host to check for IP changes
    #[arg(long, default_value = "google.com")]
    pub host: String,

    /// Path to nginx configuration file or directory
    #[arg(short = 'c', long = "config")]
    pub nginx_config: Option<PathBuf>,

    /// Directory containing nginx configuration files
    #[arg(short = 'd', long = "config-dir")]
    pub config_dir: Option<PathBuf>,

    /// Pattern to match nginx config files (used with --config-dir)
    #[arg(short = 'p', long = "pattern", default_value = "*.conf")]
    pub pattern: String,

    /// Directory to store backup files
    #[arg(short = 'b', long = "backup-dir")]
    pub backup_dir: Option<PathBuf>,

    /// Don't reload nginx after updating configuration
    #[arg(long = "no-reload")]
    pub no_reload: bool,

    /// Verbose output
    #[arg(short, long)]
    pub verbose: bool,
}

impl Args {
    pub fn parse_args() -> Self {
        Args::parse()
    }

    /// Get the nginx config path, either from explicit config or from config directory
    pub fn get_nginx_config_path(&self) -> Result<PathBuf, String> {
        if let Some(config) = &self.nginx_config {
            if config.exists() {
                Ok(config.clone())
            } else {
                Err(format!("Nginx config file not found: {}", config.display()))
            }
        } else if let Some(config_dir) = &self.config_dir {
            if !config_dir.exists() {
                return Err(format!(
                    "Config directory not found: {}",
                    config_dir.display()
                ));
            }
            if !config_dir.is_dir() {
                return Err(format!("Path is not a directory: {}", config_dir.display()));
            }

            // Find the first matching config file in the directory
            let pattern = &self.pattern;
            match self.find_config_files(config_dir, pattern) {
                Ok(files) => {
                    if files.is_empty() {
                        Err(format!(
                            "No config files matching '{}' found in {}",
                            pattern,
                            config_dir.display()
                        ))
                    } else {
                        if self.verbose && files.len() > 1 {
                            println!("Multiple config files found, using: {}", files[0].display());
                            for file in &files[1..] {
                                println!("  Also found: {}", file.display());
                            }
                        }
                        Ok(files[0].clone())
                    }
                }
                Err(e) => Err(e),
            }
        } else {
            // Default to test config for development
            Ok(PathBuf::from("test_nginx_no_comment.conf"))
        }
    }

    /// Get ALL nginx config paths (for processing multiple files in a directory)
    pub fn get_nginx_config_paths(&self) -> Result<Vec<PathBuf>, String> {
        if let Some(config) = &self.nginx_config {
            if config.exists() {
                Ok(vec![config.clone()])
            } else {
                Err(format!("Nginx config file not found: {}", config.display()))
            }
        } else if let Some(config_dir) = &self.config_dir {
            if !config_dir.exists() {
                return Err(format!(
                    "Config directory not found: {}",
                    config_dir.display()
                ));
            }
            if !config_dir.is_dir() {
                return Err(format!("Path is not a directory: {}", config_dir.display()));
            }

            // Find ALL matching config files in the directory
            let pattern = &self.pattern;
            match self.find_config_files(config_dir, pattern) {
                Ok(files) => {
                    if files.is_empty() {
                        Err(format!(
                            "No config files matching '{}' found in {}",
                            pattern,
                            config_dir.display()
                        ))
                    } else {
                        if self.verbose {
                            println!("Found {} config files to process:", files.len());
                            for file in &files {
                                println!("  {}", file.display());
                            }
                        }
                        Ok(files)
                    }
                }
                Err(e) => Err(e),
            }
        } else {
            // Default to test config for development
            Ok(vec![PathBuf::from("test_nginx_no_comment.conf")])
        }
    }

    /// Find config files matching the pattern in the given directory
    fn find_config_files(&self, dir: &PathBuf, pattern: &str) -> Result<Vec<PathBuf>, String> {
        use std::fs;

        let entries = fs::read_dir(dir)
            .map_err(|e| format!("Failed to read directory {}: {}", dir.display(), e))?;

        let mut config_files = Vec::new();
        let mut skipped_files = Vec::new();

        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let path = entry.path();

            if path.is_file()
                && let Some(filename) = path.file_name().and_then(|n| n.to_str())
                && self.matches_pattern(filename, pattern)
            {
                // Validate that it's actually an nginx config file
                match crate::is_nginx_config_file(&path.to_string_lossy()) {
                    Ok(true) => {
                        config_files.push(path);
                    }
                    Ok(false) => {
                        skipped_files.push((path, "not an nginx config file".to_string()));
                    }
                    Err(e) => {
                        skipped_files.push((path, format!("validation error: {}", e)));
                    }
                }
            }
        }

        // Show skipped files in verbose mode
        if self.verbose && !skipped_files.is_empty() {
            println!("Skipped files that don't appear to be nginx configs:");
            for (path, reason) in &skipped_files {
                println!("  {} ({})", path.display(), reason);
            }
        }

        config_files.sort();
        Ok(config_files)
    }

    /// Simple pattern matching (supports * wildcard)
    fn matches_pattern(&self, filename: &str, pattern: &str) -> bool {
        if pattern == "*" {
            return true;
        }

        if pattern.contains('*') {
            let parts: Vec<&str> = pattern.split('*').collect();
            if parts.len() == 2 {
                let prefix = parts[0];
                let suffix = parts[1];
                return filename.starts_with(prefix) && filename.ends_with(suffix);
            }
        }

        filename == pattern
    }

    /// Get the backup directory path, creating it if necessary
    pub fn get_backup_dir(&self) -> Result<PathBuf, String> {
        let backup_path = if let Some(backup_dir) = &self.backup_dir {
            backup_dir.clone()
        } else {
            // Default to current directory + "backups"
            PathBuf::from("backups")
        };

        // Create the backup directory if it doesn't exist
        if !backup_path.exists() {
            std::fs::create_dir_all(&backup_path).map_err(|e| {
                format!(
                    "Failed to create backup directory {}: {}",
                    backup_path.display(),
                    e
                )
            })?;

            if self.verbose {
                println!("Created backup directory: {}", backup_path.display());
            }
        } else if !backup_path.is_dir() {
            return Err(format!(
                "Backup path exists but is not a directory: {}",
                backup_path.display()
            ));
        }

        Ok(backup_path)
    }
}
