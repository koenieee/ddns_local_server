#!/bin/bash
# Build script for Debian 12 (libc 2.36-9+deb12u10) compatibility

set -e

echo "Building DDNS Updater for Debian 12 compatibility..."

# Set environment variables for glibc compatibility
export RUSTFLAGS="-C target-feature=-crt-static -C link-arg=-Wl,--as-needed"
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-./target}"

# Clean previous builds
echo "Cleaning previous builds..."
cargo clean

# Update dependencies to ensure compatibility
echo "Updating Cargo.lock for compatibility..."
cargo update

# Build for x86_64 (AMD64)
echo "Building for x86_64-unknown-linux-gnu..."
cargo build --release --target x86_64-unknown-linux-gnu

# Build for aarch64 (ARM64) if cross compilation tools are available
if command -v aarch64-linux-gnu-gcc &> /dev/null; then
    echo "Building for aarch64-unknown-linux-gnu..."
    cargo build --release --target aarch64-unknown-linux-gnu
else
    echo "Cross compilation tools not found, skipping ARM64 build"
fi

# Check glibc version requirements
echo "Checking glibc version requirements..."
if command -v objdump &> /dev/null; then
    echo "GLIBC version requirements for x86_64 binary:"
    objdump -T "${CARGO_TARGET_DIR}/x86_64-unknown-linux-gnu/release/ddns_updater" 2>/dev/null | grep GLIBC | sort -u || echo "No GLIBC symbols found"
    
    echo "Dynamic library dependencies:"
    ldd "${CARGO_TARGET_DIR}/x86_64-unknown-linux-gnu/release/ddns_updater" 2>/dev/null || echo "ldd not available"
fi

echo "Build complete! Binaries are in ${CARGO_TARGET_DIR}/*/release/"
echo "Test on Debian 12 with: ./target/x86_64-unknown-linux-gnu/release/ddns_updater --version"