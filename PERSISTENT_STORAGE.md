# Persistent Storage Configuration

## Overview

This document explains the persistent storage configuration for DDNS Updater and the changes made to ensure JSON files are stored in a persistent location.

## Problem

Previously, the DDNS Updater would fall back to `/tmp/ddns-updater` when `/var/lib/ddns-updater` was not accessible. This caused several issues:

1. **Data Loss**: Files in `/tmp` are typically cleared on reboot
2. **Inconsistent Behavior**: Service behavior differed between installations
3. **systemd PrivateTmp**: With `PrivateTmp=true`, the `/tmp` directory is isolated and not persistent

## Solution

### Changes Made

#### 1. Systemd Service Files
- **File**: `systemd/ddns-updater.service`
- **File**: `systemd/ddns-updater@.service`
- **Change**: Removed `/tmp/ddns-updater` from `ReadWritePaths`
- **Result**: Service can only write to `/var/lib/ddns-updater`

#### 2. Installation Script
- **File**: `systemd/install-systemd.sh`
- **Change**: Added creation of `/var/lib/ddns-updater` directory
- **Details**: 
  ```bash
  mkdir -p /var/lib/ddns-updater
  chmod 755 /var/lib/ddns-updater
  chown root:root /var/lib/ddns-updater
  ```

#### 3. CLI Interface
- **File**: `src/interface/cli_interface.rs`
- **Change**: Removed `/tmp/ddns-updater` fallback
- **Result**: Application exits with clear error message if persistent storage unavailable

#### 4. Error Handling
When `/var/lib/ddns-updater` is not accessible, the application now:
1. Displays a clear error message
2. Provides instructions for manual directory creation
3. Exits gracefully instead of using temporary storage

## Directory Structure

### Production Environment
```
/var/lib/ddns-updater/          # Persistent storage directory
├── hostname1.json              # IP data for hostname1
├── hostname2.json              # IP data for hostname2
└── ...                         # Additional hostname files
```

### Test Environment
```
./test_storage/                 # Local test directory (DDNS_TEST_MODE=1)
├── hostname1.json              # Test IP data
└── ...                         # Additional test files
```

## Permissions

| Directory | Owner | Group | Permissions | Purpose |
|-----------|-------|-------|-------------|---------|
| `/var/lib/ddns-updater` | root | root | 755 | JSON storage |
| `/var/log/ddns-updater` | ddns-updater | ddns-updater | 750 | Log files (Debian pkg) |
| `/etc/ddns-updater` | root | ddns-updater | 750 | Config files (Debian pkg) |

## Installation Methods

### 1. Systemd Installation Script
```bash
sudo ./systemd/install-systemd.sh
```
This automatically creates the required directories.

### 2. Debian Package
```bash
sudo dpkg -i ddns-updater_*.deb
```
The postinst script creates all required directories.

### 3. Manual Installation
If installing manually:
```bash
sudo mkdir -p /var/lib/ddns-updater
sudo chmod 755 /var/lib/ddns-updater
sudo chown root:root /var/lib/ddns-updater
```

## Benefits

1. **Data Persistence**: IP data survives reboots and system maintenance
2. **Predictable Behavior**: Service always uses the same storage location
3. **Better Error Handling**: Clear error messages when storage is unavailable
4. **Security**: No reliance on world-writable temporary directories
5. **Compliance**: Follows Linux filesystem hierarchy standards

## Troubleshooting

### Error: "Cannot access /var/lib/ddns-updater"
1. Check if directory exists: `ls -la /var/lib/ddns-updater`
2. Create if missing: `sudo mkdir -p /var/lib/ddns-updater`
3. Set permissions: `sudo chmod 755 /var/lib/ddns-updater`
4. Set ownership: `sudo chown root:root /var/lib/ddns-updater`

### Service fails to start
1. Check systemd logs: `journalctl -u ddns-updater.service`
2. Verify directory permissions
3. Ensure service has write access to `/var/lib/ddns-updater`

## Migration

For existing installations using `/tmp/ddns-updater`:
1. Copy any important JSON files to `/var/lib/ddns-updater`
2. Update systemd service files
3. Restart the service

## Version History

- **v1.2.1**: Removed `/tmp` fallback, enforced persistent storage
- **v1.2.0**: Enhanced logging with absolute paths
- **v1.0.0**: Initial release with `/tmp` fallback