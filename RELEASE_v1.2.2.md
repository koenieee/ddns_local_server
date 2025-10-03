# 🚀 Release v1.2.2: Persistent Storage Enforcement

## 📅 Release Information
- **Version**: 1.2.2
- **Release Date**: October 3, 2025
- **Package**: `ddns-updater_1.2.2-1_amd64.deb` (2.0 MB)
- **GitHub Tag**: `v1.2.2`

## 🎯 Key Improvements

### 🔧 Persistent Storage Enforcement
- **Removed `/tmp/ddns-updater` fallback** - ensures complete data persistence
- **Application now only uses `/var/lib/ddns-updater`** for JSON storage
- **Clear error messages** when persistent storage is unavailable
- **Eliminates data loss** from temporary directory cleanup

### 🛠️ Enhanced Installation
- **Automatic directory creation** in installation script
- **Proper permissions setup** (755, root:root) 
- **No manual setup required** for storage directory
- **Systemd service files updated** to enforce persistent storage only

### 📚 Comprehensive Documentation
- **Added `PERSISTENT_STORAGE.md`** - complete storage configuration guide
- **Troubleshooting instructions** for storage accessibility issues
- **Migration guide** for existing installations
- **Clear directory structure explanation**

## 🏗️ Technical Changes

### Modified Files:
1. **`src/interface/cli_interface.rs`**
   - Removed `/tmp/ddns-updater` fallback logic
   - Added graceful error handling with instructions
   - Clear exit with helpful error messages

2. **`systemd/ddns-updater.service`**
   - Removed `/tmp/ddns-updater` from `ReadWritePaths`
   - Service restricted to persistent storage only

3. **`systemd/ddns-updater@.service`**
   - Removed `/tmp/ddns-updater` from `ReadWritePaths`
   - Template service also enforces persistent storage

4. **`systemd/install-systemd.sh`**
   - Added automatic `/var/lib/ddns-updater` creation
   - Proper ownership and permissions setup
   - Eliminates manual directory setup

## 📁 Storage Structure

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

## ✅ Benefits

1. **Data Persistence**: IP data survives reboots and system maintenance
2. **Predictable Behavior**: Service always uses the same storage location
3. **Better Error Handling**: Clear error messages when storage unavailable
4. **Security**: No reliance on world-writable temporary directories
5. **Standards Compliance**: Follows Linux filesystem hierarchy standards
6. **Automated Setup**: Installation script handles all directory creation

## 🔄 Migration Guide

### For Existing Installations:
1. **Using Installation Script** (Recommended):
   ```bash
   sudo ./systemd/install-systemd.sh
   ```

2. **Manual Setup**:
   ```bash
   sudo mkdir -p /var/lib/ddns-updater
   sudo chmod 755 /var/lib/ddns-updater
   sudo chown root:root /var/lib/ddns-updater
   ```

3. **Copy Existing Data** (if any):
   ```bash
   sudo cp /tmp/ddns-updater/*.json /var/lib/ddns-updater/ 2>/dev/null || true
   ```

## 🛠️ Installation Options

### 1. Debian Package (Recommended)
```bash
# Download from GitHub Releases
sudo apt install ./ddns-updater_1.2.2-1_amd64.deb
```

### 2. Systemd Installation Script
```bash
git clone https://github.com/koenieee/ddns_local_server.git
cd ddns_local_server
cargo build --release
sudo ./systemd/install-systemd.sh
```

## 🔍 Troubleshooting

### Error: "Cannot access /var/lib/ddns-updater"
1. Check directory exists: `ls -la /var/lib/ddns-updater`
2. Create if missing: `sudo mkdir -p /var/lib/ddns-updater`
3. Set permissions: `sudo chmod 755 /var/lib/ddns-updater`
4. Set ownership: `sudo chown root:root /var/lib/ddns-updater`

### Service Fails to Start
1. Check logs: `journalctl -u ddns-updater.service`
2. Verify directory permissions
3. Ensure service has write access to storage directory

## 🧪 Testing

- ✅ **All 25 unit tests passing**
- ✅ **All 6 integration tests passing**
- ✅ **Cargo fmt/clippy clean**
- ✅ **Debian package builds successfully**
- ✅ **Systemd service files validated**

## 📋 Version History

| Version | Date | Key Features |
|---------|------|--------------|
| **v1.2.2** | 2025-10-03 | **Persistent storage enforcement, enhanced installation** |
| v1.2.1 | 2025-10-03 | Absolute path logging, backup optimization |
| v1.2.0 | 2025-10-03 | Smart JSON storage, intelligent config checking |
| v1.0.0 | 2025-09-XX | Initial release |

---

## 📖 Documentation

- **Installation**: [INTERACTIVE_INSTALLATION.md](INTERACTIVE_INSTALLATION.md)
- **Systemd Setup**: [systemd/SYSTEMD.md](systemd/SYSTEMD.md) 
- **Storage Configuration**: [PERSISTENT_STORAGE.md](PERSISTENT_STORAGE.md)
- **Debian Packaging**: [DEBIAN_PACKAGE.md](DEBIAN_PACKAGE.md)
- **Testing**: [TESTING.md](TESTING.md)

## 🤝 Contributing

See [README.md](README.md) for development setup and contribution guidelines.

---

**This release ensures your DDNS IP data is always preserved and accessible, providing the reliability your infrastructure depends on!** 🎉