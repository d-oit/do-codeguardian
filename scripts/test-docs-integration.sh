#!/bin/bash

# Test script for automatic documentation integration
# This script tests the setup without requiring opencode authentication

set -e  # Exit on any error

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

# Test 1: Check if scripts exist and are executable
test_scripts_exist() {
    print_status "Testing script existence and permissions..."

    local scripts=(
        "scripts/update-docs.sh"
        "scripts/setup-opencode.sh"
        "scripts/test-docs-integration.sh"
    )

    for script in "${scripts[@]}"; do
        if [ ! -f "$script" ]; then
            print_error "Script not found: $script"
            return 1
        fi

        if [ ! -x "$script" ]; then
            print_error "Script not executable: $script"
            return 1
        fi

        print_success "✓ $script"
    done

    return 0
}

# Test 2: Check pre-commit hook
test_pre_commit_hook() {
    print_status "Testing pre-commit hook..."

    local hook=".git/hooks/pre-commit"

    if [ ! -f "$hook" ]; then
        print_error "Pre-commit hook not found: $hook"
        return 1
    fi

    if [ ! -x "$hook" ]; then
        print_error "Pre-commit hook not executable: $hook"
        return 1
    fi

    # Check if hook contains our documentation update code
    if grep -q "update-docs.sh" "$hook"; then
        print_success "✓ Pre-commit hook contains documentation update logic"
    else
        print_error "Pre-commit hook missing documentation update logic"
        return 1
    fi

    return 0
}

# Test 3: Check configuration files
test_config_files() {
    print_status "Testing configuration files..."

    local configs=(
        ".opencode/config.json"
        "docs/AUTOMATIC_DOCUMENTATION.md"
    )

    for config in "${configs[@]}"; do
        if [ ! -f "$config" ]; then
            print_error "Configuration file not found: $config"
            return 1
        fi
        print_success "✓ $config"
    done

    return 0
}

# Test 4: Check README integration
test_readme_integration() {
    print_status "Testing README integration..."

    if grep -q "Automatic Documentation" README.md; then
        print_success "✓ README.md contains automatic documentation section"
    else
        print_error "README.md missing automatic documentation section"
        return 1
    fi

    if grep -q "opencode.ai" README.md; then
        print_success "✓ README.md contains opencode references"
    else
        print_error "README.md missing opencode references"
        return 1
    fi

    return 0
}

# Test 5: Validate script syntax
test_script_syntax() {
    print_status "Testing script syntax..."

    local scripts=(
        "scripts/update-docs.sh"
        "scripts/setup-opencode.sh"
    )

    for script in "${scripts[@]}"; do
        if bash -n "$script" 2>/dev/null; then
            print_success "✓ $script syntax is valid"
        else
            print_error "$script has syntax errors"
            return 1
        fi
    done

    return 0
}

# Test 6: Check opencode availability (optional)
test_opencode_availability() {
    print_status "Testing opencode availability..."

    if command -v opencode &> /dev/null; then
        print_success "✓ opencode is installed: $(opencode --version)"
        return 0
    else
        print_warning "⚠ opencode not found in PATH"
        print_warning "   Install with: curl -fsSL https://opencode.ai/install | bash"
        return 0  # Don't fail the test, just warn
    fi
}

# Main test function
main() {
    print_status "Running automatic documentation integration tests..."
    echo

    local tests_passed=0
    local total_tests=6

    # Run all tests
    if test_scripts_exist; then ((tests_passed++)); fi
    echo

    if test_pre_commit_hook; then ((tests_passed++)); fi
    echo

    if test_config_files; then ((tests_passed++)); fi
    echo

    if test_readme_integration; then ((tests_passed++)); fi
    echo

    if test_script_syntax; then ((tests_passed++)); fi
    echo

    if test_opencode_availability; then ((tests_passed++)); fi
    echo

    # Summary
    if [ $tests_passed -eq $total_tests ]; then
        print_success "All tests passed! ($tests_passed/$total_tests)"
        print_status "Automatic documentation integration is ready to use."
        print_status "Run 'bash scripts/setup-opencode.sh' to complete the setup."
        exit 0
    else
        print_error "Some tests failed: $tests_passed/$total_tests passed"
        print_status "Please fix the issues above before using the automatic documentation system."
        exit 1
    fi
}

# Show usage if requested
if [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
    echo "CodeGuardian Documentation Integration Test Script"
    echo ""
    echo "This script tests the automatic documentation integration without requiring"
    echo "opencode authentication or running the actual documentation updates."
    echo ""
    echo "Usage: $0 [options]"
    echo ""
    echo "Options:"
    echo "  --help, -h    Show this help message"
    echo ""
    echo "Tests performed:"
    echo "  1. Script existence and permissions"
    echo "  2. Pre-commit hook setup"
    echo "  3. Configuration files"
    echo "  4. README integration"
    echo "  5. Script syntax validation"
    echo "  6. Opencode availability"
    exit 0
fi

# Run main function
main "$@"