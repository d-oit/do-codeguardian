#!/bin/bash

# CodeGuardian Release Management - Changelog Generation Script (Simplified)
# This script automates changelog generation from git commits

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
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

# Function to get latest tag
get_latest_tag() {
    git describe --tags --abbrev=0 2>/dev/null || echo ""
}

# Function to get commits since last tag
get_commits_since_tag() {
    local since_tag=$1
    if [[ -n "$since_tag" ]]; then
        git log --oneline --pretty=format:"%h %s" "${since_tag}..HEAD"
    else
        git log --oneline --pretty=format:"%h %s"
    fi
}

# Function to categorize commits based on conventional commits
categorize_commit() {
    local commit_msg=$1

    # Extract commit type using grep
    if echo "$commit_msg" | grep -q "^feat"; then
        local description=$(echo "$commit_msg" | sed 's/^feat[^:]*: //' | sed 's/^feat: //')
        echo "Added|$description"
    elif echo "$commit_msg" | grep -q "^fix"; then
        local description=$(echo "$commit_msg" | sed 's/^fix[^:]*: //' | sed 's/^fix: //')
        echo "Fixed|$description"
    elif echo "$commit_msg" | grep -q "^docs"; then
        local description=$(echo "$commit_msg" | sed 's/^docs[^:]*: //' | sed 's/^docs: //')
        echo "Documentation|$description"
    elif echo "$commit_msg" | grep -q "^style"; then
        local description=$(echo "$commit_msg" | sed 's/^style[^:]*: //' | sed 's/^style: //')
        echo "Style|$description"
    elif echo "$commit_msg" | grep -q "^refactor"; then
        local description=$(echo "$commit_msg" | sed 's/^refactor[^:]*: //' | sed 's/^refactor: //')
        echo "Refactoring|$description"
    elif echo "$commit_msg" | grep -q "^perf"; then
        local description=$(echo "$commit_msg" | sed 's/^perf[^:]*: //' | sed 's/^perf: //')
        echo "Performance|$description"
    elif echo "$commit_msg" | grep -q "^test"; then
        local description=$(echo "$commit_msg" | sed 's/^test[^:]*: //' | sed 's/^test: //')
        echo "Testing|$description"
    elif echo "$commit_msg" | grep -q "^chore"; then
        local description=$(echo "$commit_msg" | sed 's/^chore[^:]*: //' | sed 's/^chore: //')
        echo "Maintenance|$description"
    elif echo "$commit_msg" | grep -q "^ci"; then
        local description=$(echo "$commit_msg" | sed 's/^ci[^:]*: //' | sed 's/^ci: //')
        echo "CI/CD|$description"
    elif echo "$commit_msg" | grep -q "^build"; then
        local description=$(echo "$commit_msg" | sed 's/^build[^:]*: //' | sed 's/^build: //')
        echo "Build|$description"
    elif echo "$commit_msg" | grep -q "^security"; then
        local description=$(echo "$commit_msg" | sed 's/^security[^:]*: //' | sed 's/^security: //')
        echo "Security|$description"
    else
        # Non-conventional commit
        echo "Other|$commit_msg"
    fi
}

# Function to generate changelog entry
generate_changelog_entry() {
    local version=$1
    local date=$2
    local commits=$3

    local added_items=""
    local fixed_items=""
    local changed_items=""
    local deprecated_items=""
    local removed_items=""
    local security_items=""
    local other_items=""

    # Process each commit
    while IFS= read -r commit; do
        if [[ -z "$commit" ]]; then
            continue
        fi

        local category_desc=$(categorize_commit "$commit")
        IFS='|' read -r category description <<< "$category_desc"

        case $category in
            Added|feat)
                added_items="${added_items}- ${description}\n"
                ;;
            Fixed|fix)
                fixed_items="${fixed_items}- ${description}\n"
                ;;
            Changed|refactor|perf|style)
                changed_items="${changed_items}- ${description}\n"
                ;;
            Security|security)
                security_items="${security_items}- ${description}\n"
                ;;
            Documentation|docs)
                changed_items="${changed_items}- ${description}\n"
                ;;
            Testing|test)
                changed_items="${changed_items}- ${description}\n"
                ;;
            CI/CD|ci|build)
                changed_items="${changed_items}- ${description}\n"
                ;;
            Maintenance|chore)
                changed_items="${changed_items}- ${description}\n"
                ;;
            *)
                other_items="${other_items}- ${description}\n"
                ;;
        esac
    done <<< "$commits"

    # Generate changelog entry
    local entry="## [${version}] - ${date}\n\n"

    if [[ -n "$added_items" ]]; then
        entry="${entry}### Added\n\n${added_items}\n"
    fi

    if [[ -n "$changed_items" ]]; then
        entry="${entry}### Changed\n\n${changed_items}\n"
    fi

    if [[ -n "$fixed_items" ]]; then
        entry="${entry}### Fixed\n\n${fixed_items}\n"
    fi

    if [[ -n "$security_items" ]]; then
        entry="${entry}### Security\n\n${security_items}\n"
    fi

    if [[ -n "$other_items" ]]; then
        entry="${entry}### Other\n\n${other_items}\n"
    fi

    echo -e "$entry"
}

# Function to update CHANGELOG.md
update_changelog() {
    local new_entry=$1

    # Read existing changelog
    local existing_content=""
    if [[ -f "$CHANGELOG_MD" ]]; then
        existing_content=$(cat "$CHANGELOG_MD")
    fi

    # Create new changelog content
    local header="# Changelog\n\nAll notable changes to this project will be documented in this file.\n\nThe format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),\nand this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).\n\n"

    # Insert new entry after header
    local new_content="${header}${new_entry}${existing_content#*$'\n\n'}"

    # Write to file
    echo -e "$new_content" > "$CHANGELOG_MD"
}

# Main function
main() {
    local version=""
    local date=""
    local since_tag=""

    # Parse command line arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            --version)
                version="$2"
                shift 2
                ;;
            --date)
                date="$2"
                shift 2
                ;;
            --since)
                since_tag="$2"
                shift 2
                ;;
            --help)
                echo "Usage: $0 --version VERSION [--date DATE] [--since TAG]"
                echo ""
                echo "Options:"
                echo "  --version   Version for the changelog entry"
                echo "  --date      Date for the changelog entry (default: today)"
                echo "  --since     Generate changelog since this tag (default: latest tag)"
                echo "  --help      Show this help"
                exit 0
                ;;
            *)
                log_error "Unknown option: $1"
                exit 1
                ;;
        esac
    done

    # Validate required arguments
    if [[ -z "$version" ]]; then
        log_error "Version is required. Use --version"
        exit 1
    fi

    # Set defaults
    if [[ -z "$date" ]]; then
        date=$(date +%Y-%m-%d)
    fi

    if [[ -z "$since_tag" ]]; then
        since_tag=$(get_latest_tag)
    fi

    # Change to repository root
    cd "$GIT_REPO_ROOT"

    log_info "Generating changelog for version $version"
    log_info "Date: $date"
    if [[ -n "$since_tag" ]]; then
        log_info "Since tag: $since_tag"
    else
        log_info "Since: beginning of repository"
    fi

    # Get commits
    local commits=$(get_commits_since_tag "$since_tag")

    if [[ -z "$commits" ]]; then
        log_warning "No commits found since $since_tag"
        exit 0
    fi

    # Generate changelog entry
    local changelog_entry=$(generate_changelog_entry "$version" "$date" "$commits")

    # Show preview
    echo "=== Changelog Entry Preview ==="
    echo "$changelog_entry"
    echo "================================="

    # Confirm
    read -p "Add this entry to CHANGELOG.md? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        log_info "Changelog generation cancelled"
        exit 0
    fi

    # Update changelog
    update_changelog "$changelog_entry"

    log_success "Changelog updated successfully!"
    log_info "Review the changes in $CHANGELOG_MD"
}

# Run main function with all arguments
main "$@"
