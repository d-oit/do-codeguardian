#!/bin/bash

# CodeGuardian Automated Release Workflow Script
# Combines version detection, changelog generation, and release creation
# with automatic semantic versioning based on commit analysis

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
GIT_REPO_ROOT="$(git rev-parse --show-toplevel)"
GITHUB_REPO="${GITHUB_REPO:-d-oit/do-codeguardian}"
CARGO_TOML="$GIT_REPO_ROOT/Cargo.toml"
CHANGELOG_MD="$GIT_REPO_ROOT/CHANGELOG.md"

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

log_header() {
    echo -e "${PURPLE}[WORKFLOW]${NC} $1"
}

log_step() {
    echo -e "${CYAN}[STEP]${NC} $1"
}

# Function to validate prerequisites
validate_prerequisites() {
    log_header "🔍 Validating Prerequisites"

    # Check if we're in a git repository
    if ! git rev-parse --git-dir >/dev/null 2>&1; then
        log_error "Not in a git repository"
        exit 1
    fi

    # Check if scripts exist
    local scripts=("$VERSION_BUMP_SCRIPT" "$CHANGELOG_SCRIPT" "$RELEASE_NOTES_SCRIPT")
    for script in "${scripts[@]}"; do
        if [[ ! -f "$script" ]]; then
            log_error "Required script not found: $script"
            exit 1
        fi
    done

    # Check if required tools are available
    local tools=("git" "cargo")
    for tool in "${tools[@]}"; do
        if ! command -v "$tool" >/dev/null 2>&1; then
            log_error "Required tool not found: $tool"
            exit 1
        fi
    done

    # Check if working directory is clean
    if ! git diff --quiet || ! git diff --staged --quiet; then
        log_error "Working directory is not clean. Please commit or stash changes first."
        exit 1
    fi

    # Check if on main/master branch
    local current_branch=$(git branch --show-current)
    if [[ "$current_branch" != "main" && "$current_branch" != "master" ]]; then
        log_warning "Not on main/master branch. Current branch: $current_branch"
        read -p "Continue anyway? (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            log_info "Release cancelled"
            exit 0
        fi
    fi

    log_success "Prerequisites validated"
}

# Function to analyze commits for semantic versioning
analyze_commits_for_versioning() {
    local since_ref=$1
    local until_ref=${2:-HEAD}

    log_step "🔍 Analyzing commits for semantic versioning"
    log_info "Analyzing commits from $since_ref to $until_ref"

    # Get commits in the specified range
    local commits
    if [[ "$since_ref" == "initial" ]]; then
        commits=$(git log --oneline --pretty=format:"%s" "$until_ref")
    else
        commits=$(git log --oneline --pretty=format:"%s" "${since_ref}..${until_ref}")
    fi

    if [[ -z "$commits" ]]; then
        log_warning "No commits found in the specified range"
        echo "patch"  # Default to patch for safety
        return
    fi

    # Analyze commit messages for breaking changes and features
    local has_breaking_change=false
    local has_feature=false
    local has_fix=false

    while IFS= read -r commit_msg; do
        # Check for breaking changes (conventional commits with ! or BREAKING CHANGE)
        if echo "$commit_msg" | grep -q "!" || echo "$commit_msg" | grep -qi "BREAKING CHANGE"; then
            has_breaking_change=true
            log_info "Found breaking change: $commit_msg"
        fi

        # Check for features
        if echo "$commit_msg" | grep -q "^feat" || echo "$commit_msg" | grep -qi "feature\|add\|new"; then
            has_feature=true
        fi

        # Check for fixes
        if echo "$commit_msg" | grep -q "^fix" || echo "$commit_msg" | grep -qi "fix\|bug\|issue"; then
            has_fix=true
        fi
    done <<< "$commits"

    # Determine version bump type based on conventional commits
    local bump_type="patch"

    if [[ "$has_breaking_change" == true ]]; then
        bump_type="major"
        log_info "Detected breaking changes - recommending major version bump"
    elif [[ "$has_feature" == true ]]; then
        bump_type="minor"
        log_info "Detected new features - recommending minor version bump"
    else
        bump_type="patch"
        log_info "No breaking changes or features detected - recommending patch version bump"
    fi

    echo "$bump_type"
}

# Function to get current version
get_current_version() {
    grep '^version = ' "$CARGO_TOML" | sed 's/version = "\(.*\)"/\1/'
}

# Function to get latest tag
get_latest_tag() {
    git describe --tags --abbrev=0 2>/dev/null || echo ""
}

# Function to calculate next version
calculate_next_version() {
    local current_version=$1
    local bump_type=$2

    # Parse current version
    local major=$(echo "$current_version" | cut -d. -f1)
    local minor=$(echo "$current_version" | cut -d. -f2)
    local patch=$(echo "$current_version" | cut -d. -f3 | sed 's/[^0-9]*$//')

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
            exit 1
            ;;
    esac

    echo "$major.$minor.$patch"
}

# Function to run tests and validation
run_validation() {
    log_step "🧪 Running Validation Suite"

    cd "$GIT_REPO_ROOT"

    # Run cargo check
    log_info "Running cargo check..."
    if ! cargo check --quiet; then
        log_error "Cargo check failed"
        exit 1
    fi

    # Run tests
    log_info "Running tests..."
    if ! cargo test --release --quiet; then
        log_error "Tests failed"
        exit 1
    fi

    # Run clippy
    log_info "Running clippy..."
    if ! cargo clippy -- -D warnings --quiet; then
        log_error "Clippy checks failed"
        exit 1
    fi

    # Run format check
    log_info "Checking code formatting..."
    if ! cargo fmt --check --quiet; then
        log_error "Code formatting check failed"
        exit 1
    fi

    log_success "All validation checks passed"
}

# Function to build release artifacts
build_release_artifacts() {
    local version=$1

    log_step "🔨 Building Release Artifacts"

    cd "$GIT_REPO_ROOT"

    # Build release binary
    log_info "Building release binary..."
    if ! cargo build --release --quiet; then
        log_error "Release build failed"
        exit 1
    fi

    # Create artifacts directory
    local artifacts_dir="$GIT_REPO_ROOT/artifacts"
    mkdir -p "$artifacts_dir"

    # Copy binary
    local binary_path="target/release/codeguardian"
    if [[ -f "$binary_path" ]]; then
        cp "$binary_path" "$artifacts_dir/codeguardian-$version"
        log_success "Release artifact created: codeguardian-$version"
    else
        log_error "Release binary not found at $binary_path"
        exit 1
    fi
}

# Function to update version
update_version() {
    local new_version=$1

    log_step "📝 Updating Version to $new_version"

    # Update Cargo.toml
    sed -i "s/^version = \".*\"/version = \"$new_version\"/" "$CARGO_TOML"
    log_info "Updated version in Cargo.toml"

    # Create version commit
    git add "$CARGO_TOML"
    git commit -m "chore: bump version to $new_version"
    log_success "Created version bump commit"
}

# Function to generate changelog
generate_changelog() {
    local version=$1
    local since_tag=$2

    log_step "📝 Generating Changelog"

    local date=$(date +%Y-%m-%d)

    if [[ -n "$since_tag" ]]; then
        bash "$CHANGELOG_SCRIPT" --version "$version" --date "$date" --since "$since_tag"
    else
        bash "$CHANGELOG_SCRIPT" --version "$version" --date "$date"
    fi

    log_success "Changelog updated"
}

# Function to generate release notes
generate_release_notes() {
    local version=$1
    local since_ref=$2
    local until_ref=$3
    local output_file=$4

    log_step "📋 Generating Release Notes"

    bash "$RELEASE_NOTES_SCRIPT" \
        --version "$version" \
        --since "$since_ref" \
        --until "$until_ref" \
        --output "$output_file"

    log_success "Release notes generated: $output_file"
}

# Function to create git tag
create_git_tag() {
    local version=$1

    log_step "🏷️ Creating Git Tag"

    git tag -a "v$version" -m "Release v$version"
    log_success "Created git tag v$version"
}

# Function to publish to crates.io
publish_to_crates() {
    local dry_run=${1:-false}

    log_step "📦 Publishing to Crates.io"

    cd "$GIT_REPO_ROOT"

    if [[ "$dry_run" == "true" ]]; then
        log_info "Dry run mode - checking publication"
        if ! cargo publish --dry-run --quiet; then
            log_error "Dry run publication failed"
            exit 1
        fi
        log_success "Dry run publication successful"
    else
        if ! cargo publish --quiet; then
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

    log_step "🎉 Creating GitHub Release"

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

# Function to push changes
push_changes() {
    local dry_run=${1:-false}

    log_step "📤 Pushing Changes"

    if [[ "$dry_run" == "true" ]]; then
        log_info "Dry run mode - would push changes and tags"
        return 0
    fi

    git push && git push --tags
    log_success "Changes and tags pushed to remote"
}

# Function to cleanup
cleanup() {
    local temp_dir=$1

    if [[ -d "$temp_dir" ]]; then
        rm -rf "$temp_dir"
        log_info "Cleaned up temporary files"
    fi
}

# Function to show summary
show_summary() {
    local version=$1
    local bump_type=$2
    local dry_run=$3

    log_header "📊 Release Summary"

    echo "Version: $version"
    echo "Bump Type: $bump_type"
    echo "Mode: $([[ "$dry_run" == "true" ]] && echo "Dry Run" || echo "Production")"
    echo ""
    echo "Next Steps:"
    if [[ "$dry_run" == "true" ]]; then
        echo "  - Review the dry run output above"
        echo "  - Run without --dry-run to perform actual release"
    else
        echo "  - Monitor CI/CD pipelines"
        echo "  - Check GitHub release: https://github.com/${GITHUB_REPO}/releases/tag/v${version}"
        echo "  - Check crates.io: https://crates.io/crates/do-codeguardian/${version}"
    fi
}

# Main function
main() {
    local bump_type=""
    local custom_version=""
    local force_bump=""
    local skip_validation=false
    local skip_build=false
    local skip_publish=false
    local skip_github_release=false
    local dry_run=false
    local since_ref=""

    # Parse command line arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            --major)
                force_bump="major"
                shift
                ;;
            --minor)
                force_bump="minor"
                shift
                ;;
            --patch)
                force_bump="patch"
                shift
                ;;
            --version)
                custom_version="$2"
                shift 2
                ;;
            --since)
                since_ref="$2"
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
            --help)
                echo "Usage: $0 [options]"
                echo ""
                echo "Automated Release Workflow Script"
                echo "Combines version detection, changelog generation, and release creation"
                echo ""
                echo "Options:"
                echo "  --major               Force major version bump"
                echo "  --minor               Force minor version bump"
                echo "  --patch               Force patch version bump"
                echo "  --version VERSION     Set specific version"
                echo "  --since REF           Analyze commits since this reference"
                echo "  --skip-validation     Skip validation steps"
                echo "  --skip-build          Skip building release artifacts"
                echo "  --skip-publish        Skip publishing to crates.io"
                echo "  --skip-github-release Skip creating GitHub release"
                echo "  --dry-run             Dry run mode (no actual changes)"
                echo "  --help                Show this help"
                echo ""
                echo "Examples:"
                echo "  $0                          # Auto-detect version bump"
                echo "  $0 --minor                  # Force minor version bump"
                echo "  $0 --version 1.2.3          # Set specific version"
                echo "  $0 --since v1.0.0 --dry-run # Analyze since v1.0.0 in dry-run mode"
                exit 0
                ;;
            *)
                log_error "Unknown option: $1"
                exit 1
                ;;
        esac
    done

    # Validate arguments
    if [[ -n "$force_bump" && -n "$custom_version" ]]; then
        log_error "Cannot specify both forced bump type and custom version"
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

    log_header "🚀 Starting Automated Release Workflow"
    log_info "Current version: $current_version"
    log_info "Latest tag: ${latest_tag:-none}"

    # Determine since reference for commit analysis
    if [[ -z "$since_ref" ]]; then
        if [[ -n "$latest_tag" ]]; then
            since_ref="$latest_tag"
        else
            since_ref="initial"
        fi
    fi

    # Determine version bump type
    if [[ -n "$custom_version" ]]; then
        bump_type="custom"
        new_version="$custom_version"
        log_info "Using custom version: $new_version"
    elif [[ -n "$force_bump" ]]; then
        bump_type="$force_bump"
        new_version=$(calculate_next_version "$current_version" "$bump_type")
        log_info "Forced $bump_type version bump: $current_version -> $new_version"
    else
        # Auto-detect version bump from commits
        bump_type=$(analyze_commits_for_versioning "$since_ref")
        new_version=$(calculate_next_version "$current_version" "$bump_type")
        log_info "Auto-detected $bump_type version bump: $current_version -> $new_version"
    fi

    # Confirm release
    if [[ "$dry_run" != "true" ]]; then
        echo ""
        log_warning "About to create release v$new_version"
        read -p "Proceed with release? (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            log_info "Release cancelled"
            exit 0
        fi
    fi

    # Step 1: Validation
    if [[ "$skip_validation" != "true" ]]; then
        run_validation
    else
        log_info "Skipping validation"
    fi

    # Step 2: Build release artifacts
    if [[ "$skip_build" != "true" ]]; then
        build_release_artifacts "$new_version"
    else
        log_info "Skipping build"
    fi

    # Step 3: Update version
    if [[ "$dry_run" != "true" ]]; then
        update_version "$new_version"
    else
        log_info "Dry run: Would update version to $new_version"
    fi

    # Step 4: Generate changelog
    if [[ "$dry_run" != "true" ]]; then
        generate_changelog "$new_version" "$latest_tag"
    else
        log_info "Dry run: Would generate changelog"
    fi

    # Step 5: Generate release notes
    local release_notes_file="$temp_dir/release-notes.md"
    if [[ "$dry_run" != "true" ]]; then
        if [[ -n "$latest_tag" ]]; then
            generate_release_notes "$new_version" "$latest_tag" "HEAD" "$release_notes_file"
        else
            log_warning "No previous tag found, creating basic release notes"
            cat > "$release_notes_file" << EOF
# Release ${new_version}

**Full Changelog**: https://github.com/${GITHUB_REPO}/compare/HEAD...v${new_version}

## 🚀 Features

- Initial release

EOF
        fi
    else
        log_info "Dry run: Would generate release notes"
    fi

    # Step 6: Create git tag
    if [[ "$dry_run" != "true" ]]; then
        create_git_tag "$new_version"
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
    if [[ "$skip_github_release" != "true" ]]; then
        create_github_release "$new_version" "$release_notes_file" "$dry_run"
    else
        log_info "Skipping GitHub release creation"
    fi

    # Step 9: Push changes
    push_changes "$dry_run"

    # Show summary
    show_summary "$new_version" "$bump_type" "$dry_run"

    if [[ "$dry_run" == "true" ]]; then
        log_success "Dry run completed successfully!"
    else
        log_success "Release v$new_version completed successfully!"
    fi
}

# Run main function with all arguments
main "$@"