#[cfg(test)]
mod tests {
    use crate::application::services::{AppConfig, ServiceFactory};
    use crate::domain::entities::WebServerType;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn test_app_config_creation() {
        let temp_dir = tempdir().unwrap();

        let app_config = AppConfig {
            storage_dir: temp_dir.path().to_path_buf(),
            backup_dir: Some(temp_dir.path().join("backups")),
            no_reload: true,
            verbose: false,
            backup_retention_days: 7,
            max_backups: 10,
        };

        assert_eq!(app_config.storage_dir, temp_dir.path().to_path_buf());
        assert!(app_config.backup_dir.is_some());
        assert_eq!(
            app_config.backup_dir.unwrap(),
            temp_dir.path().join("backups")
        );
        assert!(app_config.no_reload);
        assert!(!app_config.verbose);
        assert_eq!(app_config.backup_retention_days, 7);
        assert_eq!(app_config.max_backups, 10);
    }

    #[test]
    fn test_app_config_defaults() {
        let app_config = AppConfig::default();

        assert_eq!(
            app_config.storage_dir,
            PathBuf::from("/var/lib/ddns-updater")
        );
        assert!(app_config.backup_dir.is_none());
        assert!(!app_config.no_reload);
        assert!(!app_config.verbose);
        assert_eq!(app_config.backup_retention_days, 30);
        assert_eq!(app_config.max_backups, 10);
    }

    #[test]
    fn test_create_web_server_handler_nginx() {
        let temp_dir = tempdir().unwrap();

        let handler = ServiceFactory::create_web_server_handler(
            WebServerType::Nginx,
            Some(temp_dir.path().join("backups")),
        );

        // Verify handler was created successfully by checking server type
        assert_eq!(handler.server_type(), WebServerType::Nginx);
    }

    #[test]
    fn test_create_web_server_handler_apache() {
        let handler = ServiceFactory::create_web_server_handler(WebServerType::Apache, None);

        // Should still create handler successfully even without backup_dir
        assert_eq!(handler.server_type(), WebServerType::Apache);
    }

    #[test]
    fn test_create_web_server_handler_with_backup_dir() {
        let custom_backup = PathBuf::from("/custom/backup/location");

        let handler =
            ServiceFactory::create_web_server_handler(WebServerType::Nginx, Some(custom_backup));

        // Verify handler creation succeeds with custom backup directory
        assert_eq!(handler.server_type(), WebServerType::Nginx);
    }

    #[test]
    fn test_app_config_verbose_and_no_reload_combination() {
        let temp_dir = tempdir().unwrap();

        let app_config = AppConfig {
            storage_dir: temp_dir.path().to_path_buf(),
            backup_dir: Some(PathBuf::from("/var/backups/nginx")),
            no_reload: true,
            verbose: true,
            backup_retention_days: 14,
            max_backups: 25,
        };

        // Test that both flags can be set simultaneously
        assert!(app_config.no_reload);
        assert!(app_config.verbose);
        assert!(app_config.backup_dir.is_some());
        assert_eq!(app_config.backup_retention_days, 14);
        assert_eq!(app_config.max_backups, 25);
    }

    #[test]
    fn test_create_ip_repository() {
        let temp_dir = tempdir().unwrap();

        let result = ServiceFactory::create_ip_repository(temp_dir.path().to_path_buf());
        assert!(result.is_ok());

        // Verify repository was created by checking it exists
        let _repo = result.unwrap();
        // Repository creation succeeded if we reach this point
        assert!(true);
    }

    #[test]
    fn test_create_network_service() {
        let _service = ServiceFactory::create_network_service();
        // Service creation succeeded if we reach this point
        assert!(true);
    }

    #[test]
    fn test_create_notification_service() {
        let _service = ServiceFactory::create_notification_service(true);
        // Service creation succeeded if we reach this point
        assert!(true);
    }

    #[test]
    fn test_create_config_discovery_service() {
        let _service = ServiceFactory::create_config_discovery_service();
        // Service creation succeeded if we reach this point
        assert!(true);
    }

    #[test]
    fn test_create_web_server_handler_all_types() {
        let server_types = vec![
            (WebServerType::Nginx, WebServerType::Nginx),
            (WebServerType::Apache, WebServerType::Apache),
            (WebServerType::Caddy, WebServerType::Nginx), // Fallback to Nginx
            (WebServerType::Traefik, WebServerType::Nginx), // Fallback to Nginx
        ];

        for (input_type, expected_type) in server_types {
            let handler = ServiceFactory::create_web_server_handler(input_type, None);
            // Verify handler was created by checking server type matches expected
            assert_eq!(handler.server_type(), expected_type);
        }
    }
}
