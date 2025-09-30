#[cfg(test)]
mod tests {
    use crate::cli::Args;
    use std::path::PathBuf;

    #[test]
    fn test_args_creation_with_all_fields() {
        // Test creating Args with all possible field combinations
        let args = Args {
            host: "example.com".to_string(),
            nginx_config: Some(PathBuf::from("/etc/nginx/nginx.conf")),
            config_dir: None,
            pattern: "*.conf".to_string(),
            backup_dir: Some(PathBuf::from("/var/backups")),
            no_reload: true,
            verbose: true,
        };

        // Verify all CLI arguments are accessible
        assert_eq!(args.host, "example.com");
        assert_eq!(args.pattern, "*.conf");
        assert!(args.no_reload);
        assert!(args.verbose);
        assert!(args.nginx_config.is_some());
        assert!(args.config_dir.is_none());
        assert!(args.backup_dir.is_some());
    }

    #[test]
    fn test_args_creation_with_directory_config() {
        let args = Args {
            host: "test.local".to_string(),
            nginx_config: None,
            config_dir: Some(PathBuf::from("/etc/nginx/conf.d")),
            pattern: "*.nginx".to_string(),
            backup_dir: None,
            no_reload: false,
            verbose: false,
        };

        // Test directory-based configuration
        assert_eq!(args.host, "test.local");
        assert_eq!(args.pattern, "*.nginx");
        assert!(!args.no_reload);
        assert!(!args.verbose);
        assert!(args.nginx_config.is_none());
        assert!(args.config_dir.is_some());
        assert!(args.backup_dir.is_none());
    }

    #[test]
    fn test_args_flag_combinations() {
        // Test different flag combinations
        let args_verbose_no_reload = Args {
            host: "flags.test".to_string(),
            nginx_config: Some(PathBuf::from("test.conf")),
            config_dir: None,
            pattern: "*.conf".to_string(),
            backup_dir: Some(PathBuf::from("/custom/backup")),
            no_reload: true,
            verbose: true,
        };

        assert!(args_verbose_no_reload.no_reload && args_verbose_no_reload.verbose);

        let args_defaults = Args {
            host: "default.test".to_string(),
            nginx_config: None,
            config_dir: None,
            pattern: "*.conf".to_string(),
            backup_dir: None,
            no_reload: false,
            verbose: false,
        };

        assert!(!args_defaults.no_reload && !args_defaults.verbose);
    }

    #[test]
    fn test_args_pattern_variations() {
        let patterns = vec!["*.conf", "*.nginx", "*.config", "site-*.conf"];

        for pattern in patterns {
            let args = Args {
                host: "pattern.test".to_string(),
                nginx_config: None,
                config_dir: Some(PathBuf::from("/etc/nginx")),
                pattern: pattern.to_string(),
                backup_dir: None,
                no_reload: false,
                verbose: false,
            };

            assert_eq!(args.pattern, pattern);
        }
    }

    #[test]
    fn test_args_hostname_variations() {
        let hostnames = vec![
            "google.com",
            "example.com",
            "test.local",
            "sub.domain.example.org",
            "long-hostname-with-dashes.example.net",
        ];

        for hostname in hostnames {
            let args = Args {
                host: hostname.to_string(),
                nginx_config: None,
                config_dir: None,
                pattern: "*.conf".to_string(),
                backup_dir: None,
                no_reload: false,
                verbose: false,
            };

            assert_eq!(args.host, hostname);
        }
    }

    #[test]
    fn test_args_backup_directory_options() {
        // Test with custom backup directory
        let args_with_backup = Args {
            host: "backup.test".to_string(),
            nginx_config: Some(PathBuf::from("config.conf")),
            config_dir: None,
            pattern: "*.conf".to_string(),
            backup_dir: Some(PathBuf::from("/custom/backup/location")),
            no_reload: false,
            verbose: false,
        };

        assert!(args_with_backup.backup_dir.is_some());
        assert_eq!(
            args_with_backup.backup_dir.unwrap(),
            PathBuf::from("/custom/backup/location")
        );

        // Test without backup directory (default behavior)
        let args_no_backup = Args {
            host: "no-backup.test".to_string(),
            nginx_config: Some(PathBuf::from("config.conf")),
            config_dir: None,
            pattern: "*.conf".to_string(),
            backup_dir: None,
            no_reload: false,
            verbose: false,
        };

        assert!(args_no_backup.backup_dir.is_none());
    }

    #[test]
    fn test_args_config_source_mutual_exclusivity() {
        // Test single file configuration
        let args_single_file = Args {
            host: "single.test".to_string(),
            nginx_config: Some(PathBuf::from("single.conf")),
            config_dir: None,
            pattern: "*.conf".to_string(),
            backup_dir: None,
            no_reload: false,
            verbose: false,
        };

        assert!(args_single_file.nginx_config.is_some());
        assert!(args_single_file.config_dir.is_none());

        // Test directory configuration
        let args_directory = Args {
            host: "directory.test".to_string(),
            nginx_config: None,
            config_dir: Some(PathBuf::from("/etc/nginx")),
            pattern: "*.conf".to_string(),
            backup_dir: None,
            no_reload: false,
            verbose: false,
        };

        assert!(args_directory.nginx_config.is_none());
        assert!(args_directory.config_dir.is_some());
    }
}
