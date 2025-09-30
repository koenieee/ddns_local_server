#!/bin/bash

# DDNS Updater - Installation Test Script
# This script demonstrates the new interactive installation feature

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_info() {
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

echo "==============================================="
echo "🎯 DDNS Updater - Installation Test"
echo "==============================================="
echo ""

# Check if package exists
DEB_FILE="../ddns-updater_0.1.0-1_amd64.deb"
if [[ ! -f "$DEB_FILE" ]]; then
    print_error "Package file not found: $DEB_FILE"
    print_info "Run './build-deb.sh' first to create the package"
    exit 1
fi

print_info "Found package: $(basename "$DEB_FILE")"
print_info "Package size: $(du -h "$DEB_FILE" | cut -f1)"
echo ""

# Check if package is already installed
if dpkg -l ddns-updater >/dev/null 2>&1; then
    print_warning "DDNS Updater package is already installed"
    echo ""
    echo "Current installation status:"
    dpkg -s ddns-updater | grep -E "Status|Version"
    echo ""
    
    while true; do
        echo -n "Do you want to reinstall the package? [y/N]: "
        read -r response
        case $response in
            [Yy]* )
                print_info "Removing existing installation..."
                sudo apt remove -y ddns-updater >/dev/null 2>&1 || true
                sudo apt purge -y ddns-updater >/dev/null 2>&1 || true
                break
                ;;
            [Nn]* | "" )
                print_info "Installation test cancelled"
                exit 0
                ;;
            * )
                echo "Please answer yes or no."
                ;;
        esac
    done
fi

echo ""
print_info "🚀 Installing DDNS Updater package..."
echo ""
echo "The package will:"
echo "  ✅ Create system user and directories"
echo "  ✅ Install systemd services"
echo "  ✅ Ask if you want to run interactive setup"
echo ""

while true; do
    echo -n "Proceed with installation? [Y/n]: "
    read -r response
    case $response in
        [Yy]* | "" )
            break
            ;;
        [Nn]* )
            print_info "Installation test cancelled"
            exit 0
            ;;
        * )
            echo "Please answer yes or no."
            ;;
    esac
done

echo ""
print_info "Installing package with APT (recommended method)..."
echo "=========================================="

# Install the package
if sudo apt install -y "$DEB_FILE"; then
    echo ""
    echo "=========================================="
    print_success "Package installation completed!"
    
    # Check installation
    echo ""
    print_info "Installation verification:"
    echo "  📦 Package status: $(dpkg -s ddns-updater | grep Status | cut -d' ' -f2-)"
    echo "  👤 User created: $(getent passwd ddns-updater >/dev/null && echo "✅ Yes" || echo "❌ No")"
    echo "  📁 Config dir: $([ -d /etc/ddns-updater ] && echo "✅ Yes" || echo "❌ No")"
    echo "  📝 Log dir: $([ -d /var/log/ddns-updater ] && echo "✅ Yes" || echo "❌ No")"
    echo "  🔧 Services: $(systemctl list-unit-files | grep -c ddns-updater || echo "0") units installed"
    
    echo ""
    print_info "Service status:"
    systemctl --no-pager status ddns-updater.target 2>/dev/null || echo "  Service not yet configured"
    
    echo ""
    print_success "🎉 Installation test completed successfully!"
    echo ""
    echo "Next steps:"
    echo "  • If you skipped setup, run: sudo /usr/share/ddns-updater/install-systemd.sh"
    echo "  • View logs: sudo journalctl -u ddns-updater.service -f"
    echo "  • Manage services: sudo systemctl {start|stop|status} ddns-updater.target"
    echo "  • Remove package: sudo apt purge ddns-updater"
    
else
    print_error "Package installation failed"
    exit 1
fi