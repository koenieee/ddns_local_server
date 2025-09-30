# DDNS Updater - Debian Package

This directory contains the Debian packaging files for creating a `.deb` package of the DDNS updater.

## Building the Package

### Prerequisites

Install the required build tools:

```bash
sudo apt update
sudo apt install dpkg-dev debhelper cargo rustc pkg-config
```

### Build Process

1. **Automated Build (Recommended)**:
   ```bash
   ./build-deb.sh
   ```

2. **Manual Build**:
   ```bash
   # Build the Rust binary
   cargo build --release
   
   # Build the Debian package
   dpkg-buildpackage -us -uc -b
   ```

## Package Structure

### Debian Control Files

- `control` - Package metadata and dependencies
- `changelog` - Version history and changes
- `copyright` - License and copyright information
- `rules` - Build rules and installation instructions
- `compat` - Debhelper compatibility level

### Package Scripts

- `postinst` - Post-installation script (creates user, directories, etc.)
- `prerm` - Pre-removal script (stops services)
- `postrm` - Post-removal script (cleanup on purge)

### Installation Layout

The package installs files to the following locations:

```
/usr/bin/ddns-updater                          # Main binary
/usr/bin/ddns-backup-cleanup                   # Backup cleanup utility
/usr/share/ddns-updater/                       # Installation scripts
/usr/share/doc/ddns-updater/                   # Documentation
/lib/systemd/system/                           # Systemd service files
/etc/ddns-updater/                             # Configuration directory
/var/lib/ddns-updater/                         # Data directory
/var/log/ddns-updater/                         # Log directory
```

## Installation and Usage

### Installing the Package

```bash
# Install with dependency resolution
sudo apt install ./ddns-updater_*.deb

# Or install directly
sudo dpkg -i ddns-updater_*.deb
```

### Post-Installation Setup

After installation, configure the service:

```bash
# Interactive setup
sudo /usr/share/ddns-updater/install-systemd.sh

# Advanced multi-host setup
sudo /usr/share/ddns-updater/install-systemd-advanced.sh
```

### Package Management

```bash
# Check package status
dpkg -l ddns-updater

# View package information
dpkg --info ddns-updater_*.deb

# List package contents
dpkg --contents ddns-updater_*.deb

# Remove package (keeps configuration)
sudo apt remove ddns-updater

# Purge package (removes everything)
sudo apt purge ddns-updater
```

## Features

The Debian package provides:

### ✅ **Complete System Integration**
- Systemd service files with proper dependencies
- Service grouping for unified management
- Automated backup cleanup with timers
- System user and group creation
- Proper file permissions and security

### ✅ **Professional Installation**
- Pre and post installation scripts
- Dependency management
- Configuration directory setup
- Log directory creation
- Service registration

### ✅ **Easy Management**
- Interactive installation scripts
- Advanced multi-host deployment
- Comprehensive documentation
- Example configurations
- Uninstallation support

### ✅ **Production Ready**
- Follows Debian packaging standards
- Proper file layout and permissions
- Security hardening
- Comprehensive cleanup on removal
- Professional metadata and documentation

## Package Details

- **Package Name**: `ddns-updater`
- **Architecture**: `any` (builds for target architecture)
- **Section**: `net` (networking utilities)
- **Priority**: `optional`
- **Dependencies**: `systemd`, `curl`, `ca-certificates`
- **Recommends**: `nginx`

## Maintenance

### Updating the Package

1. Update version in `debian/changelog`
2. Update any dependencies in `debian/control`
3. Test the build process
4. Rebuild the package

### Adding New Features

1. Update `debian/rules` if new files need installation
2. Update `debian/control` if new dependencies are required
3. Update documentation files
4. Test installation and removal

## Quality Assurance

The build script includes validation for:

- ✅ Build dependency availability
- ✅ Required file presence
- ✅ Script syntax validation
- ✅ Binary compilation
- ✅ Package metadata validation

## Support

The package includes comprehensive documentation and examples to help with:

- Initial setup and configuration
- Service management and monitoring
- Troubleshooting common issues
- Advanced deployment scenarios