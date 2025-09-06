#!/bin/bash

# Comprehensive test runner for broken files detection feature
# This script runs all tests related to the broken files detection enhancement

set -e

echo "🧪 Running Broken Files Detection Tests"
echo "======================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to run test with timing
run_test() {
    local test_name="$1"
    local test_command="$2"

    print_status "Running $test_name..."
    start_time=$(date +%s)

    # Use separate target directory to avoid build lock contention
    local target_dir="/tmp/cargo-target-$(date +%s)-$$"
    mkdir -p "$target_dir"

    if CARGO_TARGET_DIR="$target_dir" eval "$test_command"; then
        end_time=$(date +%s)
        duration=$((end_time - start_time))
        print_success "$test_name completed in ${duration}s"
        rm -rf "$target_dir"
        return 0
    else
        print_error "$test_name failed"
        rm -rf "$target_dir"
        return 1
    fi
}

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    print_error "Please run this script from the project root directory"
    exit 1
fi

# Initialize test results
total_tests=0
passed_tests=0
failed_tests=0

echo ""
print_status "Starting broken files detection test suite..."
echo ""

# 1. Unit Tests for Individual Analyzers
echo "📋 Phase 1: Unit Tests"
echo "---------------------"

if run_test "Git Conflict Analyzer Unit Tests" "cargo test git_conflict_unit_tests --lib"; then
    ((passed_tests++))
else
    ((failed_tests++))
fi
((total_tests++))

if run_test "AI Content Analyzer Unit Tests" "cargo test ai_content_unit_tests --lib"; then
    ((passed_tests++))
else
    ((failed_tests++))
fi
((total_tests++))

if run_test "Duplicate Analyzer Unit Tests" "cargo test duplicate_analyzer_unit_tests --lib"; then
    ((passed_tests++))
else
    ((failed_tests++))
fi
((total_tests++))

echo ""

# 2. Integration Tests
echo "🔗 Phase 2: Integration Tests"
echo "-----------------------------"

if run_test "Broken Files Integration Tests" "cargo test broken_files_integration_tests"; then
    ((passed_tests++))
else
    ((failed_tests++))
fi
((total_tests++))

if run_test "Configuration Integration Tests" "cargo test broken_files_config_tests"; then
    ((passed_tests++))
else
    ((failed_tests++))
fi
((total_tests++))

echo ""

# 3. Performance Tests
echo "⚡ Phase 3: Performance Tests"
echo "----------------------------"

if run_test "Performance Tests" "cargo test broken_files_performance_tests --release"; then
    ((passed_tests++))
else
    ((failed_tests++))
    print_warning "Performance tests failed - this might indicate performance regressions"
fi
((total_tests++))

echo ""

# 4. End-to-End Tests
echo "🎯 Phase 4: End-to-End Tests"
echo "---------------------------"

if run_test "E2E CLI Tests" "cargo test broken_files_e2e_tests"; then
    ((passed_tests++))
else
    ((failed_tests++))
fi
((total_tests++))

echo ""

# 5. Analyzer-Specific Tests (from existing test files)
echo "🔍 Phase 5: Analyzer-Specific Tests"
echo "----------------------------------"

if run_test "Git Conflict Analyzer Tests" "cargo test git_conflict --lib"; then
    ((passed_tests++))
else
    ((failed_tests++))
fi
((total_tests++))

if run_test "AI Content Analyzer Tests" "cargo test ai_content --lib"; then
    ((passed_tests++))
else
    ((failed_tests++))
fi
((total_tests++))

if run_test "Duplicate Analyzer Tests" "cargo test duplicate --lib"; then
    ((passed_tests++))
else
    ((failed_tests++))
fi
((total_tests++))

echo ""

# 6. Configuration Tests
echo "⚙️  Phase 6: Configuration Tests"
echo "-------------------------------"

if run_test "Config Serialization Tests" "cargo test config_tests --lib"; then
    ((passed_tests++))
else
    ((failed_tests++))
fi
((total_tests++))

if run_test "Analyzer Registry Config Tests" "cargo test analyzer_registry_config_tests --lib"; then
    ((passed_tests++))
else
    ((failed_tests++))
fi
((total_tests++))

echo ""

# 7. Benchmark Tests (optional, only if --benchmark flag is passed)
if [[ "$1" == "--benchmark" ]]; then
    echo "📊 Phase 7: Benchmark Tests"
    echo "--------------------------"

    if run_test "Benchmark Tests" "cargo test benchmark_tests --release"; then
        ((passed_tests++))
    else
        ((failed_tests++))
        print_warning "Benchmark tests failed - performance may need optimization"
    fi
    ((total_tests++))

    echo ""
fi

# 8. Documentation Tests
echo "📚 Phase 8: Documentation Tests"
echo "------------------------------"

if run_test "Doc Tests" "cargo test --doc"; then
    ((passed_tests++))
else
    ((failed_tests++))
    print_warning "Documentation tests failed - check code examples in docs"
fi
((total_tests++))

echo ""

# 9. Clippy and Format Checks
echo "🧹 Phase 9: Code Quality Checks"
echo "------------------------------"

if run_test "Clippy Lints" "cargo clippy --all-targets --all-features -- -D warnings"; then
    ((passed_tests++))
else
    ((failed_tests++))
    print_warning "Clippy found issues - please fix linting warnings"
fi
((total_tests++))

if run_test "Format Check" "cargo fmt --all -- --check"; then
    ((passed_tests++))
else
    ((failed_tests++))
    print_warning "Code formatting issues found - run 'cargo fmt' to fix"
fi
((total_tests++))

echo ""

# Summary
echo "📊 Test Results Summary"
echo "======================"
echo "Total tests: $total_tests"
echo "Passed: $passed_tests"
echo "Failed: $failed_tests"

if [ $failed_tests -eq 0 ]; then
    print_success "All tests passed! 🎉"
    echo ""
    echo "✅ Broken files detection feature is ready for deployment"
    echo ""
    echo "Next steps:"
    echo "  1. Update documentation with new features"
    echo "  2. Create release notes"
    echo "  3. Deploy to staging environment"
    echo "  4. Run integration tests in CI/CD pipeline"
    exit 0
else
    print_error "$failed_tests test(s) failed"
    echo ""
    echo "❌ Please fix failing tests before deployment"
    echo ""
    echo "Debugging tips:"
    echo "  1. Run individual failing tests with: cargo test <test_name> -- --nocapture"
    echo "  2. Check test logs for specific error messages"
    echo "  3. Verify test data and expectations"
    echo "  4. Run tests in debug mode for more detailed output"
    exit 1
fi
