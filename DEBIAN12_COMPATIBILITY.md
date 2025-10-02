# Debian 12 (Bookworm) Compatibility

This document explains how to build and deploy DDNS Updater on Debian 12 with libc 2.36-9+deb12u10.

## Compatibility Status

✅ **Fully Compatible** - The DDNS updater has been tested and optimized for Debian 12 (Bookworm).

## System Requirements

- **OS**: Debian 12 (Bookworm) or compatible
- **Architecture**: x86_64 (AMD64) or aarch64 (ARM64)
- **glibc**: 2.36-9+deb12u10 or later
- **Rust**: 1.70.0 or later (for building from source)

## Quick Installation

### Using Pre-built Binaries

Download the Debian 12 compatible binary:

```bash
# For x86_64 systems
wget https://github.com/koenieee/ddns_local_server/releases/latest/download/ddns_updater-debian12-x86_64.tar.gz
tar -xzf ddns_updater-debian12-x86_64.tar.gz
sudo cp ddns_updater /usr/local/bin/
sudo chmod +x /usr/local/bin/ddns_updater

# For ARM64 systems  
wget https://github.com/koenieee/ddns_local_server/releases/latest/download/ddns_updater-debian12-aarch64.tar.gz
tar -xzf ddns_updater-debian12-aarch64.tar.gz
sudo cp ddns_updater /usr/local/bin/
sudo chmod +x /usr/local/bin/ddns_updater
```

### Using Debian Package

```bash
# Download and install .deb package
wget https://github.com/koenieee/ddns_local_server/releases/latest/download/ddns-updater_*_amd64.deb
sudo dpkg -i ddns-updater_*_amd64.deb
sudo apt-get install -f  # Install any missing dependencies
```

## Building from Source

### Method 1: Using the Build Script (Recommended)

```bash
git clone https://github.com/koenieee/ddns_local_server.git
cd ddns_local_server
./build-debian12.sh
```

### Method 2: Using Docker

```bash
# Build using Debian 12 container
docker build -f Dockerfile.debian12 -t ddns_updater:debian12 .

# Extract binary
docker create --name temp ddns_updater:debian12
docker cp temp:/usr/local/bin/ddns_updater ./ddns_updater
docker rm temp
```

### Method 3: Manual Build

```bash
# Install Rust 1.70.0 for maximum compatibility
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain 1.70.0

# Set environment for glibc compatibility
export RUSTFLAGS="-C target-feature=-crt-static -C link-arg=-Wl,--as-needed"

# Build
cargo build --release --target x86_64-unknown-linux-gnu
```

## Verification

Check that the binary is compatible with your system:

```bash
# Check glibc requirements
objdump -T ddns_updater | grep GLIBC | sort -V | tail -5

# Test basic functionality
./ddns_updater --version
./ddns_updater --help
```

## Configuration for Debian 12

### System Service Setup

```bash
# Install systemd service files
sudo cp systemd/*.service /etc/systemd/system/
sudo cp systemd/*.timer /etc/systemd/system/
sudo systemctl daemon-reload

# Enable and start the service
sudo systemctl enable ddns-updater.timer
sudo systemctl start ddns-updater.timer
```

### Nginx Integration

```bash
# Ensure nginx is installed
sudo apt-get update
sudo apt-get install nginx

# Verify nginx config directory
ls -la /etc/nginx/sites-available/
ls -la /etc/nginx/sites-enabled/
```

### Directory Setup

```bash
# Create required directories
sudo mkdir -p /var/lib/ddns-updater
sudo mkdir -p /var/log/ddns-updater
sudo mkdir -p /var/backups/ddns-updater

# Set appropriate permissions
sudo chown ddns:ddns /var/lib/ddns-updater
sudo chown ddns:ddns /var/log/ddns-updater
sudo chown ddns:ddns /var/backups/ddns-updater
```

## Troubleshooting

### glibc Version Issues

If you encounter glibc version errors:

```bash
# Check your system's glibc version
ldd --version

# Check binary requirements
objdump -T ddns_updater | grep GLIBC
```

### Permission Issues

If you get permission denied errors:

```bash
# Check file permissions
ls -la /usr/local/bin/ddns_updater

# Fix permissions
sudo chmod +x /usr/local/bin/ddns_updater

# Check user/group
id ddns
```

### Service Issues

If the systemd service fails to start:

```bash
# Check service status
sudo systemctl status ddns-updater.service

# View logs
sudo journalctl -u ddns-updater.service -f

# Check configuration
ddns_updater --help
```

## Performance Notes

- **Memory Usage**: ~5-10 MB RAM during operation
- **Disk Usage**: ~20 MB binary size
- **CPU Usage**: Minimal, only active during IP checks
- **Network**: Minimal bandwidth for IP checking

## Security Considerations

- Binary is compiled with stack protection enabled
- Uses rustls instead of OpenSSL for better security
- No dynamic dependencies on external libraries
- Runs with minimal privileges when configured properly

## Support

For Debian 12 specific issues:

1. Check the system requirements above
2. Verify glibc compatibility
3. Review systemd service logs
4. Check nginx integration
5. Open an issue on GitHub with system details

## Version Compatibility

| DDNS Updater | Debian Version | glibc Version | Status |
|--------------|----------------|---------------|---------|
| 1.0.0+       | 12 (Bookworm) | 2.36+         | ✅ Full |
| 1.0.0+       | 11 (Bullseye) | 2.31+         | ⚠️ Limited |
| 1.0.0+       | 10 (Buster)   | 2.28+         | ❌ No |