#!/bin/bash
# DDNS Updater Storage Fix Script
# This script fixes storage directory permission issues after installation

set -e

echo "🔧 DDNS Updater Storage Directory Fix"
echo "=====================================

This script fixes storage directory permission issues that may occur
when running DDNS Updater as a systemd service.

"

# Check if running as root
if [ "$(id -u)" -ne 0 ]; then
    echo "❌ This script must be run as root"
    echo "   Please run: sudo $0"
    exit 1
fi

echo "🔍 Checking current setup..."

# Create storage directory if it doesn't exist
if [ ! -d "/var/lib/ddns-updater" ]; then
    echo "📂 Creating /var/lib/ddns-updater directory..."
    mkdir -p /var/lib/ddns-updater
fi

# Set proper ownership and permissions
echo "🔐 Setting proper ownership and permissions..."
chown root:ddns-updater /var/lib/ddns-updater
chmod 750 /var/lib/ddns-updater

# Create fallback directory
if [ ! -d "/tmp/ddns-updater" ]; then
    echo "📂 Creating fallback directory /tmp/ddns-updater..."
    mkdir -p /tmp/ddns-updater
    chmod 755 /tmp/ddns-updater
fi

# Check systemd services
echo "🔄 Reloading systemd configuration..."
systemctl daemon-reload

# Restart services if they're running
for service in ddns-updater.service ddns-updater.timer; do
    if systemctl is-active --quiet "$service" 2>/dev/null; then
        echo "🔄 Restarting $service..."
        systemctl restart "$service"
    fi
done

echo "
✅ Storage directory fix completed successfully!

📁 Storage directories:
   - Primary: /var/lib/ddns-updater (owner: root:ddns-updater, mode: 750)
   - Fallback: /tmp/ddns-updater (owner: root:root, mode: 755)

🧪 Test the fix:
   sudo ddns_updater --verbose --host google.com --config-dir /etc/nginx/sites-available

📋 If you still encounter issues:
   1. Check systemd service logs: sudo journalctl -u ddns-updater.service -f
   2. Verify file permissions: ls -la /var/lib/ddns-updater
   3. Check if /var/lib is mounted read-only: mount | grep /var/lib

For more help, see: https://github.com/koenieee/ddns_local_server/issues
"