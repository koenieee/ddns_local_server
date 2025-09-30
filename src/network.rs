use std::net::{IpAddr, UdpSocket};

/// Check if a host is online and return its IP address
pub fn get_host_ip(host: &str) -> Result<IpAddr, Box<dyn std::error::Error + Send + Sync>> {
    // Use a UDP socket to force a fresh DNS resolution
    let addr = format!("{}:80", host);
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.connect(&addr)?;
    let peer = socket.peer_addr()?;
    Ok(peer.ip())
}
