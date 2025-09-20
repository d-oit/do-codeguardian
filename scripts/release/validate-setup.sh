#!/bin/bash

# CodeGuardian Release Management - Setup Validation Script
# This script validates that the release management system is properly configured

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to check file exists and is executable
check_script() {
    local script_path=$1
    local script_name=$2

    if [[ ! -f "$script_path" ]]; then
        log_error "$script_name script not found: $script_path"
        return 1
    fi

    if [[ ! -x "$script_path" ]]; then
        log_warning "$script_name script is not executable: $script_path"
        log_info "Making script executable..."
        chmod +x "$script_path"
    fi

    log_success "$script_name script is ready"
}

# Function to check GitHub workflow files
check_workflow() {
    local workflow_path=$1
    local workflow_name=$2

    if [[ ! -f "$workflow_path" ]]; then
        log_error "$workflow_name workflow not found: $workflow_path"
        return 1
    fi

    log_success "$workflow_name workflow exists"
}

# Function to check required tools
check_tools() {
    log_info "Checking required tools..."

    local missing_tools=()

    if ! command_exists git; then
        missing_tools+=("git")
    fi

    if ! command_exists cargo; then
        missing_tools+=("cargo")
    fi

    if ! command_exists jq; then
        missing_tools+=("jq")
    fi

    # GitHub CLI is optional but recommended
    if ! command_exists gh; then
        log_warning "GitHub CLI (gh) not found - release notes generation will be limited"
        log_info "Install GitHub CLI: https://cli.github.com/"
    else
        log_success "GitHub CLI found"
    fi

    if [[ ${#missing_tools[@]} -gt 0 ]]; then
        log_error "Missing required tools: ${missing_tools[*]}"
        return 1
    fi

    log_success "All required tools are available"
}

# Function to check repository configuration
check_repository() {
    log_info "Checking repository configuration..."

    # Check if we're in a git repository
    if ! git rev-parse --git-dir >/dev/null 2>&1; then
        log_error "Not in a git repository"
        return 1
    fi

    # Check if Cargo.toml exists
    if [[ ! -f "Cargo.toml" ]]; then
        log_error "Cargo.toml not found in repository root"
        return 1
    fi

    # Check if CHANGELOG.md exists
    if [[ ! -f "CHANGELOG.md" ]]; then
        log_warning "CHANGELOG.md not found - it will be created during first release"
    else
        log_success "CHANGELOG.md exists"
    fi

    # Check current version in Cargo.toml
    local current_version
    current_version=$(grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/')
    if [[ -z "$current_version" ]]; then
        log_error "Could not determine current version from Cargo.toml"
        return 1
    fi

    log_success "Current version: $current_version"

    # Check if there are any existing tags
    local latest_tag
    latest_tag=$(git describe --tags --abbrev=0 2>/dev/null || echo "")
    if [[ -n "$latest_tag" ]]; then
        log_success "Latest tag: $latest_tag"
    else
        log_info "No existing tags found - this appears to be the first release"
    fi

    log_success "Repository configuration is valid"
}

# Function to check GitHub Actions configuration
check_github_actions() {
    log_info "Checking GitHub Actions configuration..."

    local workflows_dir=".github/workflows"

    if [[ ! -d "$workflows_dir" ]]; then
        log_error "GitHub workflows directory not found: $workflows_dir"
        return 1
    fi

    # Check automated release workflow
    check_workflow "$workflows_dir/automated-release.yml" "Automated Release"

    # Check deployment workflow
    check_workflow "$workflows_dir/deployment.yml" "Deployment Pipeline"

    # Check existing workflows for compatibility
    if [[ -f "$workflows_dir/turbo-release.yml" ]]; then
        log_info "Found existing turbo-release.yml - ensure no conflicts"
    fi

    if [[ -f "$workflows_dir/release-notes.yml" ]]; then
        log_info "Found existing release-notes.yml - ensure no conflicts"
    fi

    log_success "GitHub Actions configuration looks good"
}

# Function to validate script functionality
validate_scripts() {
    log_info "Validating script functionality..."

    # Test version bump script (dry run)
    log_info "Testing version bump script..."
    if bash scripts/release/version-bump.sh --help >/dev/null 2>&1; then
        log_success "Version bump script is functional"
    else
        log_error "Version bump script has issues"
        return 1
    fi

    # Test changelog generation script (dry run)
    log_info "Testing changelog generation script..."
    if bash scripts/release/generate-changelog.sh --help >/dev/null 2>&1; then
        log_success "Changelog generation script is functional"
    else
        log_error "Changelog generation script has issues"
        return 1
    fi

    # Test release notes generation script (dry run)
    log_info "Testing release notes generation script..."
    if bash scripts/release/generate-release-notes.sh --help >/dev/null 2>&1; then
        log_success "Release notes generation script is functional"
    else
        log_error "Release notes generation script has issues"
        return 1
    fi

    # Test main release manager script (dry run)
    log_info "Testing main release manager script..."
    if bash scripts/release/release-manager.sh --help >/dev/null 2>&1; then
        log_success "Main release manager script is functional"
    else
        log_error "Main release manager script has issues"
        return 1
    fi

    log_success "All scripts are functional"
}

# Function to check permissions
check_permissions() {
    log_info "Checking permissions..."

    # Check if scripts are executable
    check_script "scripts/release/release-manager.sh" "Release Manager"
    check_script "scripts/release/version-bump.sh" "Version Bump"
    check_script "scripts/release/generate-changelog.sh" "Changelog Generator"
    check_script "scripts/release/generate-release-notes.sh" "Release Notes Generator"
    check_script "scripts/release/validate-setup.sh" "Setup Validator"

    log_success "Permissions are correctly configured"
}

# Main function
main() {
    log_info "🚀 CodeGuardian Release Management Setup Validation"
    log_info "=================================================="

    local all_checks_passed=true

    # Run all validation checks
    if ! check_tools; then
        all_checks_passed=false
    fi

    if ! check_repository; then
        all_checks_passed=false
    fi

    if ! check_github_actions; then
        all_checks_passed=false
    fi

    if ! check_permissions; then
        all_checks_passed=false
    fi

    if ! validate_scripts; then
        all_checks_passed=false
    fi

    echo
    log_info "=================================================="

    if [[ "$all_checks_passed" == true ]]; then
        log_success "✅ All validation checks passed!"
        log_info ""
        log_info "🎉 Your release management system is ready!"
        log_info ""
        log_info "Next steps:"
        log_info "  1. Set up GitHub secrets (CRATES_IO_TOKEN)"
        log_info "  2. Test with dry-run: ./scripts/release/release-manager.sh --patch --dry-run"
        log_info "  3. Create your first release!"
        log_info ""
        log_info "For more information, see: scripts/release/README.md"
    else
        log_error "❌ Some validation checks failed"
        log_info ""
        log_info "Please fix the issues above and run this script again."
        log_info "For help, see: scripts/release/README.md"
        exit 1
    fi
}

# Run main function
main "$@"
