# DDNS Updater Configuration Display

## Overview
This document explains how to view and monitor the DDNS updater configuration using systemctl and the included tools.

## Configuration Visibility in systemctl status

When you run `systemctl status ddns-updater.service`, you will now see:

1. **Service Description**: Shows what the service does
2. **Configuration Pre-check**: Before the service runs, it displays the current configuration
3. **Command Line**: The actual ExecStart command with all parameters
4. **Environment Variables**: All DDNS_* environment variables are set and visible

### Example Output

```bash
$ systemctl status ddns-updater.service

● ddns-updater.service - DDNS Updater - Nginx Allow List Manager
     Loaded: loaded (/etc/systemd/system/ddns-updater.service; enabled)
     Active: active (exited) since Mon 2025-09-30 08:30:15 CEST; 2min ago
    Process: 12345 ExecStartPre=/bin/sh -c echo "DDNS Configuration: Host=google.com, Config=2(/etc/nginx/sites-available), Pattern=*.conf, Interval=5min, Verbose=true, Cleanup=true" (code=exited, status=0/SUCCESS)
    Process: 12346 ExecStart=/usr/bin/ddns-updater --host google.com --config-dir /etc/nginx/sites-available --pattern *.conf --backup-dir /var/backups/nginx --verbose (code=exited, status=0/SUCCESS)

Sep 30 08:30:15 hostname ddns-updater[12345]: DDNS Configuration: Host=google.com, Config=2(/etc/nginx/sites-available), Pattern=*.conf, Interval=5min, Verbose=true, Cleanup=true
Sep 30 08:30:15 hostname ddns-updater[12346]: Starting DDNS update check...
```

## Environment Variables

The following environment variables are set in the systemd service and visible in the status:

### Main Service Variables
- `DDNS_HOST`: The hostname being monitored (e.g., "google.com")
- `DDNS_CONFIG_MODE`: Configuration mode (1=single file, 2=directory)
- `DDNS_CONFIG_DIR`: Directory containing nginx config files (when mode=2)
- `DDNS_CONFIG_FILE`: Single config file path (when mode=1)
- `DDNS_PATTERN`: File pattern for directory mode (e.g., "*.conf")
- `DDNS_BACKUP_DIR`: Directory where backups are stored
- `DDNS_INTERVAL`: Update check interval (e.g., "5min")
- `DDNS_VERBOSE`: Whether verbose logging is enabled (true/false)
- `DDNS_AUTO_RELOAD`: Whether nginx auto-reload is enabled (true/false)
- `DDNS_CLEANUP_ENABLED`: Whether backup cleanup is enabled (true/false)
- `DDNS_CLEANUP_DAYS`: Backup retention period in days (when cleanup enabled)

### Template Service Variables (ddns-updater@.service)
- `DDNS_INSTANCE`: The instance name (e.g., "google-com")
- All the above variables plus instance-specific overrides

### Cleanup Service Variables
- `DDNS_CLEANUP_BACKUP_DIR`: Directory to clean up
- `DDNS_CLEANUP_RETENTION_DAYS`: How many days to keep backups
- `DDNS_CLEANUP_VERBOSE`: Whether verbose cleanup logging is enabled

## Configuration Display Tools

### 1. show-config.sh Script

Use the included script for a comprehensive configuration overview:

```bash
# From installed package
/usr/share/ddns-updater/show-config.sh

# From development directory  
./systemd/show-config.sh
```

This script shows:
- Service status and health
- All environment variables and their values
- Timer information (next run, last run)
- Template instance details
- Management commands

### 2. Direct systemctl Commands

```bash
# View current configuration environment
systemctl show ddns-updater.service --property=Environment

# View detailed status with configuration
systemctl status ddns-updater.service -l --no-pager

# View timer scheduling
systemctl status ddns-updater.timer

# View logs with configuration context
journalctl -u ddns-updater.service -n 20
```

### 3. Real-time Monitoring

```bash
# Watch service status changes
watch -n 5 'systemctl status ddns-updater.service --no-pager'

# Follow logs in real-time
journalctl -u ddns-updater.service -f

# Monitor timer activations
journalctl -u ddns-updater.timer -f
```

## Installation-Specific Configuration

When you run the installation scripts (`install-systemd.sh` or `install-systemd-advanced.sh`), they:

1. **Collect Configuration**: Ask for your specific settings (host, directories, intervals, etc.)
2. **Generate Service Files**: Create customized systemd service files with your settings
3. **Set Environment Variables**: Populate all DDNS_* variables based on your choices
4. **Enable Pre-check**: Add ExecStartPre commands to display configuration before each run

## Troubleshooting Configuration Issues

### Check Current Settings
```bash
# Quick overview
/usr/share/ddns-updater/show-config.sh

# Detailed environment
systemctl show ddns-updater.service --property=Environment --value | grep DDNS_
```

### Verify Configuration Matches Expectations
```bash
# Compare what's configured vs what's running
systemctl cat ddns-updater.service | grep -E "(ExecStart|Environment)"
```

### Update Configuration
```bash
# Re-run installation to update settings
sudo /usr/share/ddns-updater/install-systemd.sh

# Or manually edit and reload
sudo systemctl edit ddns-updater.service
sudo systemctl daemon-reload
sudo systemctl restart ddns-updater.service
```

## Benefits

1. **Immediate Visibility**: See configuration without digging through files
2. **Easy Debugging**: Quickly verify what settings are actually being used
3. **Monitoring Integration**: Configuration visible in monitoring tools that parse systemctl output
4. **Audit Trail**: Configuration changes are logged and visible in service status
5. **Template Support**: Instance-specific configuration clearly displayed for template services