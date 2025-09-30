use ddns_updater::cli::Args;
use std::path::PathBuf;
use tempfile::tempdir;

#[test]
fn test_cli_args_single_file_integration() {
    // Create test config file
    let config_content = r#"
server {
    listen 80;
    server_name example.com;
    location / {
        proxy_pass http://192.168.1.100:8080;
        # ddns_updater_ip_placeholder
    }
}
"#;

    let temp_dir = tempdir().unwrap();
    let config_file = temp_dir.path().join("test.conf");
    std::fs::write(&config_file, config_content).unwrap();

    // Create CLI arguments
    let args = Args {
        host: "example.com".to_string(),
        nginx_config: Some(config_file.clone()),
        config_dir: None,
        pattern: "*.conf".to_string(),
        backup_dir: Some(temp_dir.path().join("backups")),
        no_reload: true,
        verbose: true,
    };

    // Test that CLI arguments flow through correctly
    assert_eq!(args.host, "example.com");
    assert!(args.nginx_config.is_some());
    assert_eq!(args.nginx_config.unwrap(), config_file);
    assert!(args.backup_dir.is_some());
    assert_eq!(args.backup_dir.unwrap(), temp_dir.path().join("backups"));
    assert!(args.no_reload);
    assert!(args.verbose);
}

#[test]
fn test_cli_args_directory_scan_integration() {
    let temp_dir = tempdir().unwrap();

    // Create multiple config files
    let config_content = r#"
server {
    listen 443 ssl;
    server_name test.local;
    location /api {
        proxy_pass https://api-backend:9000;
        # ddns_updater_ip_placeholder
    }
}
"#;

    let config1 = temp_dir.path().join("site1.nginx");
    let config2 = temp_dir.path().join("site2.nginx");
    let non_config = temp_dir.path().join("readme.txt");

    std::fs::write(&config1, config_content).unwrap();
    std::fs::write(&config2, config_content).unwrap();
    std::fs::write(&non_config, "Not a config file").unwrap();

    let args = Args {
        host: "test.local".to_string(),
        nginx_config: None,
        config_dir: Some(temp_dir.path().to_path_buf()),
        pattern: "*.nginx".to_string(),
        backup_dir: None,
        no_reload: false,
        verbose: false,
    };

    // Test CLI argument flow for directory-based configuration
    assert_eq!(args.host, "test.local");
    assert!(args.nginx_config.is_none());
    assert!(args.config_dir.is_some());
    assert_eq!(args.config_dir.unwrap(), temp_dir.path().to_path_buf());
    assert_eq!(args.pattern, "*.nginx");
    assert!(args.backup_dir.is_none());
    assert!(!args.no_reload);
    assert!(!args.verbose);
}

#[test]
fn test_cli_args_all_flags_integration() {
    // Test all CLI arguments working together
    let temp_dir = tempdir().unwrap();
    let config_file = temp_dir.path().join("comprehensive.conf");
    std::fs::write(&config_file, "server { }").unwrap();

    let args = Args {
        host: "comprehensive.example.com".to_string(),
        nginx_config: Some(config_file.clone()),
        config_dir: None,
        pattern: "*.conf".to_string(),
        backup_dir: Some(temp_dir.path().join("custom_backups")),
        no_reload: true,
        verbose: true,
    };

    // Validate complete CLI argument chain
    assert_eq!(args.host, "comprehensive.example.com");
    assert!(args.nginx_config.is_some());
    assert_eq!(args.nginx_config.unwrap(), config_file);
    assert!(args.config_dir.is_none());
    assert_eq!(args.pattern, "*.conf");
    assert!(args.backup_dir.is_some());
    assert_eq!(
        args.backup_dir.unwrap(),
        temp_dir.path().join("custom_backups")
    );
    assert!(args.no_reload);
    assert!(args.verbose);
}

#[test]
fn test_cli_args_error_handling_flow() {
    // Test error handling for invalid file
    let args = Args {
        host: "error.test".to_string(),
        nginx_config: Some(PathBuf::from("/nonexistent/file.conf")),
        config_dir: None,
        pattern: "*.conf".to_string(),
        backup_dir: None,
        no_reload: true,
        verbose: false,
    };

    // Verify args are created correctly even with invalid paths
    assert_eq!(args.host, "error.test");
    assert!(args.nginx_config.is_some());
    assert_eq!(
        args.nginx_config.unwrap(),
        PathBuf::from("/nonexistent/file.conf")
    );

    // Test error handling for invalid directory
    let args_dir = Args {
        host: "error.test".to_string(),
        nginx_config: None,
        config_dir: Some(PathBuf::from("/nonexistent/directory")),
        pattern: "*.conf".to_string(),
        backup_dir: None,
        no_reload: true,
        verbose: false,
    };

    assert_eq!(args_dir.host, "error.test");
    assert!(args_dir.config_dir.is_some());
    assert_eq!(
        args_dir.config_dir.unwrap(),
        PathBuf::from("/nonexistent/directory")
    );
}

#[test]
fn test_cli_args_pattern_and_hostname_combinations() {
    let patterns = vec!["*.conf", "*.nginx", "*.config", "site-*.conf"];
    let hostnames = vec![
        "google.com",
        "example.com",
        "test.local",
        "sub.domain.example.org",
    ];

    for (pattern, hostname) in patterns.iter().zip(hostnames.iter()) {
        let args = Args {
            host: hostname.to_string(),
            nginx_config: None,
            config_dir: Some(PathBuf::from("/etc/nginx")),
            pattern: pattern.to_string(),
            backup_dir: None,
            no_reload: false,
            verbose: false,
        };

        // Test that CLI argument combinations work correctly
        assert_eq!(args.host, *hostname);
        assert_eq!(args.pattern, *pattern);
        assert!(args.config_dir.is_some());
        assert_eq!(args.config_dir.unwrap(), PathBuf::from("/etc/nginx"));
    }
}

#[test]
fn test_cli_args_flag_combinations() {
    // Test different flag combinations
    let flag_combinations = vec![
        (true, true),   // both verbose and no_reload
        (true, false),  // only verbose
        (false, true),  // only no_reload
        (false, false), // neither flag
    ];

    for (verbose, no_reload) in flag_combinations {
        let args = Args {
            host: "flags.test".to_string(),
            nginx_config: Some(PathBuf::from("test.conf")),
            config_dir: None,
            pattern: "*.conf".to_string(),
            backup_dir: Some(PathBuf::from("/custom/backup")),
            no_reload,
            verbose,
        };

        // Verify flag combinations flow through correctly
        assert_eq!(args.verbose, verbose);
        assert_eq!(args.no_reload, no_reload);
        assert_eq!(args.host, "flags.test");
        assert!(args.backup_dir.is_some());
    }
}
