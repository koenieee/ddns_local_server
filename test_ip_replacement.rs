use ddns_updater::domain::models::WebServerConfig;
use ddns_updater::infrastructure::webservers::WebServerHandler;
use ddns_updater::infrastructure::webservers::nginx::NginxHandler;
use std::net::IpAddr;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing IP-based replacement in nginx config...");

    // Create nginx handler
    let handler = NginxHandler::new();

    // Create config
    let config = WebServerConfig {
        server_type: "nginx".to_string(),
        path: "test_configs/valid/basic_server.conf".to_string(),
    };

    // Test replacing 142.250.102.138 with 8.8.8.8
    let old_ip = IpAddr::from_str("142.250.102.138")?;
    let new_ip = IpAddr::from_str("8.8.8.8")?;

    println!(
        "Attempting to replace {} with {} for hostname 'example.com'",
        old_ip, new_ip
    );

    let result = handler
        .update_allow_list(&config, "example.com", Some(old_ip), new_ip)
        .await?;

    println!("Update result: {}", result);

    // Read the file to see the result
    let content = std::fs::read_to_string("test_configs/valid/basic_server.conf")?;
    println!("Updated config content:\n{}", content);

    Ok(())
}
