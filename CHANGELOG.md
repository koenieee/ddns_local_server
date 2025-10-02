# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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