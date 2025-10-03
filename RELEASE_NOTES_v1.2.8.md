# Release Notes - DDNS Updater v1.2.8

🎉 **Major Release: DNS Auto-initialization & Production Reliability**

## 🚀 New Features

### DNS Host File Auto-initialization
- **Automatic JSON file creation** at first startup in `/var/lib/ddns-updater/`
- **Safe conditional logic** - only creates files when they don't exist
- **Placeholder IP initialization** (0.0.0.0) with descriptive comments
- **Structured data format** with hostname, IP, timestamps, and comments
- **Test mode support** with `./test_storage/` directory

### Enhanced Nginx Reload System
- **Multiple reload strategies** with intelligent fallback
- **Full path support** for `/usr/sbin/nginx` (fixes production issues)
- **Comprehensive error handling** with detailed logging
- **Production reliability** improvements

## 🔧 Technical Improvements

### Infrastructure Enhancements
- Added `initialize_host_file()` method to `IpRepository` trait
- Full implementation in `FileIpRepository` with JSON structure
- Default no-op implementation for other repository types
- Trait-based architecture maintains extensibility

### Nginx Reload Reliability
- **Method 1**: `/usr/sbin/nginx -s reload` (full path, most reliable)
- **Method 2**: `nginx -s reload` (PATH fallback)
- **Method 3**: `systemctl reload nginx` (systemd method)
- **Method 4**: `service nginx reload` (SysV init method)

## 🎯 Bug Fixes

### Production Issues Resolved
- ✅ Fixed: `"Failed to reload nginx.service"` error in production
- ✅ Fixed: PATH issues with nginx command execution  
- ✅ Fixed: First-time startup DNS data tracking
- ✅ Fixed: Service reliability in containerized environments

### CI/CD Improvements
- Enhanced GitHub Actions workflows with timeout protection
- Network-resilient testing with `DDNS_CI_MODE` support
- Job-specific timeout limits (10-60 minutes)

## 📊 Usage Examples

### DNS Host File Structure
```json
{
  "ip": "192.168.1.100",
  "hostname": "example.com",
  "comment": "Initial DNS host file created at first startup",
  "created_at": "2025-10-03T14:57:16Z",
  "updated_at": "2025-10-03T14:57:16Z"
}
```

### Production Deployment
```bash
# Install via Debian package
sudo dpkg -i ddns-updater_1.2.8_amd64.deb

# Or manual installation
sudo systemctl daemon-reload
sudo systemctl enable ddns-updater.target
sudo systemctl start ddns-updater.target
```

## 🔄 Migration Notes

### From v1.2.7
- **No breaking changes** - fully backward compatible
- DNS host files will be created automatically on first run
- Existing installations will benefit from improved nginx reload
- No configuration changes required

### File Locations
- **Production**: `/var/lib/ddns-updater/hostname.json`
- **Test mode**: `./test_storage/hostname.json`
- **Logs**: `/var/log/ddns-updater/` (systemd) or stderr (manual)

## 🚀 Installation

### Debian/Ubuntu (Recommended)
```bash
wget https://github.com/koenieee/ddns_local_server/releases/download/v1.2.8/ddns-updater_1.2.8_amd64.deb
sudo dpkg -i ddns-updater_1.2.8_amd64.deb
```

### From Source
```bash
git clone https://github.com/koenieee/ddns_local_server.git
cd ddns_local_server
git checkout v1.2.8
cargo build --release
```

## 🔍 Verification

Test the new features:
```bash
# Test DNS initialization (creates host file if missing)
ddns_updater --host github.com --config /etc/nginx/sites-available/default --verbose

# Test nginx reload functionality  
sudo ddns_updater --host example.com --config /etc/nginx/sites-available/example.conf

# Check created DNS files
ls -la /var/lib/ddns-updater/
cat /var/lib/ddns-updater/github.com.json
```

---

**Full Changelog**: [v1.2.7...v1.2.8](https://github.com/koenieee/ddns_local_server/compare/v1.2.7...v1.2.8)

**Downloads**: Available as Debian packages and source archives below ⬇️