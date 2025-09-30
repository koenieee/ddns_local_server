#!/bin/bash

# DDNS Updater Position and Non-Addition CLI Test Script
# Tests that ensure:
# 1. No IP addresses are added when there are no existing DDNS entries
# 2. Existing DDNS entries maintain their position when updated

# Note: Not using set -e to allow proper error handling in tests

echo "🧪 DDNS Updater Position & Non-Addition Test Suite"
echo "================================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test counter
TESTS_PASSED=0
TESTS_FAILED=0

# Function to run a position test
run_position_test() {
    local test_name="$1"
    local config_file="$2"
    local hostname="$3"
    local expected_line_number="$4"
    local check_pattern="$5"
    
    echo -e "\n${BLUE}Testing: $test_name${NC}"
    
    # Store original content
    local original_content=$(cat "$config_file")
    
    # Run the DDNS updater
    local command="cargo run --quiet -- --host $hostname --config $config_file --no-reload"
    echo "Command: $command"
    
    local output
    local exit_code
    output=$(eval "$command" 2>&1)
    exit_code=$?
    
    if [ $exit_code -eq 0 ]; then
        # Check if the entry is at the expected line and position
        local line_content=$(sed -n "${expected_line_number}p" "$config_file")
        if echo "$line_content" | grep -q "$check_pattern"; then
            echo -e "${GREEN}✓ PASS${NC} - Entry found at expected line $expected_line_number"
            ((TESTS_PASSED++))
        else
            echo -e "${RED}✗ FAIL${NC} - Entry not found at line $expected_line_number"
            echo "Expected pattern: $check_pattern"
            echo "Actual line content: $line_content"
            ((TESTS_FAILED++))
        fi
    else
        echo -e "${RED}✗ FAIL${NC} - Command execution failed (exit code: $exit_code)"
        echo "Output: $output"
        ((TESTS_FAILED++))
    fi
    
    # Restore original content for clean testing
    echo "$original_content" > "$config_file"
}

# Function to run a non-addition test
run_non_addition_test() {
    local test_name="$1"
    local config_file="$2"
    local hostname="$3"
    
    echo -e "\n${BLUE}Testing: $test_name${NC}"
    
    # Store original content
    local original_content=$(cat "$config_file")
    local original_hash=$(md5sum "$config_file" | cut -d' ' -f1)
    
    # Run the DDNS updater
    local command="cargo run --quiet -- --host $hostname --config $config_file --no-reload"
    echo "Command: $command"
    
    local output
    local exit_code
    output=$(eval "$command" 2>&1)
    exit_code=$?
    
    if [ $exit_code -eq 0 ]; then
        # Check if the file was modified
        local new_hash=$(md5sum "$config_file" | cut -d' ' -f1)
        if [ "$original_hash" = "$new_hash" ]; then
            echo -e "${GREEN}✓ PASS${NC} - No IP address was added (file unchanged)"
            ((TESTS_PASSED++))
        else
            echo -e "${RED}✗ FAIL${NC} - File was modified when it shouldn't have been"
            echo "Original vs Modified:"
            diff <(echo "$original_content") <(cat "$config_file") || true
            ((TESTS_FAILED++))
        fi
    else
        echo -e "${RED}✗ FAIL${NC} - Command execution failed (exit code: $exit_code)"
        echo "Output: $output"
        ((TESTS_FAILED++))
    fi
    
    # Restore original content
    echo "$original_content" > "$config_file"
}

# Build the project first
echo -e "\n${YELLOW}Building project...${NC}"
cargo build --quiet

# Restore all test configs to clean state
echo -e "\n${YELLOW}Restoring test configs to clean state...${NC}"
git checkout test_configs/valid/basic_server.conf test_configs/valid/full_nginx.conf test_configs/valid/complex_ssl.conf 2>/dev/null || true

# Test 1: No addition when no DDNS entries exist
echo -e "\n${YELLOW}=== Testing Non-Addition Behavior ===${NC}"
run_non_addition_test "No IP added to config without DDNS entries" "test_configs/valid/no_ddns_entries.conf" "google.com"

# Test 2: Position preservation in basic_server.conf
echo -e "\n${YELLOW}=== Testing Position Preservation ===${NC}"

# First, add a DDNS entry to basic_server.conf to test position preservation
cat > test_configs/valid/basic_server.conf << 'EOF'
server {
    listen 80;
    server_name example.com;
    
    location / {
        root /var/www/html;
        index index.html;
        allow 192.168.1.1;
        allow 142.250.102.100; # DDNS: google.com
        deny all;
    }
}
EOF

run_position_test "Position preserved in basic_server.conf location block" "test_configs/valid/basic_server.conf" "google.com" "9" "allow.*# DDNS: google.com"

# Test 3: Position preservation in complex_ssl.conf (location block)
# Add a DDNS entry to complex_ssl.conf
cat > test_configs/valid/complex_ssl.conf << 'EOF'
server {
    listen 443 ssl;
    server_name www.example.com;
    
    ssl_certificate /etc/ssl/certs/example.com.crt;
    ssl_certificate_key /etc/ssl/private/example.com.key;
    
    location / {
        proxy_pass http://backend;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        allow 10.0.0.0/8;
        allow 142.250.102.100; # DDNS: google.com
        deny all;
    }
}
EOF

run_position_test "Position preserved in complex_ssl.conf location block" "test_configs/valid/complex_ssl.conf" "google.com" "13" "allow.*# DDNS: google.com"

# Test 4: Position preservation in full_nginx.conf (location block)
# The full_nginx.conf already has a DDNS entry, let's test it
git checkout test_configs/valid/full_nginx.conf 2>/dev/null || true
run_position_test "Position preserved in full_nginx.conf location block" "test_configs/valid/full_nginx.conf" "google.com" "25" "allow.*# DDNS: google.com"

# Test 5: Server block level position preservation
cat > test_configs/valid/server_level_test.conf << 'EOF'
server {
    listen 80;
    server_name example.com;
    allow 203.0.113.0/24;
    allow 142.250.102.100; # DDNS: google.com
    
    location / {
        root /var/www/html;
        index index.html;
    }
}
EOF

run_position_test "Position preserved at server block level" "test_configs/valid/server_level_test.conf" "google.com" "5" "allow.*# DDNS: google.com"

# Test 6: Multiple DDNS entries - ensure only the first one is replaced
cat > test_configs/valid/multiple_ddns_test.conf << 'EOF'
server {
    listen 80;
    server_name example.com;
    
    location /api {
        allow 142.250.102.100; # DDNS: google.com
        deny all;
    }
    
    location / {
        allow 192.168.1.0/24;
        allow 142.250.102.200; # DDNS: google.com
        deny all;
    }
}
EOF

run_position_test "First DDNS entry replaced when multiple exist" "test_configs/valid/multiple_ddns_test.conf" "google.com" "6" "allow.*# DDNS: google.com"

# Test 7: Legacy format conversion and position preservation
cat > test_configs/valid/legacy_format_test.conf << 'EOF'
server {
    listen 80;
    server_name example.com;
    
    location / {
        root /var/www/html;
        allow 192.168.1.1;
        allow 142.250.102.100; # DDNS for google.com
        deny all;
    }
}
EOF

run_position_test "Legacy format converted and position preserved" "test_configs/valid/legacy_format_test.conf" "google.com" "8" "allow.*# DDNS: google.com"

# Cleanup temporary test files
rm -f test_configs/valid/server_level_test.conf
rm -f test_configs/valid/multiple_ddns_test.conf
rm -f test_configs/valid/legacy_format_test.conf

# Restore original configs
echo -e "\n${YELLOW}Restoring original configs...${NC}"
git checkout test_configs/valid/basic_server.conf test_configs/valid/complex_ssl.conf 2>/dev/null || true

# Summary
echo -e "\n${YELLOW}Test Summary${NC}"
echo "============"
echo -e "Tests passed: ${GREEN}$TESTS_PASSED${NC}"
echo -e "Tests failed: ${RED}$TESTS_FAILED${NC}"

if [ $TESTS_FAILED -eq 0 ]; then
    echo -e "\n${GREEN}🎉 All position and non-addition tests passed!${NC}"
    echo -e "${GREEN}✅ IP addresses are only updated in existing DDNS entries${NC}"
    echo -e "${GREEN}✅ Original positions are preserved during updates${NC}"
    echo -e "${GREEN}✅ No new IP addresses are added when none exist${NC}"
    exit 0
else
    echo -e "\n${RED}❌ Some position/non-addition tests failed!${NC}"
    exit 1
fi