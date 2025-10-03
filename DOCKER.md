# 🐳 Docker Configuration Guide

This guide helps you configure DDNS Updater to work properly with Docker containers and mounted volumes.

## 🚨 Common Issues

### "Read-only file system (os error 30)"
This error occurs when DDNS Updater cannot write to nginx configuration files because:
- Docker volumes are mounted read-only
- Container user lacks write permissions
- Filesystem is mounted as read-only

## 🔧 Solutions

### 1. Ensure Volumes Have Write Permissions

```bash
# ✅ Correct - Mount with read-write permissions
docker run -v /host/nginx:/data/nginx:rw yourimage

# ❌ Incorrect - Read-only mount (default for some setups)
docker run -v /host/nginx:/data/nginx:ro yourimage
```

### 2. Docker Compose Configuration

```yaml
version: '3.8'
services:
  ddns-updater:
    image: your-ddns-updater-image
    volumes:
      # Ensure :rw (read-write) is specified or implied
      - /host/nginx/config:/data/nginx:rw
      - /host/backups:/var/backups/nginx:rw
      - /host/ddns-storage:/var/lib/ddns-updater:rw
    environment:
      - DDNS_HOST=your-hostname.com
      - DDNS_CONFIG_DIR=/data/nginx/proxy_host
```

### 3. Directory Permission Fix Script

Run the included permission fix script:

```bash
# Install DDNS Updater first, then run:
sudo /usr/share/ddns-updater/scripts/fix-docker-permissions.sh
```

Or if you have the source:

```bash
sudo ./scripts/fix-docker-permissions.sh
```

## 📁 Common Directory Mappings

### Nginx Proxy Manager (NPM)
```bash
# NPM typically uses:
-v /host/data/nginx:/data/nginx:rw
-v /host/backups:/var/backups/nginx:rw

# DDNS Updater config:
--config-dir /data/nginx/proxy_host
--backup-dir /var/backups/nginx
```

### Standard Nginx
```bash
# Standard nginx setup:
-v /host/nginx/sites:/etc/nginx/sites-available:rw
-v /host/nginx/backups:/var/backups/nginx:rw

# DDNS Updater config:
--config-dir /etc/nginx/sites-available
--backup-dir /var/backups/nginx
```

### Custom Nginx Location
```bash
# Custom location:
-v /host/custom/nginx:/opt/nginx/conf.d:rw
-v /host/custom/backups:/opt/backups:rw

# DDNS Updater config:
--config-dir /opt/nginx/conf.d
--backup-dir /opt/backups
```

## 🔑 User and Permission Setup

### Running as Root (Recommended)
Most nginx containers require root access to modify configuration files:

```dockerfile
# In your Dockerfile
USER root
```

```yaml
# In docker-compose.yml
services:
  ddns-updater:
    user: "0:0"  # root:root
```

### Running as Non-Root User
If you must run as non-root, ensure the user has write access:

```bash
# Create directories with proper permissions on host
sudo mkdir -p /host/nginx /host/backups
sudo chown -R 1000:1000 /host/nginx /host/backups
sudo chmod -R 755 /host/nginx /host/backups
```

```yaml
services:
  ddns-updater:
    user: "1000:1000"
    volumes:
      - /host/nginx:/data/nginx:rw
      - /host/backups:/var/backups/nginx:rw
```

## 🧪 Testing Configuration

Test if your container can write to mounted volumes:

```bash
# Run a test container
docker run --rm -v /host/nginx:/data/nginx:rw ubuntu:latest \
  sh -c "echo 'test' > /data/nginx/test.txt && rm /data/nginx/test.txt && echo 'Write test successful'"
```

## 🛠️ Troubleshooting Commands

### Check Mount Permissions
```bash
# Inside container
ls -la /data/nginx/
touch /data/nginx/test-write.txt
rm /data/nginx/test-write.txt
```

### Check User Context
```bash
# Inside container
whoami
id
groups
```

### View Mount Information
```bash
# On host
docker inspect container_name | grep -A 20 "Mounts"
```

## 📋 Pre-flight Checklist

Before running DDNS Updater in Docker:

- [ ] Volumes mounted with `:rw` (read-write)
- [ ] Host directories exist and are writable
- [ ] Container runs with appropriate user permissions
- [ ] Nginx configuration directory is accessible
- [ ] Backup directory is writable
- [ ] Storage directory (`/var/lib/ddns-updater`) is persistent

## 🚀 Example Docker Run Commands

### Nginx Proxy Manager
```bash
docker run -d \
  --name ddns-updater \
  -v /opt/npm/data/nginx:/data/nginx:rw \
  -v /opt/backups:/var/backups/nginx:rw \
  -v /opt/ddns-storage:/var/lib/ddns-updater:rw \
  -e DDNS_HOST=example.com \
  your-ddns-updater-image \
  --config-dir /data/nginx/proxy_host \
  --backup-dir /var/backups/nginx \
  --host example.com
```

### Standard Nginx
```bash
docker run -d \
  --name ddns-updater \
  -v /etc/nginx/sites-available:/etc/nginx/sites-available:rw \
  -v /var/backups/nginx:/var/backups/nginx:rw \
  -v /var/lib/ddns-updater:/var/lib/ddns-updater:rw \
  -e DDNS_HOST=example.com \
  your-ddns-updater-image \
  --config-dir /etc/nginx/sites-available \
  --backup-dir /var/backups/nginx \
  --host example.com
```

## 🆘 Getting Help

If you continue to experience permission issues:

1. Run the permission fix script: `sudo ./scripts/fix-docker-permissions.sh`
2. Check Docker logs: `docker logs ddns-updater`
3. Verify mount points: `docker inspect ddns-updater`
4. Test write access manually in the container
5. Review this guide and ensure all steps are followed

For more help, see the main README.md or open an issue on GitHub.