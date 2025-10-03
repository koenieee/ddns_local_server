#!/bin/bash

# DDNS Updater - Systemd Installation Script
# This script installs the DDNS updater as a systemd service
#
# Usage:
#   sudo ./install-systemd.sh              # Interactive mode
#   sudo ./install-systemd.sh -y           # Non-interactive mode (uses defaults)
#   sudo ./install-systemd.sh --help       # Show help

set -e

# Show help
if [[ "$1" == "--help" || "$1" == "-h" ]]; then
    echo "DDNS Updater - Systemd Installation Script"
    echo ""
    echo "This script installs the DDNS updater as a systemd service."
    echo ""
    echo "Usage:"
    echo "  sudo $0              Interactive mode (asks for configuration)"
    echo "  sudo $0 -y           Non-interactive mode (uses defaults)"
    echo "  sudo $0 --help       Show this help"
    echo ""
    echo "Default configuration (non-interactive mode):"
    echo "  Host:           google.com"
    echo "  Config Dir:     /etc/nginx/sites-available"
    echo "  Pattern:        *.conf"
    echo "  Backup Dir:     /var/backups/nginx"
    echo "  Backup Cleanup: Enabled (3 days retention)"
    echo "  Interval:       5 minutes"
    echo "  Verbose:        Yes"
    echo "  Auto Reload:    Yes"
    echo ""
    exit 0
fi

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

# Check for non-interactive mode
INTERACTIVE=true
if [[ "$1" == "--non-interactive" || "$1" == "-y" ]]; then
    INTERACTIVE=false
fi

# Check if running as root
if [[ $EUID -ne 0 ]]; then
   print_error "This script must be run as root (use sudo)"
   exit 1
fi

print_status "Installing DDNS Updater systemd service..."

# Detect if we're running from development environment or installed package
DEVELOPMENT_MODE=false
BINARY_PATH="/usr/bin/ddns-updater"

if [ -f "../Cargo.toml" ]; then
    # Development environment - need to build
    DEVELOPMENT_MODE=true
    BINARY_PATH="../target/release/ddns_updater"
elif [ ! -f "/usr/bin/ddns-updater" ]; then
    print_error "DDNS updater binary not found. Please install the package or run from development directory."
    exit 1
fi

# Configuration (interactive or defaults)
if [[ "$INTERACTIVE" == "true" ]]; then
    echo ""
    echo "=== DDNS Updater Configuration ==="
    echo ""
    echo "Press Enter to accept defaults shown in [brackets]"
    echo ""

    # Ask for host
    read -p "Enter the hostname to monitor for IP changes [google.com]: " HOST
    HOST=${HOST:-google.com}
else
    # Non-interactive defaults
    HOST="google.com"
    CONFIG_MODE="2"
    CONFIG_DIR="/etc/nginx/sites-available"
    PATTERN="*.conf"
    BACKUP_DIR="/var/backups/nginx"
    CLEANUP_ENABLED=true
    CLEANUP_DAYS=30
    INTERVAL="5min"
    VERBOSE_FLAG="--verbose"
    RELOAD_FLAG=""
    
    print_status "Using default configuration (non-interactive mode)"
    print_status "Host: $HOST, Config Dir: $CONFIG_DIR, Interval: $INTERVAL"
fi

if [[ "$INTERACTIVE" == "true" ]]; then

# Ask for configuration mode
echo ""
echo "Choose configuration mode:"
echo "  1) Single config file"
echo "  2) Directory of config files"
read -p "Select option (1 or 2) [2]: " CONFIG_MODE
CONFIG_MODE=${CONFIG_MODE:-2}

if [[ "$CONFIG_MODE" == "1" ]]; then
    # Single file mode
    read -p "Enter path to nginx config file [/etc/nginx/sites-available/default]: " CONFIG_FILE
    CONFIG_FILE=${CONFIG_FILE:-/etc/nginx/sites-available/default}
    CONFIG_ARGS="--config \"$CONFIG_FILE\""
else
    # Directory mode
    read -p "Enter nginx config directory [/etc/nginx/sites-available]: " CONFIG_DIR
    CONFIG_DIR=${CONFIG_DIR:-/etc/nginx/sites-available}
    
    read -p "Enter file pattern [*.conf]: " PATTERN
    PATTERN=${PATTERN:-*.conf}
    
    CONFIG_ARGS="--config-dir \"$CONFIG_DIR\" --pattern \"$PATTERN\""
fi

# Ask for backup directory
read -p "Enter backup directory [/var/backups/nginx]: " BACKUP_DIR
BACKUP_DIR=${BACKUP_DIR:-/var/backups/nginx}

# Ask for backup cleanup settings
echo ""
echo "Backup Cleanup Configuration:"
read -p "Enable automatic cleanup of old backup files? [Y/n]: " ENABLE_CLEANUP
if [[ "${ENABLE_CLEANUP,,}" =~ ^(n|no)$ ]]; then
    CLEANUP_ENABLED=false
    CLEANUP_DAYS=3
else
    CLEANUP_ENABLED=true
    echo "  1) Keep backups for 1 day"
    echo "  2) Keep backups for 3 days (recommended)"
    echo "  3) Keep backups for 7 days"
    echo "  4) Keep backups for 14 days"
    echo "  5) Custom retention period"
    read -p "Select retention period (1-5) [2]: " CLEANUP_CHOICE
    CLEANUP_CHOICE=${CLEANUP_CHOICE:-2}
    
    case $CLEANUP_CHOICE in
        1) CLEANUP_DAYS=1 ;;
        2) CLEANUP_DAYS=3 ;;
        3) CLEANUP_DAYS=7 ;;
        4) CLEANUP_DAYS=14 ;;
        5) 
            read -p "Enter custom retention days: " CUSTOM_DAYS
            CLEANUP_DAYS=${CUSTOM_DAYS:-3}
            ;;
        *) CLEANUP_DAYS=3 ;;
    esac
fi

# Ask for update interval
echo ""
echo "Choose update interval:"
echo "  1) Every minute (testing/development)"
echo "  2) Every 5 minutes (recommended)"
echo "  3) Every 10 minutes"
echo "  4) Every 30 minutes"
echo "  5) Every hour"
echo "  6) Custom interval"
read -p "Select option (1-6) [2]: " INTERVAL_CHOICE
INTERVAL_CHOICE=${INTERVAL_CHOICE:-2}

case $INTERVAL_CHOICE in
    1) INTERVAL="1min" ;;
    2) INTERVAL="5min" ;;
    3) INTERVAL="10min" ;;
    4) INTERVAL="30min" ;;
    5) INTERVAL="1h" ;;
    6) 
        read -p "Enter custom interval (e.g., 2min, 15min, 1h): " CUSTOM_INTERVAL
        INTERVAL=${CUSTOM_INTERVAL:-5min}
        ;;
    *) INTERVAL="5min" ;;
esac

# Ask for verbose mode
read -p "Enable verbose logging? (y/N): " VERBOSE
if [[ $VERBOSE =~ ^[Yy]$ ]]; then
    VERBOSE_FLAG="--verbose"
else
    VERBOSE_FLAG=""
fi

# Ask for nginx reload
read -p "Automatically reload nginx after updates? (Y/n): " RELOAD
if [[ $RELOAD =~ ^[Nn]$ ]]; then
    RELOAD_FLAG="--no-reload"
else
    RELOAD_FLAG=""
fi

fi  # End of interactive section

# Build and install binary if in development mode
if [ "$DEVELOPMENT_MODE" = true ]; then
    print_status "Building DDNS updater in release mode..."
    cd ..
    cargo build --release
    cd systemd
    
    # Copy binary to system location
    print_status "Installing binary to /usr/local/bin/ddns_updater..."
    cp ../target/release/ddns_updater /usr/local/bin/ddns_updater
    chmod +x /usr/local/bin/ddns_updater
    BINARY_PATH="/usr/local/bin/ddns_updater"
else
    print_status "Using installed binary: $BINARY_PATH"
fi

# Create backup directory
print_status "Creating backup directory: $BACKUP_DIR"
mkdir -p "$BACKUP_DIR"
chmod 755 "$BACKUP_DIR"

# Create storage directory for JSON files
print_status "Creating storage directory: /var/lib/ddns-updater"
mkdir -p /var/lib/ddns-updater
chmod 755 /var/lib/ddns-updater
chown root:root /var/lib/ddns-updater

# Install management scripts
print_status "Installing management scripts..."
mkdir -p /usr/share/ddns-updater
if [ "$DEVELOPMENT_MODE" = true ]; then
    cp show-config.sh /usr/share/ddns-updater/show-config.sh
    chmod +x /usr/share/ddns-updater/show-config.sh
elif [ -f "/usr/share/ddns-updater/show-config.sh" ]; then
    print_status "Using installed show-config script"
fi

# Create customized service file
print_status "Creating customized service file..."
cat > /etc/systemd/system/ddns-updater.service << EOF
[Unit]
Description=DDNS Updater - Nginx Allow List Manager
Documentation=https://github.com/koenieee/ddns_local_server
After=network-online.target
Wants=network-online.target$([ "$CLEANUP_ENABLED" == "true" ] && echo " ddns-backup-cleanup.service" || echo "")

[Service]
Type=oneshot
User=root
Group=root
ExecStartPre=/bin/sh -c 'echo "DDNS Configuration: Host=\${DDNS_HOST}, Config=\${DDNS_CONFIG_MODE}(\${DDNS_CONFIG_DIR}\${DDNS_CONFIG_FILE}), Pattern=\${DDNS_PATTERN}, Interval=\${DDNS_INTERVAL}, Verbose=\${DDNS_VERBOSE}, Cleanup=\${DDNS_CLEANUP_ENABLED}"'
ExecStart=$BINARY_PATH --host "$HOST" $CONFIG_ARGS --backup-dir "$BACKUP_DIR" $VERBOSE_FLAG $RELOAD_FLAG
Environment=PATH=/usr/local/bin:/usr/bin:/bin
Environment=DDNS_HOST=$HOST
Environment=DDNS_CONFIG_MODE=$CONFIG_MODE
$([ "$CONFIG_MODE" == "1" ] && echo "Environment=DDNS_CONFIG_FILE=$CONFIG_FILE" || echo "Environment=DDNS_CONFIG_DIR=$CONFIG_DIR")
$([ "$CONFIG_MODE" == "2" ] && echo "Environment=DDNS_PATTERN=$PATTERN" || echo "")
Environment=DDNS_BACKUP_DIR=$BACKUP_DIR
Environment=DDNS_INTERVAL=$INTERVAL
Environment=DDNS_VERBOSE=$([ -n "$VERBOSE_FLAG" ] && echo "true" || echo "false")
Environment=DDNS_AUTO_RELOAD=$([ -z "$RELOAD_FLAG" ] && echo "true" || echo "false")
Environment=DDNS_CLEANUP_ENABLED=$CLEANUP_ENABLED
$([ "$CLEANUP_ENABLED" == "true" ] && echo "Environment=DDNS_CLEANUP_DAYS=$CLEANUP_DAYS" || echo "")
WorkingDirectory=$(dirname "$BINARY_PATH")
StandardOutput=journal
StandardError=journal

# Security settings
NoNewPrivileges=true
ProtectSystem=strict
ProtectHome=true
EOF

# Build ReadWritePaths dynamically based on configuration
READWRITE_PATHS="/var/lib/ddns-updater $BACKUP_DIR"

# Add config directory or parent directory of config file
if [ "$CONFIG_MODE" == "1" ]; then
    # Single file mode - add parent directory
    CONFIG_PARENT_DIR=$(dirname "$CONFIG_FILE")
    READWRITE_PATHS="$READWRITE_PATHS $CONFIG_PARENT_DIR"
else
    # Directory mode - add the config directory
    READWRITE_PATHS="$READWRITE_PATHS $CONFIG_DIR"
fi

# Remove duplicates and sort
READWRITE_PATHS=$(echo "$READWRITE_PATHS" | tr ' ' '\n' | sort -u | tr '\n' ' ')

# Add ReadWritePaths to service file
echo "ReadWritePaths=$READWRITE_PATHS" >> /etc/systemd/system/ddns-updater.service

cat >> /etc/systemd/system/ddns-updater.service << EOF
PrivateTmp=true
ProtectKernelTunables=true
ProtectKernelModules=true
ProtectControlGroups=true
RestrictRealtime=true
RestrictNamespaces=true

[Install]
WantedBy=multi-user.target
EOF

# Create customized timer file
print_status "Creating customized timer file..."
cat > /etc/systemd/system/ddns-updater.timer << EOF
[Unit]
Description=Run DDNS Updater every $INTERVAL
Requires=ddns-updater.service

[Timer]
OnBootSec=2min
OnUnitActiveSec=$INTERVAL
RandomizedDelaySec=30sec

[Install]
WantedBy=timers.target
EOF

# Set proper permissions
chmod 644 /etc/systemd/system/ddns-updater.service
chmod 644 /etc/systemd/system/ddns-updater.timer

# Install backup cleanup system
if [[ "$CLEANUP_ENABLED" == "true" ]]; then
    print_status "Installing backup cleanup system..."
    
    # Install cleanup script if in development mode
    if [ "$DEVELOPMENT_MODE" = true ]; then
        # Copy cleanup script from local directory
        cp ddns-backup-cleanup.sh /usr/local/bin/ddns-backup-cleanup
        chmod +x /usr/local/bin/ddns-backup-cleanup
        CLEANUP_SCRIPT_PATH="/usr/local/bin/ddns-backup-cleanup"
    elif [ -f "/usr/bin/ddns-backup-cleanup" ]; then
        # Use installed cleanup script
        CLEANUP_SCRIPT_PATH="/usr/bin/ddns-backup-cleanup"
        print_status "Using installed cleanup script: $CLEANUP_SCRIPT_PATH"
    else
        print_error "Cleanup script not found. Please install the package or run from development directory."
        exit 1
    fi
    
    # Create cleanup service file
    cat > /etc/systemd/system/ddns-backup-cleanup.service << EOF
[Unit]
Description=DDNS Updater - Backup Cleanup Service
Documentation=https://github.com/koenieee/ddns_local_server
After=network.target
PartOf=ddns-updater.service
Requisite=ddns-updater.service

[Service]
Type=oneshot
User=root
Group=root

# Clean up backup files older than $CLEANUP_DAYS days
ExecStartPre=/bin/sh -c 'echo "DDNS Cleanup Configuration: Directory=\${DDNS_CLEANUP_BACKUP_DIR}, Retention=\${DDNS_CLEANUP_RETENTION_DAYS} days, Verbose=\${DDNS_CLEANUP_VERBOSE}"'
ExecStart=/usr/local/bin/ddns-backup-cleanup --backup-dir "$BACKUP_DIR" --days $CLEANUP_DAYS --verbose
Environment=DDNS_CLEANUP_BACKUP_DIR=$BACKUP_DIR
Environment=DDNS_CLEANUP_RETENTION_DAYS=$CLEANUP_DAYS
Environment=DDNS_CLEANUP_VERBOSE=true

# Security settings
NoNewPrivileges=true
ProtectSystem=strict
ProtectKernelTunables=true
ProtectKernelModules=true
ProtectControlGroups=true
RestrictRealtime=true
MemoryDenyWriteExecute=true
LockPersonality=true
RestrictSUIDSGID=true

# Allow access to backup directories
ReadWritePaths=$BACKUP_DIR

# Logging
StandardOutput=journal
StandardError=journal
SyslogIdentifier=ddns-backup-cleanup
EOF

    # Create cleanup timer file
    cat > /etc/systemd/system/ddns-backup-cleanup.timer << EOF
[Unit]
Description=DDNS Updater - Backup Cleanup Timer
Requires=ddns-backup-cleanup.service
PartOf=ddns-updater.timer
After=ddns-updater.timer

[Timer]
# Run daily at 3:00 AM
OnCalendar=daily
Persistent=true
RandomizedDelaySec=1800

[Install]
WantedBy=timers.target
EOF

    # Set permissions for cleanup files
    chmod 644 /etc/systemd/system/ddns-backup-cleanup.service
    chmod 644 /etc/systemd/system/ddns-backup-cleanup.timer
    
    print_status "Backup cleanup configured to run daily, keeping files for $CLEANUP_DAYS days"
fi

# Create a target file for grouping all DDNS updater services
print_status "Creating DDNS updater service group target..."
cat > /etc/systemd/system/ddns-updater.target << EOF
[Unit]
Description=DDNS Updater Service Group
Documentation=https://github.com/koenieee/ddns_local_server
Wants=ddns-updater.timer$([ "$CLEANUP_ENABLED" == "true" ] && echo " ddns-backup-cleanup.timer" || echo "")
After=ddns-updater.timer$([ "$CLEANUP_ENABLED" == "true" ] && echo " ddns-backup-cleanup.timer" || echo "")

[Install]
WantedBy=multi-user.target
EOF

chmod 644 /etc/systemd/system/ddns-updater.target

# Reload systemd daemon
print_status "Reloading systemd daemon..."
systemctl daemon-reload

# Enable and start the service group
print_status "Enabling and starting DDNS updater service group..."
systemctl enable ddns-updater.target
systemctl enable ddns-updater.timer
systemctl start ddns-updater.timer

# Enable cleanup timer if configured
if [[ "$CLEANUP_ENABLED" == "true" ]]; then
    print_status "Enabling backup cleanup timer..."
    systemctl enable ddns-backup-cleanup.timer
    systemctl start ddns-backup-cleanup.timer
fi

print_status "Starting DDNS updater service group..."
systemctl start ddns-updater.target

print_success "DDNS Updater systemd service installed successfully!"
echo ""
echo "=== Installed Configuration ==="
echo "  Host:         $HOST"
if [[ "$CONFIG_MODE" == "1" ]]; then
echo "  Config File:  $CONFIG_FILE"
else
echo "  Config Dir:   $CONFIG_DIR"
echo "  Pattern:      $PATTERN"
fi
echo "  Backup Dir:   $BACKUP_DIR"
if [[ "$CLEANUP_ENABLED" == "true" ]]; then
echo "  Backup Cleanup: Enabled ($CLEANUP_DAYS days retention)"
else
echo "  Backup Cleanup: Disabled"
fi
echo "  Interval:     $INTERVAL"
echo "  Verbose:      $([ -n "$VERBOSE_FLAG" ] && echo "Yes" || echo "No")"
echo "  Auto Reload:  $([ -z "$RELOAD_FLAG" ] && echo "Yes" || echo "No")"
echo ""
echo "=== System Files ==="
echo "  Service Group:   /etc/systemd/system/ddns-updater.target"
echo "  Service File:    /etc/systemd/system/ddns-updater.service"
echo "  Timer File:      /etc/systemd/system/ddns-updater.timer"
echo "  Binary:          $BINARY_PATH"
if [[ "$CLEANUP_ENABLED" == "true" ]]; then
echo "  Cleanup Service: /etc/systemd/system/ddns-backup-cleanup.service"
echo "  Cleanup Timer:   /etc/systemd/system/ddns-backup-cleanup.timer"
echo "  Cleanup Script:  /usr/local/bin/ddns-backup-cleanup"
fi
echo ""
echo "=== Management Commands ==="
echo "  Show configuration:    /usr/share/ddns-updater/show-config.sh"
echo "  Check group status:    systemctl status ddns-updater.target"
echo "  Check timer status:    systemctl status ddns-updater.timer"
echo "  View logs:             journalctl -u ddns-updater.service -f"
echo "  Stop service group:    systemctl stop ddns-updater.target"
echo "  Start service group:   systemctl start ddns-updater.target"
echo "  Disable service group: systemctl disable ddns-updater.target"
echo "  Manual run:            systemctl start ddns-updater.service"
if [[ "$CLEANUP_ENABLED" == "true" ]]; then
echo ""
echo "=== Backup Cleanup Commands ==="
echo "  Check cleanup status:  systemctl status ddns-backup-cleanup.timer"
echo "  View cleanup logs:     journalctl -u ddns-backup-cleanup.service -f"
echo "  Manual cleanup:        systemctl start ddns-backup-cleanup.service"
echo "  Test cleanup (dry-run): /usr/local/bin/ddns-backup-cleanup --backup-dir $BACKUP_DIR --days $CLEANUP_DAYS --dry-run --verbose"
fi
echo ""
echo "The service will run every $INTERVAL and check for IP changes on $HOST."
echo ""
echo "To modify the configuration:"
echo "  1. Edit /etc/systemd/system/ddns-updater.service"
echo "  2. Run: systemctl daemon-reload"
echo "  3. Run: systemctl restart ddns-updater.timer"