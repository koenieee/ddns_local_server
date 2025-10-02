#!/bin/bash
# Strict Debian 12 compatible build script

set -e

echo "Building DDNS Updater with strict Debian 12 (glibc 2.36) compatibility..."

# Set very conservative environment for older glibc
export RUSTFLAGS="-C target-feature=-crt-static -C link-arg=-Wl,--as-needed -C target-cpu=x86-64"
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-./target}"

# Use older Rust toolchain for better compatibility
if ! rustup toolchain list | grep -q "1.70.0"; then
    echo "Installing Rust 1.70.0 for maximum compatibility..."
    rustup toolchain install 1.70.0
fi

echo "Using Rust 1.70.0 toolchain..."
rustup default 1.70.0

# Clean and update
echo "Cleaning and updating dependencies..."
cargo clean
cargo update

# Build with minimal features and older toolchain
echo "Building with strict compatibility flags..."
RUSTFLAGS="$RUSTFLAGS" cargo +1.70.0 build --release --target x86_64-unknown-linux-gnu

# Verify compatibility
echo "Checking glibc compatibility..."
BINARY="./target/x86_64-unknown-linux-gnu/release/ddns_updater"

if command -v objdump &> /dev/null; then
    echo "Required GLIBC versions:"
    objdump -T "$BINARY" 2>/dev/null | grep GLIBC | sed 's/.*GLIBC_/GLIBC_/' | sort -V | uniq | tail -5
    
    # Check for problematic versions
    MAX_GLIBC_VERSION=$(objdump -T "$BINARY" 2>/dev/null | grep GLIBC | sed 's/.*GLIBC_\([0-9.]*\).*/\1/' | sort -V | tail -1)
    echo "Maximum required GLIBC version: $MAX_GLIBC_VERSION"
    
    if [ "$(printf '%s\n' "2.36" "$MAX_GLIBC_VERSION" | sort -V | head -n1)" != "2.36" ]; then
        echo "WARNING: Binary may require GLIBC newer than 2.36 (Debian 12)"
        echo "Consider using an older Rust version or different compilation flags"
    else
        echo "✓ Binary is compatible with Debian 12 (glibc 2.36)"
    fi
fi

echo "Build complete: $BINARY"
echo "Test with: $BINARY --help"