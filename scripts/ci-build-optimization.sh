#!/bin/bash
# CI Build Optimization Script for CodeGuardian
# Handles build failures, dependency issues, and optimization

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Configuration
MAX_RETRIES=3
BUILD_TIMEOUT=900  # 15 minutes
CARGO_INCREMENTAL=0

echo -e "${BLUE}🚀 CodeGuardian CI Build Optimization${NC}"
echo "====================================="

# Function to measure build time
measure_build_time() {
    local cmd="$1"
    local label="$2"
    local timeout="${3:-$BUILD_TIMEOUT}"

    echo -e "${YELLOW}Building: ${label}${NC}"
    local start_time=$(date +%s.%3N)

    # Run with timeout
    if timeout "$timeout" bash -c "$cmd"; then
        local end_time=$(date +%s.%3N)
        local duration=$(echo "$end_time - $start_time" | bc)
        echo -e "${GREEN}✅ ${label} completed in ${duration}s${NC}"
        return 0
    else
        local end_time=$(date +%s.%3N)
        local duration=$(echo "$end_time - $start_time" | bc)
        echo -e "${RED}❌ ${label} failed after ${duration}s${NC}"
        return 1
    fi
}

# Function to retry build with cleanup
retry_build() {
    local cmd="$1"
    local label="$2"
    local retries="$3"

    for i in $(seq 1 "$retries"); do
        echo -e "${YELLOW}Attempt $i/$retries: ${label}${NC}"

        # Clean on retry
        if [ "$i" -gt 1 ]; then
            echo "Cleaning build artifacts..."
            cargo clean
        fi

        if measure_build_time "$cmd" "${label} (attempt $i)"; then
            return 0
        fi

        if [ "$i" -lt "$retries" ]; then
            echo "Waiting before retry..."
            sleep 5
        fi
    done

    return 1
}

# Pre-build setup
setup_build_environment() {
    echo -e "${YELLOW}Setting up build environment...${NC}"

    # Disable incremental compilation for CI
    export CARGO_INCREMENTAL=0

    # Set build profile
    export RUST_BACKTRACE=1

    # Clean stale artifacts
    rm -rf target/debug/.cargo-lock target/release/.cargo-lock

    # Update registry
    cargo update --quiet

    echo -e "${GREEN}✅ Build environment setup complete${NC}"
}

# Test different feature combinations
test_feature_builds() {
    echo -e "${BLUE}Testing feature combinations...${NC}"

    local features=(
        ""
        "--features git"
        "--features security"
        "--features logging"
        "--features git,security,logging"
        "--features full"
        "--all-features"
    )

    for feature_flag in "${features[@]}"; do
        echo -e "${YELLOW}Testing: cargo check $feature_flag${NC}"
        if cargo check $feature_flag --quiet; then
            echo -e "${GREEN}✅ Features work: $feature_flag${NC}"
        else
            echo -e "${RED}❌ Features failed: $feature_flag${NC}"
            return 1
        fi
    done

    echo -e "${GREEN}✅ All feature combinations tested${NC}"
}

# Main build function
main_build() {
    setup_build_environment

    # Test features first
    if ! test_feature_builds; then
        echo -e "${RED}❌ Feature testing failed${NC}"
        exit 1
    fi

    # Build with default features
    if ! retry_build "cargo build --release" "Release Build" "$MAX_RETRIES"; then
        echo -e "${RED}❌ Release build failed${NC}"
        exit 1
    fi

    # Build all targets
    if ! retry_build "cargo build --all-targets --release" "All Targets Build" "$MAX_RETRIES"; then
        echo -e "${RED}❌ All targets build failed${NC}"
        exit 1
    fi

    # Run checks
    if ! retry_build "cargo check --all-features --all-targets" "Full Check" "$MAX_RETRIES"; then
        echo -e "${RED}❌ Full check failed${NC}"
        exit 1
    fi

    # Test compilation
    if ! retry_build "cargo test --all-features --no-run" "Test Compilation" "$MAX_RETRIES"; then
        echo -e "${RED}❌ Test compilation failed${NC}"
        exit 1
    fi

    echo -e "${GREEN}🎉 All builds completed successfully!${NC}"
}

# Run main build
case "${1:-build}" in
    "build")
        main_build
        ;;
    "features")
        test_feature_builds
        ;;
    "clean")
        echo -e "${YELLOW}Cleaning build artifacts...${NC}"
        cargo clean
        echo -e "${GREEN}✅ Clean completed${NC}"
        ;;
    "setup")
        setup_build_environment
        ;;
    *)
        echo "Usage: $0 [build|features|clean|setup]"
        echo "  build    - Run full build pipeline"
        echo "  features - Test feature combinations"
        echo "  clean    - Clean build artifacts"
        echo "  setup    - Setup build environment"
        exit 1
        ;;
esac
