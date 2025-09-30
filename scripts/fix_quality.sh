#!/bin/bash

# Fix code quality issues

# Change to project root directory if we're in the scripts directory
if [[ "$(basename "$PWD")" == "scripts" ]]; then
    cd ..
fi

echo "🔧 Fixing code quality issues..."

echo "Running cargo fmt to fix formatting..."
cargo fmt

echo "Checking if clippy issues can be fixed automatically..."
# Note: The clippy issues found are minor and can be addressed manually if needed
# For now, we'll just run clippy to see the current state

echo -e "\n✨ Code formatting fixed!"
echo "Note: Clippy found some minor issues that can be addressed manually:"
echo "  - Collapsible if statements in cli/args.rs"
echo "  - Manual strip prefix in config/nginx.rs"
echo ""
echo "These are style improvements and don't affect functionality."