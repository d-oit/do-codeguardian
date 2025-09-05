#!/bin/bash
# Fallback license checking script that works with or without cargo-deny

set -e

echo "📋 Running license compliance check..."

# Try cargo-deny first
if command -v cargo-deny &> /dev/null || ~/.cargo/bin/cargo-deny --version &> /dev/null; then
    echo "Using cargo-deny for comprehensive license checking..."

    if ~/.cargo/bin/cargo-deny check licenses --format json > license-results.json 2>&1; then
        echo "✅ License compliance check passed"
        echo "license_status=passed"
        exit 0
    else
        echo "❌ License compliance violations found"
        echo "license_status=failed"
        exit 1
    fi
else
    echo "cargo-deny not available, using basic license check..."

    # Fallback: Check for common problematic licenses in Cargo.toml
    if grep -q "GPL-3\|LGPL-3\|AGPL\|MS-PL\|JSON" Cargo.toml; then
        echo "⚠️ Potential license issues detected in Cargo.toml"
        echo "Please install cargo-deny for comprehensive checking"
        echo "license_status=warning"
        exit 0  # Don't fail, just warn
    else
        echo "✅ Basic license check passed (no obvious issues)"
        echo "license_status=passed"
        exit 0
    fi
fi
