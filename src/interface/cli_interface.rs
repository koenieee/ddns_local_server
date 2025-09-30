use std::path::PathBuf;
use tokio::runtime::Runtime;

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
        // Check if we can actually write to /var/lib by testing directory creation
        let can_use_var_lib = match std::fs::create_dir_all("/var/lib/ddns-updater-test") {
            Ok(()) => {
                // Clean up test directory
                let _ = std::fs::remove_dir("/var/lib/ddns-updater-test");
                true
            }
            Err(_) => false,
        };

        let storage_dir = if std::env::var("DDNS_TEST_MODE").is_ok() || !can_use_var_lib {
            // Use local directory for tests or when /var/lib is not writable
            let local_dir = PathBuf::from("./test_storage");
            if args.verbose {
                println!("Using local storage directory: {}", local_dir.display());
            }
            local_dir
        } else {
            PathBuf::from("/var/lib/ddns-updater")
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
}
