#!/bin/bash

# Simple DDNS Updater CLI Test Script
# Tests the command line interface with test configurations

# Change to project root directory if we're in the scripts directory
if [[ "$(basename "$PWD")" == "scripts" ]]; then
    cd ..
fi

echo "🧪 DDNS Updater CLI Test Suite"
echo "=============================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test results
PASSED=0
FAILED=0

# Function to run test and check exit code
test_command() {
    local name="$1"
    local cmd="$2"
    local expected_exit="$3"
    
    echo -e "\n${BLUE}Test: $name${NC}"
    echo "Command: $cmd"
    
    # Run command and capture exit code
    bash -c "$cmd" >/dev/null 2>&1
    local actual_exit=$?
    
    if [ "$actual_exit" -eq "$expected_exit" ]; then
        echo -e "${GREEN}✓ PASS${NC} (exit code: $actual_exit)"
        ((PASSED++))
    else
        echo -e "${RED}✗ FAIL${NC} (expected: $expected_exit, got: $actual_exit)"
        ((FAILED++))
    fi
}

# Build first
echo -e "\n${YELLOW}Building project...${NC}"
cargo build --quiet

echo -e "\n${YELLOW}Running CLI tests...${NC}"

# Test 1: Help (should succeed)
test_command "Help command" "cargo run --quiet -- --help" 0

# Test 2: Version (should succeed)
test_command "Version command" "cargo run --quiet -- --version" 0

# Test 3: Valid single config
test_command "Valid single config" "cargo run --quiet -- --config test_configs/valid/basic_server.conf --no-reload" 0

# Test 4: Valid config directory
test_command "Valid config directory" "cargo run --quiet -- --config-dir test_configs/valid --no-reload" 0

# Test 5: Invalid config file
test_command "Invalid config file" "cargo run --quiet -- --config test_configs/invalid/plain_text.conf --no-reload" 1

# Test 6: Non-existent file
test_command "Non-existent file" "cargo run --quiet -- --config /non/existent.conf --no-reload" 1

# Test 7: Custom host
test_command "Custom host" "cargo run --quiet -- --host example.com --config test_configs/valid/minimal_valid.conf --no-reload" 0

# Test 8: Pattern matching (use basic pattern that should work)
test_command "Pattern matching" "cargo run --quiet -- --config-dir test_configs/valid --pattern '*.conf' --no-reload" 0

# Test 9: Verbose mode
echo -e "\n${BLUE}Test: Verbose mode (with output)${NC}"
echo "Command: cargo run --quiet -- --config test_configs/valid/complex_ssl.conf --verbose --no-reload"
echo "Output:"
cargo run --quiet -- --config test_configs/valid/complex_ssl.conf --verbose --no-reload
if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ PASS${NC}"
    ((PASSED++))
else
    echo -e "${RED}✗ FAIL${NC}"
    ((FAILED++))
fi

# Clean up any test backup directories
if [ -d "test_backups" ]; then
    rm -rf test_backups
fi

# Summary
echo -e "\n${YELLOW}========================${NC}"
echo -e "${YELLOW}Test Results Summary${NC}"
echo -e "${YELLOW}========================${NC}"
echo -e "Passed: ${GREEN}$PASSED${NC}"
echo -e "Failed: ${RED}$FAILED${NC}"

if [ $FAILED -eq 0 ]; then
    echo -e "\n${GREEN}🎉 All CLI tests passed!${NC}"
    exit 0
else
    echo -e "\n${RED}❌ Some tests failed!${NC}"
    exit 1
fi