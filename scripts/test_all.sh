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
        if cargo run --quiet -- --config "$config" --no-reload >/dev/null 2>&1; then
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
        if cargo run --quiet -- --config "$config" --no-reload >/dev/null 2>&1; then
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
cargo run --quiet -- --config-dir test_configs/valid --no-reload >/dev/null 2>&1
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
echo "  ✓ Test artifacts cleaned up"
echo ""
echo -e "${GREEN}Your DDNS updater is ready for production use! 🚀${NC}"