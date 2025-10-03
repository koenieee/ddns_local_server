#!/bin/bash

# DDNS Updater Docker/Mount Permission Fix Script
# This script helps fix permissions when using DDNS Updater with Docker volumes 
# or mounted filesystems that may not have proper write permissions.

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

echo "🔧 DDNS Updater Permission Fix for Docker/Mounted Volumes"
echo "======================================================="

# Function to fix directory permissions
fix_directory_permissions() {
    local dir_path="$1"
    local description="$2"
    
    if [ -d "$dir_path" ]; then
        print_status "Found $description: $dir_path"
        
        # Check if directory is writable
        if [ -w "$dir_path" ]; then
            print_success "Directory is already writable"
        else
            print_warning "Directory is not writable, attempting to fix..."
            
            # Try to make it writable
            if chmod 755 "$dir_path" 2>/dev/null; then
                print_success "Successfully made directory writable"
            else
                print_warning "Could not change permissions (might be read-only mount)"
                print_warning "For Docker volumes, ensure they are mounted with write permissions"
                print_warning "Example: -v /host/path:/container/path:rw"
            fi
        fi
        
        # Check if ddns-updater user exists and add to relevant groups
        if getent passwd ddns-updater >/dev/null 2>&1; then
            # Get directory owner group
            dir_group=$(stat -c '%G' "$dir_path" 2>/dev/null || echo "")
            if [ -n "$dir_group" ] && [ "$dir_group" != "root" ]; then
                if getent group "$dir_group" >/dev/null 2>&1; then
                    print_status "Adding ddns-updater user to group: $dir_group"
                    usermod -a -G "$dir_group" ddns-updater 2>/dev/null || true
                fi
            fi
        fi
    else
        print_warning "$description not found: $dir_path"
    fi
}

# Common nginx configuration directories to check
print_status "Checking common nginx configuration directories..."

# Standard nginx directories
fix_directory_permissions "/etc/nginx/sites-available" "Standard nginx sites-available"
fix_directory_permissions "/etc/nginx/sites-enabled" "Standard nginx sites-enabled"
fix_directory_permissions "/etc/nginx/conf.d" "Standard nginx conf.d"

# Docker/containerized nginx directories
fix_directory_permissions "/data/nginx/proxy_host" "Docker nginx proxy_host (NPM)"
fix_directory_permissions "/app/data/nginx/proxy_host" "Docker app nginx proxy_host"
fix_directory_permissions "/opt/nginx/conf.d" "Docker opt nginx conf.d"

# Custom directories (ask user)
echo ""
print_status "Do you have nginx configuration files in other directories?"
read -p "Enter additional nginx config directory (or press Enter to skip): " CUSTOM_DIR

if [ -n "$CUSTOM_DIR" ]; then
    fix_directory_permissions "$CUSTOM_DIR" "Custom nginx directory"
fi

# Check backup directories
print_status "Checking backup directories..."
fix_directory_permissions "/var/backups/nginx" "Standard backup directory"

# Check if user wants to specify custom backup directory
read -p "Enter custom backup directory (or press Enter to skip): " CUSTOM_BACKUP_DIR
if [ -n "$CUSTOM_BACKUP_DIR" ]; then
    fix_directory_permissions "$CUSTOM_BACKUP_DIR" "Custom backup directory"
fi

# Ensure ddns-updater storage directory exists and is writable
print_status "Checking DDNS Updater storage directory..."
fix_directory_permissions "/var/lib/ddns-updater" "DDNS storage directory"

echo ""
print_success "Permission check complete!"
echo ""
print_status "If you're using Docker, make sure to:"
print_status "1. Mount volumes with write permissions (add :rw)"
print_status "2. Consider using bind mounts instead of volumes for config files"
print_status "3. Ensure the container user has write access to mounted paths"
echo ""
print_status "Example Docker run command:"
echo "  docker run -v /host/nginx:/data/nginx:rw -v /host/backups:/var/backups/nginx:rw ..."
echo ""
print_status "For Docker Compose, ensure volume definitions include write access:"
echo "  volumes:"
echo "    - /host/nginx:/data/nginx:rw"
echo "    - /host/backups:/var/backups/nginx:rw"
echo ""
print_status "After fixing permissions, restart the DDNS Updater service:"
echo "  sudo systemctl restart ddns-updater.service"