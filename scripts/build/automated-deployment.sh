#!/bin/bash
# Automated Deployment Pipeline for CodeGuardian
# Handles AST-enhanced builds, testing, and deployment

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m'

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
DEPLOY_DIR="$PROJECT_ROOT/deploy"
ARTIFACTS_DIR="$PROJECT_ROOT/artifacts"
TIMESTAMP=$(date +%Y%m%d-%H%M%S)
VERSION=$(git rev-parse --short HEAD 2>/dev/null || echo "dev")

# Deployment configuration
DEPLOY_ENV="${DEPLOY_ENV:-staging}"
DEPLOY_TARGET="${DEPLOY_TARGET:-local}"
ENABLE_AST="${ENABLE_AST:-true}"
ENABLE_ML="${ENABLE_ML:-true}"

# Function to log deployment steps
log_step() {
    local step="$1"
    local message="$2"
    echo -e "${BLUE}[$TIMESTAMP] ${step}: ${message}${NC}"
}

# Function to log errors
log_error() {
    local message="$1"
    echo -e "${RED}❌ ERROR: ${message}${NC}" >&2
}

# Function to log success
log_success() {
    local message="$1"
    echo -e "${GREEN}✅ ${message}${NC}"
}

# Setup deployment environment
setup_deployment() {
    log_step "SETUP" "Setting up deployment environment"

    # Create directories
    mkdir -p "$DEPLOY_DIR"
    mkdir -p "$ARTIFACTS_DIR"

    # Clean previous deployments
    rm -rf "$DEPLOY_DIR"/*

    # Set environment variables
    export CARGO_INCREMENTAL=0
    export RUST_BACKTRACE=1

    log_success "Deployment environment ready"
}

# Build AST-enhanced binary
build_ast_enhanced() {
    log_step "BUILD" "Building AST-enhanced CodeGuardian"

    cd "$PROJECT_ROOT"

    # Determine features
    local features="git,security,logging"
    if [ "$ENABLE_ML" = "true" ]; then
        features="$features,ml"
    fi
    if [ "$ENABLE_AST" = "true" ]; then
        features="$features,ast"
    fi

    echo -e "${CYAN}Building with features: $features${NC}"

    # Build optimized release
    if ! cargo build --release --features "$features" --quiet; then
        log_error "Build failed"
        exit 1
    fi

    # Strip binary for smaller size
    strip target/release/do-codeguardian

    # Verify binary
    if ! ./target/release/do-codeguardian --version; then
        log_error "Binary verification failed"
        exit 1
    fi

    log_success "AST-enhanced build completed"
}

# Run comprehensive tests
run_tests() {
    log_step "TEST" "Running comprehensive test suite"

    cd "$PROJECT_ROOT"

    # Unit tests
    if ! cargo test --features ml-enhanced --quiet; then
        log_error "Unit tests failed"
        exit 1
    fi

    # Integration tests
    if ! cargo test --test integration_tests --features ml-enhanced --quiet; then
        log_error "Integration tests failed"
        exit 1
    fi

    # AST-specific tests
    if [ "$ENABLE_AST" = "true" ]; then
        if ! cargo test --features ast --quiet; then
            log_error "AST tests failed"
            exit 1
        fi
    fi

    log_success "All tests passed"
}

# Run AST analysis validation
validate_ast_analysis() {
    if [ "$ENABLE_AST" != "true" ]; then
        log_step "VALIDATE" "Skipping AST validation (AST disabled)"
        return 0
    fi

    log_step "VALIDATE" "Validating AST analysis capabilities"

    cd "$PROJECT_ROOT"

    # Create test file with AST features
    cat > test_ast_file.rs << 'EOF'
/// Test function with documentation
#[test]
fn test_example() {
    let x = "hello".to_string();
    let y = x.unwrap();
    if y.len() > 0 {
        println!("Not empty");
    }
}

struct TestStruct {
    field: i32,
}

unsafe fn dangerous_function() {
    // Unsafe code here
}
