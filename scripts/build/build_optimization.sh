#!/bin/bash
# CodeGuardian Build Optimization Script
# Provides fast development builds and optimized release builds

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}🚀 CodeGuardian Build Optimization${NC}"
echo "=================================="

# Function to measure build time
measure_build_time() {
    local cmd="$1"
    local label="$2"

    echo -e "${YELLOW}Building: ${label}${NC}"
    local start_time=$(date +%s.%3N)

    if eval "$cmd"; then
        local end_time=$(date +%s.%3N)
        local duration=$(echo "$end_time - $start_time" | bc)
        echo -e "${GREEN}✅ ${label} completed in ${duration}s${NC}"
        return 0
    else
        echo -e "${RED}❌ ${label} failed${NC}"
        return 1
    fi
}

# Fast development build
fast_dev_build() {
    echo -e "${BLUE}Fast Development Build (minimal features)${NC}"
    measure_build_time "cargo build --profile dev-fast --features dev" "Fast Dev Build"
}

# Check build
check_build() {
    echo -e "${BLUE}Check Build (no codegen)${NC}"
    measure_build_time "cargo check --profile dev-fast --features dev" "Check Build"
}

# Optimized release build
release_build() {
    echo -e "${BLUE}Optimized Release Build${NC}"
    measure_build_time "cargo build --release --features full" "Release Build"
}

# Clean build artifacts
clean_build() {
    echo -e "${YELLOW}Cleaning build artifacts...${NC}"
    cargo clean
    echo -e "${GREEN}✅ Clean completed${NC}"
}

# Show usage
usage() {
    echo "Usage: $0 [COMMAND]"
    echo ""
    echo "Commands:"
    echo "  fast      - Fast development build (minimal features)"
    echo "  check     - Quick check build (no codegen)"
    echo "  release   - Optimized release build"
    echo "  clean     - Clean build artifacts"
    echo "  all       - Run all builds sequentially"
    echo "  help      - Show this help"
    echo ""
    echo "Examples:"
    echo "  $0 fast     # Fast iterative development"
    echo "  $0 check    # Quick syntax checking"
    echo "  $0 release  # Production build"
}

case "${1:-help}" in
    "fast")
        fast_dev_build
        ;;
    "check")
        check_build
        ;;
    "release")
        release_build
        ;;
    "clean")
        clean_build
        ;;
    "all")
        check_build
        echo ""
        fast_dev_build
        echo ""
        release_build
        ;;
    "help"|*)
        usage
        ;;
esac
