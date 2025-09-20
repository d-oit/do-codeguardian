#!/bin/bash

# CodeGuardian Release Management - Version Bumping Script
# This script automates version bumping for CodeGuardian releases

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
CARGO_TOML="Cargo.toml"
CHANGELOG_MD="CHANGELOG.md"
GIT_REPO_ROOT="$(git rev-parse --show-toplevel)"

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

# Function to validate semantic version
validate_version() {
    local version=$1
    if [[ ! $version =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.-]+)?(\+[a-zA-Z0-9.-]+)?$ ]]; then
        log_error "Invalid version format: $version"
        log_error "Expected format: major.minor.patch[-prerelease][+build]"
        exit 1
    fi
}

# Function to get current version from Cargo.toml
get_current_version() {
    grep '^version = ' "$CARGO_TOML" | sed 's/version = "\(.*\)"/\1/'
}

# Function to bump version in Cargo.toml
bump_cargo_version() {
    local new_version=$1
    sed -i "s/^version = \".*\"/version = \"$new_version\"/" "$CARGO_TOML"
    log_success "Updated version in $CARGO_TOML to $new_version"
}

# Function to determine next version based on bump type
get_next_version() {
    local current_version=$1
    local bump_type=$2

    # Remove pre-release and build metadata for calculation
    local base_version=$(echo "$current_version" | sed 's/[-+].*//')
    local major=$(echo "$base_version" | cut -d. -f1)
    local minor=$(echo "$base_version" | cut -d. -f2)
    local patch=$(echo "$base_version" | cut -d. -f3)

    case $bump_type in
        major)
            major=$((major + 1))
            minor=0
            patch=0
            ;;
        minor)
            minor=$((minor + 1))
            patch=0
            ;;
        patch)
            patch=$((patch + 1))
            ;;
        *)
            log_error "Invalid bump type: $bump_type"
            log_error "Valid types: major, minor, patch"
            exit 1
            ;;
    esac

    echo "$major.$minor.$patch"
}

# Function to check if working directory is clean
check_git_status() {
    if ! git diff --quiet || ! git diff --staged --quiet; then
        log_error "Working directory is not clean. Please commit or stash changes first."
        exit 1
    fi
}

# Function to create git commit for version bump
create_version_commit() {
    local new_version=$1
    git add "$CARGO_TOML"
    git commit -m "chore: bump version to $new_version"
    log_success "Created commit for version bump to $new_version"
}

# Function to create git tag
create_version_tag() {
    local new_version=$1
    git tag -a "v$new_version" -m "Release v$new_version"
    log_success "Created tag v$new_version"
}

# Main function
main() {
    local bump_type=""
    local custom_version=""
    local create_commit=false
    local create_tag=false

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
            --commit)
                create_commit=true
                shift
                ;;
            --tag)
                create_tag=true
                shift
                ;;
            --help)
                echo "Usage: $0 [--major|--minor|--patch] [--version VERSION] [--commit] [--tag]"
                echo ""
                echo "Options:"
                echo "  --major     Bump major version"
                echo "  --minor     Bump minor version"
                echo "  --patch     Bump patch version"
                echo "  --version   Set specific version"
                echo "  --commit    Create git commit for version bump"
                echo "  --tag       Create git tag for version"
                echo "  --help      Show this help"
                exit 0
                ;;
            *)
                log_error "Unknown option: $1"
                exit 1
                ;;
        esac
    done

    # Change to repository root
    cd "$GIT_REPO_ROOT"

    # Check git status
    check_git_status

    # Get current version
    local current_version=$(get_current_version)
    log_info "Current version: $current_version"

    # Determine new version
    local new_version=""
    if [[ -n "$custom_version" ]]; then
        validate_version "$custom_version"
        new_version="$custom_version"
    elif [[ -n "$bump_type" ]]; then
        new_version=$(get_next_version "$current_version" "$bump_type")
    else
        log_error "Must specify either --major, --minor, --patch, or --version"
        exit 1
    fi

    log_info "New version will be: $new_version"

    # Confirm version bump
    read -p "Proceed with version bump to $new_version? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        log_info "Version bump cancelled"
        exit 0
    fi

    # Update Cargo.toml
    bump_cargo_version "$new_version"

    # Create commit if requested
    if [[ "$create_commit" == true ]]; then
        create_version_commit "$new_version"
    fi

    # Create tag if requested
    if [[ "$create_tag" == true ]]; then
        create_version_tag "$new_version"
    fi

    log_success "Version bump completed successfully!"
    log_info "Next steps:"
    if [[ "$create_commit" != true ]]; then
        log_info "  - Review and commit the changes: git add $CARGO_TOML && git commit -m 'chore: bump version to $new_version'"
    fi
    if [[ "$create_tag" != true ]]; then
        log_info "  - Create release tag: git tag -a v$new_version -m 'Release v$new_version'"
    fi
    log_info "  - Push changes: git push && git push --tags"
    log_info "  - Create GitHub release: https://github.com/d-oit/do-codeguardian/releases/new"
}

# Run main function with all arguments
main "$@"
