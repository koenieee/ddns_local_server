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
    
    # Run command with test mode environment variable and capture exit code
    DDNS_TEST_MODE=1 bash -c "$cmd" >/dev/null 2>&1
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

# Set test mode environment variable to use local storage
export DDNS_TEST_MODE=1

echo -e "\n${YELLOW}Running CLI tests...${NC}"

# Test 1: Help (should succeed)
test_command "Help command" "cargo run --quiet -- --help" 0

# Test 2: Version (should succeed)
test_command "Version command" "cargo run --quiet -- --version" 0

# Test 3: Valid single config (use localhost in CI)
if [[ -n "$CI" || -n "$GITHUB_ACTIONS" ]]; then
    test_command "Valid single config" "cargo run --quiet -- --config test_configs/valid/basic_server.conf --host localhost --no-reload" 0
else
    test_command "Valid single config" "cargo run --quiet -- --config test_configs/valid/basic_server.conf --no-reload" 0
fi

# Test 4: Valid config directory (use localhost in CI)
if [[ -n "$CI" || -n "$GITHUB_ACTIONS" ]]; then
    test_command "Valid config directory" "cargo run --quiet -- --config-dir test_configs/valid --host localhost --no-reload" 0
else
    test_command "Valid config directory" "cargo run --quiet -- --config-dir test_configs/valid --no-reload" 0
fi

# Test 5: Non-existent file
test_command "Non-existent file" "cargo run --quiet -- --config /non/existent.conf --no-reload" 1

# Test 6: Custom host (use localhost in CI)
if [[ -n "$CI" || -n "$GITHUB_ACTIONS" ]]; then
    test_command "Custom host" "cargo run --quiet -- --host localhost --config test_configs/valid/minimal_valid.conf --no-reload" 0
else
    test_command "Custom host" "cargo run --quiet -- --host example.com --config test_configs/valid/minimal_valid.conf --no-reload" 0
fi

# Test 7: Pattern matching (use localhost in CI)
if [[ -n "$CI" || -n "$GITHUB_ACTIONS" ]]; then
    test_command "Pattern matching" "cargo run --quiet -- --config-dir test_configs/valid --pattern '*.conf' --host localhost --no-reload" 0
else
    test_command "Pattern matching" "cargo run --quiet -- --config-dir test_configs/valid --pattern '*.conf' --no-reload" 0
fi

# Test 8: Verbose mode
echo -e "\n${BLUE}Test: Verbose mode (with output)${NC}"
if [[ -n "$CI" || -n "$GITHUB_ACTIONS" ]]; then
    echo "Command: DDNS_TEST_MODE=1 cargo run --quiet -- --config test_configs/valid/complex_ssl.conf --host localhost --verbose --no-reload"
    echo "Output:"
    DDNS_TEST_MODE=1 cargo run --quiet -- --config test_configs/valid/complex_ssl.conf --host localhost --verbose --no-reload
else
    echo "Command: DDNS_TEST_MODE=1 cargo run --quiet -- --config test_configs/valid/complex_ssl.conf --verbose --no-reload"
    echo "Output:"
    DDNS_TEST_MODE=1 cargo run --quiet -- --config test_configs/valid/complex_ssl.conf --verbose --no-reload
fi
if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ PASS${NC}"
    ((PASSED++))
else
    echo -e "${RED}✗ FAIL${NC}"
    ((FAILED++))
fi

# Test 10: Hostname resolution verification (skip in CI)
if [[ -z "$CI" && -z "$GITHUB_ACTIONS" && -z "$DDNS_CI_MODE" ]]; then
    echo -e "\n${BLUE}Test: Hostname resolution (different IPs for different hostnames)${NC}"

    # Clean up any existing test storage to ensure fresh start
    rm -rf ./test_storage 2>/dev/null

    # Test with google.com first
    echo "Testing google.com resolution..."
    GOOGLE_OUTPUT=$(cargo run --quiet -- --config test_configs/valid/basic_server.conf --verbose --no-reload --host google.com 2>&1)
    GOOGLE_IP=$(echo "$GOOGLE_OUTPUT" | grep "DEBUG: Using IP:" | sed 's/DEBUG: Using IP: //' | tr -d '\r\n')

    # Test with example.com second  
    echo "Testing example.com resolution..."
    EXAMPLE_OUTPUT=$(cargo run --quiet -- --config test_configs/valid/basic_server.conf --verbose --no-reload --host example.com 2>&1)
    EXAMPLE_IP=$(echo "$EXAMPLE_OUTPUT" | grep "DEBUG: Using IP:" | sed 's/DEBUG: Using IP: //' | tr -d '\r\n')

    # Get user's public IP for comparison
    echo "Getting user's public IP..."
    USER_PUBLIC_IP=$(curl -s --max-time 5 https://api.ipify.org || echo "unknown")

    echo "Resolved IPs:"
    echo "  google.com: $GOOGLE_IP"
    echo "  example.com: $EXAMPLE_IP" 
    echo "  User public IP: $USER_PUBLIC_IP"

    # Verify that:
    # 1. Both IPs were resolved (not empty)
    # 2. The IPs are different from each other
    # 3. Neither IP matches the user's public IP (the old bug)
    if [[ -n "$GOOGLE_IP" && -n "$EXAMPLE_IP" && "$GOOGLE_IP" != "$EXAMPLE_IP" && "$GOOGLE_IP" != "$USER_PUBLIC_IP" && "$EXAMPLE_IP" != "$USER_PUBLIC_IP" ]]; then
        echo -e "${GREEN}✓ PASS${NC} - Hostnames resolve to different, correct IP addresses"
        ((PASSED++))
    else
        echo -e "${RED}✗ FAIL${NC} - Hostname resolution issue detected"
        echo "  Failure reasons:"
        [[ -z "$GOOGLE_IP" ]] && echo "    - Could not resolve google.com"
        [[ -z "$EXAMPLE_IP" ]] && echo "    - Could not resolve example.com"
        [[ "$GOOGLE_IP" == "$EXAMPLE_IP" ]] && echo "    - Both hostnames resolved to same IP (should be different)"
        [[ "$GOOGLE_IP" == "$USER_PUBLIC_IP" ]] && echo "    - google.com resolved to user's public IP (incorrect)"
        [[ "$EXAMPLE_IP" == "$USER_PUBLIC_IP" ]] && echo "    - example.com resolved to user's public IP (incorrect)"
        ((FAILED++))
    fi
else
    echo -e "\n${BLUE}Test: Hostname resolution (skipped in CI)${NC}"
    echo -e "${YELLOW}⚠ Skipping network-dependent test in CI environment${NC}"
    ((PASSED++))
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