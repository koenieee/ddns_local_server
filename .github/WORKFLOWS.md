# GitHub Actions Workflows

This directory contains GitHub Actions workflows for the DDNS Updater project.

> 📋 **For main project documentation, see the [root README.md](../README.md)**

## Workflows

### `ci.yml` - Main CI/CD Pipeline
**Triggers:** Push to main/develop, Pull Requests, Releases

**Jobs:**
- **Test Suite**: Runs tests, linting, and code formatting checks on stable and beta Rust
- **Build**: Cross-compiles binaries for Linux (AMD64, ARM64, MUSL)
- **Build Debian Package**: Creates .deb package for distribution
- **Security Audit**: Checks for known vulnerabilities
- **Release**: Creates GitHub releases with binaries and packages
- **Cleanup**: Removes old artifacts

### `nightly.yml` - Nightly Testing
**Triggers:** Daily at 2 AM UTC, Manual dispatch

**Jobs:**
- **Nightly Test**: Tests with Rust nightly toolchain
- **Dependency Check**: Scans for vulnerabilities and outdated dependencies
- **Benchmark**: Performance and binary size testing

### `docs.yml` - Documentation Validation
**Triggers:** Changes to documentation files

**Jobs:**
- **Validate Documentation**: Checks markdown links and documentation structure
- **Check Debian Package**: Validates Debian packaging files
- **Validate Scripts**: Syntax checking for shell scripts and systemd files

### `release.yml` - Release Management
**Triggers:** Manual workflow dispatch

**Jobs:**
- **Prepare Release**: Updates version numbers, creates tags, and GitHub releases

## Configuration Files

### `dependabot.yml`
Configures automatic dependency updates for:
- Rust/Cargo dependencies (weekly)
- GitHub Actions (weekly)

### `markdown-link-config.json`
Configuration for markdown link checking, ignoring localhost and local IPs.

## Issue Templates

### `bug_report.yml`
Structured bug report template with:
- Problem description and expected behavior
- Steps to reproduce
- Environment information
- Log output sections

### `feature_request.yml`
Feature request template with:
- Problem and solution description
- Use case and priority assessment
- Implementation willingness

### `pull_request_template.md`
PR template with:
- Change type classification
- Testing checklist
- Code quality checklist

## Usage

### Running Workflows Manually

```bash
# Trigger nightly build
gh workflow run nightly.yml

# Create a release
gh workflow run release.yml -f version=v1.0.0 -f prerelease=false
```

### Viewing Results

```bash
# Check workflow status
gh run list

# View specific run
gh run view <run-id>

# Download artifacts
gh run download <run-id>
```

## Artifacts

The workflows generate the following artifacts:

- **Build Artifacts**: Cross-compiled binaries (30 days retention)
- **Debian Package**: .deb package (90 days retention)
- **Documentation**: Generated documentation overview (30 days retention)
- **Reports**: Dependency and benchmark reports (7 days retention)

## Security

- Uses minimal permissions principle
- Caches dependencies for faster builds
- Validates input parameters
- Uses official GitHub Actions where possible
- Generates checksums for release artifacts

## Maintenance

- Dependabot automatically updates dependencies
- Workflows clean up old artifacts automatically
- Security audits run nightly
- Performance benchmarks track binary size and startup time