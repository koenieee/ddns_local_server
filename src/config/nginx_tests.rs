use crate::config::nginx::*;
use std::path::Path;

#[test]
fn test_is_nginx_config_content_valid_configs() {
    // Test basic server block
    let basic_server = r#"
server {
    listen 80;
    server_name example.com;
    
    location / {
        root /var/www/html;
        index index.html;
    }
}
"#;
    assert!(
        is_nginx_config_content(basic_server),
        "Basic server block should be valid"
    );

    // Test complex config with upstream
    let complex_config = r#"
upstream backend {
    server 127.0.0.1:3000;
}

server {
    listen 443 ssl;
    server_name api.example.com;
    
    location /api {
        proxy_pass http://backend;
        allow 10.0.0.1;
        deny all;
    }
}
"#;
    assert!(
        is_nginx_config_content(complex_config),
        "Complex config should be valid"
    );

    // Test minimal valid config
    let minimal_config = r#"
server {
    listen 80;
    location / {
        return 200 "OK";
    }
}
"#;
    assert!(
        is_nginx_config_content(minimal_config),
        "Minimal config should be valid"
    );

    // Test events block
    let events_config = r#"
events {
    worker_connections 1024;
}

http {
    server {
        listen 80;
        server_name test.com;
    }
}
"#;
    assert!(
        is_nginx_config_content(events_config),
        "Events config should be valid"
    );
}

#[test]
fn test_is_nginx_config_content_invalid_configs() {
    // Test plain text
    let plain_text = "This is just plain text with no nginx directives.";
    assert!(
        !is_nginx_config_content(plain_text),
        "Plain text should be invalid"
    );

    // Test config without braces
    let no_braces = r#"
server example.com
listen 80
root /var/www
"#;
    assert!(
        !is_nginx_config_content(no_braces),
        "Config without braces should be invalid"
    );

    // Test only comments
    let only_comments = r#"
# This is a comment
# server_name example.com;
# All lines are commented
"#;
    assert!(
        !is_nginx_config_content(only_comments),
        "Only comments should be invalid"
    );

    // Test insufficient structure
    let insufficient = "server_name example.com;\n# Not enough structure";
    assert!(
        !is_nginx_config_content(insufficient),
        "Insufficient structure should be invalid"
    );

    // Test empty content
    assert!(
        !is_nginx_config_content(""),
        "Empty content should be invalid"
    );

    // Test JSON (has braces but no nginx directives)
    let json_content = r#"
{
    "name": "test",
    "server": "not nginx"
}
"#;
    assert!(
        !is_nginx_config_content(json_content),
        "JSON should be invalid"
    );
}

#[test]
fn test_nginx_config_files_in_test_directory() {
    // Test valid config files
    let valid_configs = [
        "test_configs/valid/basic_server.conf",
        "test_configs/valid/complex_ssl.conf",
        "test_configs/valid/full_nginx.conf",
        "test_configs/valid/minimal_valid.conf",
    ];

    for config_path in &valid_configs {
        if Path::new(config_path).exists() {
            match is_nginx_config_file(config_path) {
                Ok(is_valid) => {
                    assert!(
                        is_valid,
                        "Valid config file {} should pass validation",
                        config_path
                    );
                }
                Err(e) => {
                    panic!("Failed to validate {}: {}", config_path, e);
                }
            }
        }
    }

    // Test invalid config files
    let invalid_configs = [
        "test_configs/invalid/plain_text.conf",
        "test_configs/invalid/no_braces.conf",
        "test_configs/invalid/only_comments.conf",
        "test_configs/invalid/insufficient_structure.conf",
        "test_configs/invalid/json_file.conf",
        "test_configs/invalid/empty_file.conf",
    ];

    for config_path in &invalid_configs {
        if Path::new(config_path).exists() {
            match is_nginx_config_file(config_path) {
                Ok(is_valid) => {
                    assert!(
                        !is_valid,
                        "Invalid config file {} should fail validation",
                        config_path
                    );
                }
                Err(e) => {
                    panic!("Failed to read {}: {}", config_path, e);
                }
            }
        }
    }
}

#[test]
fn test_validate_nginx_config_file_function() {
    // Test with valid file
    if Path::new("test_configs/valid/basic_server.conf").exists() {
        assert!(
            validate_nginx_config_file("test_configs/valid/basic_server.conf", false).is_ok(),
            "Valid config should pass validation function"
        );
    }

    // Test with invalid file
    if Path::new("test_configs/invalid/plain_text.conf").exists() {
        assert!(
            validate_nginx_config_file("test_configs/invalid/plain_text.conf", false).is_err(),
            "Invalid config should fail validation function"
        );
    }

    // Test with non-existent file
    assert!(
        validate_nginx_config_file("non_existent_file.conf", false).is_err(),
        "Non-existent file should fail validation"
    );
}

#[test]
fn test_nginx_directive_detection() {
    // Test that various nginx directives are detected
    let directives_test = r#"
server {
    listen 443 ssl http2;
    server_name example.com www.example.com;
    root /var/www/html;
    index index.html index.htm;
    
    error_log /var/log/nginx/error.log;
    access_log /var/log/nginx/access.log;
    
    gzip on;
    client_max_body_size 100M;
    
    location / {
        try_files $uri $uri/ =404;
        add_header X-Frame-Options DENY;
    }
    
    location ~ \.php$ {
        fastcgi_pass unix:/var/run/php/php-fpm.sock;
        include fastcgi_params;
    }
    
    if ($scheme != "https") {
        return 301 https://$host$request_uri;
    }
}
"#;
    assert!(
        is_nginx_config_content(directives_test),
        "Config with various directives should be valid"
    );
}

#[test]
fn test_edge_cases() {
    // Test config with only whitespace and comments
    let whitespace_comments = r#"
        
        # Comment 1
        
        # Comment 2
        
        "#;
    assert!(
        !is_nginx_config_content(whitespace_comments),
        "Only whitespace and comments should be invalid"
    );

    // Test config with braces but no nginx directives
    let braces_no_directives = r#"
        {
            "key": "value",
            "array": [1, 2, 3]
        }
        "#;
    assert!(
        !is_nginx_config_content(braces_no_directives),
        "Braces without nginx directives should be invalid"
    );

    // Test minimal valid config (exactly at threshold)
    let minimal_threshold = r#"
server {
    listen 80;
}
"#;
    assert!(
        is_nginx_config_content(minimal_threshold),
        "Minimal threshold config should be valid"
    );
}

/// Cleanup function to remove test artifacts
fn cleanup_test_artifacts() {
    use std::fs;
    use std::path::Path;

    let mut cleaned_items = Vec::new();

    // Remove backup directories
    let backup_dirs = ["backups", "test_backups", "my_backups"];
    for backup_dir in &backup_dirs {
        if Path::new(backup_dir).exists() {
            match fs::remove_dir_all(backup_dir) {
                Ok(_) => cleaned_items.push(format!("directory {}/", backup_dir)),
                Err(e) => eprintln!(
                    "Warning: Failed to remove backup directory {}: {}",
                    backup_dir, e
                ),
            }
        }
    }

    // Remove IP storage files (common patterns)
    let ip_file_patterns = [
        "google_com_ip.txt",
        "example_com_ip.txt",
        "localhost_ip.txt",
    ];

    // Remove specific IP files
    for pattern in &ip_file_patterns {
        if Path::new(pattern).exists() {
            match fs::remove_file(pattern) {
                Ok(_) => cleaned_items.push(format!("file {}", pattern)),
                Err(e) => eprintln!("Warning: Failed to remove IP file {}: {}", pattern, e),
            }
        }
    }

    // Remove any *_ip.txt files in current directory
    if let Ok(entries) = fs::read_dir(".") {
        for entry in entries.flatten() {
            if let Some(file_name) = entry.file_name().to_str() {
                if file_name.ends_with("_ip.txt") && !ip_file_patterns.contains(&file_name) {
                    // Don't double-report
                    match fs::remove_file(entry.path()) {
                        Ok(_) => cleaned_items.push(format!("file {}", file_name)),
                        Err(e) => {
                            eprintln!("Warning: Failed to remove IP file {}: {}", file_name, e)
                        }
                    }
                }
            }
        }
    }

    // Report what was cleaned up
    if !cleaned_items.is_empty() {
        println!("🧹 Cleaned up: {}", cleaned_items.join(", "));
    }
}

/// Test cleanup - runs after all other tests
#[test]
fn test_zzz_cleanup() {
    // This test runs last (alphabetically with zzz prefix) and cleans up test artifacts
    cleanup_test_artifacts();

    // Verify cleanup worked
    use std::path::Path;

    let backup_dirs = ["backups", "test_backups", "my_backups", "extra_backups"];
    for backup_dir in &backup_dirs {
        assert!(
            !Path::new(backup_dir).exists(),
            "Backup directory {} should be removed",
            backup_dir
        );
    }

    println!("✓ Test cleanup completed successfully");
}
