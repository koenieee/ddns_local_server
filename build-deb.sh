#!/bin/bash

# DDNS Updater - Debian Package Builder
# This script builds a .deb package for the DDNS updater

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
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

# Check if we're in the right directory
if [[ ! -f "Cargo.toml" ]] || [[ ! -d "debian" ]]; then
    print_error "This script must be run from the ddns_updater project root directory"
    print_error "Make sure you have both Cargo.toml and debian/ directory present"
    exit 1
fi

print_status "Building DDNS Updater Debian Package"

# Check for required tools
print_status "Checking build dependencies..."

MISSING_TOOLS=()

if ! command -v dpkg-buildpackage >/dev/null 2>&1; then
    MISSING_TOOLS+=("dpkg-dev")
fi

if ! command -v cargo >/dev/null 2>&1; then
    MISSING_TOOLS+=("cargo")
fi

if ! command -v rustc >/dev/null 2>&1; then
    MISSING_TOOLS+=("rustc")
fi

if ! command -v dh >/dev/null 2>&1; then
    MISSING_TOOLS+=("debhelper")
fi

if [[ ${#MISSING_TOOLS[@]} -gt 0 ]]; then
    print_error "Missing required build tools: ${MISSING_TOOLS[*]}"
    print_status "Install them with:"
    echo "sudo apt update"
    echo "sudo apt install ${MISSING_TOOLS[*]}"
    exit 1
fi

print_success "All build dependencies are available"

# Clean previous build artifacts
print_status "Cleaning previous build artifacts..."
cargo clean || true
rm -rf ../ddns-updater_* || true

# Build the Rust binary first to catch any compilation errors
print_status "Building Rust binary..."

# Support cross-compilation if environment variables are set
CARGO_TARGET=${CARGO_TARGET:-"x86_64-unknown-linux-gnu"}
USE_CROSS=${USE_CROSS:-"false"}
DEB_HOST_ARCH=${DEB_HOST_ARCH:-"amd64"}

print_status "Build Configuration:"
print_status "  CARGO_TARGET: $CARGO_TARGET"
print_status "  USE_CROSS: $USE_CROSS"
print_status "  DEB_HOST_ARCH: $DEB_HOST_ARCH"

# Check for cross-compilation tools if needed
if [[ "$USE_CROSS" == "true" && "$CARGO_TARGET" = "aarch64-unknown-linux-gnu" ]]; then
    print_status "Checking ARM64 cross-compilation tools..."
    if command -v aarch64-linux-gnu-gcc >/dev/null 2>&1; then
        print_success "ARM64 GCC found: $(which aarch64-linux-gnu-gcc)"
    else
        print_warning "ARM64 GCC not found - build may fail"
    fi
    
    if command -v aarch64-linux-gnu-objcopy >/dev/null 2>&1; then
        print_success "ARM64 objcopy found: $(which aarch64-linux-gnu-objcopy)"
    else
        print_warning "ARM64 objcopy not found - may cause stripping issues"
    fi
fi

if [[ "$USE_CROSS" == "true" ]]; then
    print_status "Using cross-compilation..."
    cross build --release --target "$CARGO_TARGET"
    BINARY_PATH="target/$CARGO_TARGET/release/ddns_updater"
else
    print_status "Using native compilation..."
    cargo build --release --target "$CARGO_TARGET"
    BINARY_PATH="target/$CARGO_TARGET/release/ddns_updater"
fi

if [[ ! -f "$BINARY_PATH" ]]; then
    print_error "Failed to build the ddns_updater binary at $BINARY_PATH"
    exit 1
fi

print_success "Rust binary built successfully at $BINARY_PATH"

# Validate package files
print_status "Validating Debian package files..."

# Check that all systemd files exist
SYSTEMD_FILES=(
    "systemd/ddns-updater.service"
    "systemd/ddns-updater.timer" 
    "systemd/ddns-updater@.service"
    "systemd/ddns-updater@.timer"
    "systemd/ddns-backup-cleanup.service"
    "systemd/ddns-backup-cleanup.timer"
    "systemd/ddns-updater.target"
)

for file in "${SYSTEMD_FILES[@]}"; do
    if [[ ! -f "$file" ]]; then
        print_error "Missing required file: $file"
        exit 1
    fi
done

# Check that scripts exist
SCRIPT_FILES=(
    "systemd/install-systemd.sh"
    "systemd/install-systemd-advanced.sh" 
    "systemd/uninstall-systemd.sh"
    "systemd/ddns-backup-cleanup.sh"
)

for file in "${SCRIPT_FILES[@]}"; do
    if [[ ! -f "$file" ]]; then
        print_error "Missing required script: $file"
        exit 1
    fi
done

print_success "All required files are present"

# Validate script syntax
print_status "Validating script syntax..."
for file in "${SCRIPT_FILES[@]}"; do
    if ! bash -n "$file"; then
        print_error "Syntax error in $file"
        exit 1
    fi
done

print_success "All scripts have valid syntax"

# Build the package
print_status "Building Debian package..."

# Set environment variables for reproducible builds
export SOURCE_DATE_EPOCH=$(date +%s)

# Build the package (override build dependencies since we have rustc/cargo via rustup)
if dpkg-buildpackage -us -uc -b -d; then
    print_success "Debian package built successfully!"
    
    # Find and display the generated package
    DEB_FILE=$(find .. -name "ddns-updater_*.deb" -type f | head -n1)
    
    if [[ -n "$DEB_FILE" ]]; then
        print_status "Package details:"
        echo "  File: $(basename "$DEB_FILE")"
        echo "  Size: $(du -h "$DEB_FILE" | cut -f1)"
        echo "  Location: $DEB_FILE"
        
        # Show package information
        print_status "Package information:"
        dpkg --info "$DEB_FILE" | grep -E "Package|Version|Architecture|Description"
        
        # Show package contents
        print_status "Package contents:"
        dpkg --contents "$DEB_FILE" | head -20
        if [[ $(dpkg --contents "$DEB_FILE" | wc -l) -gt 20 ]]; then
            echo "  ... and $(( $(dpkg --contents "$DEB_FILE" | wc -l) - 20 )) more files"
        fi
        
        echo ""
        print_success "✅ Build completed successfully!"
        echo ""
        echo "To install the package:"
        echo "  sudo dpkg -i $DEB_FILE"
        echo ""
        echo "To install with dependency resolution (recommended):"
        echo "  sudo apt install $DEB_FILE"
        echo ""
        echo "📋 Installation Features:"
        echo "  ✅ Automatic user and directory creation"
        echo "  ✅ Systemd service registration" 
        echo "  ✅ Interactive setup prompt after installation"
        echo "  ✅ Professional service management"
        echo ""
        echo "The package will automatically ask if you want to run the"
        echo "interactive setup script after installation completes!"
        echo ""
        
    else
        print_warning "Package was built but could not locate the .deb file"
    fi
    
else
    print_error "Failed to build Debian package"
    exit 1
fi