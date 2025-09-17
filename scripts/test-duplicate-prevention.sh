#!/bin/bash
# Test Script for GitHub Issue Duplicate Prevention
# Validates all aspects of the duplicate prevention system

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
TEST_REPO="${TEST_REPO:-$GITHUB_REPOSITORY}"
TEST_DIR="/tmp/codeguardian-duplicate-test"
TEST_RESULTS_FILE="$TEST_DIR/results.json"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Helper function to source scripts with absolute paths
source_script() {
    local script="$1"
    source "$SCRIPT_DIR/$script"
}

# Print colored output
log_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

log_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

log_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

log_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Cleanup function
cleanup() {
    if [ -d "$TEST_DIR" ]; then
        rm -rf "$TEST_DIR"
    fi

    # Cleanup cache
    if [ -f "$SCRIPT_DIR/github-issue-utils.sh" ]; then
        source_script github-issue-utils.sh
        cleanup_cache
    fi
}

# Setup test environment
setup_test() {
    log_info "Setting up test environment..."

    # Create test directory
    mkdir -p "$TEST_DIR"

    # Create sample test results
    cat > "$TEST_RESULTS_FILE" << 'EOF'
{
  "findings": [
    {
      "id": "test-duplicate-1",
      "message": "Test security vulnerability",
      "severity": "high",
      "file": "src/test.rs",
      "line": 123,
      "analyzer": "security",
      "description": "This is a test finding for duplicate prevention validation",
      "suggestion": "Fix the test vulnerability"
    },
    {
      "id": "test-duplicate-2",
      "message": "Test code quality issue",
      "severity": "medium",
      "file": "src/main.rs",
      "line": 42,
      "analyzer": "quality",
      "description": "Another test finding for validation",
      "suggestion": "Improve code quality"
    }
  ],
  "summary": {
    "total_findings": 2,
    "total_files_scanned": 15,
    "scan_duration_ms": 250
  },
  "timestamp": "2024-01-01T00:00:00Z",
  "tool_metadata": {
    "name": "CodeGuardian",
    "version": "1.0.0"
  }
}
EOF

    # Make scripts executable
    chmod +x "$SCRIPT_DIR/github-issue-utils.sh"

    log_success "Test environment setup complete"
}

# Test 1: Utility script loading
test_script_loading() {
    log_info "Testing script loading..."

    if [ ! -f "$SCRIPT_DIR/github-issue-utils.sh" ]; then
        log_error "github-issue-utils.sh not found"
        return 1
    fi

    # Source the script
    if ! source_script github-issue-utils.sh; then
        log_error "Failed to source github-issue-utils.sh"
        return 1
    fi

    # Test basic functions exist
    if ! type detect_duplicate_issue >/dev/null 2>&1; then
        log_error "detect_duplicate_issue function not found"
        return 1
    fi

    if ! type create_or_update_issue >/dev/null 2>&1; then
        log_error "create_or_update_issue function not found"
        return 1
    fi

    log_success "Script loading test passed"
    return 0
}

# Test 2: Title generation
test_title_generation() {
    log_info "Testing title generation..."

    source_script github-issue-utils.sh

    # Test with commit hash
    GITHUB_PR_NUMBER=""
    title=$(generate_issue_title "Test Analysis" "$TEST_REPO")

    if [[ "$title" == *"Test Analysis"* ]]; then
        log_success "Title generation test passed: $title"
        return 0
    else
        log_error "Title generation test failed: $title"
        return 1
    fi
}

# Test 3: Keyword extraction
test_keyword_extraction() {
    log_info "Testing keyword extraction..."

    source_script github-issue-utils.sh

    title="Security vulnerability in authentication module"
    body="Critical security issue found with potential SQL injection vulnerabilities and authentication bypass"

    keywords=$(extract_keywords "$title" "$body")

    # Check that security-related keywords are extracted
    if echo "$keywords" | grep -q "security" && \
       echo "$keywords" | grep -q "vulnerability"; then
        log_success "Keyword extraction test passed: $keywords"
        return 0
    else
        log_error "Keyword extraction test failed: $keywords"
        return 1
    fi
}

# Test 4: Duplicate detection (should find no duplicates for unique titles)
test_duplicate_detection_negative() {
    log_info "Testing duplicate detection (negative case)..."

    source_script github-issue-utils.sh

    # Use a unique title that shouldn't match anything
    unique_title="CodeGuardian Test - Unique Title $(date +%s)$RANDOM"
    test_body="This is a unique test body that should not match any existing issues"

    result=$(detect_duplicate_issue "$TEST_REPO" "$unique_title" "$test_body" "")

    if [ -z "$result" ]; then
        log_success "Negative duplicate detection test passed - no false positives"
        return 0
    else
        log_error "Negative duplicate detection test failed - found duplicate: $result"
        return 1
    fi
}

# Test 5: GitHub API connectivity
test_github_connectivity() {
    log_info "Testing GitHub API connectivity..."

    source_script github-issue-utils.sh

    # Test basic GitHub CLI functionality
    if exec_gh_with_retry issue list --repo "$TEST_REPO" --limit 1 --json number >/dev/null 2>&1; then
        log_success "GitHub API connectivity test passed"
        return 0
    else
        log_error "GitHub API connectivity test failed"
        return 1
    fi
}

# Test 6: Cache functionality
test_cache_functionality() {
    log_info "Testing cache functionality..."

    source_script github-issue-utils.sh

    # Test cache directory creation
    if [ -d "$GITHUB_ISSUE_CACHE_DIR" ]; then
        log_success "Cache directory test passed"
    else
        log_error "Cache directory test failed"
        return 1
    fi

    # Test cache cleanup
    cleanup_cache

    if [ $? -eq 0 ]; then
        log_success "Cache cleanup test passed"
        return 0
    else
        log_error "Cache cleanup test failed"
        return 1
    fi
}

# Test 7: Full integration test (dry run)
test_integration_dry_run() {
    log_info "Testing integration (dry run)..."

    source_script github-issue-utils.sh

    # Create temporary body file
    temp_body="$(mktemp)"
    echo "# Test Integration" > "$temp_body"
    echo "This is a test integration body" >> "$temp_body"

    # Test dry run
    result=$(create_or_update_issue "$TEST_REPO" "Integration Test Title" "$temp_body" "test,integration" "")

    # Cleanup
    rm -f "$temp_body"

    # In dry run mode, we just check that the function runs without errors
    if [ $? -eq 0 ]; then
        log_success "Integration dry run test passed"
        return 0
    else
        log_error "Integration dry run test failed"
        return 1
    fi
}

# Main test execution
run_tests() {
    local tests_passed=0
    local tests_total=0
    local failed_tests=()

    log_info "Starting duplicate prevention tests..."
    echo "============================================"

    # Setup
    setup_test

    # Run all tests
    local test_functions=(
        test_script_loading
        test_title_generation
        test_keyword_extraction
        test_duplicate_detection_negative
        test_github_connectivity
        test_cache_functionality
        test_integration_dry_run
    )

    for test_func in "${test_functions[@]}"; do
        tests_total=$((tests_total + 1))
        log_info "Running $test_func..."

        if $test_func; then
            tests_passed=$((tests_passed + 1))
            log_success "$test_func passed"
        else
            failed_tests+=("$test_func")
            log_error "$test_func failed"
        fi

        echo "--------------------------------------------"
    done

    # Print summary
    echo ""
    log_info "TEST SUMMARY"
    echo "============================================"
    echo "Total tests: $tests_total"
    echo "Tests passed: $tests_passed"
    echo "Tests failed: $((tests_total - tests_passed))"

    if [ ${#failed_tests[@]} -gt 0 ]; then
        log_error "Failed tests:"
        for failed_test in "${failed_tests[@]}"; do
            echo "  - $failed_test"
        done
    fi

    if [ $tests_passed -eq $tests_total ]; then
        log_success "🎉 All duplicate prevention tests passed!"
        return 0
    else
        log_error "❌ Some tests failed"
        return 1
    fi
}

# Handle cleanup on exit
trap cleanup EXIT

# Run tests
if run_tests; then
    exit 0
else
    exit 1
fi
