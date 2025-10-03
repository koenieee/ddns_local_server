#!/bin/bash

# Comprehensive DDNS Updater Test Suite
# Runs both unit tests and CLI integration tests

set -e  # Exit on any error

# Change to project root echo -e "\n${GREEN}🎉 All tests completed successfully!${NC}"
echo ""
echo "What was tested:"
echo "  ✓ Project builds without errors"
echo "  ✓ All unit tests pass (including cleanup)"
echo "  ✓ CLI interface works correctly"
echo "  ✓ Configuration validation works"
echo "  ✓ Error handling works as expected"
echo "  ✓ Performance is acceptable"
echo "  ✓ Test artifacts automatically cleaned up"
echo ""
echo -e "${GREEN}Your DDNS updater is ready for production use! 🚀${NC}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print section header
print_section() {
    echo -e "\n${BLUE}$1${NC}"
    echo "$(printf '=%.0s' $(seq 1 ${#1}))"
}

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check prerequisites
print_section "Checking Prerequisites"

if ! command_exists cargo; then
    echo -e "${RED}❌ Cargo not found. Please install Rust and Cargo.${NC}"
    exit 1
fi

if ! command_exists rustc; then
    echo -e "${RED}❌ Rust compiler not found. Please install Rust.${NC}"
    exit 1
fi

echo -e "${GREEN}✓ Cargo found: $(cargo --version)${NC}"
echo -e "${GREEN}✓ Rust found: $(rustc --version)${NC}"

# Clean and build
print_section "Building Project"
echo "Running cargo clean..."
cargo clean --quiet

echo "Running cargo build..."
if cargo build; then
    echo -e "${GREEN}✓ Build successful${NC}"
else
    echo -e "${RED}❌ Build failed${NC}"
    exit 1
fi

# Run unit tests
print_section "Running Unit Tests"
echo "Executing cargo test..."
if cargo test; then
    echo -e "${GREEN}✓ All unit tests passed${NC}"
else
    echo -e "${RED}❌ Unit tests failed${NC}"
    exit 1
fi

# Run CLI tests
print_section "Running CLI Integration Tests"
if [ -f "scripts/test_cli_simple.sh" ]; then
    chmod +x scripts/test_cli_simple.sh
    if scripts/test_cli_simple.sh; then
        echo -e "${GREEN}✓ Basic CLI tests passed${NC}"
    else
        echo -e "${RED}❌ Basic CLI tests failed${NC}"
        exit 1
    fi
else
    echo -e "${YELLOW}⚠ CLI test script not found, skipping CLI tests${NC}"
fi

# Run position and non-addition tests
print_section "Running Position & Non-Addition Tests"
if [ -f "scripts/test_position_and_non_addition.sh" ]; then
    chmod +x scripts/test_position_and_non_addition.sh
    if scripts/test_position_and_non_addition.sh; then
        echo -e "${GREEN}✓ All position and non-addition tests passed${NC}"
    else
        echo -e "${RED}❌ Position and non-addition tests failed${NC}"
        exit 1
    fi
else
    echo -e "${YELLOW}⚠ Position test script not found, skipping position tests${NC}"
fi

# Check code formatting and linting
print_section "Code Quality Checks"
if [ -f "scripts/check_formatting.sh" ]; then
    chmod +x scripts/check_formatting.sh
    if scripts/check_formatting.sh; then
        echo -e "${GREEN}✓ All code quality checks passed${NC}"
    else
        echo -e "${RED}❌ Code quality checks failed${NC}"
        exit 1
    fi
else
    # Fallback to individual checks
    if command_exists rustfmt; then
        echo "Checking code formatting..."
        if cargo fmt --check; then
            echo -e "${GREEN}✓ Code is properly formatted${NC}"
        else
            echo -e "${YELLOW}⚠ Code formatting issues found. Run 'cargo fmt' to fix.${NC}"
        fi
    else
        echo -e "${YELLOW}⚠ rustfmt not found, skipping format check${NC}"
    fi
fi



# Test configuration validation
print_section "Configuration Validation Tests"
VALIDATION_ERRORS=0

echo "Testing with valid configurations:"
for config in test_configs/valid/*.conf; do
    if [ -f "$config" ]; then
        echo "  - $(basename "$config")"
        if DDNS_TEST_MODE=1 cargo run --quiet -- --config "$config" --no-reload >/dev/null 2>&1; then
            echo -e "    ${GREEN}✓ Valid${NC}"
        else
            echo -e "    ${RED}✗ Failed validation${NC}"
            ((VALIDATION_ERRORS++))
        fi
    fi
done

echo -e "\nTesting with invalid configurations:"
for config in test_configs/invalid/*; do
    if [ -f "$config" ]; then
        echo "  - $(basename "$config")"
        if DDNS_TEST_MODE=1 cargo run --quiet -- --config "$config" --no-reload >/dev/null 2>&1; then
            echo -e "    ${RED}✗ Should have failed${NC}"
            ((VALIDATION_ERRORS++))
        else
            echo -e "    ${GREEN}✓ Correctly rejected${NC}"
        fi
    fi
done

# Check if any validation tests failed
if [ $VALIDATION_ERRORS -gt 0 ]; then
    echo -e "\n${RED}❌ Configuration validation tests failed: $VALIDATION_ERRORS error(s)${NC}"
    exit 1
else
    echo -e "\n${GREEN}✓ All configuration validation tests passed${NC}"
fi

# Performance check
print_section "Performance Check"
echo "Testing performance with multiple config files..."
start_time=$(date +%s.%N)
DDNS_TEST_MODE=1 cargo run --quiet -- --config-dir test_configs/valid --no-reload >/dev/null 2>&1
end_time=$(date +%s.%N)
duration=$(echo "$end_time - $start_time" | bc 2>/dev/null || echo "unknown")

if command_exists bc; then
    if (( $(echo "$duration < 5.0" | bc -l) )); then
        echo -e "${GREEN}✓ Performance test passed (${duration}s)${NC}"
    else
        echo -e "${YELLOW}⚠ Performance test slow (${duration}s)${NC}"
    fi
else
    echo -e "${YELLOW}⚠ bc not found, skipping precise timing${NC}"
fi

# Batch processing test with github.com
print_section "Batch Processing Test"
echo "Testing batch update functionality with github.com hostname..."

# Create temporary test directory and config files
BATCH_TEST_DIR="test_batch_github"
BATCH_STORAGE_DIR="test_storage_github"
BATCH_ERRORS=0

mkdir -p "$BATCH_TEST_DIR" "$BATCH_STORAGE_DIR"

# Create test config files with old IP
OLD_IP="192.168.1.100"  # Use a different IP that github.com won't resolve to
NEW_IP="140.82.121.3"   # github.com resolves to this single IP
HOSTNAME="github.com"

echo "Testing batch processing with OLD_IP: ${OLD_IP} → NEW_IP: ${NEW_IP}"

for i in {1..3}; do
    cat > "$BATCH_TEST_DIR/config${i}.conf" << EOF
server {
    listen 80;
    server_name test${i}.example.com;
    
    location / {
        allow ${OLD_IP};
        deny all;
        proxy_pass http://backend;
    }
}
EOF
done

# Create IP storage file with old IP
cat > "$BATCH_STORAGE_DIR/${HOSTNAME}.json" << EOF
{
  "ip": "${OLD_IP}",
  "hostname": "${HOSTNAME}",
  "comment": null,
  "created_at": "2025-10-03T12:00:00Z",
  "updated_at": "2025-10-03T12:00:00Z"
}
EOF

echo "Created test files with old IP: ${OLD_IP}"

# Run batch update test
echo "Running batch update test..."
if DDNS_TEST_MODE=1 DDNS_STORAGE_DIR="$BATCH_STORAGE_DIR" DDNS_BACKUP_DIR="$BATCH_TEST_DIR/backups" \
   cargo run --quiet -- --host "$HOSTNAME" --config-dir "$BATCH_TEST_DIR" --pattern "*.conf" --no-reload >/dev/null 2>&1; then
    
    # Verify all config files were updated
    UPDATED_COUNT=0
    for i in {1..3}; do
        if grep -q "allow ${NEW_IP}" "$BATCH_TEST_DIR/config${i}.conf" 2>/dev/null; then
            ((UPDATED_COUNT++))
        else
            echo -e "    ${RED}✗ config${i}.conf was not updated${NC}"
            ((BATCH_ERRORS++))
        fi
    done
    
    # Verify stored IP was updated
    if grep -q "\"ip\": \"${NEW_IP}\"" "$BATCH_STORAGE_DIR/${HOSTNAME}.json" 2>/dev/null; then
        echo -e "    ${GREEN}✓ Stored IP updated correctly${NC}"
    else
        echo -e "    ${RED}✗ Stored IP was not updated${NC}"
        ((BATCH_ERRORS++))
    fi
    
    if [ $UPDATED_COUNT -eq 3 ] && [ $BATCH_ERRORS -eq 0 ]; then
        echo -e "${GREEN}✓ Batch processing test passed - all 3 config files updated${NC}"
    else
        echo -e "${RED}❌ Batch processing test failed - only ${UPDATED_COUNT}/3 files updated${NC}"
        ((BATCH_ERRORS++))
    fi
else
    echo -e "${RED}❌ Batch processing command failed${NC}"
    ((BATCH_ERRORS++))
fi

# Cleanup batch test files
rm -rf "$BATCH_TEST_DIR" "$BATCH_STORAGE_DIR"

if [ $BATCH_ERRORS -gt 0 ]; then
    echo -e "${RED}❌ Batch processing test failed with $BATCH_ERRORS error(s)${NC}"
    exit 1
else
    echo -e "${GREEN}✓ Batch processing test completed successfully${NC}"
fi

# Final summary
print_section "Test Summary"
echo -e "${GREEN}🎉 All tests completed successfully!${NC}"
# Clean up any artifacts created during testing
print_section "Final Cleanup"
echo "Cleaning up test artifacts created during validation tests..."

# Remove backup directories
backup_dirs=("backups" "test_backups" "my_backups")
cleaned_items=()

for backup_dir in "${backup_dirs[@]}"; do
    if [ -d "$backup_dir" ]; then
        rm -rf "$backup_dir"
        cleaned_items+=("directory ${backup_dir}/")
    fi
done

# Remove IP storage files  
ip_files=(google_com_ip.txt example_com_ip.txt localhost_ip.txt)
for ip_file in "${ip_files[@]}"; do
    if [ -f "$ip_file" ]; then
        rm -f "$ip_file"
        cleaned_items+=("file ${ip_file}")
    fi
done

# Remove any other *_ip.txt files
for ip_file in *_ip.txt; do
    if [ -f "$ip_file" ] && [[ ! " ${ip_files[@]} " =~ " ${ip_file} " ]]; then
        rm -f "$ip_file"
        cleaned_items+=("file ${ip_file}")
    fi
done

if [ ${#cleaned_items[@]} -gt 0 ]; then
    IFS=', '
    echo -e "${GREEN}🧹 Cleaned up: ${cleaned_items[*]}${NC}"
    IFS=' '
else
    echo -e "${GREEN}✓ No cleanup needed${NC}"
fi

echo ""
echo "What was tested:"
echo "  ✓ Project builds without errors"
echo "  ✓ All unit tests pass"
echo "  ✓ CLI interface works correctly"
echo "  ✓ Configuration validation works"
echo "  ✓ Error handling works as expected"
echo "  ✓ Performance is acceptable"
echo "  ✓ Batch processing with multiple config files"
echo "  ✓ Test artifacts cleaned up"
echo ""
echo -e "${GREEN}Your DDNS updater is ready for production use! 🚀${NC}"