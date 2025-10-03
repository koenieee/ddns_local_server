#!/bin/bash

# DDNS Updater CLI Test Script
# Tests the command line interface with various configuration scenarios

# Note: We don't use "set -e" here because we need to handle test failures gracefully

echo "🧪 DDNS Updater CLI Test Suite"
echo "=============================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test counter
TESTS_PASSED=0
TESTS_FAILED=0

# Function to run a test
run_test() {
    local test_name="$1"
    local command="$2"
    local expected_exit_code="${3:-0}"
    
    echo -e "\n${BLUE}Testing: $test_name${NC}"
    echo "Command: $command"
    
    # Capture both stdout and stderr, and get exit code
    local output
    local actual_exit_code
    
    set +e  # Disable exit on error for this test
    output=$(DDNS_TEST_MODE=1 eval "$command" 2>&1)
    actual_exit_code=$?
    set -e  # Re-enable exit on error (though not using set -e globally anymore)
    
    if [ $actual_exit_code -eq $expected_exit_code ]; then
        echo -e "${GREEN}✓ PASS${NC}"
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        echo -e "${RED}✗ FAIL${NC} (expected exit code $expected_exit_code, got $actual_exit_code)"
        echo "Output: $output"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
}

# Function to run a verbose test (show output)
run_verbose_test() {
    local test_name="$1"
    local command="$2"
    local expected_exit_code="${3:-0}"
    
    echo -e "\n${BLUE}Testing: $test_name${NC}"
    echo "Command: $command"
    echo "Output:"
    echo "-------"
    
    set +e  # Disable exit on error for this test
    DDNS_TEST_MODE=1 eval "$command"
    actual_exit_code=$?
    set -e  # Re-enable exit on error
    
    if [ $actual_exit_code -eq $expected_exit_code ]; then
        echo -e "${GREEN}✓ PASS${NC}"
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        echo -e "${RED}✗ FAIL${NC} (expected exit code $expected_exit_code, got $actual_exit_code)"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
}

# Build the project first
echo -e "\n${YELLOW}Building project...${NC}"
cargo build --quiet

# Set test mode environment variable to use local storage
export DDNS_TEST_MODE=1

# Test 1: Help command
run_test "Help command" "cargo run --quiet -- --help"

# Test 2: Version command  
run_test "Version command" "cargo run --quiet -- --version"

# Test 3: Valid single config file
run_test "Valid single config file" "cargo run --quiet -- --config test_configs/valid/basic_server.conf --no-reload"

# Test 4: Valid config directory
run_test "Valid config directory" "cargo run --quiet -- --config-dir test_configs/valid --no-reload"

# Test 5: Invalid single config file (should fail)
run_test "Invalid single config file" "cargo run --quiet -- --config test_configs/invalid/plain_text.conf --no-reload" 1

# Test 6: Invalid config directory (should fail) 
run_test "Invalid config directory" "cargo run --quiet -- --config-dir test_configs/invalid --no-reload" 1

# Test 7: Non-existent config file (should fail)
run_test "Non-existent config file" "cargo run --quiet -- --config non_existent.conf --no-reload" 1

# Test 8: Non-existent config directory (should fail)
run_test "Non-existent config directory" "cargo run --quiet -- --config-dir /non/existent/dir --no-reload" 1

# Test 9: Different host
run_test "Different host" "cargo run --quiet -- --host example.com --config test_configs/valid/minimal_valid.conf --no-reload"

# Test 10: Custom backup directory
run_test "Custom backup directory" "cargo run --quiet -- --config test_configs/valid/basic_server.conf --backup-dir test_backups --no-reload"

# Test 11: Verbose mode with valid config
echo -e "\n${YELLOW}Running verbose tests (with output)...${NC}"
run_verbose_test "Verbose mode with valid config" "cargo run --quiet -- --config test_configs/valid/complex_ssl.conf --verbose --no-reload"

# Test 12: Pattern matching in config directory
run_verbose_test "Pattern matching" "cargo run --quiet -- --config-dir test_configs/valid --pattern complex* --verbose --no-reload"

# Cleanup test backup directory if created
if [ -d "test_backups" ]; then
    rm -rf test_backups
    echo -e "\n${YELLOW}Cleaned up test backup directory${NC}"
fi

# Summary
echo -e "\n${YELLOW}Test Summary${NC}"
echo "============"
echo -e "Tests passed: ${GREEN}$TESTS_PASSED${NC}"
echo -e "Tests failed: ${RED}$TESTS_FAILED${NC}"

if [ $TESTS_FAILED -eq 0 ]; then
    echo -e "\n${GREEN}🎉 All CLI tests passed!${NC}"
    exit 0
else
    echo -e "\n${RED}❌ Some CLI tests failed!${NC}"
    exit 1
fi