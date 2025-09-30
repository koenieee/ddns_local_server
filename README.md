# DDNS Updater - Nginx Allow List Manager

[![CI/CD Pipeline](https://github.com/koenieee/ddns_local_server/actions/workflows/ci.yml/badge.svg)](https://github.com/koenieee/ddns_local_server/actions/workflows/ci.yml)
[![Security Audit](https://github.com/koenieee/ddns_local_server/actions/workflows/nightly.yml/badge.svg)](https://github.com/koenieee/ddns_local_server/actions/workflows/nightly.yml)
[![Documentation](https://github.com/koenieee/ddns_local_server/actions/workflows/docs.yml/badge.svg)](https://github.com/koenieee/ddns_local_server/actions/workflows/docs.yml)

A Rust-based Dynamic DNS (DDNS) updater that automatically manages nginx allow lists when your public IP address changes.

## Features

- **Automatic IP Detection**: Monitors your public IP address changes
- **Multiple Config Support**: Process single files or entire directories
- **Smart Cleanup**: Removes old duplicate IP entries for the same host
- **Nginx Integration**: Automatically validates and reloads nginx configurations
- **Backup Management**: Creates timestamped backups before making changes
- **Pattern Matching**: Flexible file selection with glob patterns
- **Systemd Integration**: Run as a system service with automatic scheduling
- **Security Hardening**: Comprehensive security features for production use

## Quick Start

### Installation

#### Option 1: Debian Package (Recommended)
```bash
# Download latest release
wget https://github.com/koenieee/ddns_local_server/releases/latest/download/ddns-updater_*.deb

# Install package
sudo apt install ./ddns-updater_*.deb

# Run interactive setup
sudo /usr/share/ddns-updater/install-systemd.sh
```

#### Option 2: Pre-built Binary
```bash
# Download binary for your architecture
wget https://github.com/koenieee/ddns_local_server/releases/latest/download/ddns-updater-linux-amd64

# Make executable and install
chmod +x ddns-updater-linux-amd64
sudo mv ddns-updater-linux-amd64 /usr/local/bin/ddns-updater
```

#### Option 3: Build from Source
```bash
git clone https://github.com/koenieee/ddns_local_server.git
cd ddns_local_server
cargo build --release
sudo cp target/release/ddns_updater /usr/local/bin/
```

#### Testing Installation
```bash
ddns_updater --host google.com --config /path/to/nginx.conf --verbose --no-reload
```

### Basic Usage

**Single config file:**
```bash
ddns_updater --host example.com --config /etc/nginx/sites-available/example.conf
```

**Directory of config files:**
```bash
ddns_updater --host example.com --config-dir /etc/nginx/sites-available
```

**With custom backup location:**
```bash
ddns_updater --host example.com --config-dir /etc/nginx/sites-available --backup-dir /var/backups/nginx
```

**Specific file pattern:**
```bash
ddns_updater --host example.com --config-dir /etc/nginx/conf.d --pattern "*example*"
```

## Systemd Service Installation

For production use, install as a systemd service that runs automatically:

### Simple Installation
```bash
sudo ./systemd/install-systemd.sh
```

### Advanced Multi-Host Installation
```bash
sudo ./systemd/install-systemd-advanced.sh
systemctl enable ddns-updater@google-com.timer
systemctl start ddns-updater@google-com.timer
```

See [systemd/SYSTEMD.md](systemd/SYSTEMD.md) for detailed systemd configuration and management.

## Command Line Options

```
Options:
      --host <HOST>              Host to check for IP changes [default: google.com]
  -c, --config <NGINX_CONFIG>    Path to nginx configuration file
  -d, --config-dir <CONFIG_DIR>  Directory containing nginx configuration files
  -p, --pattern <PATTERN>        Pattern to match config files [default: *.conf]
  -b, --backup-dir <BACKUP_DIR>  Directory to store backup files
      --no-reload                Don't reload nginx after updating configuration
  -v, --verbose                  Verbose output
  -h, --help                     Print help
  -V, --version                  Print version
```

## How It Works

1. **IP Detection**: Resolves the current IP address for the specified host
2. **Change Detection**: Compares with previously stored IP address
3. **File Discovery**: Finds nginx config files matching your criteria
4. **Backup Creation**: Creates timestamped backups of all files to be modified
5. **Smart Updates**: Removes ALL old IP entries for the host and adds the current IP
6. **Nginx Reload**: Optionally reloads nginx configuration (if nginx is installed)

### Example Nginx Config Transformation

**Before:**
```nginx
server {
    listen 80;
    server_name example.com;
    location / {
        allow 192.168.1.1;
        allow 142.250.102.138; # DDNS for google.com
        allow 142.250.102.139; # DDNS for google.com  
        allow 142.250.102.100; # DDNS for google.com
        # ... more duplicate entries
        deny all;
    }
}
```

**After:**
```nginx
server {
    listen 80;
    server_name example.com;
    location / {
        allow 192.168.1.1;
        allow 142.250.102.101; # DDNS for google.com
        deny all;
    }
}
```

## Configuration

### Environment Variables

When running as a systemd service, configuration is managed through environment files:

**Example: `/etc/default/ddns-updater-google-com`**
```bash
DDNS_ARGS="--host google.com --config-dir /etc/nginx/sites-available --backup-dir /var/backups/nginx --verbose"
```

### Multiple Hosts

You can run separate instances for different hosts:

```bash
# Google services
systemctl enable ddns-updater@google-com.timer

# Your domain
systemctl enable ddns-updater@mydomain-com.timer

# Another service
systemctl enable ddns-updater@api-server.timer
```

## Security Features

- **Minimal Privileges**: Runs with only necessary permissions
- **Protected Filesystem**: Read-only access except for specific directories
- **Private Temporary Files**: Isolated temporary file system
- **Namespace Restrictions**: Limited system access
- **Input Validation**: Validates all nginx configuration files before processing

## Monitoring and Logging

### Systemd Journal
```bash
# View recent logs
journalctl -u ddns-updater.service -n 50

# Follow logs in real-time
journalctl -u ddns-updater.service -f

# View logs for specific host
journalctl -u ddns-updater@google-com.service -f
```

### Log Messages
- **IP Changes**: `IP address has changed! Updating nginx allow list...`
- **File Processing**: `✓ Updated: /etc/nginx/sites-available/example.conf`
- **Backup Creation**: `Created backup: /var/backups/nginx/example.conf.backup.1234567890`
- **No Changes**: `No IP change detected. Nginx config unchanged.`

## Testing

The project includes comprehensive tests:

```bash
# Run unit tests
cargo test

# Run comprehensive test suite
./scripts/test_all.sh

# Test with actual config files
cargo run -- --config test_configs/valid/basic_server.conf --host google.com --verbose --no-reload
```

See [TESTING.md](TESTING.md) for detailed testing information.

## Project Structure

```
ddns_updater/
├── src/
│   ├── main.rs              # Entry point
│   ├── lib.rs               # Main library logic
│   ├── cli/                 # Command line interface
│   ├── config/              # Configuration management
│   └── core/                # Core functionality
├── systemd/                 # Systemd service files and scripts
│   ├── *.service, *.timer   # Systemd service files
│   ├── install-systemd*.sh  # Installation scripts
│   ├── examples/            # Example systemd configurations
│   └── SYSTEMD.md          # Systemd documentation
├── test_configs/            # Test configuration files
├── scripts/                 # Testing and utility scripts
└── README.md               # Main documentation
```

## Development & CI/CD

### Automated Builds
This project uses GitHub Actions for continuous integration and delivery:

- **🔄 CI/CD Pipeline**: Builds, tests, and creates releases automatically
- **📦 Multi-Platform**: Builds for Linux AMD64, ARM64, and MUSL targets  
- **🔍 Security Scanning**: Nightly vulnerability and dependency checks
- **📚 Documentation**: Validates markdown links and systemd configurations
- **🚀 Automated Releases**: Tagged releases with binaries and Debian packages

### Release Process
Releases are automated through GitHub Actions:

```bash
# Create a new release (maintainers only)
gh workflow run release.yml -f version=v1.0.0 -f prerelease=false
```

### Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Add tests for new functionality
4. Ensure all tests pass: `./scripts/test_all.sh`
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

The CI pipeline will automatically:
- Run tests on multiple Rust versions
- Build for all supported platforms
- Check code formatting and linting
- Validate documentation
- Run security audits

### Local Development

```bash
# Run tests
cargo test --verbose

# Check formatting
cargo fmt --all -- --check

# Run clippy lints
cargo clippy --all-targets -- -D warnings

# Build Debian package
./build-deb.sh

# Test systemd installation
sudo ./systemd/install-systemd.sh
```

## License

[Add your license information here]

## Troubleshooting

### Common Issues

**Permission Denied:**
```bash
sudo chown -R root:root /etc/nginx/sites-available
sudo chmod 644 /etc/nginx/sites-available/*.conf
```

**Nginx Reload Fails:**
```bash
nginx -t  # Test configuration syntax
systemctl status nginx
```

**Service Won't Start:**
```bash
systemctl status ddns-updater.service
journalctl -u ddns-updater.service --no-pager
```

### Support

- Check the logs: `journalctl -u ddns-updater.service -f`
- Test manually: `ddns_updater --verbose --no-reload`
- Validate nginx configs: `nginx -t`
- Review systemd setup: See [systemd/SYSTEMD.md](systemd/SYSTEMD.md)

## Development

### CI/CD Pipeline
This project uses GitHub Actions for continuous integration, testing, and deployment. For detailed information about the workflows, see [.github/WORKFLOWS.md](.github/WORKFLOWS.md).

### Contributing
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `./scripts/test_all.sh`
5. Submit a pull request

### Testing
- **Unit tests**: `cargo test`
- **Integration tests**: `./scripts/test_all.sh`
- **Debian package**: `./build-deb.sh`