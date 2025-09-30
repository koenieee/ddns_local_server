use std::path::PathBuf;
use tokio::runtime::Runtime;

use crate::application::{AppConfig, DdnsApplication};
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
        let app_config = AppConfig::new()
            .with_verbose(args.verbose)
            .with_storage_dir(PathBuf::from("/var/lib/ddns-updater"));

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
        let results = app.update_ddns_multiple(&args.host, config_paths).await?;

        // Display results
        Self::display_results(&args.host, &results, args.verbose).await;

        Ok(())
    }

    /// Display the results of DDNS updates
    async fn display_results(hostname: &str, results: &[UpdateResult], verbose: bool) {
        if results.is_empty() {
            println!("No configurations were updated.");
            return;
        }

        let mut updated_count = 0;
        let mut no_change_count = 0;

        for result in results {
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

        println!("\n📊 Summary:");
        println!("   Updated: {}", updated_count);
        println!("   No change: {}", no_change_count);
        println!("   Total processed: {}", results.len());

        if updated_count > 0 {
            println!("\n🔥 Server configuration updated and reloaded successfully!");
        }
    }
}
