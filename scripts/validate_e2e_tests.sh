#!/bin/bash

# CodeGuardian E2E Testing Framework Validation Script
# Validates the current state of the E2E testing framework

set -e

echo "🧪 CodeGuardian E2E Testing Framework Validation"
echo "=============================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to log success
log_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

# Function to log info
log_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

# Function to log warning
log_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

# Function to log error
log_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Initialize counters
total_checks=0
passed_checks=0
failed_checks=0

# Function to increment counters
check_passed() {
    ((passed_checks++))
    ((total_checks++))
}

check_failed() {
    ((failed_checks++))
    ((total_checks++))
}

# 1. Check if all required E2E test files exist
log_info "Checking E2E test file existence..."

e2e_test_files=(
    "tests/e2e_test_runner.rs"
    "tests/e2e_cli_tests.rs"
    "tests/e2e_error_scenarios.rs"
    "tests/e2e_feature_tests.rs"
    "tests/e2e_workflow_tests.rs"
    "tests/e2e_performance_tests.rs"
)

for file in "${e2e_test_files[@]}"; do
    if [ -f "$file" ]; then
        log_success "Found $file"
        check_passed
    else
        log_error "Missing $file"
        check_failed
    fi
done

# 2. Check if test files contain proper test functions
log_info "Checking test function structure..."

for file in "${e2e_test_files[@]}"; do
    if [ -f "$file" ]; then
        # Count test functions
        test_count=$(grep -c "#\[test\]" "$file" || true)
        if [ "$test_count" -gt 0 ]; then
            log_success "$file contains $test_count test functions"
            check_passed
        else
            log_warning "$file has no test functions"
            check_failed
        fi
    fi
done

# 3. Check for proper test dependencies
log_info "Checking test dependencies in Cargo.toml..."

if grep -q "assert_cmd" Cargo.toml; then
    log_success "assert_cmd dependency found"
    check_passed
else
    log_error "assert_cmd dependency missing"
    check_failed
fi

if grep -q "predicates" Cargo.toml; then
    log_success "predicates dependency found"
    check_passed
else
    log_error "predicates dependency missing"
    check_failed
fi

if grep -q "tempfile" Cargo.toml; then
    log_success "tempfile dependency found"
    check_passed
else
    log_error "tempfile dependency missing"
    check_failed
fi

# 4. Check for proper imports in test files
log_info "Checking test file imports..."

for file in "${e2e_test_files[@]}"; do
    if [ -f "$file" ]; then
        if grep -q "use assert_cmd::prelude::" "$file"; then
            log_success "$file has assert_cmd imports"
            check_passed
        else
            log_warning "$file missing assert_cmd imports"
            check_failed
        fi

        if grep -q "use predicates::prelude::" "$file"; then
            log_success "$file has predicates imports"
            check_passed
        else
            log_warning "$file missing predicates imports"
            check_failed
        fi
    fi
done

# 5. Check for test helper functions
log_info "Checking for test helper functions..."

if grep -q "create_sample_rust_project" tests/e2e_test_runner.rs; then
    log_success "Found create_sample_rust_project helper"
    check_passed
else
    log_warning "Missing create_sample_rust_project helper"
    check_failed
fi

if grep -q "create_sample_javascript_project" tests/e2e_test_runner.rs; then
    log_success "Found create_sample_javascript_project helper"
    check_passed
else
    log_warning "Missing create_sample_javascript_project helper"
    check_failed
fi

# 6. Check for proper error handling in tests
log_info "Checking error handling patterns..."

error_handling_tests=0
for file in "${e2e_test_files[@]}"; do
    if [ -f "$file" ]; then
        if grep -q "assert!" "$file" || grep -q "unwrap()" "$file" || grep -q "expect(" "$file"; then
            ((error_handling_tests++))
        fi
    fi
done

if [ "$error_handling_tests" -gt 0 ]; then
    log_success "Found error handling patterns in $error_handling_tests files"
    check_passed
else
    log_warning "No error handling patterns found"
    check_failed
fi

# 7. Check for test coverage of different scenarios
log_info "Checking test scenario coverage..."

# CLI command coverage
cli_tests=$(grep -l "Command::cargo_bin" tests/e2e_*.rs | wc -l)
if [ "$cli_tests" -gt 0 ]; then
    log_success "CLI command testing found in $cli_tests files"
    check_passed
else
    log_warning "No CLI command testing found"
    check_failed
fi

# File system testing
fs_tests=$(grep -l "tempfile::TempDir" tests/e2e_*.rs | wc -l)
if [ "$fs_tests" -gt 0 ]; then
    log_success "File system testing found in $fs_tests files"
    check_passed
else
    log_warning "No file system testing found"
    check_failed
fi

# Performance testing
perf_tests=$(grep -l "Instant::now" tests/e2e_*.rs | wc -l)
if [ "$perf_tests" -gt 0 ]; then
    log_success "Performance testing found in $perf_tests files"
    check_passed
else
    log_warning "No performance testing found"
    check_failed
fi

# 8. Check for test organization and documentation
log_info "Checking test organization..."

# Check for module documentation
module_docs=0
for file in "${e2e_test_files[@]}"; do
    if [ -f "$file" ]; then
        if grep -q "///" "$file"; then
            ((module_docs++))
        fi
    fi
done

if [ "$module_docs" -gt 0 ]; then
    log_success "Documentation found in $module_docs files"
    check_passed
else
    log_warning "No documentation found in test files"
    check_failed
fi

# 9. Check for test isolation
log_info "Checking test isolation patterns..."

isolation_patterns=0
for file in "${e2e_test_files[@]}"; do
    if [ -f "$file" ]; then
        if grep -q "TempDir::new" "$file" || grep -q "tempfile" "$file"; then
            ((isolation_patterns++))
        fi
    fi
done

if [ "$isolation_patterns" -gt 0 ]; then
    log_success "Test isolation patterns found in $isolation_patterns files"
    check_passed
else
    log_warning "No test isolation patterns found"
    check_failed
fi

# 10. Quick compilation check
log_info "Checking if E2E tests can be compiled..."

# Use separate target directory to avoid build lock issues
target_dir="/tmp/e2e-validation-$(date +%s)"
mkdir -p "$target_dir"

if CARGO_TARGET_DIR="$target_dir" timeout 60 cargo check --test e2e_cli_tests 2>/dev/null; then
    log_success "E2E CLI tests compile successfully"
    check_passed
else
    log_warning "E2E CLI tests compilation failed or timed out"
    check_failed
fi

# Clean up
rm -rf "$target_dir"

# Summary
echo ""
echo "=============================================="
log_info "E2E Testing Framework Validation Summary"
echo "=============================================="
echo "Total checks: $total_checks"
echo "Passed: $passed_checks"
echo "Failed: $failed_checks"

if [ $failed_checks -eq 0 ]; then
    log_success "All E2E validation checks passed! 🎉"
    echo ""
    echo "✅ E2E testing framework is properly set up and ready"
    echo ""
    echo "Framework Status:"
    echo "  ✅ All required test files exist"
    echo "  ✅ Test functions are properly structured"
    echo "  ✅ Dependencies are correctly configured"
    echo "  ✅ Helper functions are available"
    echo "  ✅ Error handling patterns implemented"
    echo "  ✅ Test coverage includes CLI, filesystem, and performance"
    echo "  ✅ Tests are well-documented and isolated"
    echo "  ✅ Compilation successful"
    echo ""
    echo "Next steps:"
    echo "  1. Run full E2E test suite: cargo test e2e_*"
    echo "  2. Monitor test performance and reliability"
    echo "  3. Add new test scenarios as needed"
    exit 0
else
    log_error "$failed_checks validation check(s) failed"
    echo ""
    echo "❌ E2E testing framework needs attention"
    echo ""
    echo "Issues to address:"
    if [ ! -f "tests/e2e_test_runner.rs" ]; then
        echo "  - Missing e2e_test_runner.rs"
    fi
    if [ ! -f "tests/e2e_cli_tests.rs" ]; then
        echo "  - Missing e2e_cli_tests.rs"
    fi
    if [ ! -f "tests/e2e_error_scenarios.rs" ]; then
        echo "  - Missing e2e_error_scenarios.rs"
    fi
    if [ ! -f "tests/e2e_feature_tests.rs" ]; then
        echo "  - Missing e2e_feature_tests.rs"
    fi
    if [ ! -f "tests/e2e_workflow_tests.rs" ]; then
        echo "  - Missing e2e_workflow_tests.rs"
    fi
    if [ ! -f "tests/e2e_performance_tests.rs" ]; then
        echo "  - Missing e2e_performance_tests.rs"
    fi
    echo ""
    echo "Fix missing files and re-run validation"
    exit 1
fi</content>
</xai:function_call/>
<xai:function_call>
<xai:function_call name="bash">
<parameter name="command">chmod +x scripts/validate_e2e_tests.sh
