use std::fs;
use std::io::{Read, Write};
use std::net::IpAddr;

/// Open and read the contents of a file
pub fn open_and_read_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = std::fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

/// Write contents to a file
pub fn write_file(path: &str, contents: &str) -> Result<(), std::io::Error> {
    let mut file = fs::File::create(path)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

/// Check if a file is likely an nginx configuration file
pub fn is_nginx_config_file(path: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let content = open_and_read_file(path)?;
    Ok(is_nginx_config_content(&content))
}

/// Check if content appears to be nginx configuration
pub fn is_nginx_config_content(content: &str) -> bool {
    // Common nginx directives that indicate this is an nginx config
    let nginx_directives = [
        "server",
        "location",
        "listen",
        "server_name",
        "root",
        "index",
        "proxy_pass",
        "upstream",
        "events",
        "http",
        "worker_processes",
        "error_log",
        "access_log",
        "include",
        "gzip",
        "ssl_certificate",
        "return",
        "rewrite",
        "try_files",
        "add_header",
        "set",
        "if",
        "allow",
        "deny",
        "auth_basic",
        "fastcgi_pass",
        "client_max_body_size",
    ];

    // Count how many nginx directives we find
    let mut directive_count = 0;
    let mut total_lines = 0;
    let mut has_braces = false;

    for line in content.lines() {
        let trimmed = line.trim();

        // Skip empty lines and comments
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        total_lines += 1;

        // Check for nginx-style braces
        if trimmed.contains('{') || trimmed.contains('}') {
            has_braces = true;
        }

        // Check for nginx directives
        for directive in &nginx_directives {
            if let Some(after_directive) = trimmed.strip_prefix(directive) {
                // Make sure it's followed by a space, tab, or brace (not part of another word)
                if after_directive.is_empty()
                    || after_directive.starts_with(' ')
                    || after_directive.starts_with('\t')
                    || after_directive.starts_with('{')
                {
                    directive_count += 1;
                    break;
                }
            }
        }
    }

    // If we have no meaningful content, it's not a config file
    if total_lines == 0 {
        return false;
    }

    // Nginx config files should have braces and some nginx directives
    // We require at least 20% of non-empty lines to contain nginx directives
    let directive_ratio = directive_count as f64 / total_lines as f64;
    has_braces && directive_ratio >= 0.2
}

/// Validate nginx config file and provide detailed feedback
pub fn validate_nginx_config_file(path: &str, verbose: bool) -> Result<(), String> {
    // Check if file exists
    if !std::path::Path::new(path).exists() {
        return Err(format!("Config file not found: {}", path));
    }

    // Check if it's actually an nginx config
    match is_nginx_config_file(path) {
        Ok(is_nginx) => {
            if !is_nginx {
                return Err(format!(
                    "File '{}' does not appear to be an nginx configuration file. \
                    Expected to find nginx directives like 'server', 'location', 'listen', etc.",
                    path
                ));
            }

            if verbose {
                println!("✓ Validated nginx config file: {}", path);
            }
            Ok(())
        }
        Err(e) => Err(format!("Failed to read config file '{}': {}", path, e)),
    }
}

/// Update IP address in nginx allow list
pub fn update_nginx_allow_ip(
    config_path: &str,
    old_ip: Option<IpAddr>,
    new_ip: IpAddr,
    comment: Option<&str>,
) -> Result<bool, Box<dyn std::error::Error>> {
    let config_content = open_and_read_file(config_path)?;
    let mut lines: Vec<String> = config_content.lines().map(|s| s.to_string()).collect();
    let comment_text = comment.unwrap_or("DDNS");

    // Remove ALL old IP addresses for this host (identified by comment)
    if comment.is_some() {
        let mut removed_ips = Vec::new();
        lines.retain(|line| {
            let trimmed = line.trim();
            if trimmed.starts_with("allow") && line.contains(comment_text) {
                // Extract the IP address from the allow directive
                if let Some(ip_start) = trimmed.find("allow ") {
                    let after_allow = &trimmed[ip_start + 6..];
                    if let Some(semicolon_pos) = after_allow.find(';') {
                        let ip_part = after_allow[..semicolon_pos].trim();
                        removed_ips.push(ip_part.to_string());
                    }
                }
                false // Remove this line
            } else {
                true // Keep this line
            }
        });

        if !removed_ips.is_empty() {
            println!(
                "Removed {} old IP addresses for {}: [{}]",
                removed_ips.len(),
                comment_text,
                removed_ips.join(", ")
            );
        }
    } else if let Some(old_ip) = old_ip {
        // Fallback to old behavior if no comment provided
        println!("Looking for old IP {} to replace with {}", old_ip, new_ip);

        for line in &mut lines {
            if line.trim().starts_with("allow") && line.contains(&old_ip.to_string()) {
                // Preserve existing indentation and extract any existing comment
                let indent = line.len() - line.trim_start().len();
                let indent_str = " ".repeat(indent);

                // Check if there's already a comment, preserve it or use the new one
                let final_comment = if line.contains('#') {
                    // Extract existing comment
                    let parts: Vec<&str> = line.split('#').collect();
                    if parts.len() > 1 {
                        format!(" #{}", parts[1])
                    } else {
                        format!(" # {}", comment_text)
                    }
                } else {
                    format!(" # {}", comment_text)
                };

                *line = format!("{}allow {};{}", indent_str, new_ip, final_comment);
                println!("Replaced allow directive: {} -> {}", old_ip, new_ip);
                break;
            }
        }
    }

    // Always add the new IP address (after potentially removing old ones)
    // Find the best place to insert the new allow directive
    let mut insert_index = None;

    // Look for existing allow directives first
    for (i, line) in lines.iter().enumerate() {
        if line.trim().starts_with("allow") {
            insert_index = Some(i + 1);
        }
    }

    // If no allow directives found, look for location or server block
    if insert_index.is_none() {
        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            if trimmed.starts_with("location")
                || (trimmed.starts_with("server") && trimmed.ends_with("{"))
            {
                // Insert after the opening brace
                insert_index = Some(i + 1);
                break;
            }
        }
    }

    // Determine appropriate indentation
    let indent = if let Some(idx) = insert_index {
        if idx > 0 && idx < lines.len() {
            let prev_line = &lines[idx - 1];
            prev_line.len() - prev_line.trim_start().len()
        } else {
            4 // Default indentation
        }
    } else {
        4
    };

    let insert_pos = insert_index.unwrap_or(lines.len());
    let indent_str = " ".repeat(indent);
    let new_allow_line = format!("{}allow {}; # {}", indent_str, new_ip, comment_text);
    lines.insert(insert_pos, new_allow_line);
    println!("Added new allow directive: {}", new_ip);

    // We always make changes since we always add the new IP
    let changes_made = true;

    if changes_made {
        let updated_content = lines.join("\n");
        write_file(config_path, &updated_content)?;
    }

    Ok(changes_made)
}

/// Create a backup of nginx config file
pub fn backup_nginx_config(config_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    backup_nginx_config_to_dir(config_path, None)
}

/// Create a backup of nginx config file to a specific directory
pub fn backup_nginx_config_to_dir(
    config_path: &str,
    backup_dir: Option<&std::path::Path>,
) -> Result<String, Box<dyn std::error::Error>> {
    use std::path::Path;

    let config_file = Path::new(config_path);
    let filename = config_file
        .file_name()
        .ok_or("Invalid config path")?
        .to_string_lossy();

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();

    let backup_filename = format!("{}.backup.{}", filename, timestamp);

    let backup_path = if let Some(backup_dir) = backup_dir {
        backup_dir.join(backup_filename)
    } else {
        // Default behavior: backup next to original file
        config_file.with_file_name(backup_filename)
    };

    let content = open_and_read_file(config_path)?;
    write_file(&backup_path.to_string_lossy(), &content)?;

    println!("Created backup: {}", backup_path.display());
    Ok(backup_path.to_string_lossy().to_string())
}

/// Check if nginx is installed and available
pub fn is_nginx_installed() -> bool {
    use std::process::Command;

    match Command::new("nginx").arg("-v").output() {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

/// Reload nginx configuration (only if nginx is installed)
pub fn reload_nginx() -> Result<(), Box<dyn std::error::Error>> {
    use std::process::Command;

    if !is_nginx_installed() {
        return Err("Nginx is not installed or not available in PATH".into());
    }

    println!("Reloading nginx configuration...");
    let output = Command::new("nginx").arg("-s").arg("reload").output()?;

    if output.status.success() {
        println!("Nginx configuration reloaded successfully");
        Ok(())
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        Err(format!("Failed to reload nginx: {}", error).into())
    }
}

/// Reload nginx configuration if installed, otherwise show manual instruction
pub fn reload_nginx_if_available() {
    if is_nginx_installed() {
        match reload_nginx() {
            Ok(()) => {
                println!("Nginx reloaded successfully");
            }
            Err(e) => {
                println!("Warning: Failed to reload nginx: {}", e);
                println!("Please reload nginx manually: sudo nginx -s reload");
            }
        }
    } else {
        println!("Nginx not detected. If installed, reload manually with: sudo nginx -s reload");
    }
}

// Include the test module
#[cfg(test)]
#[path = "nginx_tests.rs"]
mod tests;
