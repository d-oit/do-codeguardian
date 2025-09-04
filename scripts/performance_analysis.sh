#!/bin/bash
# CodeGuardian Performance Analysis Script
# Runs comprehensive performance analysis and benchmarks

set -e

echo "⚡ CodeGuardian Performance Analysis"
echo "===================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to log errors without failing
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

# Run benchmarks if available
log_info "Checking for available benchmarks..."
if cargo bench --list >/dev/null 2>&1; then
    log_info "Running comprehensive benchmarks..."
    if cargo bench --bench comprehensive_performance_benchmark; then
        log_success "Comprehensive benchmarks completed"
    else
        log_warning "Comprehensive benchmarks failed"
        OVERALL_STATUS=1
    fi

    log_info "Running hashing benchmarks..."
    if cargo bench --bench hashing_benchmark; then
        log_success "Hashing benchmarks completed"
    else
        log_warning "Hashing benchmarks failed"
        OVERALL_STATUS=1
    fi

    log_info "Running existing performance benchmarks..."
    if cargo bench --bench performance_benchmark; then
        log_success "Performance benchmarks completed"
    else
        log_warning "Performance benchmarks failed"
        OVERALL_STATUS=1
    fi
else
    log_warning "Benchmarks not available or criterion not configured"
    OVERALL_STATUS=1
fi

# Run performance regression tests
log_info "Running performance regression tests..."
if cargo test performance_regression_tests --release -- --nocapture; then
    log_success "Performance regression tests completed"
else
    log_error "Performance regression tests failed"
    OVERALL_STATUS=1
fi

# Summary
echo ""
echo "===================================="
if [ $OVERALL_STATUS -eq 0 ]; then
    log_success "Performance analysis complete!"
    log_info "Check the output above for performance insights and recommendations."
else
    log_warning "Performance analysis completed with some failures"
    log_info "Check the output above for details. Some benchmarks may not be available."
fi

# Exit with status (but don't fail the CI since this is non-critical)
exit $OVERALL_STATUS
