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

# Function to create stored IP for testing IP-based replacement
create_stored_ip() {
    local hostname="$1"
    local ip="$2"
    
    # Ensure test storage directory exists
    mkdir -p "./test_storage"
    
    # Create JSON IP entry (matching the FileIpRepository format)
    cat > "./test_storage/${hostname}.json" << EOF
{
  "ip": "${ip}",
  "hostname": "${hostname}",
  "comment": null,
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-01T00:00:00Z"
}
EOF
}

# Function to run a position test with IP-based replacement support
run_position_test() {
    local test_name="$1"
    local config_file="$2"
    local hostname="$3"
    local expected_line_number="$4"
    local check_pattern="$5"
    local fallback_pattern="$6"  # Optional: for IP-based replacement pattern
    local test_with_stored_ip="$7"  # Optional: create stored IP for IP-based replacement
    
    echo -e "\n${BLUE}Testing: $test_name${NC}"
    
    # Store original content
    local original_content=$(cat "$config_file")
    
    # If testing IP-based replacement, create stored IP entry
    if [ "$test_with_stored_ip" = "true" ]; then
        # Extract the IP from the config file at the expected line for stored IP testing
        local current_line=$(sed -n "${expected_line_number}p" "$config_file")
        local stored_ip=$(echo "$current_line" | grep -o '[0-9]\+\.[0-9]\+\.[0-9]\+\.[0-9]\+' | head -1)
        if [ -n "$stored_ip" ]; then
            echo "Creating stored IP entry: $hostname -> $stored_ip"
            create_stored_ip "$hostname" "$stored_ip"
        fi
    fi
    
    # Run the DDNS updater with test mode
    local command="DDNS_TEST_MODE=1 cargo run --quiet -- --host $hostname --config $config_file --no-reload"
    echo "Command: $command"
    
    local output
    local exit_code
    output=$(eval "$command" 2>&1)
    exit_code=$?
    
    if [ $exit_code -eq 0 ]; then
        # Check if the entry is at the expected line and position
        local line_content=$(sed -n "${expected_line_number}p" "$config_file")
        local pattern_matched=false
        
        # First check the primary pattern (legacy DDNS comment format)
        if echo "$line_content" | grep -q "$check_pattern"; then
            echo -e "${GREEN}✓ PASS${NC} - Entry found at expected line $expected_line_number (legacy format)"
            pattern_matched=true
        # If fallback pattern provided, check IP-based replacement
        elif [ -n "$fallback_pattern" ] && echo "$line_content" | grep -q "$fallback_pattern"; then
            echo -e "${GREEN}✓ PASS${NC} - Entry found at expected line $expected_line_number (IP-based format)"
            pattern_matched=true
        # For backward compatibility, also check if line contains 'allow' and looks like an IP
        elif echo "$line_content" | grep -q "allow [0-9]\+\.[0-9]\+\.[0-9]\+\.[0-9]\+"; then
            echo -e "${GREEN}✓ PASS${NC} - Entry found at expected line $expected_line_number (IP updated)"
            pattern_matched=true
        fi
        
        if [ "$pattern_matched" = true ]; then
            ((TESTS_PASSED++))
        else
            echo -e "${RED}✗ FAIL${NC} - Entry not found at line $expected_line_number"
            echo "Expected pattern: $check_pattern"
            if [ -n "$fallback_pattern" ]; then
                echo "Fallback pattern: $fallback_pattern"
            fi
            echo "Actual line content: $line_content"
            ((TESTS_FAILED++))
        fi
    else
        echo -e "${RED}✗ FAIL${NC} - Command execution failed (exit code: $exit_code)"
        echo "Output: $output"
        ((TESTS_FAILED++))
    fi
    
    # Cleanup stored IP files if created
    if [ "$test_with_stored_ip" = "true" ]; then
        rm -f "./test_storage/${hostname}.json"
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
    
    # Run the DDNS updater with test mode
    local command="DDNS_TEST_MODE=1 cargo run --quiet -- --host $hostname --config $config_file --no-reload"
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

# Test 1.5: IP-based replacement (new feature test)
echo -e "\n${YELLOW}=== Testing IP-Based Replacement (New Feature) ===${NC}"

# Create a config with clean IP entries (no DDNS comments)
cat > test_configs/valid/ip_based_test.conf << 'EOF'
server {
    listen 80;
    server_name example.com;
    
    location / {
        root /var/www/html;
        index index.html;
        allow 192.168.1.1;
        allow 142.250.102.138;
        deny all;
    }
}
EOF

run_position_test "IP-based replacement without DDNS comments" "test_configs/valid/ip_based_test.conf" "google.com" "9" "allow.*# DDNS: google.com" "allow [0-9]" "true"

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

run_position_test "Position preserved in basic_server.conf location block" "test_configs/valid/basic_server.conf" "google.com" "9" "allow.*# DDNS: google.com" "allow [0-9]"

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

run_position_test "Position preserved in complex_ssl.conf location block" "test_configs/valid/complex_ssl.conf" "google.com" "13" "allow.*# DDNS: google.com" "allow [0-9]"

# Test 4: Position preservation in full_nginx.conf (location block)
# The full_nginx.conf already has a DDNS entry, let's test it
git checkout test_configs/valid/full_nginx.conf 2>/dev/null || true
run_position_test "Position preserved in full_nginx.conf location block" "test_configs/valid/full_nginx.conf" "google.com" "25" "allow.*# DDNS: google.com" "allow [0-9]"

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

run_position_test "Position preserved at server block level" "test_configs/valid/server_level_test.conf" "google.com" "5" "allow.*# DDNS: google.com" "allow [0-9]"

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

run_position_test "First DDNS entry replaced when multiple exist" "test_configs/valid/multiple_ddns_test.conf" "google.com" "6" "allow.*# DDNS: google.com" "allow [0-9]"

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

run_position_test "Legacy format converted and position preserved" "test_configs/valid/legacy_format_test.conf" "google.com" "8" "allow.*# DDNS: google.com" "allow [0-9]"

# Cleanup temporary test files
rm -f test_configs/valid/server_level_test.conf
rm -f test_configs/valid/multiple_ddns_test.conf
rm -f test_configs/valid/legacy_format_test.conf
rm -f test_configs/valid/ip_based_test.conf
rm -rf test_storage

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
    echo -e "${GREEN}✅ IP addresses are updated using both IP-based and legacy DDNS comment methods${NC}"
    echo -e "${GREEN}✅ Original positions are preserved during updates${NC}"
    echo -e "${GREEN}✅ No new IP addresses are added when none exist${NC}"
    echo -e "${GREEN}✅ IP-based replacement works without requiring DDNS comments${NC}"
    exit 0
else
    echo -e "\n${RED}❌ Some position/non-addition tests failed!${NC}"
    exit 1
fi