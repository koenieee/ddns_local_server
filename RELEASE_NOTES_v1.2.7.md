# DDNS Updater v1.2.7 Release Notes

**Release Date**: October 3, 2025
**Type**: Critical Bug Fix + CI/CD Enhancement

## 🚨 Critical Fix

### Batch Processing Race Condition Resolved
- **Issue**: In multi-config file scenarios, only the first config file was being updated
- **Root Cause**: IP was stored immediately after first file update, causing subsequent files to see "no change needed"
- **Solution**: Implemented deferred IP storage - all files are processed first, then IP is stored once atomically
- **Impact**: **All config files now update consistently** when IP changes occur

## 🔧 Technical Improvements

### Enhanced Batch Processing Architecture
- Added `update_file_only()` methods in both application and domain layers
- Implemented atomic update semantics: all files update together or none do
- Improved error handling and rollback capabilities
- Enhanced logging for better debugging of batch operations

### Environment Variable Support
- Fixed `DDNS_STORAGE_DIR` environment variable handling in test mode
- Improved test isolation and reliability
- Better support for automated testing environments

## 🧪 CI/CD Enhancements

### Comprehensive Test Suite Improvements
- **Enhanced batch processing test**: Increased from 3 to 5 config files for better race condition detection
- **Dynamic IP resolution**: Tests adapt to changing DNS resolution (no hardcoded IPs)
- **Detailed error reporting**: Specific failure reasons for better debugging in CI environments
- **Integrated into main test suite**: Runs automatically in CI/CD pipelines

### Test Reliability
- Improved test infrastructure with better cleanup and isolation
- Enhanced error messages for actionable debugging
- Validates atomic update behavior across all configuration files

## 🎯 Production Impact

### Before v1.2.7 (Issue)
```
Config files: site1.conf, site2.conf, site3.conf
Old IP: 192.168.1.100, New IP: 85.146.26.129

Result: 
- site1.conf: ✅ Updated to 85.146.26.129
- site2.conf: ❌ Still has 192.168.1.100  
- site3.conf: ❌ Still has 192.168.1.100
```

### After v1.2.7 (Fixed)
```
Config files: site1.conf, site2.conf, site3.conf  
Old IP: 192.168.1.100, New IP: 85.146.26.129

Result:
- site1.conf: ✅ Updated to 85.146.26.129
- site2.conf: ✅ Updated to 85.146.26.129
- site3.conf: ✅ Updated to 85.146.26.129
```

## 📦 Installation

### Debian/Ubuntu Package
```bash
wget https://github.com/koenieee/ddns_local_server/releases/download/v1.2.7/ddns-updater_1.2.7-1_amd64.deb
sudo dpkg -i ddns-updater_1.2.7-1_amd64.deb
```

### From Source
```bash
git clone https://github.com/koenieee/ddns_local_server.git
cd ddns_local_server
git checkout v1.2.7
cargo build --release
```

## 🔄 Upgrade Notes

### For Production Users
- **Immediate upgrade recommended** if you use multiple config files
- The race condition fix ensures all your config files will be updated consistently
- No configuration changes required - fix is automatic

### For Developers/CI
- Enhanced test suite provides better validation of batch processing
- New environment variable support improves test isolation
- CI/CD pipelines will automatically benefit from improved test coverage

## ⚡ Performance & Compatibility

- **Performance**: No performance impact - fix maintains existing optimizations from v1.2.6
- **Compatibility**: Fully backward compatible with existing configurations
- **Dependencies**: No new dependencies added
- **Rust Version**: Still requires Rust 1.82+ (unchanged)

## 🧰 Developer Notes

### Key Changes
- New methods: `update_ddns_file_only()`, `update_file_only()`
- Enhanced `update_ddns_multiple()` with deferred IP storage
- Improved test infrastructure with dynamic IP detection
- Better error handling and logging throughout batch processing flow

### Testing
```bash
# Run the enhanced test suite
./scripts/test_all.sh

# Test specific batch processing
cargo test batch_processing
```

---

**Full Changelog**: [v1.2.6...v1.2.7](https://github.com/koenieee/ddns_local_server/compare/v1.2.6...v1.2.7)

**Issues Fixed**: Batch processing race condition affecting multi-config deployments
**Contributors**: DDNS Updater Team