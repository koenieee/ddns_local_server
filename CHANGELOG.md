# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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