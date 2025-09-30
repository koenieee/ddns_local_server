#!/bin/bash

# DDNS Updater - Configuration Display Script
# Shows the current configuration of running DDNS updater services

print_header() {
    echo "============================== DDNS Updater Configuration =============================="
    echo ""
}

print_service_config() {
    local service_name="$1"
    echo "=== $service_name ==="
    
    if systemctl is-loaded "$service_name" &>/dev/null; then
        echo "Status: $(systemctl is-active "$service_name") ($(systemctl is-enabled "$service_name"))"
        
        # Show environment variables if the service is loaded
        local env_vars=$(systemctl show "$service_name" --property=Environment --value 2>/dev/null | grep "DDNS_" || true)
        
        if [ -n "$env_vars" ]; then
            echo "Configuration:"
            echo "$env_vars" | while IFS= read -r env_var; do
                if [[ "$env_var" =~ ^DDNS_ ]]; then
                    echo "  $env_var"
                fi
            done
        fi
        
        # Show the actual command being executed
        local exec_start=$(systemctl show "$service_name" --property=ExecStart --value 2>/dev/null)
        if [ -n "$exec_start" ]; then
            echo "Command:"
            echo "  $exec_start" | sed 's/^[^/]*/  /'
        fi
        
        # Show last execution info if available
        local last_run=$(systemctl show "$service_name" --property=ExecMainStartTimestamp --value 2>/dev/null)
        if [ -n "$last_run" ] && [ "$last_run" != "n/a" ]; then
            echo "Last Run: $last_run"
        fi
        
    else
        echo "Status: Not installed/loaded"
    fi
    echo ""
}

print_timer_config() {
    local timer_name="$1"
    echo "=== $timer_name ==="
    
    if systemctl is-loaded "$timer_name" &>/dev/null; then
        echo "Status: $(systemctl is-active "$timer_name") ($(systemctl is-enabled "$timer_name"))"
        
        # Show timer details
        local next_run=$(systemctl show "$timer_name" --property=NextElapseUSecRealtime --value 2>/dev/null)
        if [ -n "$next_run" ] && [ "$next_run" != "0" ]; then
            local next_run_human=$(date -d "@$((next_run / 1000000))" 2>/dev/null || echo "Unknown")
            echo "Next Run: $next_run_human"
        fi
        
        local last_trigger=$(systemctl show "$timer_name" --property=LastTriggerUSec --value 2>/dev/null)
        if [ -n "$last_trigger" ] && [ "$last_trigger" != "0" ]; then
            local last_trigger_human=$(date -d "@$((last_trigger / 1000000))" 2>/dev/null || echo "Unknown")
            echo "Last Trigger: $last_trigger_human"
        fi
        
    else
        echo "Status: Not installed/loaded"
    fi
    echo ""
}

# Main script
print_header

# Check main DDNS updater service
print_service_config "ddns-updater.service"
print_timer_config "ddns-updater.timer"

# Check backup cleanup service
print_service_config "ddns-backup-cleanup.service"
print_timer_config "ddns-backup-cleanup.timer"

# Check target
echo "=== ddns-updater.target ==="
if systemctl is-loaded "ddns-updater.target" &>/dev/null; then
    echo "Status: $(systemctl is-active "ddns-updater.target") ($(systemctl is-enabled "ddns-updater.target"))"
    
    # Show what services are wanted by this target
    local wants=$(systemctl show "ddns-updater.target" --property=Wants --value 2>/dev/null)
    if [ -n "$wants" ]; then
        echo "Managed Services: $wants"
    fi
else
    echo "Status: Not installed/loaded"
fi
echo ""

# Check for template instances
echo "=== Template Instances ==="
instances=$(systemctl list-units "ddns-updater@*.service" "ddns-updater@*.timer" --no-legend --no-pager 2>/dev/null | awk '{print $1}' | sort -u || true)

if [ -n "$instances" ]; then
    echo "$instances" | while IFS= read -r instance; do
        if [[ "$instance" =~ \.service$ ]]; then
            print_service_config "$instance"
        elif [[ "$instance" =~ \.timer$ ]]; then
            print_timer_config "$instance"
        fi
    done
else
    echo "No template instances found"
    echo ""
fi

echo "============================== End Configuration =============================="
echo ""
echo "Usage:"
echo "  systemctl status ddns-updater.target    # Overall status"
echo "  systemctl start ddns-updater.target     # Start all services"
echo "  systemctl stop ddns-updater.target      # Stop all services"
echo "  journalctl -u ddns-updater.service      # View logs"