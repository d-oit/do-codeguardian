#!/bin/bash

# CodeGuardian Security Audit Script
# Runs comprehensive security checks including cargo-deny and cargo-audit

set -e

echo "🔒 CodeGuardian Security Audit"
echo "=============================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to log errors
log_error() {
    echo -e "${RED}❌ $1${NC}" >&2
}

# Function to log warnings
log_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

# Function to log success
log_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

# Function to log info
log_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

# Track overall status
OVERALL_STATUS=0

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    log_error "Please run this script from the project root directory"
    exit 1
fi

# 1. Run cargo-deny for security and license checks
log_info "Running cargo-deny security and license checks..."
if command -v cargo-deny >/dev/null 2>&1; then
    if cargo deny check; then
        log_success "cargo-deny checks passed"
    else
        log_error "cargo-deny checks failed"
        OVERALL_STATUS=1
    fi

    # Run advisories check specifically
    if cargo deny check advisories; then
        log_success "Security advisories check passed"
    else
        log_error "Security advisories check failed"
        OVERALL_STATUS=1
    fi

    # Run licenses check
    if cargo deny check licenses; then
        log_success "License compliance check passed"
    else
        log_error "License compliance check failed"
        OVERALL_STATUS=1
    fi
else
    log_warning "cargo-deny not installed. Installing..."
    if cargo install cargo-deny; then
        log_success "cargo-deny installed successfully"
        # Retry the checks
        if cargo deny check && cargo deny check advisories && cargo deny check licenses; then
            log_success "cargo-deny checks passed after installation"
        else
            log_error "cargo-deny checks failed after installation"
            OVERALL_STATUS=1
        fi
    else
        log_error "Failed to install cargo-deny"
        OVERALL_STATUS=1
    fi
fi

# 2. Run cargo-audit if available
log_info "Running cargo-audit security checks..."
if cargo audit --version >/dev/null 2>&1; then
    if cargo audit; then
        log_success "cargo-audit checks passed"
    else
        log_error "cargo-audit checks failed"
        OVERALL_STATUS=1
    fi
else
    log_warning "cargo-audit not available. Run 'cargo install cargo-audit' to enable"
fi

# 3. Check for known vulnerable dependencies
log_info "Checking for outdated dependencies..."
if cargo outdated --exit-code 1 >/dev/null 2>&1; then
    log_warning "Some dependencies are outdated"
    OVERALL_STATUS=1
else
    log_success "Dependencies are up to date"
fi

# Summary
echo ""
echo "=============================="
if [ $OVERALL_STATUS -eq 0 ]; then
    log_success "Security audit completed successfully!"
    log_info "All security and license checks passed."
else
    log_warning "Security audit completed with issues"
    log_info "Review the output above and address any security vulnerabilities or license issues."
fi

exit $OVERALL_STATUS</content>
</xai:function_call">

<xai:function_call name="bash">
<parameter name="command">chmod +x scripts/security_audit.sh
