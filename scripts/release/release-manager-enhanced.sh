#!/bin/bash

# CodeGuardian Enhanced Release Management - Main Release Script
# This script orchestrates the entire release process with advanced features
#
# Features:
# - Enhanced changelog and release notes generation
# - Pre-release version support (alpha, beta, rc)
# - Configuration file support
# - Better error handling and rollback
# - Custom release workflows
# - GitHub CLI integration for PR/issue management
# - Release validation and testing
# - Progress tracking and logging
# - Backward compatibility with existing scripts

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Configuration defaults
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
GIT_REPO_ROOT="$(git rev-parse --show-toplevel)"
GITHUB_REPO="${GITHUB_REPO:-d-oit/do-codeguardian}"
CONFIG_FILE="${CONFIG_FILE:-scripts/release/release-config.toml}"
LOG_FILE="${LOG_FILE:-release-$(date +%Y%m%d-%H%M%S).log}"

# Scripts
VERSION_BUMP_SCRIPT="$SCRIPT_DIR/version-bump.sh"
CHANGELOG_SCRIPT="$SCRIPT_DIR/generate-changelog-enhanced.sh"
RELEASE_NOTES_SCRIPT="$SCRIPT_DIR/generate-release-notes.sh"
VALIDATE_SETUP_SCRIPT="$SCRIPT_DIR/validate-setup.sh"

# State tracking
RELEASE_STATE_FILE=""
CURRENT_STEP=""
STEPS_COMPLETED=()
FAILED_STEP=""

# Logging functions
log_info() {
    local timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    echo -e "${BLUE}[$timestamp INFO]${NC} $1" | tee -a "$LOG_FILE"
}

log_success() {
    local timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    echo -e "${GREEN}[$timestamp SUCCESS]${NC} $1" | tee -a "$LOG_FILE"
}

log_warning() {
    local timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    echo -e "${YELLOW}[$timestamp WARNING]${NC} $1" | tee -a "$LOG_FILE"
}

log_error() {
    local timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    echo -e "${RED}[$timestamp ERROR]${NC} $1" | tee -a "$LOG_FILE"
}

log_progress() {
    local step=$1
    local total=$2
    local current=$3
    local timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    echo -e "${CYAN}[$timestamp PROGRESS]${NC} [$current/$total] $step" | tee -a "$LOG_FILE"
}

# Function to load configuration
load_config() {
    if [[ -f "$CONFIG_FILE" ]]; then
        log_info "Loading configuration from $CONFIG_FILE"
        # Simple TOML-like parsing
        while IFS='=' read -r key value; do
            key=$(echo "$key" | sed 's/^[[:space:]]*//' | sed 's/[[:space:]]*$//')
            value=$(echo "$value" | sed 's/^[[:space:]]*//' | sed 's/[[:space:]]*$//' | sed 's/^"//' | sed 's/"$//')
            case $key in
                github_repo) GITHUB_REPO="$value" ;;
                changelog_include_authors) CHANGELOG_INCLUDE_AUTHORS="$value" ;;
                changelog_include_pr_info) CHANGELOG_INCLUDE_PR_INFO="$value" ;;
                enable_pre_release) ENABLE_PRE_RELEASE="$value" ;;
                enable_validation) ENABLE_VALIDATION="$value" ;;
                enable_github_integration) ENABLE_GITHUB_INTEGRATION="$value" ;;
                custom_workflow) CUSTOM_WORKFLOW="$value" ;;
                rollback_on_failure) ROLLBACK_ON_FAILURE="$value" ;;
                verbose_logging) VERBOSE_LOGGING="$value" ;;
            esac
        done < <(grep -E '^(github_repo|changelog_include_authors|changelog_include_pr_info|enable_pre_release|enable_validation|enable_github_integration|custom_workflow|rollback_on_failure|verbose_logging)' "$CONFIG_FILE" || true)
    else
        log_warning "Configuration file not found: $CONFIG_FILE"
    fi
}

# Function to initialize release state
init_release_state() {
    local version=$1
    RELEASE_STATE_FILE="/tmp/release-state-${version}-$$.json"

    cat > "$RELEASE_STATE_FILE" << EOF
{
    "version": "$version",
    "start_time": "$(date -Iseconds)",
    "steps_completed": [],
    "current_step": "",
    "failed_step": "",
    "rollback_actions": []
}
EOF

    log_info "Release state initialized: $RELEASE_STATE_FILE"
}

# Function to update release state
update_release_state() {
    local step=$1
    local status=$2
    local details=${3:-""}

    if [[ -f "$RELEASE_STATE_FILE" ]]; then
        # Update current step
        jq --arg step "$step" --arg status "$status" --arg details "$details" \
           '.current_step = $step | .steps_completed += [{"step": $step, "status": $status, "timestamp": "'$(date -Iseconds)'", "details": $details}]' \
           "$RELEASE_STATE_FILE" > "${RELEASE_STATE_FILE}.tmp" && mv "${RELEASE_STATE_FILE}.tmp" "$RELEASE_STATE_FILE"

        if [[ "$status" == "failed" ]]; then
            jq --arg step "$step" '.failed_step = $step' "$RELEASE_STATE_FILE" > "${RELEASE_STATE_FILE}.tmp" && mv "${RELEASE_STATE_FILE}.tmp" "$RELEASE_STATE_FILE"
        fi
    fi
}

# Function to validate prerequisites
validate_prerequisites() {
    log_progress "Validating prerequisites" 10 1
    update_release_state "validate_prerequisites" "running"

    # Check if we're in a git repository
    if ! git rev-parse --git-dir >/dev/null 2>&1; then
        log_error "Not in a git repository"
        update_release_state "validate_prerequisites" "failed" "Not in git repository"
        exit 1
    fi

    # Check if scripts exist
    local scripts=("$VERSION_BUMP_SCRIPT" "$CHANGELOG_SCRIPT" "$RELEASE_NOTES_SCRIPT")
    for script in "${scripts[@]}"; do
        if [[ ! -f "$script" ]]; then
            log_error "Required script not found: $script"
            update_release_state "validate_prerequisites" "failed" "Missing script: $script"
            exit 1
        fi
    done

    # Check if working directory is clean
    if ! git diff --quiet || ! git diff --staged --quiet; then
        log_error "Working directory is not clean. Please commit or stash changes first."
        update_release_state "validate_prerequisites" "failed" "Working directory not clean"
        exit 1
    fi

    # Check GitHub CLI if integration is enabled
    if [[ "${ENABLE_GITHUB_INTEGRATION:-true}" == "true" ]]; then
        if ! command -v gh >/dev/null 2>&1; then
            log_warning "GitHub CLI not found. GitHub integration features will be disabled."
            ENABLE_GITHUB_INTEGRATION=false
        fi
    fi

    update_release_state "validate_prerequisites" "completed"
    log_success "Prerequisites validated"
}

# Function to validate version format
validate_version() {
    local version=$1

    # Check basic semver format
    if ! echo "$version" | grep -E '^v?[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.-]+)?(\+[a-zA-Z0-9.-]+)?$' >/dev/null; then
        log_error "Invalid version format: $version"
        log_info "Expected format: v1.2.3 or 1.2.3 (optionally with pre-release: -alpha.1, -beta.2, -rc.3)"
        return 1
    fi

    # Check pre-release format if present
    if echo "$version" | grep -q '-'; then
        local pre_release=$(echo "$version" | sed 's/.*-//' | sed 's/+.*//')
        if ! echo "$pre_release" | grep -E '^(alpha|beta|rc)(\.[0-9]+)?$' >/dev/null; then
            log_error "Invalid pre-release format: $pre_release"
            log_info "Supported pre-release formats: alpha[.number], beta[.number], rc[.number]"
            return 1
        fi
    fi

    return 0
}

# Function to get current version
get_current_version() {
    grep '^version = ' "$GIT_REPO_ROOT/Cargo.toml" | sed 's/version = "\(.*\)"/\1/'
}

# Function to get latest tag
get_latest_tag() {
    git describe --tags --abbrev=0 2>/dev/null || echo ""
}

# Function to determine next version
get_next_version() {
    local current_version=$1
    local bump_type=$2
    local pre_release_type=${3:-""}
    local pre_release_number=${4:-""}

    # Remove 'v' prefix if present
    current_version=$(echo "$current_version" | sed 's/^v//')

    # Parse current version
    local major=$(echo "$current_version" | cut -d. -f1)
    local minor=$(echo "$current_version" | cut -d. -f2)
    local patch=$(echo "$current_version" | cut -d. -f3 | sed 's/-.*//' | sed 's/+.*//')

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
            return 1
            ;;
    esac

    local new_version="$major.$minor.$patch"

    # Add pre-release suffix if specified
    if [[ -n "$pre_release_type" ]]; then
        new_version="$new_version-$pre_release_type"
        if [[ -n "$pre_release_number" ]]; then
            new_version="$new_version.$pre_release_number"
        fi
    fi

    echo "$new_version"
}

# Function to run validation tests
run_validation_tests() {
    log_progress "Running validation tests" 10 2
    update_release_state "validation_tests" "running"

    if [[ "${ENABLE_VALIDATION:-true}" != "true" ]]; then
        log_info "Validation tests skipped (disabled in config)"
        update_release_state "validation_tests" "skipped"
        return 0
    fi

    cd "$GIT_REPO_ROOT"

    # Run cargo check
    log_info "Running cargo check..."
    if ! cargo check --quiet; then
        log_error "Cargo check failed"
        update_release_state "validation_tests" "failed" "cargo check failed"
        return 1
    fi

    # Run cargo test
    log_info "Running cargo test..."
    if ! cargo test --quiet; then
        log_error "Tests failed"
        update_release_state "validation_tests" "failed" "cargo test failed"
        return 1
    fi

    # Run clippy
    log_info "Running clippy..."
    if ! cargo clippy --quiet -- -D warnings; then
        log_error "Clippy checks failed"
        update_release_state "validation_tests" "failed" "clippy failed"
        return 1
    fi

    # Run format check
    log_info "Running format check..."
    if ! cargo fmt --check --quiet; then
        log_error "Code formatting check failed"
        update_release_state "validation_tests" "failed" "format check failed"
        return 1
    fi

    update_release_state "validation_tests" "completed"
    log_success "All validation tests passed"
}

# Function to build release artifacts
build_release_artifacts() {
    log_progress "Building release artifacts" 10 3
    update_release_state "build_artifacts" "running"

    cd "$GIT_REPO_ROOT"

    # Build release binary
    log_info "Building release binary..."
    if ! cargo build --release --quiet; then
        log_error "Release build failed"
        update_release_state "build_artifacts" "failed" "cargo build failed"
        return 1
    fi

    # Verify binary exists
    if [[ ! -f "target/release/codeguardian" ]]; then
        log_error "Release binary not found after build"
        update_release_state "build_artifacts" "failed" "binary not found"
        return 1
    fi

    update_release_state "build_artifacts" "completed"
    log_success "Release artifacts built successfully"
}

# Function to bump version
perform_version_bump() {
    local new_version=$1
    local dry_run=$2

    log_progress "Performing version bump" 10 4
    update_release_state "version_bump" "running"

    if [[ "$dry_run" == "true" ]]; then
        log_info "Dry run: Would bump version to $new_version"
        update_release_state "version_bump" "completed" "dry run"
        return 0
    fi

    log_info "Bumping version to $new_version"
    if ! bash "$VERSION_BUMP_SCRIPT" --version "$new_version" --commit; then
        log_error "Version bump failed"
        update_release_state "version_bump" "failed" "version bump script failed"
        return 1
    fi

    update_release_state "version_bump" "completed"
    log_success "Version bumped to $new_version"
}

# Function to generate changelog
generate_changelog() {
    local version=$1
    local dry_run=$2

    log_progress "Generating changelog" 10 5
    update_release_state "generate_changelog" "running"

    if [[ "$dry_run" == "true" ]]; then
        log_info "Dry run: Would generate changelog for $version"
        update_release_state "generate_changelog" "completed" "dry run"
        return 0
    fi

    log_info "Generating changelog for version $version"

    local changelog_args="--version $version"
    if [[ "${CHANGELOG_INCLUDE_AUTHORS:-false}" == "true" ]]; then
        changelog_args="$changelog_args --include-authors"
    fi
    if [[ "${CHANGELOG_INCLUDE_PR_INFO:-false}" == "true" ]]; then
        changelog_args="$changelog_args --include-pr-info"
    fi

    # Generate changelog without confirmation (assume yes for automation)
    echo "y" | bash "$CHANGELOG_SCRIPT" $changelog_args

    if [[ $? -ne 0 ]]; then
        log_error "Changelog generation failed"
        update_release_state "generate_changelog" "failed" "changelog script failed"
        return 1
    fi

    update_release_state "generate_changelog" "completed"
    log_success "Changelog generated successfully"
}

# Function to generate release notes
generate_release_notes() {
    local version=$1
    local latest_tag=$2
    local temp_dir=$3
    local dry_run=$4

    log_progress "Generating release notes" 10 6
    update_release_state "generate_release_notes" "running"

    local release_notes_file="$temp_dir/release-notes.md"

    if [[ "$dry_run" == "true" ]]; then
        log_info "Dry run: Would generate release notes for $version"
        update_release_state "generate_release_notes" "completed" "dry run"
        return 0
    fi

    log_info "Generating release notes for version $version"

    if [[ -n "$latest_tag" ]]; then
        if ! bash "$RELEASE_NOTES_SCRIPT" --version "$version" \
            --since "$latest_tag" --until "HEAD" \
            --output "$release_notes_file"; then
            log_error "Release notes generation failed"
            update_release_state "generate_release_notes" "failed" "release notes script failed"
            return 1
        fi
    else
        log_warning "No previous tag found, skipping detailed release notes"
        # Create basic release notes
        cat > "$release_notes_file" << EOF
# Release ${version}

**Full Changelog**: https://github.com/${GITHUB_REPO}/compare/...v${version}

## Changes

This is the initial release of CodeGuardian.

## Installation

\`\`\`bash
cargo install codeguardian
\`\`\`
EOF
    fi

    update_release_state "generate_release_notes" "completed"
    log_success "Release notes generated successfully"
    echo "$release_notes_file"
}

# Function to create git tag
create_git_tag() {
    local version=$1
    local dry_run=$2

    log_progress "Creating git tag" 10 7
    update_release_state "create_git_tag" "running"

    if [[ "$dry_run" == "true" ]]; then
        log_info "Dry run: Would create git tag v$version"
        update_release_state "create_git_tag" "completed" "dry run"
        return 0
    fi

    log_info "Creating git tag v$version"
    if ! git tag -a "v$version" -m "Release v$version"; then
        log_error "Failed to create git tag"
        update_release_state "create_git_tag" "failed" "git tag creation failed"
        return 1
    fi

    update_release_state "create_git_tag" "completed"
    log_success "Git tag v$version created"
}

# Function to publish to crates.io
publish_to_crates() {
    local dry_run=$1

    log_progress "Publishing to crates.io" 10 8
    update_release_state "publish_crates" "running"

    cd "$GIT_REPO_ROOT"

    if [[ "$dry_run" == "true" ]]; then
        log_info "Dry run: Checking crates.io publication"
        if ! cargo publish --dry-run --quiet; then
            log_error "Dry run publication failed"
            update_release_state "publish_crates" "failed" "dry run failed"
            return 1
        fi
        update_release_state "publish_crates" "completed" "dry run successful"
        log_success "Dry run publication successful"
        return 0
    fi

    log_info "Publishing to crates.io"
    if ! cargo publish --quiet; then
        log_error "Publication to crates.io failed"
        update_release_state "publish_crates" "failed" "cargo publish failed"
        return 1
    fi

    update_release_state "publish_crates" "completed"
    log_success "Published to crates.io successfully"
}

# Function to create GitHub release
create_github_release() {
    local version=$1
    local release_notes_file=$2
    local dry_run=$3

    log_progress "Creating GitHub release" 10 9
    update_release_state "create_github_release" "running"

    if [[ "${ENABLE_GITHUB_INTEGRATION:-true}" != "true" ]]; then
        log_info "GitHub integration disabled, skipping GitHub release"
        update_release_state "create_github_release" "skipped" "integration disabled"
        return 0
    fi

    if ! command -v gh >/dev/null 2>&1; then
        log_warning "GitHub CLI not found. Skipping GitHub release creation."
        update_release_state "create_github_release" "skipped" "gh cli not found"
        return 0
    fi

    if [[ "$dry_run" == "true" ]]; then
        log_info "Dry run: Would create GitHub release v$version"
        update_release_state "create_github_release" "completed" "dry run"
        return 0
    fi

    log_info "Creating GitHub release v$version"

    # Create release
    if [[ -f "$release_notes_file" ]]; then
        if ! gh release create "v$version" \
            --title "Release v$version" \
            --notes-file "$release_notes_file" \
            --latest; then
            log_error "GitHub release creation failed"
            update_release_state "create_github_release" "failed" "gh release create failed"
            return 1
        fi
    else
        if ! gh release create "v$version" \
            --title "Release v$version" \
            --generate-notes \
            --latest; then
            log_error "GitHub release creation failed"
            update_release_state "create_github_release" "failed" "gh release create failed"
            return 1
        fi
    fi

    update_release_state "create_github_release" "completed"
    log_success "GitHub release created"
}

# Function to push changes
push_changes() {
    local dry_run=$1

    log_progress "Pushing changes" 10 10
    update_release_state "push_changes" "running"

    if [[ "$dry_run" == "true" ]]; then
        log_info "Dry run: Would push changes and tags"
        update_release_state "push_changes" "completed" "dry run"
        return 0
    fi

    log_info "Pushing changes to remote"
    if ! git push && git push --tags; then
        log_error "Failed to push changes"
        update_release_state "push_changes" "failed" "git push failed"
        return 1
    fi

    update_release_state "push_changes" "completed"
    log_success "Changes pushed successfully"
}

# Function to perform rollback
perform_rollback() {
    local failed_step=$1

    log_warning "Performing rollback due to failure at step: $failed_step"

    # Get rollback actions from state file
    if [[ -f "$RELEASE_STATE_FILE" ]]; then
        local rollback_actions=$(jq -r '.rollback_actions[]?' "$RELEASE_STATE_FILE" 2>/dev/null || echo "")

        if [[ -n "$rollback_actions" ]]; then
            log_info "Executing rollback actions..."
            # This would need to be implemented based on what actions were taken
            # For now, just log the intent
            log_info "Rollback actions would be executed here"
        fi
    fi

    # Basic rollback: reset to previous state if possible
    if git rev-parse --verify HEAD~1 >/dev/null 2>&1; then
        log_info "Resetting to previous commit..."
        git reset --hard HEAD~1
        log_info "Reset complete"
    fi
}

# Function to show usage
show_usage() {
    cat << EOF
Enhanced CodeGuardian Release Manager

USAGE:
    $0 [OPTIONS] [--major|--minor|--patch] [--version VERSION]

OPTIONS:
    --major                      Bump major version
    --minor                      Bump minor version
    --patch                      Bump patch version
    --version VERSION            Set specific version
    --pre-release TYPE           Pre-release type: alpha, beta, rc
    --pre-release-number NUM     Pre-release number (default: 1)
    --skip-validation            Skip validation tests
    --skip-build                 Skip building release artifacts
    --skip-publish               Skip publishing to crates.io
    --skip-github-release        Skip creating GitHub release
    --dry-run                    Dry run mode (no actual changes)
    --config FILE                Configuration file (default: scripts/release/release-config.toml)
    --log-file FILE              Log file (default: auto-generated)
    --no-rollback                Disable automatic rollback on failure
    --custom-workflow SCRIPT     Custom workflow script to run
    --verbose                    Enable verbose logging
    --help                       Show this help message

PRE-RELEASE VERSIONS:
    Supported formats: alpha[.number], beta[.number], rc[.number]
    Examples: --pre-release alpha, --pre-release beta --pre-release-number 2

CONFIGURATION:
    Create a release-config.toml file with options:
    github_repo = "owner/repo"
    changelog_include_authors = true
    changelog_include_pr_info = true
    enable_pre_release = true
    enable_validation = true
    enable_github_integration = true
    rollback_on_failure = true
    verbose_logging = false

EXAMPLES:
    $0 --minor
    $0 --version 1.2.3
    $0 --patch --pre-release beta
    $0 --version 2.0.0-rc.1 --dry-run
    $0 --major --custom-workflow scripts/custom-release.sh

EOF
}

# Main function
main() {
    local bump_type=""
    local custom_version=""
    local pre_release_type=""
    local pre_release_number=""
    local skip_validation=false
    local skip_build=false
    local skip_publish=false
    local skip_github_release=false
    local dry_run=false
    local no_rollback=false
    local custom_workflow=""

    # Load configuration first
    load_config

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
            --pre-release)
                pre_release_type="$2"
                shift 2
                ;;
            --pre-release-number)
                pre_release_number="$2"
                shift 2
                ;;
            --skip-validation)
                skip_validation=true
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
            --skip-github-release)
                skip_github_release=true
                shift
                ;;
            --dry-run)
                dry_run=true
                shift
                ;;
            --config)
                CONFIG_FILE="$2"
                load_config
                shift 2
                ;;
            --log-file)
                LOG_FILE="$2"
                shift 2
                ;;
            --no-rollback)
                no_rollback=true
                shift
                ;;
            --custom-workflow)
                custom_workflow="$2"
                shift 2
                ;;
            --verbose)
                VERBOSE_LOGGING=true
                shift
                ;;
            --help)
                show_usage
                exit 0
                ;;
            *)
                log_error "Unknown option: $1"
                echo "Use --help for usage information"
                exit 1
                ;;
        esac
    done

    # Initialize logging
    touch "$LOG_FILE"
    log_info "Starting enhanced release process"
    log_info "Log file: $LOG_FILE"

    # Validate arguments
    if [[ -z "$bump_type" && -z "$custom_version" ]]; then
        log_error "Must specify either --major, --minor, --patch, or --version"
        exit 1
    fi

    # Validate pre-release arguments
    if [[ -n "$pre_release_type" ]]; then
        case $pre_release_type in
            alpha|beta|rc) ;;
            *)
                log_error "Invalid pre-release type: $pre_release_type"
                log_info "Supported types: alpha, beta, rc"
                exit 1
                ;;
        esac
    fi

    # Validate prerequisites
    validate_prerequisites

    # Create temporary directory
    local temp_dir=$(mktemp -d)
    trap "rm -rf $temp_dir" EXIT

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
        new_version=$(get_next_version "$current_version" "$bump_type" "$pre_release_type" "$pre_release_number")
    fi

    # Validate version format
    if ! validate_version "$new_version"; then
        exit 1
    fi

    # Initialize release state
    init_release_state "$new_version"

    log_info "Target version: $new_version"

    # Confirm release
    if [[ "$dry_run" != "true" ]]; then
        echo
        log_warning "About to create release v$new_version"
        read -p "Proceed with release? (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            log_info "Release cancelled"
            exit 0
        fi
    fi

    # Execute release steps with error handling
    local release_success=true
    local failed_step=""

    # Step 1: Validation tests
    if [[ "$skip_validation" != "true" ]]; then
        if ! run_validation_tests; then
            release_success=false
            failed_step="validation_tests"
        fi
    else
        log_info "Skipping validation tests"
    fi

    # Step 2: Build release artifacts
    if [[ "$release_success" == "true" && "$skip_build" != "true" ]]; then
        if ! build_release_artifacts; then
            release_success=false
            failed_step="build_artifacts"
        fi
    else
        log_info "Skipping build"
    fi

    # Step 3: Custom workflow (if specified)
    if [[ "$release_success" == "true" && -n "$custom_workflow" ]]; then
        log_progress "Running custom workflow" 10 3.5
        update_release_state "custom_workflow" "running"
        log_info "Running custom workflow: $custom_workflow"
        if [[ -f "$custom_workflow" ]]; then
            if ! bash "$custom_workflow" "$new_version"; then
                log_error "Custom workflow failed"
                update_release_state "custom_workflow" "failed" "custom workflow failed"
                release_success=false
                failed_step="custom_workflow"
            else
                update_release_state "custom_workflow" "completed"
                log_success "Custom workflow completed"
            fi
        else
            log_error "Custom workflow script not found: $custom_workflow"
            update_release_state "custom_workflow" "failed" "script not found"
            release_success=false
            failed_step="custom_workflow"
        fi
    fi

    # Step 4: Version bump
    if [[ "$release_success" == "true" ]]; then
        if ! perform_version_bump "$new_version" "$dry_run"; then
            release_success=false
            failed_step="version_bump"
        fi
    fi

    # Step 5: Generate changelog
    if [[ "$release_success" == "true" ]]; then
        if ! generate_changelog "$new_version" "$dry_run"; then
            release_success=false
            failed_step="generate_changelog"
        fi
    fi

    # Step 6: Generate release notes
    local release_notes_file=""
    if [[ "$release_success" == "true" ]]; then
        release_notes_file=$(generate_release_notes "$new_version" "$latest_tag" "$temp_dir" "$dry_run")
        if [[ $? -ne 0 ]]; then
            release_success=false
            failed_step="generate_release_notes"
        fi
    fi

    # Step 7: Create git tag
    if [[ "$release_success" == "true" ]]; then
        if ! create_git_tag "$new_version" "$dry_run"; then
            release_success=false
            failed_step="create_git_tag"
        fi
    fi

    # Step 8: Publish to crates.io
    if [[ "$release_success" == "true" && "$skip_publish" != "true" ]]; then
        if ! publish_to_crates "$dry_run"; then
            release_success=false
            failed_step="publish_crates"
        fi
    else
        log_info "Skipping crates.io publication"
    fi

    # Step 9: Create GitHub release
    if [[ "$release_success" == "true" && "$skip_github_release" != "true" ]]; then
        if ! create_github_release "$new_version" "$release_notes_file" "$dry_run"; then
            release_success=false
            failed_step="create_github_release"
        fi
    else
        log_info "Skipping GitHub release creation"
    fi

    # Step 10: Push changes
    if [[ "$release_success" == "true" ]]; then
        if ! push_changes "$dry_run"; then
            release_success=false
            failed_step="push_changes"
        fi
    fi

    # Handle failure and rollback
    if [[ "$release_success" == "false" ]]; then
        log_error "Release failed at step: $failed_step"
        if [[ "${ROLLBACK_ON_FAILURE:-true}" == "true" && "$no_rollback" != "true" && "$dry_run" != "true" ]]; then
            perform_rollback "$failed_step"
        fi
        log_info "Check the log file for details: $LOG_FILE"
        exit 1
    fi

    # Success
    if [[ "$dry_run" == "true" ]]; then
        log_success "Dry run completed successfully!"
        log_info "To perform actual release, run without --dry-run"
    else
        log_success "Release v$new_version completed successfully!"
        echo
        log_info "Summary:"
        log_info "  📦 Version: v$new_version"
        log_info "  📝 Changelog: Updated CHANGELOG.md"
        log_info "  🏷️  Git tag: v$new_version"
        if [[ "$skip_publish" != "true" ]]; then
            log_info "  📤 Crates.io: https://crates.io/crates/do-codeguardian/${new_version}"
        fi
        if [[ "$skip_github_release" != "true" ]]; then
            log_info "  🎉 GitHub Release: https://github.com/${GITHUB_REPO}/releases/tag/v${new_version}"
        fi
        log_info "  📄 Log file: $LOG_FILE"
    fi

    # Cleanup
    if [[ -f "$RELEASE_STATE_FILE" ]]; then
        rm -f "$RELEASE_STATE_FILE"
    fi
}

# Run main function with all arguments
main "$@"