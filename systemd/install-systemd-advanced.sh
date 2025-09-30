#!/bin/bash

# DDNS Updater - Advanced Systemd Installation Script
# This script installs template-based systemd services for multiple hosts

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

print_status "Installing DDNS Updater template systemd services..."

echo ""
echo "=== DDNS Updater Advanced Installation ==="
echo "This will install template services that support multiple hosts."
echo ""

# Ask for default backup directory
read -p "Enter default backup directory [/var/backups/nginx]: " DEFAULT_BACKUP_DIR
DEFAULT_BACKUP_DIR=${DEFAULT_BACKUP_DIR:-/var/backups/nginx}

# Ask for default config directory
read -p "Enter default nginx config directory [/etc/nginx/sites-available]: " DEFAULT_CONFIG_DIR
DEFAULT_CONFIG_DIR=${DEFAULT_CONFIG_DIR:-/etc/nginx/sites-available}

# Ask for default pattern
read -p "Enter default file pattern [*.conf]: " DEFAULT_PATTERN
DEFAULT_PATTERN=${DEFAULT_PATTERN:-*.conf}

# Ask for default interval
echo ""
echo "Choose default update interval:"
echo "  1) Every minute (testing/development)"
echo "  2) Every 5 minutes (recommended)"
echo "  3) Every 10 minutes"
echo "  4) Every 30 minutes"
echo "  5) Every hour"
echo "  6) Custom interval"
read -p "Select option (1-6) [2]: " INTERVAL_CHOICE
INTERVAL_CHOICE=${INTERVAL_CHOICE:-2}

case $INTERVAL_CHOICE in
    1) DEFAULT_INTERVAL="1min" ;;
    2) DEFAULT_INTERVAL="5min" ;;
    3) DEFAULT_INTERVAL="10min" ;;
    4) DEFAULT_INTERVAL="30min" ;;
    5) DEFAULT_INTERVAL="1h" ;;
    6) 
        read -p "Enter custom interval (e.g., 2min, 15min, 1h): " CUSTOM_INTERVAL
        DEFAULT_INTERVAL=${CUSTOM_INTERVAL:-5min}
        ;;
    *) DEFAULT_INTERVAL="5min" ;;
esac

# Ask which hosts to configure initially
echo ""
echo "Configure initial hosts? (You can add more later)"
read -p "Configure google.com service? (Y/n): " SETUP_GOOGLE
SETUP_GOOGLE=${SETUP_GOOGLE:-Y}

read -p "Configure another host? (y/N): " SETUP_CUSTOM
if [[ $SETUP_CUSTOM =~ ^[Yy]$ ]]; then
    read -p "Enter hostname: " CUSTOM_HOST
    read -p "Config file or directory for $CUSTOM_HOST [directory]: " CUSTOM_CONFIG_TYPE
    if [[ $CUSTOM_CONFIG_TYPE =~ ^[Ff]ile$ ]]; then
        read -p "Enter config file path: " CUSTOM_CONFIG_FILE
    else
        read -p "Enter config directory [$DEFAULT_CONFIG_DIR]: " CUSTOM_CONFIG_DIR
        CUSTOM_CONFIG_DIR=${CUSTOM_CONFIG_DIR:-$DEFAULT_CONFIG_DIR}
        read -p "Enter file pattern [$DEFAULT_PATTERN]: " CUSTOM_PATTERN
        CUSTOM_PATTERN=${CUSTOM_PATTERN:-$DEFAULT_PATTERN}
    fi
fi

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

# Create directories
print_status "Creating directories..."
mkdir -p /var/backups/nginx
mkdir -p /etc/default
mkdir -p /var/lib/ddns-updater
mkdir -p /usr/share/ddns-updater

# Install management scripts
if [ "$DEVELOPMENT_MODE" = true ]; then
    cp show-config.sh /usr/share/ddns-updater/show-config.sh
    chmod +x /usr/share/ddns-updater/show-config.sh
elif [ -f "/usr/share/ddns-updater/show-config.sh" ]; then
    print_status "Using installed show-config script"
fi
chmod 755 /var/backups/nginx
chmod 755 /var/lib/ddns-updater

# Create systemd template files
print_status "Creating systemd template files..."
cat > /etc/systemd/system/ddns-updater@.service << EOF
[Unit]
Description=DDNS Updater - Nginx Allow List Manager (%i)
Documentation=https://github.com/koenieee/ddns_local_server
After=network-online.target
Wants=network-online.target

[Service]
Type=oneshot
User=root
Group=root

# Template variables (customize these):
# %i = instance name (e.g., google-com, example-com)
# Set these in environment file: /etc/default/ddns-updater-%i

EnvironmentFile=-/etc/default/ddns-updater-%i
ExecStartPre=/bin/sh -c 'echo "DDNS Configuration [%i]: Host=\${DDNS_HOST}, Config=\${DDNS_CONFIG_DIR}, Instance=\${DDNS_INSTANCE}, Interval=\${DDNS_INTERVAL}"'
ExecStart=$BINARY_PATH \${DDNS_ARGS}

# Default environment
Environment=PATH=/usr/local/bin:/usr/bin:/bin
Environment=DDNS_ARGS=--host google.com --config-dir $DEFAULT_CONFIG_DIR --backup-dir $DEFAULT_BACKUP_DIR
Environment=DDNS_HOST=google.com
Environment=DDNS_CONFIG_DIR=$DEFAULT_CONFIG_DIR
Environment=DDNS_BACKUP_DIR=$DEFAULT_BACKUP_DIR
Environment=DDNS_INTERVAL=$DEFAULT_INTERVAL
Environment=DDNS_INSTANCE=%i

WorkingDirectory=/usr/local/bin
StandardOutput=journal
StandardError=journal

# Security settings
NoNewPrivileges=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=$DEFAULT_BACKUP_DIR /etc/nginx /var/lib/ddns-updater
PrivateTmp=true
ProtectKernelTunables=true
ProtectKernelModules=true
ProtectControlGroups=true
RestrictRealtime=true
RestrictNamespaces=true

[Install]
WantedBy=multi-user.target
EOF

cat > /etc/systemd/system/ddns-updater@.timer << EOF
[Unit]
Description=Run DDNS Updater for %i every $DEFAULT_INTERVAL
Requires=ddns-updater@%i.service

[Timer]
OnBootSec=2min
OnUnitActiveSec=$DEFAULT_INTERVAL
RandomizedDelaySec=30sec

[Install]
WantedBy=timers.target
EOF

# Set proper permissions
chmod 644 /etc/systemd/system/ddns-updater@.service
chmod 644 /etc/systemd/system/ddns-updater@.timer

# Create configuration files for requested hosts
print_status "Creating configuration files..."

if [[ $SETUP_GOOGLE =~ ^[Yy]$ ]]; then
    cat > /etc/default/ddns-updater-google-com << EOF
# DDNS Updater Configuration for google.com
DDNS_ARGS="--host google.com --config-dir $DEFAULT_CONFIG_DIR --pattern '$DEFAULT_PATTERN' --backup-dir $DEFAULT_BACKUP_DIR --verbose"
EOF
    print_status "Created configuration for google.com"
fi

if [[ -n "$CUSTOM_HOST" ]]; then
    CUSTOM_HOST_FILE=$(echo "$CUSTOM_HOST" | sed 's/\./-/g')
    if [[ -n "$CUSTOM_CONFIG_FILE" ]]; then
        CUSTOM_ARGS="--host $CUSTOM_HOST --config '$CUSTOM_CONFIG_FILE' --backup-dir $DEFAULT_BACKUP_DIR --verbose"
    else
        CUSTOM_ARGS="--host $CUSTOM_HOST --config-dir '$CUSTOM_CONFIG_DIR' --pattern '$CUSTOM_PATTERN' --backup-dir $DEFAULT_BACKUP_DIR --verbose"
    fi
    
    cat > /etc/default/ddns-updater-$CUSTOM_HOST_FILE << EOF
# DDNS Updater Configuration for $CUSTOM_HOST
DDNS_ARGS="$CUSTOM_ARGS"
EOF
    print_status "Created configuration for $CUSTOM_HOST"
fi

# Reload systemd daemon
print_status "Reloading systemd daemon..."
systemctl daemon-reload

print_success "DDNS Updater template services installed successfully!"
echo ""
echo "=== Installed Configuration ==="
echo "  Default Config Dir:  $DEFAULT_CONFIG_DIR"
echo "  Default Pattern:     $DEFAULT_PATTERN"
echo "  Default Backup Dir:  $DEFAULT_BACKUP_DIR"
echo "  Default Interval:    $DEFAULT_INTERVAL"
echo ""
echo "=== System Files ==="
echo "  Service Template: /etc/systemd/system/ddns-updater@.service"
echo "  Timer Template:   /etc/systemd/system/ddns-updater@.timer"
echo "  Binary:           $BINARY_PATH"
echo "  Config Dir:       /etc/default/"
echo "  Data Dir:         /var/lib/ddns-updater"
echo ""
echo "=== Configured Hosts ==="
if [[ $SETUP_GOOGLE =~ ^[Yy]$ ]]; then
echo "  google.com - /etc/default/ddns-updater-google-com"
fi
if [[ -n "$CUSTOM_HOST" ]]; then
    CUSTOM_HOST_FILE=$(echo "$CUSTOM_HOST" | sed 's/\./-/g')
echo "  $CUSTOM_HOST - /etc/default/ddns-updater-$CUSTOM_HOST_FILE"
fi
echo ""
echo "=== Enable Services ==="
if [[ $SETUP_GOOGLE =~ ^[Yy]$ ]]; then
echo "For google.com:"
echo "  systemctl enable ddns-updater@google-com.timer"
echo "  systemctl start ddns-updater@google-com.timer"
echo ""
fi
if [[ -n "$CUSTOM_HOST" ]]; then
    CUSTOM_HOST_FILE=$(echo "$CUSTOM_HOST" | sed 's/\./-/g')
echo "For $CUSTOM_HOST:"
echo "  systemctl enable ddns-updater@$CUSTOM_HOST_FILE.timer"
echo "  systemctl start ddns-updater@$CUSTOM_HOST_FILE.timer"
echo ""
fi
echo "=== Management Commands ==="
echo "  Check status:      systemctl status ddns-updater@HOST.timer"
echo "  View logs:         journalctl -u ddns-updater@HOST.service -f"
echo "  List all timers:   systemctl list-timers 'ddns-updater@*'"
echo "  Manual run:        systemctl start ddns-updater@HOST.service"
echo ""
echo "=== Adding New Hosts ==="
echo "  1. Create config: /etc/default/ddns-updater-HOSTNAME"
echo "     Example content: DDNS_ARGS=\"--host example.com --config-dir /etc/nginx/sites-available --backup-dir $DEFAULT_BACKUP_DIR --verbose\""
echo "  2. Enable timer:  systemctl enable ddns-updater@HOSTNAME.timer"
echo "  3. Start timer:   systemctl start ddns-updater@HOSTNAME.timer"