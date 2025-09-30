use std::fs;
use std::io::Write;
use std::net::IpAddr;
use std::path::Path;

/// Store an IP address to a file
pub fn store_ip(ip: IpAddr, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = fs::File::create(file_path)?;
    writeln!(file, "{}", ip)?;
    Ok(())
}

/// Load an IP address from a file
pub fn load_ip(file_path: &str) -> Result<IpAddr, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let ip_str = content.trim();
    let ip: IpAddr = ip_str.parse()?;
    Ok(ip)
}

/// Compare current IP with stored IP and update if different
pub fn check_and_update_ip(
    current_ip: IpAddr,
    file_path: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let ip_changed = if Path::new(file_path).exists() {
        match load_ip(file_path) {
            Ok(stored_ip) => {
                if stored_ip != current_ip {
                    println!("IP changed from {} to {}", stored_ip, current_ip);
                    store_ip(current_ip, file_path)?;
                    true
                } else {
                    println!("IP unchanged: {}", current_ip);
                    false
                }
            }
            Err(e) => {
                println!("Error reading stored IP: {}, storing new IP", e);
                store_ip(current_ip, file_path)?;
                true
            }
        }
    } else {
        println!("No stored IP found, storing current IP: {}", current_ip);
        store_ip(current_ip, file_path)?;
        true
    };

    Ok(ip_changed)
}

/// Get the file path for storing IP for a specific host
pub fn get_ip_file_path(host: &str) -> String {
    format!("{}_ip.txt", host.replace(".", "_"))
}
