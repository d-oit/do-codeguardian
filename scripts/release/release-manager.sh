#!/bin/bash

# CodeGuardian Release Management - Main Release Script
# This script orchestrates the entire release process

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
GIT_REPO_ROOT="$(git rev-parse --show-toplevel)"
GITHUB_REPO="${GITHUB_REPO:-d-oit/do-codeguardian}"

# Scripts
VERSION_BUMP_SCRIPT="$SCRIPT_DIR/version-bump.sh"
CHANGELOG_SCRIPT="$SCRIPT_DIR/generate-changelog.sh"
RELEASE_NOTES_SCRIPT="$SCRIPT_DIR/generate-release-notes.sh"

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

# Function to validate prerequisites
validate_prerequisites() {
    log_info "Validating prerequisites..."

    # Check if we're in a git repository
    if ! git rev-parse --git-dir >/dev/null 2>&1; then
        log_error "Not in a git repository"
        exit 1
    fi

    # Check if scripts exist
    if [[ ! -f "$VERSION_BUMP_SCRIPT" ]]; then
        log_error "Version bump script not found: $VERSION_BUMP_SCRIPT"
        exit 1
    fi

    if [[ ! -f "$CHANGELOG_SCRIPT" ]]; then
        log_error "Changelog script not found: $CHANGELOG_SCRIPT"
        exit 1
    fi

    if [[ ! -f "$RELEASE_NOTES_SCRIPT" ]]; then
        log_error "Release notes script not found: $RELEASE_NOTES_SCRIPT"
        exit 1
    fi

    # Check if working directory is clean
    if ! git diff --quiet || ! git diff --staged --quiet; then
        log_error "Working directory is not clean. Please commit or stash changes first."
        exit 1
    fi

    log_success "Prerequisites validated"
}

# Function to get current version
get_current_version() {
    grep '^version = ' "$GIT_REPO_ROOT/Cargo.toml" | sed 's/version = "\(.*\)"/\1/'
}

# Function to get latest tag
get_latest_tag() {
    git describe --tags --abbrev=0 2>/dev/null || echo ""
}

# Function to run tests
run_tests() {
    log_info "Running tests..."

    cd "$GIT_REPO_ROOT"

    # Run cargo test
    if ! cargo test --release; then
        log_error "Tests failed"
        exit 1
    fi

    # Run clippy
    if ! cargo clippy -- -D warnings; then
        log_error "Clippy checks failed"
        exit 1
    fi

    # Run format check
    if ! cargo fmt --check; then
        log_error "Code formatting check failed"
        exit 1
    fi

    log_success "All tests passed"
}

# Function to build release artifacts
build_release() {
    log_info "Building release artifacts..."

    cd "$GIT_REPO_ROOT"

    # Build release binary
    if ! cargo build --release; then
        log_error "Release build failed"
        exit 1
    fi

    log_success "Release build completed"
}

# Function to publish to crates.io
publish_to_crates() {
    local dry_run=${1:-false}

    log_info "Publishing to crates.io..."

    cd "$GIT_REPO_ROOT"

    if [[ "$dry_run" == "true" ]]; then
        log_info "Dry run mode - checking publication"
        if ! cargo publish --dry-run; then
            log_error "Dry run publication failed"
            exit 1
        fi
        log_success "Dry run publication successful"
    else
        if ! cargo publish; then
            log_error "Publication to crates.io failed"
            exit 1
        fi
        log_success "Published to crates.io successfully"
    fi
}

# Function to create GitHub release
create_github_release() {
    local version=$1
    local release_notes_file=$2
    local dry_run=${3:-false}

    log_info "Creating GitHub release..."

    if ! command -v gh >/dev/null 2>&1; then
        log_warning "GitHub CLI not found. Skipping GitHub release creation."
        return 0
    fi

    if [[ "$dry_run" == "true" ]]; then
        log_info "Dry run mode - would create GitHub release v$version"
        return 0
    fi

    # Create release
    if [[ -f "$release_notes_file" ]]; then
        gh release create "v$version" \
            --title "Release v$version" \
            --notes-file "$release_notes_file" \
            --latest
    else
        gh release create "v$version" \
            --title "Release v$version" \
            --generate-notes \
            --latest
    fi

    log_success "GitHub release created"
}

# Function to cleanup temporary files
cleanup() {
    local temp_dir=$1

    if [[ -d "$temp_dir" ]]; then
        rm -rf "$temp_dir"
        log_info "Cleaned up temporary files"
    fi
}

# Main function
main() {
    local bump_type=""
    local custom_version=""
    local skip_tests=false
    local skip_build=false
    local skip_publish=false
    local dry_run=false
    local create_github_release=true

    # Parse command line arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            --major)
                bump_type="major"
                shift
                ;;
            --minor)
                bump_type="minor"
                shift
                ;;
            --patch)
                bump_type="patch"
                shift
                ;;
            --version)
                custom_version="$2"
                shift 2
                ;;
            --skip-tests)
                skip_tests=true
                shift
                ;;
            --skip-build)
                skip_build=true
                shift
                ;;
            --skip-publish)
                skip_publish=true
                shift
                ;;
            --dry-run)
                dry_run=true
                shift
                ;;
            --no-github-release)
                create_github_release=false
                shift
                ;;
            --help)
                echo "Usage: $0 [--major|--minor|--patch] [--version VERSION] [options]"
                echo ""
                echo "Options:"
                echo "  --major              Bump major version"
                echo "  --minor              Bump minor version"
                echo "  --patch              Bump patch version"
                echo "  --version VERSION    Set specific version"
                echo "  --skip-tests         Skip running tests"
                echo "  --skip-build         Skip building release"
                echo "  --skip-publish       Skip publishing to crates.io"
                echo "  --dry-run            Dry run mode (no actual changes)"
                echo "  --no-github-release  Skip creating GitHub release"
                echo "  --help               Show this help"
                exit 0
                ;;
            *)
                log_error "Unknown option: $1"
                exit 1
                ;;
        esac
    done

    # Validate arguments
    if [[ -z "$bump_type" && -z "$custom_version" ]]; then
        log_error "Must specify either --major, --minor, --patch, or --version"
        exit 1
    fi

    # Validate prerequisites
    validate_prerequisites

    # Create temporary directory
    local temp_dir=$(mktemp -d)
    trap "cleanup $temp_dir" EXIT

    cd "$GIT_REPO_ROOT"

    # Get current version and latest tag
    local current_version=$(get_current_version)
    local latest_tag=$(get_latest_tag)

    log_info "Current version: $current_version"
    log_info "Latest tag: ${latest_tag:-none}"

    # Determine new version
    local new_version=""
    if [[ -n "$custom_version" ]]; then
        new_version="$custom_version"
    else
        # Source version bump logic
        source "$VERSION_BUMP_SCRIPT"
        new_version=$(get_next_version "$current_version" "$bump_type")
    fi

    log_info "Target version: $new_version"

    # Confirm release
    if [[ "$dry_run" != "true" ]]; then
        read -p "Proceed with release v$new_version? (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            log_info "Release cancelled"
            exit 0
        fi
    fi

    # Step 1: Run tests
    if [[ "$skip_tests" != "true" ]]; then
        run_tests
    else
        log_info "Skipping tests"
    fi

    # Step 2: Build release
    if [[ "$skip_build" != "true" ]]; then
        build_release
    else
        log_info "Skipping build"
    fi

    # Step 3: Version bump
    if [[ "$dry_run" != "true" ]]; then
        log_info "Bumping version to $new_version"
        bash "$VERSION_BUMP_SCRIPT" --version "$new_version" --commit
    else
        log_info "Dry run: Would bump version to $new_version"
    fi

    # Step 4: Generate changelog
    if [[ "$dry_run" != "true" ]]; then
        log_info "Generating changelog"
        bash "$CHANGELOG_SCRIPT" --version "$new_version"
    else
        log_info "Dry run: Would generate changelog"
    fi

    # Step 5: Generate release notes
    local release_notes_file="$temp_dir/release-notes.md"
    if [[ "$dry_run" != "true" ]]; then
        log_info "Generating release notes"
        if [[ -n "$latest_tag" ]]; then
            bash "$RELEASE_NOTES_SCRIPT" --version "$new_version" \
                --since "$latest_tag" --until "HEAD" \
                --output "$release_notes_file"
        else
            log_warning "No previous tag found, skipping detailed release notes"
        fi
    else
        log_info "Dry run: Would generate release notes"
    fi

    # Step 6: Create git tag
    if [[ "$dry_run" != "true" ]]; then
        log_info "Creating git tag"
        git tag -a "v$new_version" -m "Release v$new_version"
    else
        log_info "Dry run: Would create git tag v$new_version"
    fi

    # Step 7: Publish to crates.io
    if [[ "$skip_publish" != "true" ]]; then
        publish_to_crates "$dry_run"
    else
        log_info "Skipping crates.io publication"
    fi

    # Step 8: Create GitHub release
    if [[ "$create_github_release" == "true" ]]; then
        create_github_release "$new_version" "$release_notes_file" "$dry_run"
    else
        log_info "Skipping GitHub release creation"
    fi

    # Step 9: Push changes
    if [[ "$dry_run" != "true" ]]; then
        log_info "Pushing changes to remote"
        git push && git push --tags
    else
        log_info "Dry run: Would push changes and tags"
    fi

    if [[ "$dry_run" == "true" ]]; then
        log_success "Dry run completed successfully!"
        log_info "To perform actual release, run without --dry-run"
    else
        log_success "Release v$new_version completed successfully!"
        log_info "Next steps:"
        log_info "  - Monitor CI/CD pipelines"
        log_info "  - Check GitHub release: https://github.com/${GITHUB_REPO}/releases/tag/v${new_version}"
        if [[ "$skip_publish" != "true" ]]; then
            log_info "  - Check crates.io: https://crates.io/crates/do-codeguardian/${new_version}"
        fi
    fi
}

# Run main function with all arguments
main "$@"
