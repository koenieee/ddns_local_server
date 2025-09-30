# DDNS Updater - Systemd Service Setup

This directory contains systemd service files and installation scripts for running the DDNS Updater as a system service.

## Files

- `ddns-updater.service` - Main systemd service unit file
- `ddns-updater.timer` - Timer unit file for periodic execution
- `install-systemd.sh` - Installation script
- `uninstall-systemd.sh` - Uninstallation script

## Quick Installation

1. **Build and install the service:**
   ```bash
   sudo ./install-systemd.sh
   ```

2. **Check service status:**
   ```bash
   systemctl status ddns-updater.timer
   ```

3. **View logs:**
   ```bash
   journalctl -u ddns-updater.service -f
   ```

## Configuration

### Default Settings

The service is configured with these defaults:
- **Host:** `google.com` (change in service file)
- **Config Directory:** `/etc/nginx/sites-available`
- **Pattern:** `*.conf`
- **Backup Directory:** `/var/backups/nginx`
- **Run Interval:** Every 5 minutes
- **Initial Delay:** 2 minutes after boot

### Customizing the Service

Edit the service file after installation:
```bash
sudo systemctl edit ddns-updater.service
```

Or modify the main service file:
```bash
sudo nano /etc/systemd/system/ddns-updater.service
```

Common customizations:

**Change the host:**
```ini
ExecStart=/usr/local/bin/ddns_updater --host your-domain.com --config-dir /etc/nginx/sites-available
```

**Use a single config file:**
```ini
ExecStart=/usr/local/bin/ddns_updater --host your-domain.com --config /etc/nginx/sites-available/your-site.conf
```

**Change backup location:**
```ini
ExecStart=/usr/local/bin/ddns_updater --host your-domain.com --config-dir /etc/nginx/sites-available --backup-dir /your/backup/path
```

**Multiple hosts (requires multiple service files):**
```bash
# Copy and modify service file for each host
sudo cp /etc/systemd/system/ddns-updater.service /etc/systemd/system/ddns-updater-domain2.service
sudo nano /etc/systemd/system/ddns-updater-domain2.service
```

### Customizing the Timer

Edit the timer file to change execution frequency:
```bash
sudo nano /etc/systemd/system/ddns-updater.timer
```

Examples:
- Every minute: `OnUnitActiveSec=1min`
- Every 10 minutes: `OnUnitActiveSec=10min`
- Every hour: `OnUnitActiveSec=1h`
- Daily at 3 AM: `OnCalendar=*-*-* 03:00:00`

After changes, reload and restart:
```bash
sudo systemctl daemon-reload
sudo systemctl restart ddns-updater.timer
```

## Management Commands

### Service Management
```bash
# Start the timer
sudo systemctl start ddns-updater.timer

# Stop the timer
sudo systemctl stop ddns-updater.timer

# Enable on boot
sudo systemctl enable ddns-updater.timer

# Disable on boot
sudo systemctl disable ddns-updater.timer

# Run service manually (one-time)
sudo systemctl start ddns-updater.service
```

### Monitoring
```bash
# Check timer status
systemctl status ddns-updater.timer

# Check service status
systemctl status ddns-updater.service

# View recent logs
journalctl -u ddns-updater.service -n 50

# Follow logs in real-time
journalctl -u ddns-updater.service -f

# View logs since today
journalctl -u ddns-updater.service --since today
```

### Timer Information
```bash
# List all timers
systemctl list-timers

# Show when the timer will run next
systemctl list-timers ddns-updater.timer
```

## Security Features

The service includes several security hardening features:
- Runs with minimal privileges
- Protected system access
- Private temporary directories
- Read-only file system access (except for specific paths)
- Restricted namespace access

## Troubleshooting

### Service Won't Start
```bash
# Check service status for errors
systemctl status ddns-updater.service

# View detailed logs
journalctl -u ddns-updater.service --no-pager

# Test the binary manually
sudo /usr/local/bin/ddns_updater --help
```

### Permission Issues
```bash
# Ensure backup directory exists and is writable
sudo mkdir -p /var/backups/nginx
sudo chmod 755 /var/backups/nginx

# Check nginx config directory permissions
ls -la /etc/nginx/sites-available
```

### Timer Not Running
```bash
# Check if timer is active
systemctl is-active ddns-updater.timer

# Check timer configuration
systemctl cat ddns-updater.timer

# Reload if you made changes
sudo systemctl daemon-reload
sudo systemctl restart ddns-updater.timer
```

## Uninstallation

To remove the service completely:
```bash
sudo ./uninstall-systemd.sh
```

This will:
- Stop and disable the service and timer
- Remove systemd files
- Remove the binary
- Optionally remove backup directory

## Log Analysis

### Common Log Messages

**Successful IP update:**
```
IP address has changed! Updating nginx allow list...
Nginx config updated successfully
```

**No change needed:**
```
No IP change detected. Nginx config unchanged.
```

**Configuration validation:**
```
✓ Validated nginx config file: /etc/nginx/sites-available/example.conf
```

### Log Levels

The service logs to systemd journal with different levels:
- **INFO**: Normal operation messages
- **WARN**: Non-fatal issues (backup failures, etc.)
- **ERROR**: Fatal errors that prevent operation

Filter logs by priority:
```bash
# Only errors
journalctl -u ddns-updater.service -p err

# Warnings and errors
journalctl -u ddns-updater.service -p warning
```