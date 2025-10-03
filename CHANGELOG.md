# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.2.6] - 2025-10-03

### Added
- **Selective Config Processing**: Major performance optimization for multi-config setups
  - Smart IP change detection: Only process configs when IP actually changes
  - File-specific validation: Only update configs that contain the old IP address
  - Conditional backup creation: Only create backups when files will actually be modified
  - Early termination: Skip all processing when no IP change is detected
  - Significantly improved performance when managing many configuration files

### Enhanced
- **Security Hardening**: Restricted systemd file permissions to specific nginx subdirectories
  - Limited write access from `/data/nginx` to `/data/nginx/proxy_host` only
  - Applied principle of least privilege to systemd ReadWritePaths configuration
  - Enhanced Docker security documentation with permission scope clarification
  - Updated both standard and templated systemd service files

### Performance
- **Optimized Multi-Config Workflows**: 
  - Processes only configs that need updates instead of all configs blindly
  - Eliminates unnecessary file I/O operations and backup creation
  - Reduces server reload frequency by skipping unchanged configurations
  - Maintains data safety while improving efficiency for large deployments

## [1.2.5] - 2025-10-03

### Fixed
- **Debian Package Installation**: Fixed postinst script syntax error during package installation
  - Corrected shebang position to be the first line in debian/postinst script
  - Fixed "Syntax error: '(' unexpected (expecting ';')" during dpkg install
  - Set proper executable permissions on postinst script
- **Test Suite CI/CD Compatibility**: Fixed configuration validation tests for containerized environments
  - Added DDNS_TEST_MODE environment variable to all validation tests
  - Prevents "Cannot access /var/lib/ddns-updater" errors in CI/CD pipelines
  - Uses test storage directory instead of system directories during testing

### Enhanced
- **Package Reliability**: Improved Debian package installation process
- **Testing Infrastructure**: Better CI/CD and Docker container compatibility for test suites

## [1.2.4] - 2025-01-03

### Added
- **Docker Support**: Comprehensive Docker deployment capabilities
  - Dynamic ReadWritePaths configuration in systemd services based on nginx directories
  - Automated permission troubleshooting script (`scripts/fix-docker-permissions.sh`)
  - Complete Docker deployment guide in `DOCKER.md`
  - Support for Docker volume mounting with proper permission handling

### Fixed
- **Docker Container Compatibility**: Resolved "Read-only file system" errors in containerized environments
- **CI/CD Test Mode**: Added `DDNS_TEST_MODE` environment variable for local storage testing
  - Automatically uses `/tmp/ddns-updater` for storage when enabled
  - Prevents filesystem access errors in CI/CD pipelines
- **Root Group Security**: Enhanced handling of root group ownership
  - Smart detection of root group ownership on nginx directories
  - Security-conscious user management without compromising access
  - Proper warning messages for security-sensitive configurations

### Enhanced
- **Nginx Directory Detection**: Improved multi-directory pattern recognition
  - Support for `/etc/nginx/sites-available`, `/etc/nginx/conf.d`, and custom paths
  - Dynamic systemd service configuration based on detected directories
- **Permission Management**: Advanced Docker and traditional deployment permission handling
  - Automatic user group assignment based on directory ownership
  - Comprehensive permission diagnostics and troubleshooting

### Documentation
- **Docker Guide**: Complete containerization documentation with examples
- **Troubleshooting**: Enhanced permission debugging tools and guides

## [1.2.3] - 2025-10-03

### Fixed
- **Storage Directory Accessibility**: Fixed "Cannot access /var/lib/ddns-updater" error
  - Improved directory writeability test to check actual target directory
  - Fixed Debian package permissions (755 instead of 750) for storage directory
  - Changed ownership to root:root for better compatibility
  - Added detailed error messages with Unix permission information
  - Enhanced verbose logging for storage directory selection

### Changed
- **Better Error Diagnostics**: CLI now shows actual directory permissions when access fails
- **Debian Package Installation**: More reliable directory creation with proper permissions

## [1.2.2] - 2025-10-03

### Changed
- **Persistent Storage Enforcement**: Removed /tmp fallback to ensure data persistence
  - Application now only uses /var/lib/ddns-updater for JSON storage
  - Eliminates data loss from temporary directory cleanup
  - Provides clear error messages when persistent storage unavailable
- **Enhanced Installation**: Installation script now creates storage directory automatically
  - Proper permissions and ownership configuration
  - Eliminates manual setup steps for storage directory
- **Improved systemd Configuration**: Updated service files for persistent storage only
  - Removed /tmp/ddns-updater from ReadWritePaths
  - Ensures consistent storage location across all deployments

### Added
- **Comprehensive Documentation**: Added PERSISTENT_STORAGE.md
  - Detailed explanation of storage configuration
  - Troubleshooting guide for storage issues
  - Migration instructions for existing installations

### Fixed
- Data persistence across system reboots and maintenance
- Consistent storage behavior between different installation methods

## [1.2.1] - 2025-10-03

### Changed
- **Efficient Backup Creation**: Backups are now only created when IP actually changes
  - Prevents unnecessary backup files when no configuration changes occur
  - Reduces disk usage and improves performance for repeated runs
- **Enhanced Package Removal**: Comprehensive cleanup on package removal/purge
  - Removes all system files, directories, and temporary files
  - Properly stops and disables all systemd services including instance services
  - Thorough cleanup of backup files and runtime data

### Fixed
- Backup files no longer created for no-change scenarios
- Package removal now cleans up all traces of the application

## [1.2.1] - 2025-10-03

### Added
- **Full Absolute Path Logging**: Enhanced JSON file path display with complete filesystem paths
  - Shows absolute paths instead of relative paths for all JSON file operations
  - Improved debugging visibility for file storage locations
  - Better troubleshooting support with precise file path information

### Changed
- **Backup Optimization**: Backups now only created when IP actually changes
  - Reduces unnecessary backup file creation
  - Cleaner backup directory management
  - More efficient operation when no changes are needed

## [1.2.0] - 2025-10-03

### Added
- **Smart JSON Storage**: Automatic creation of IP tracking files when none exist
  - Creates JSON files with hostname, IP, and timestamps automatically
  - Prevents duplicate work by storing resolved IP addresses
- **Intelligent Config Checking**: Pre-update verification of IP presence in config files
  - Checks if current IP already exists in nginx/apache configurations
  - Avoids unnecessary updates when IP is already correctly configured
- **Non-Intrusive Behavior**: Enhanced update logic for better reliability
  - Only updates existing allow/deny entries, never adds new ones automatically
  - Provides clear feedback when no changes are needed
- **Enhanced Apache Support**: Extended IP checking for Apache configurations
  - Supports both "Allow from" and "Require ip" directive formats
  - Consistent behavior between nginx and Apache handlers

### Changed
- **Update Logic**: Improved decision-making process for config updates
  - Better handling of first-run scenarios without existing JSON files
  - More granular control over when configuration changes occur
- **Debug Logging**: Enhanced debugging output for troubleshooting
  - Clearer messages about JSON file creation and IP checking
  - Better visibility into decision-making process

## [1.1.9] - 2025-01-03

### Fixed
- **Release Workflow**: Fixed directory navigation in Debian package workflow
  - Added proper `cd ..` after processing .deb files
  - Fixed "No such file or directory" error in final listing step
  - Ensures release directory is always accessible for final operations

## [1.1.8] - 2025-01-03

### Fixed
- **Release Workflow**: Fixed Debian package handling in automated releases
  - Added proper error handling for missing .deb files
  - Corrected .deb file location detection (parent directory)
  - Graceful fallback when Debian package build fails
  - Prevents sha256sum errors on missing files

## [1.1.7] - 2025-01-03

### Added
- **Debian Package**: Automated release now builds and includes .deb package
  - Added Debian package building to GitHub Actions workflow
  - Includes systemd services, configuration files, and proper dependencies
  - Easy installation with `sudo dpkg -i ddns-updater_*.deb`
  - Automatic user/group creation and service setup

### Changed
- **Release Process**: Enhanced automated release with Debian package support
- **Installation**: Updated release notes with Debian package installation instructions

## [1.1.6] - 2025-01-03

### Security
- **Dependency Updates**: Fixed critical security vulnerabilities
  - Updated `lettre` from 0.10.4 to 0.11.18
  - Fixed RUSTSEC-2024-0421: `idna` vulnerability (updated to 1.1.0)
  - Fixed RUSTSEC-2025-0009: `ring` AES panic vulnerability (updated to 0.17.14)
  - Fixed RUSTSEC-2025-0010: `ring` unmaintained warning (updated to maintained version)
  - All security audits now pass with zero vulnerabilities

### Changed
- **Email System**: Updated to use latest `lettre` crate with improved security
- **Build Dependencies**: All dependencies updated to latest secure versions

## [1.1.5] - 2025-01-03

### Fixed
- **GitHub Actions**: Fixed YAML syntax error in automated-release workflow
  - Corrected multi-line string handling in changelog extraction
  - Workflow now validates and executes properly
  - Automated releases should work correctly

## [1.1.4] - 2025-01-03

### Changed
- **Backup Retention**: Reduced default backup retention from 30 days to 3 days
  - Updated systemd service configuration to use 3-day retention
  - Updated backup cleanup script default from 30 to 3 days
  - Updated installation script options (1, 3, 7, 14 days with 3 as recommended)
  - More aggressive cleanup prevents disk space accumulation
- **Code Quality**: Applied consistent formatting for release build

## [1.1.3] - 2025-01-03

### Fixed
- **Backup Cleanup Service**: Fixed executable path in systemd service
  - Corrected ExecStart path from `/usr/local/bin/ddns-backup-cleanup` to `/usr/bin/ddns-backup-cleanup`
  - Resolves "Failed to locate executable" error when using Debian package installation
  - Backup cleanup service now works correctly with automated cleanup functionality

## [1.1.2] - 2025-10-03

### Fixed
- **Critical Storage Directory Issue**: Resolved "Read-only file system" error in systemd service
  - Fixed storage directory logic to use proper system directories
  - Updated systemd service ReadWritePaths configuration
  - Corrected directory ownership in Debian package installation
  - Added fallback to /tmp/ddns-updater when /var/lib unavailable

### Added
- **Storage Permission Fix Script**: `scripts/fix-storage-permissions.sh`
  - Automated repair tool for existing installations
  - Comprehensive directory setup and permission fixing
  - Troubleshooting guide for common issues

### Improved
- **System Integration**: Better systemd service directory handling
- **Error Handling**: Clearer fallback logic for storage directories
- **Installation Process**: More robust Debian package post-installation

## [1.1.1] - 2025-10-02

### Added
- **Automated Release System**: Tag-based releases with GitHub Actions
  - Automatic binary builds for x86_64 and ARM64 architectures
  - SHA256 checksum generation for asset verification
  - Changelog extraction and release notes generation
  - Cross-platform compatibility testing

### Improved
- **Release Process**: Streamlined from manual to fully automated
- **Documentation**: Added comprehensive automated release guide
- **User Experience**: Consistent release assets and installation instructions

## [1.1.0] - 2025-10-02

### Added
- **Email Notifications**: Complete email notification system for IP changes
  - New `EmailConfig` entity with SMTP configuration support
  - `EmailNotificationService` with trait-based architecture
  - CLI integration with `--email-config` and `--email-enabled` flags
  - Support for SMTP authentication, TLS, and custom templates
  - Comprehensive test coverage for email functionality

### Changed
- **Rust Version**: Upgraded from Rust 1.70.0 to 1.82.0 for modern dependency compatibility
- **GitHub Actions**: Updated all workflows to use Ubuntu 22.04 and Rust 1.82.0
- **Dependencies**: Updated to latest compatible versions for better reliability
- **Architecture**: Maintained clean architecture patterns with dependency inversion

### Fixed
- **Debian 12 Compatibility**: Resolved glibc compatibility issues (now targets glibc 2.34)
- **MSRV Issues**: Fixed `std::io::Error::other` usage for Rust 1.82.0 compatibility
- **Cargo.lock Version**: Fixed lockfile version compatibility with older Cargo versions
- **GitHub Actions**: Resolved runner availability issues by switching from ubuntu-20.04 to ubuntu-22.04
- **Cross Compilation**: Fixed cross-compilation tool installation with consistent Rust versions
- **Let Chains**: Updated Rust 2024 to 2021 edition syntax compatibility

### Technical
- **Build System**: Enhanced build scripts for Debian 12 deployment
- **Testing**: All 25 unit tests + 6 integration tests passing
- **Documentation**: Added comprehensive setup and deployment guides
- **CI/CD**: Improved workflow reliability and build reproducibility

## [1.0.0] - 2025-09-30

### Added
- Initial release of DDNS Updater
- Core DDNS functionality for IP address updates
- Support for Nginx and Apache configuration updates
- CLI interface with comprehensive argument parsing
- Clean architecture with domain-driven design
- Comprehensive test suite
- Debian package support
- Systemd service integration