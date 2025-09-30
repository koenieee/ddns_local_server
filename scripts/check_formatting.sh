#!/bin/bash

# CI-style formatting and linting check script
set -e

echo "🔍 Running formatting and linting checks..."
echo "==========================================="

echo "📐 Checking code formatting..."
if cargo fmt --check; then
    echo "✅ Code formatting is correct"
else
    echo "❌ Code formatting issues found. Run 'cargo fmt' to fix."
    exit 1
fi

echo ""
echo "🔍 Running clippy analysis..."
if cargo clippy --all-targets --all-features --workspace -- -D warnings; then
    echo "✅ No clippy warnings found"
else
    echo "❌ Clippy warnings found. Fix the issues and try again."
    exit 1
fi

echo ""
echo "🎉 All formatting and linting checks passed!"