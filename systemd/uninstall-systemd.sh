#!/bin/bash

# DDNS Updater - Systemd Uninstallation Script
# This script removes the DDNS updater systemd service

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if running as root
if [[ $EUID -ne 0 ]]; then
   print_error "This script must be run as root (use sudo)"
   exit 1
fi

print_status "Uninstalling DDNS Updater systemd service..."

# Stop and disable the timer
if systemctl is-active --quiet ddns-updater.timer; then
    print_status "Stopping ddns-updater timer..."
    systemctl stop ddns-updater.timer
fi

if systemctl is-enabled --quiet ddns-updater.timer; then
    print_status "Disabling ddns-updater timer..."
    systemctl disable ddns-updater.timer
fi

# Stop the service if running
if systemctl is-active --quiet ddns-updater.service; then
    print_status "Stopping ddns-updater service..."
    systemctl stop ddns-updater.service
fi

# Handle backup cleanup services
if systemctl list-unit-files ddns-backup-cleanup.timer &>/dev/null; then
    print_status "Found backup cleanup service, removing..."
    
    # Stop and disable cleanup timer
    if systemctl is-active --quiet ddns-backup-cleanup.timer; then
        print_status "Stopping backup cleanup timer..."
        systemctl stop ddns-backup-cleanup.timer
    fi
    
    if systemctl is-enabled --quiet ddns-backup-cleanup.timer; then
        print_status "Disabling backup cleanup timer..."
        systemctl disable ddns-backup-cleanup.timer
    fi
    
    # Stop cleanup service if running
    if systemctl is-active --quiet ddns-backup-cleanup.service; then
        print_status "Stopping backup cleanup service..."
        systemctl stop ddns-backup-cleanup.service
    fi
fi

# Handle service group target
if systemctl list-unit-files ddns-updater.target &>/dev/null; then
    print_status "Found service group target, removing..."
    
    # Stop and disable target
    if systemctl is-active --quiet ddns-updater.target; then
        print_status "Stopping DDNS updater service group..."
        systemctl stop ddns-updater.target
    fi
    
    if systemctl is-enabled --quiet ddns-updater.target; then
        print_status "Disabling DDNS updater service group..."
        systemctl disable ddns-updater.target
    fi
fi

# Remove systemd files
print_status "Removing systemd service files..."
rm -f /etc/systemd/system/ddns-updater.target
rm -f /etc/systemd/system/ddns-updater.service
rm -f /etc/systemd/system/ddns-updater.timer
rm -f /etc/systemd/system/ddns-backup-cleanup.service
rm -f /etc/systemd/system/ddns-backup-cleanup.timer

# Remove binaries
print_status "Removing binaries..."
rm -f /usr/local/bin/ddns_updater
rm -f /usr/bin/ddns-backup-cleanup

# Ask about backup directory
echo ""
read -p "Remove backup directory /var/backups/nginx? (y/N): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    print_status "Removing backup directory..."
    rm -rf /var/backups/nginx
    print_success "Backup directory removed"
else
    print_warning "Backup directory preserved at /var/backups/nginx"
fi

# Reload systemd daemon
print_status "Reloading systemd daemon..."
systemctl daemon-reload

print_success "DDNS Updater systemd service uninstalled successfully!"
echo ""
echo "The following files were removed:"
echo "  /etc/systemd/system/ddns-updater.service"
echo "  /etc/systemd/system/ddns-updater.timer"
echo "  /usr/local/bin/ddns_updater"