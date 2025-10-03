# Systemd Service Files

This directory contains all systemd-related files for the DDNS Updater.

## Contents

### Service Files
- `ddns-updater.target` - Service group target for unified management
- `ddns-updater.service` - Main DDNS updater service
- `ddns-updater.timer` - Timer for periodic execution (every 5 minutes)
- `ddns-updater@.service` - Template service for multiple hosts
- `ddns-updater@.timer` - Template timer for multiple hosts
- `ddns-backup-cleanup.service` - Backup cleanup service (grouped with main)
- `ddns-backup-cleanup.timer` - Daily backup cleanup timer (grouped with main)

### Installation Scripts
- `install-systemd.sh` - Interactive installation script with customizable configuration
- `install-systemd-advanced.sh` - Advanced multi-host template services installation
- `uninstall-systemd.sh` - Remove all systemd components
- `ddns-backup-cleanup.sh` - Standalone backup cleanup utility

### Documentation
- `README.md` - This file with usage instructions
- `SERVICE_GROUPING.md` - Service grouping and dependency architecture
- `BACKUP_CLEANUP.md` - Comprehensive backup cleanup documentation
- `INTEGRATION_SUMMARY.md` - Complete feature summary

### Configuration Examples
- `examples/ddns-updater-google-com` - Example config for google.com
- `examples/ddns-updater-example-com` - Example config for example.com

### Documentation
- `SYSTEMD.md` - Comprehensive systemd setup and management guide

## Quick Start

### Installation Options

#### Native Installation
```bash
# Interactive mode - asks for all configuration options
sudo ./systemd/install-systemd.sh

# Non-interactive mode - uses sensible defaults
sudo ./systemd/install-systemd.sh -y

# Show help and default configuration
./systemd/install-systemd.sh --help
```

### Advanced Installation (Multiple Hosts)
```bash
# Option 1: Run from project root directory
sudo ./systemd/install-systemd-advanced.sh

# Option 2: Run from systemd directory
cd systemd && sudo ./install-systemd-advanced.sh

# Enable services for specific hosts
systemctl enable ddns-updater@google-com.timer
systemctl start ddns-updater@google-com.timer
```

### Uninstall
```bash
# Option 1: Run from project root directory
sudo ./systemd/uninstall-systemd.sh

# Option 2: Run from systemd directory
cd systemd && sudo ./uninstall-systemd.sh
```

## Interactive Configuration

The installation script now supports full interactive configuration:

### Configuration Options
- **Host Selection**: Choose which hostname to monitor (default: google.com)
- **Config Mode**: Single file or directory of config files
- **Backup Directory**: Where to store config backups (default: /var/backups/nginx)
- **Backup Cleanup**: Automatic cleanup of old backup files with configurable retention
- **Update Interval**: From 1 minute to 1 hour, or custom intervals
- **Verbose Logging**: Enable detailed logs for troubleshooting
- **Auto-Reload**: Automatically reload nginx after config changes

## Management Commands

### Backup Cleanup Management

The installation can optionally include automatic backup cleanup:

```bash
# Check cleanup status
systemctl status ddns-backup-cleanup.timer

# View cleanup logs
journalctl -u ddns-backup-cleanup.service -f

# Manual cleanup (dry run)
/usr/bin/ddns-backup-cleanup --dry-run --verbose

# Manual cleanup (actual)
systemctl start ddns-backup-cleanup.service

# Disable automatic cleanup
systemctl disable ddns-backup-cleanup.timer
```

## Service Group Management

The installation creates a unified service group for better management:

```bash
# Check overall service group status
systemctl status ddns-updater.target

# Start/stop entire service group
systemctl start ddns-updater.target
systemctl stop ddns-updater.target

# Enable/disable service group
systemctl enable ddns-updater.target
systemctl disable ddns-updater.target
```

## Management Commands

```bash
# Check status
systemctl status ddns-updater.timer

# View logs
journalctl -u ddns-updater.service -f

# Manual run
systemctl start ddns-updater.service
```

For detailed information, see [SYSTEMD.md](SYSTEMD.md).