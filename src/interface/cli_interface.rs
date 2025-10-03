use std::path::PathBuf;
use tokio::runtime::Runtime;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

use crate::application::{AppConfig, DdnsApplication, MultiConfigResult};
use crate::domain::services::UpdateResult;

/// CLI interface for the DDNS updater using clean architecture
pub struct CliInterface;

impl CliInterface {
    /// Main CLI entry point
    pub fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let args = crate::cli::Args::parse_args();

        // Create async runtime
        let rt = Runtime::new()?;

        rt.block_on(async { Self::run_async(args).await })
    }

    /// Async implementation of the CLI logic
    async fn run_async(
        args: crate::cli::Args,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Create application configuration
        // Check if we can actually use /var/lib/ddns-updater
        let can_use_var_lib = {
            let ddns_storage_dir = std::path::Path::new("/var/lib/ddns-updater");
            if ddns_storage_dir.exists() {
                // Directory exists, test if we can write to it
                match std::fs::create_dir_all(ddns_storage_dir) {
                    Ok(()) => {
                        // Try to create a temporary test file to verify write access
                        let test_file = ddns_storage_dir.join(".write_test");
                        match std::fs::write(&test_file, "test") {
                            Ok(()) => {
                                let _ = std::fs::remove_file(&test_file);
                                true
                            }
                            Err(_) => false,
                        }
                    }
                    Err(_) => false,
                }
            } else {
                // Directory doesn't exist, try to create it
                match std::fs::create_dir_all(ddns_storage_dir) {
                    Ok(()) => true,
                    Err(_) => false,
                }
            }
        };

        let storage_dir = if std::env::var("DDNS_TEST_MODE").is_ok() {
            // Use environment variable or local directory for tests
            let local_dir = if let Ok(test_storage_dir) = std::env::var("DDNS_STORAGE_DIR") {
                PathBuf::from(test_storage_dir)
            } else {
                PathBuf::from("./test_storage")
            };
            if args.verbose {
                println!("Using test storage directory: {}", local_dir.display());
            }
            local_dir
        } else if can_use_var_lib {
            // Production: use /var/lib/ddns-updater
            if args.verbose {
                println!("Using persistent storage directory: /var/lib/ddns-updater");
            }
            PathBuf::from("/var/lib/ddns-updater")
        } else {
            // Cannot use /var/lib/ddns-updater - this is a configuration issue
            eprintln!("ERROR: Cannot access /var/lib/ddns-updater for persistent storage.");
            let ddns_dir = std::path::Path::new("/var/lib/ddns-updater");
            if ddns_dir.exists() {
                eprintln!("Directory exists but is not writable by the current user.");
                #[cfg(unix)]
                if let Ok(metadata) = std::fs::metadata(ddns_dir) {
                    eprintln!(
                        "Directory permissions: {:o}",
                        metadata.permissions().mode() & 0o777
                    );
                }
            } else {
                eprintln!("Directory does not exist.");
            }
            eprintln!("This directory must be created and writable by the service user.");
            eprintln!("Please run: sudo mkdir -p /var/lib/ddns-updater && sudo chmod 755 /var/lib/ddns-updater");
            eprintln!("Or install using the systemd installation script which creates this directory automatically.");
            std::process::exit(1);
        };

        // Determine backup directory
        let backup_dir = if let Some(dir) = args.backup_dir.as_ref() {
            Some(dir.clone())
        } else if storage_dir.starts_with("./") {
            // For tests or when using local storage, use a local backup directory
            Some(std::path::PathBuf::from("./test_backups"))
        } else {
            None // Use default behavior (same directory as config)
        };

        let app_config = AppConfig::new()
            .with_verbose(args.verbose)
            .with_storage_dir(storage_dir)
            .with_backup_dir(backup_dir)
            .with_no_reload(args.no_reload);

        // Create application instance
        let app = DdnsApplication::new(app_config)?;

        // Initialize DNS host file on first startup (if it doesn't exist yet)
        if let Err(e) = Self::initialize_dns_host_file(&app, &args).await {
            if args.verbose {
                eprintln!("Warning: Failed to initialize DNS host file: {}", e);
            }
            // Don't exit on initialization failure - just continue
        }

        if args.verbose {
            println!("DDNS Updater - Multi-Server Allow List Manager (verbose mode)");
            println!("Host: {}", args.host);
        } else {
            println!("DDNS Updater - Multi-Server Allow List Manager");
        }

        // Get configuration paths
        let config_paths = match args.get_nginx_config_paths() {
            Ok(paths) => {
                if args.verbose {
                    if paths.len() == 1 {
                        println!("Using configuration: {}", paths[0].display());
                    } else {
                        println!("Processing {} configuration files", paths.len());
                    }
                }
                paths
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        };

        // Execute DDNS update for all configurations
        let multi_result = app.update_ddns_multiple(&args.host, config_paths).await?;

        // Display results
        Self::display_results(&args.host, &multi_result, args.verbose).await;

        // Exit with error code if there were configuration errors
        if multi_result.has_errors() {
            std::process::exit(1);
        }

        Ok(())
    }

    /// Display the results of DDNS updates
    async fn display_results(hostname: &str, multi_result: &MultiConfigResult, verbose: bool) {
        // Display errors first
        for (config_path, error) in &multi_result.errors {
            println!("❌ Error in config: {}: {}", config_path.display(), error);
        }

        if multi_result.successes.is_empty() && multi_result.errors.is_empty() {
            println!("No configurations were processed.");
            return;
        }

        let mut updated_count = 0;
        let mut no_change_count = 0;

        for result in &multi_result.successes {
            match result {
                UpdateResult::Updated {
                    old_ip,
                    new_ip,
                    backup_path,
                    ..
                } => {
                    updated_count += 1;
                    match old_ip {
                        Some(old) => {
                            println!("✅ Updated {}: {} → {}", hostname, old, new_ip);
                        }
                        None => {
                            println!("✅ Added {}: {}", hostname, new_ip);
                        }
                    }
                    if verbose {
                        println!("   Backup created: {}", backup_path.display());
                    }
                }
                UpdateResult::NoChange { ip } => {
                    no_change_count += 1;
                    if verbose {
                        println!("ℹ️  No change needed for {}: {}", hostname, ip);
                    }
                }
            }
        }

        if multi_result.successes.is_empty() && !multi_result.errors.is_empty() {
            println!("No configurations were updated.");
        } else if !multi_result.successes.is_empty() {
            println!("\n📊 Summary:");
            println!("   Updated: {}", updated_count);
            println!("   No change: {}", no_change_count);
            println!("   Errors: {}", multi_result.errors.len());
            println!("   Total processed: {}", multi_result.total_processed());

            if updated_count > 0 {
                println!("\n🔥 Server configuration updated and reloaded successfully!");
            }
        }
    }

    /// Initialize DNS host file if it doesn't exist yet
    /// This creates a placeholder JSON file for first-time setup
    async fn initialize_dns_host_file(
        app: &DdnsApplication,
        args: &crate::cli::Args,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Only initialize in production mode (not in test mode)
        if std::env::var("DDNS_TEST_MODE").is_ok() {
            return Ok(());
        }

        // Get the IP repository from the application to call the initialization method
        match app.initialize_host_file(&args.host).await {
            Ok(was_created) => {
                if was_created && args.verbose {
                    println!("🆕 Created initial DNS host file for: {}", args.host);
                    println!("   Location: /var/lib/ddns-updater/{}.json", args.host);
                    println!("   The file will be updated with the actual IP on first run.");
                }
            }
            Err(e) => {
                return Err(e);
            }
        }

        Ok(())
    }
}
