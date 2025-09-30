pub mod cli;
pub mod config;
pub mod core;

pub use cli::*;
pub use config::*;
pub use core::*;

/// Main entry point for the DDNS updater
pub fn run() {
    let args = Args::parse_args();
    run_with_args(args);
}

/// Run DDNS updater with provided arguments
pub fn run_with_args(args: Args) {
    if args.verbose {
        println!("DDNS Updater - Nginx Allow List Manager (verbose mode)");
        println!("Host: {}", args.host);
        if let Some(config) = &args.nginx_config {
            println!("Config file: {}", config.display());
        }
        if let Some(config_dir) = &args.config_dir {
            println!("Config directory: {}", config_dir.display());
            println!("Pattern: {}", args.pattern);
        }
    } else {
        println!("DDNS Updater - Nginx Allow List Manager");
    }

    let host = &args.host;

    // Get nginx config paths from command line arguments (may be multiple when using --config-dir)
    let nginx_config_paths = match args.get_nginx_config_paths() {
        Ok(paths) => {
            if args.verbose {
                if paths.len() == 1 {
                    println!("Using nginx config: {}", paths[0].display());
                } else {
                    println!("Processing {} nginx config files", paths.len());
                }
            }
            paths
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    // Validate that all files are actually nginx config files
    for nginx_config_path in &nginx_config_paths {
        let config_path_str = nginx_config_path.to_string_lossy();
        if let Err(e) = validate_nginx_config_file(&config_path_str, args.verbose) {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }

    // Get current IP
    match get_host_ip(host) {
        Ok(current_ip) => {
            println!("Current IP for {}: {}", host, current_ip);

            let ip_file = get_ip_file_path(host);

            // Get the old IP BEFORE checking for changes (since check_and_update_ip will update the file)
            let old_ip = load_ip(&ip_file).ok();

            // Check if IP has changed
            match check_and_update_ip(current_ip, &ip_file) {
                Ok(changed) => {
                    if changed {
                        println!("IP address has changed! Updating nginx allow list...");

                        // Process each config file
                        let mut updated_files = 0;
                        let mut failed_files = 0;

                        for nginx_config_path in &nginx_config_paths {
                            let config_path_str = nginx_config_path.to_string_lossy();

                            if args.verbose && nginx_config_paths.len() > 1 {
                                println!("\nProcessing: {}", config_path_str);
                            }

                            // Get backup directory from args
                            let backup_result = match args.get_backup_dir() {
                                Ok(backup_dir) => {
                                    if args.verbose {
                                        println!(
                                            "Using backup directory: {}",
                                            backup_dir.display()
                                        );
                                    }
                                    backup_nginx_config_to_dir(&config_path_str, Some(&backup_dir))
                                }
                                Err(e) => {
                                    eprintln!("Warning: {}", e);
                                    eprintln!("Falling back to default backup location");
                                    backup_nginx_config(&config_path_str)
                                }
                            };

                            match backup_result {
                                Ok(backup_path) => {
                                    if args.verbose {
                                        println!("Config backed up to: {}", backup_path);
                                    }

                                    // Update nginx allow list
                                    match update_nginx_allow_ip(
                                        &config_path_str,
                                        old_ip,
                                        current_ip,
                                        Some(&format!("DDNS for {}", host)),
                                    ) {
                                        Ok(updated) => {
                                            if updated {
                                                updated_files += 1;
                                                if nginx_config_paths.len() == 1 {
                                                    println!("Nginx config updated successfully");
                                                } else if args.verbose {
                                                    println!("✓ Updated: {}", config_path_str);
                                                }
                                            } else if args.verbose {
                                                println!(
                                                    "- No changes needed: {}",
                                                    config_path_str
                                                );
                                            }
                                        }
                                        Err(e) => {
                                            failed_files += 1;
                                            eprintln!(
                                                "✗ Error updating {}: {}",
                                                config_path_str, e
                                            );
                                        }
                                    }
                                }
                                Err(e) => {
                                    failed_files += 1;
                                    eprintln!(
                                        "✗ Error creating backup for {}: {}",
                                        config_path_str, e
                                    );
                                }
                            }
                        }

                        // Summary for multiple files
                        if nginx_config_paths.len() > 1 {
                            println!("\nUpdate Summary:");
                            println!("  {} files updated", updated_files);
                            if failed_files > 0 {
                                println!("  {} files failed", failed_files);
                            }
                            println!("  {} files processed", nginx_config_paths.len());
                        }

                        // Reload nginx only once after processing all files
                        if updated_files > 0 {
                            if !args.no_reload {
                                reload_nginx_if_available();
                            } else if args.verbose {
                                println!("Nginx reload skipped (--no-reload specified)");
                            }
                        } else if nginx_config_paths.len() == 1 {
                            println!("No nginx config changes were needed");
                        }
                    } else {
                        println!("No IP change detected. Nginx config unchanged.");
                    }
                }
                Err(e) => println!("Error checking IP: {}", e),
            }
        }
        Err(e) => println!("Failed to get current IP: {}", e),
    }

    println!("DDNS update check complete.");
}
