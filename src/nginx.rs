use std::fs;
use std::io::{Read, Write};
use std::net::IpAddr;
use std::path::Path;

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

/// Update IP address in nginx allow list
pub fn update_nginx_allow_ip(
    config_path: &str,
    old_ip: Option<IpAddr>,
    new_ip: IpAddr,
    comment: Option<&str>,
) -> Result<bool, Box<dyn std::error::Error>> {
    let config_content = open_and_read_file(config_path)?;
    let mut lines: Vec<String> = config_content.lines().map(|s| s.to_string()).collect();
    let mut updated = false;
    let comment_text = comment.unwrap_or("DDNS");

    // Look for existing IP to replace in any allow directive
    if let Some(old_ip) = old_ip {
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
                updated = true;
                println!("Replaced allow directive: {} -> {}", old_ip, new_ip);
                break;
            }
        }
    }

    // If no existing IP was found to replace, add new allow directive
    if !updated {
        // Find the best place to insert the new allow directive
        let mut insert_index = None;

        // Look for existing allow directives first
        for (i, line) in lines.iter().enumerate() {
            if line.trim().starts_with("allow") {
                insert_index = Some(i + 1);
                break;
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
        updated = true;
        println!("Added new allow directive: {}", new_ip);
    }

    if updated {
        let updated_content = lines.join("\n");
        write_file(config_path, &updated_content)?;
    }

    Ok(updated)
}

/// Create a backup of nginx config file
pub fn backup_nginx_config(config_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let backup_path = format!(
        "{}.backup.{}",
        config_path,
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs()
    );

    let content = open_and_read_file(config_path)?;
    write_file(&backup_path, &content)?;

    println!("Created backup: {}", backup_path);
    Ok(backup_path)
}

/// Reload nginx configuration
pub fn reload_nginx() -> Result<(), Box<dyn std::error::Error>> {
    use std::process::Command;

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
