# DDNS Updater - Nginx Allow List Manager

[![CI/CD Pipeline](https://github.com/koenieee/ddns_local_server/actions/workflows/ci.yml/badge.svg)](https://github.com/koenieee/ddns_local_server/actions/workflows/ci.yml)
[![Security Audit](https://github.com/koenieee/ddns_local_server/actions/workflows/nightly.yml/badge.svg)](https://github.com/koenieee/ddns_local_server/actions/workflows/nightly.yml)
[![Documentation](https://github.com/koenieee/ddns_local_server/actions/workflows/docs.yml/badge.svg)](https://github.com/koenieee/ddns_local_server/actions/workflows/docs.yml)
[![Architecture Diagrams](https://github.com/koenieee/ddns_local_server/actions/workflows/generate-diagrams.yml/badge.svg)](https://github.com/koenieee/ddns_local_server/actions/workflows/generate-diagrams.yml)

A Rust-based Dynamic DNS (DDNS) updater that automatically manages nginx allow lists when your public IP address changes.

## Features

- **Automatic IP Detection**: Monitors your public IP address changes
- **Smart JSON Storage**: Automatically creates and manages IP tracking files
- **Intelligent Config Checking**: Verifies IP presence before making changes
- **Non-Intrusive Updates**: Only updates existing entries, never adds new ones
- **Multiple Config Support**: Process single files or entire directories
- **Smart Cleanup**: Removes old duplicate IP entries for the same host
- **Nginx Integration**: Automatically validates and reloads nginx configurations
- **Backup Management**: Creates timestamped backups before making changes
- **Pattern Matching**: Flexible file selection with glob patterns
- **Systemd Integration**: Run as a system service with automatic scheduling
- **Security Hardening**: Comprehensive security features for production use

## Quick Start

### Installation

> **Available Installation Methods**: Currently only Debian packages and source builds are supported. Pre-built standalone binaries are not provided at this time.

#### Option 1: Debian Package (Recommended)

**For x86_64/AMD64 systems:**
```bash
# Download latest release for AMD64
wget https://github.com/koenieee/ddns_local_server/releases/latest/download/ddns-updater_*_amd64.deb

# Install package
sudo dpkg -i ddns-updater_*_amd64.deb
```

**For ARM64/aarch64 systems:**
```bash
# Download latest release for ARM64
wget https://github.com/koenieee/ddns_local_server/releases/latest/download/ddns-updater_*_arm64.deb

# Install package
sudo dpkg -i ddns-updater_*_arm64.deb
```

**Post-installation setup (both architectures):**
```bash
# Install dependencies if needed
sudo apt-get install -f

# Run setup (non-interactive mode available)
sudo /usr/share/ddns-updater/install-systemd.sh

# Or for automated/non-interactive setup:
sudo /usr/share/ddns-updater/install-systemd-advanced.sh
```

> **Note**: The interactive setup will ask for configuration details like your domain name, nginx config paths, and update intervals. Use the advanced script for automated deployments.

#### Option 2: Build from Source
```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Clone and build
git clone https://github.com/koenieee/ddns_local_server.git
cd ddns_local_server
cargo build --release

# Install binary
sudo cp target/release/ddns_updater /usr/local/bin/

# Create systemd services manually (optional)
sudo cp systemd/*.service /etc/systemd/system/
sudo cp systemd/*.timer /etc/systemd/system/
sudo systemctl daemon-reload
```

### Installation Notes

#### Setup Script Details
- **Interactive setup** (`install-systemd.sh`): Prompts for configuration
  - Domain/host name to monitor
  - Nginx configuration file path
  - Update interval (default: 5 minutes)
  - Backup retention settings
- **Advanced setup** (`install-systemd-advanced.sh`): Non-interactive with defaults
  - Uses sensible defaults for automated deployments
  - Can be customized by editing the script before running

#### Testing Installation
```bash
# Test the installation
ddns_updater --host google.com --config /path/to/nginx.conf --verbose --no-reload

# Check systemd status
sudo systemctl status ddns-updater.service
sudo systemctl status ddns-updater.timer
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

## 🏗️ Architecture

The DDNS updater is built using **Clean Architecture** principles with a trait-based design that supports multiple web servers and provides excellent testability and maintainability.

### Architecture Diagrams

[![System Architecture](https://img.shields.io/badge/View-System%20Architecture-blue?style=for-the-badge)](docs/images/system-architecture.svg)
[![Clean Architecture](https://img.shields.io/badge/View-Clean%20Architecture-green?style=for-the-badge)](docs/images/clean-architecture.svg)
[![Data Flow](https://img.shields.io/badge/View-Data%20Flow-orange?style=for-the-badge)](docs/images/data-flow.svg)

| Diagram | Description |
|---------|-------------|
| **[System Architecture](docs/images/system-architecture.svg)** | High-level overview of the entire system showing external dependencies and internal components |
| **[Clean Architecture](docs/images/clean-architecture.svg)** | Detailed view of architectural layers and dependency inversion through traits |
| **[Data Flow](docs/images/data-flow.svg)** | Step-by-step sequence of a DDNS update from CLI input to completion |
| **[Component Interaction](docs/images/component-interaction.svg)** | Component relationships and communication patterns |
| **[State Diagram](docs/images/state-diagram.svg)** | State machine representation of the update process |
| **[Deployment](docs/images/deployment.svg)** | Production deployment view with systemd integration |

### Key Architectural Features

- **🔄 Clean Architecture**: Domain-driven design with dependency inversion
- **🔧 Multi-Web Server Support**: Nginx ✅, Apache ✅, Caddy 🔲, Traefik 🔲
- **⚡ Async/Await**: Full async support with tokio runtime
- **🧪 Testable Design**: Each layer can be tested independently
- **🔌 Plugin Architecture**: Easy to extend with new web server types

📚 **[View Complete Architecture Documentation →](docs/README.md)**

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