#!/bin/bash

# CodeGuardian Release Management - Enhanced Changelog Generation Script
# This script automates changelog generation from git commits with advanced features
#
# Features:
# - Conventional commits with breaking change detection
# - Commit scopes support
# - PR information integration
# - Author information
# - Multiple output formats (markdown, json)
# - Configuration file support
# - Enhanced categorization

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
CHANGELOG_MD="CHANGELOG.md"
CONFIG_FILE="scripts/release/changelog-config.toml"
GIT_REPO_ROOT="$(git rev-parse --show-toplevel)"
OUTPUT_FORMAT="markdown"
INCLUDE_AUTHORS=false
INCLUDE_PR_INFO=false
VERBOSE=false

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

log_verbose() {
    if [[ "$VERBOSE" == "true" ]]; then
        echo -e "${CYAN}[VERBOSE]${NC} $1"
    fi
}

# Function to load configuration
load_config() {
    if [[ -f "$CONFIG_FILE" ]]; then
        log_verbose "Loading configuration from $CONFIG_FILE"
        # Simple TOML-like parsing (basic implementation)
        while IFS='=' read -r key value; do
            key=$(echo "$key" | sed 's/^[[:space:]]*//' | sed 's/[[:space:]]*$//')
            value=$(echo "$value" | sed 's/^[[:space:]]*//' | sed 's/[[:space:]]*$//' | sed 's/^"//' | sed 's/"$//')
            case $key in
                output_format) OUTPUT_FORMAT="$value" ;;
                include_authors) INCLUDE_AUTHORS="$value" ;;
                include_pr_info) INCLUDE_PR_INFO="$value" ;;
                verbose) VERBOSE="$value" ;;
            esac
        done < <(grep -E '^(output_format|include_authors|include_pr_info|verbose)' "$CONFIG_FILE" || true)
    fi
}

# Function to get latest tag
get_latest_tag() {
    git describe --tags --abbrev=0 2>/dev/null || echo ""
}

# Function to get commits since last tag with detailed information
get_commits_since_tag() {
    local since_tag=$1
    local format_string

    if [[ "$INCLUDE_AUTHORS" == "true" ]]; then
        format_string="%H|%h|%s|%an|%ae|%ad"
    else
        format_string="%H|%h|%s|%ad"
    fi

    if [[ -n "$since_tag" ]]; then
        git log --date=short --pretty=format:"$format_string" "${since_tag}..HEAD"
    else
        git log --date=short --pretty=format:"$format_string"
    fi
}

# Function to extract PR number from commit message
extract_pr_info() {
    local commit_msg=$1
    local hash=$2

    # Look for PR references in commit message
    local pr_number=""
    if echo "$commit_msg" | grep -q "#[0-9]\+"; then
        pr_number=$(echo "$commit_msg" | grep -o "#[0-9]\+" | head -1 | sed 's/#//')
    fi

    # Try to get PR info from git notes if available
    if [[ -z "$pr_number" ]] && git notes show "$hash" >/dev/null 2>&1; then
        local notes=$(git notes show "$hash" 2>/dev/null || true)
        if echo "$notes" | grep -q "pull request"; then
            pr_number=$(echo "$notes" | grep -o "#[0-9]\+" | head -1 | sed 's/#//')
        fi
    fi

    echo "$pr_number"
}

# Function to parse conventional commit
parse_conventional_commit() {
    local commit_line=$1
    local IFS='|'
    read -r full_hash short_hash subject author email date <<< "$commit_line"

    local commit_msg="$subject"
    local scope=""
    local breaking=false
    local pr_info=""

    if [[ "$INCLUDE_PR_INFO" == "true" ]]; then
        pr_info=$(extract_pr_info "$commit_msg" "$full_hash")
    fi

    # Check for breaking change marker (!)
    if echo "$commit_msg" | grep -q "^[a-zA-Z]\+([^)]\+)!:"; then
        breaking=true
        scope=$(echo "$commit_msg" | sed -n 's/^[a-zA-Z]\+(\([^)]*\))!:.*/\1/p')
    elif echo "$commit_msg" | grep -q "^[a-zA-Z]\+!:"; then
        breaking=true
    else
        scope=$(echo "$commit_msg" | sed -n 's/^[a-zA-Z]\+(\([^)]*\)):.*/\1/p')
    fi

    # Extract type
    local type=""
    if echo "$commit_msg" | grep -q "^[a-zA-Z]\+"; then
        type=$(echo "$commit_msg" | sed -n 's/^\([a-zA-Z]\+\).*/\1/p')
    fi

    # Check for BREAKING CHANGE footer
    local full_commit_msg=$(git show -s --format=%B "$full_hash")
    if echo "$full_commit_msg" | grep -q "^BREAKING CHANGE:"; then
        breaking=true
    fi

    # Extract description (remove type and scope)
    local description=""
    if [[ -n "$scope" ]]; then
        if [[ "$breaking" == "true" ]]; then
            description=$(echo "$commit_msg" | sed 's/^[a-zA-Z]\+([^)]\+)!: //' | sed 's/^[a-zA-Z]\+!: //')
        else
            description=$(echo "$commit_msg" | sed 's/^[a-zA-Z]\+([^)]\+): //' | sed 's/^[a-zA-Z]\+: //')
        fi
    else
        if [[ "$breaking" == "true" ]]; then
            description=$(echo "$commit_msg" | sed 's/^[a-zA-Z]\+!: //' | sed 's/^[a-zA-Z]\+!: //')
        else
            description=$(echo "$commit_msg" | sed 's/^[a-zA-Z]\+: //' | sed 's/^[a-zA-Z]\+: //')
        fi
    fi

    # Build result
    local result="$type|$scope|$breaking|$description|$short_hash|$date"
    if [[ "$INCLUDE_AUTHORS" == "true" ]]; then
        result="$result|$author|$email"
    fi
    if [[ "$INCLUDE_PR_INFO" == "true" ]]; then
        result="$result|$pr_info"
    fi

    echo "$result"
}

# Function to categorize commits
categorize_commit() {
    local parsed_commit=$1
    local IFS='|'
    if [[ "$INCLUDE_AUTHORS" == "true" && "$INCLUDE_PR_INFO" == "true" ]]; then
        read -r type scope breaking description hash date author email pr <<< "$parsed_commit"
    elif [[ "$INCLUDE_AUTHORS" == "true" ]]; then
        read -r type scope breaking description hash date author email <<< "$parsed_commit"
        pr=""
    elif [[ "$INCLUDE_PR_INFO" == "true" ]]; then
        read -r type scope breaking description hash date pr <<< "$parsed_commit"
        author=""
        email=""
    else
        read -r type scope breaking description hash date <<< "$parsed_commit"
        author=""
        email=""
        pr=""
    fi

    local category=""
    local section=""

    # Determine category and section
    case $type in
        feat)
            category="Features"
            section="Added"
            ;;
        fix)
            category="Bug Fixes"
            section="Fixed"
            ;;
        docs)
            category="Documentation"
            section="Documentation"
            ;;
        style)
            category="Style"
            section="Style"
            ;;
        refactor)
            category="Refactoring"
            section="Changed"
            ;;
        perf)
            category="Performance"
            section="Performance"
            ;;
        test)
            category="Testing"
            section="Testing"
            ;;
        chore)
            category="Maintenance"
            section="Maintenance"
            ;;
        ci)
            category="CI/CD"
            section="CI/CD"
            ;;
        build)
            category="Build"
            section="Build"
            ;;
        security)
            category="Security"
            section="Security"
            ;;
        revert)
            category="Reverts"
            section="Reverted"
            ;;
        *)
            category="Other"
            section="Other"
            ;;
    esac

    # Check for special keywords in description
    if echo "$description" | grep -qi "deprecat"; then
        section="Deprecated"
    elif echo "$description" | grep -qi "remov"; then
        section="Removed"
    fi

    # Handle breaking changes
    if [[ "$breaking" == "true" ]]; then
        section="Breaking Changes"
    fi

    # Build description with additional info
    local full_description="$description"
    if [[ -n "$scope" ]]; then
        full_description="**$scope:** $description"
    fi

    if [[ -n "$pr" ]]; then
        full_description="$full_description ([#$pr]($hash))"
    else
        full_description="$full_description ($hash)"
    fi

    if [[ "$INCLUDE_AUTHORS" == "true" && -n "$author" ]]; then
        full_description="$full_description - $author"
    fi

    echo "$section|$full_description|$category"
}

# Function to generate markdown changelog entry
generate_markdown_changelog() {
    local version=$1
    local date=$2
    local commits_data=$3

    local breaking_items=""
    local added_items=""
    local changed_items=""
    local deprecated_items=""
    local removed_items=""
    local fixed_items=""
    local security_items=""
    local performance_items=""
    local other_items=""

    # Process each commit
    while IFS= read -r commit_line; do
        if [[ -z "$commit_line" ]]; then
            continue
        fi

        local parsed=$(parse_conventional_commit "$commit_line")
        local categorized=$(categorize_commit "$parsed")
        IFS='|' read -r section description category <<< "$categorized"

        case $section in
            "Breaking Changes")
                breaking_items="${breaking_items}- ${description}\n"
                ;;
            "Added")
                added_items="${added_items}- ${description}\n"
                ;;
            "Changed")
                changed_items="${changed_items}- ${description}\n"
                ;;
            "Deprecated")
                deprecated_items="${deprecated_items}- ${description}\n"
                ;;
            "Removed")
                removed_items="${removed_items}- ${description}\n"
                ;;
            "Fixed")
                fixed_items="${fixed_items}- ${description}\n"
                ;;
            "Security")
                security_items="${security_items}- ${description}\n"
                ;;
            "Performance")
                performance_items="${performance_items}- ${description}\n"
                ;;
            *)
                other_items="${other_items}- ${description}\n"
                ;;
        esac
    done <<< "$commits_data"

    # Generate changelog entry
    local entry="## [${version}] - ${date}\n\n"

    if [[ -n "$breaking_items" ]]; then
        entry="${entry}### ⚠️ Breaking Changes\n\n${breaking_items}\n"
    fi

    if [[ -n "$added_items" ]]; then
        entry="${entry}### Added\n\n${added_items}\n"
    fi

    if [[ -n "$changed_items" ]]; then
        entry="${entry}### Changed\n\n${changed_items}\n"
    fi

    if [[ -n "$deprecated_items" ]]; then
        entry="${entry}### Deprecated\n\n${deprecated_items}\n"
    fi

    if [[ -n "$removed_items" ]]; then
        entry="${entry}### Removed\n\n${removed_items}\n"
    fi

    if [[ -n "$fixed_items" ]]; then
        entry="${entry}### Fixed\n\n${fixed_items}\n"
    fi

    if [[ -n "$security_items" ]]; then
        entry="${entry}### Security\n\n${security_items}\n"
    fi

    if [[ -n "$performance_items" ]]; then
        entry="${entry}### Performance\n\n${performance_items}\n"
    fi

    if [[ -n "$other_items" ]]; then
        entry="${entry}### Other\n\n${other_items}\n"
    fi

    echo -e "$entry"
}

# Function to generate JSON changelog entry
generate_json_changelog() {
    local version=$1
    local date=$2
    local commits_data=$3

    local json_entries="[]"

    # Process each commit
    while IFS= read -r commit_line; do
        if [[ -z "$commit_line" ]]; then
            continue
        fi

        local parsed=$(parse_conventional_commit "$commit_line")
        local categorized=$(categorize_commit "$parsed")
        IFS='|' read -r section description category <<< "$categorized"

        # Parse parsed commit for JSON
        local IFS='|'
        if [[ "$INCLUDE_AUTHORS" == "true" && "$INCLUDE_PR_INFO" == "true" ]]; then
            read -r type scope breaking desc hash commit_date author email pr <<< "$parsed"
        elif [[ "$INCLUDE_AUTHORS" == "true" ]]; then
            read -r type scope breaking desc hash commit_date author email <<< "$parsed"
            pr=""
        elif [[ "$INCLUDE_PR_INFO" == "true" ]]; then
            read -r type scope breaking desc hash commit_date pr <<< "$parsed"
            author=""
            email=""
        else
            read -r type scope breaking desc hash commit_date <<< "$parsed"
            author=""
            email=""
            pr=""
        fi

        local entry="{\"type\":\"$type\",\"scope\":\"$scope\",\"breaking\":$breaking,\"description\":\"$desc\",\"hash\":\"$hash\",\"date\":\"$commit_date\",\"section\":\"$section\",\"category\":\"$category\""
        if [[ -n "$author" ]]; then
            entry="$entry,\"author\":\"$author\",\"email\":\"$email\""
        fi
        if [[ -n "$pr" ]]; then
            entry="$entry,\"pr\":\"$pr\""
        fi
        entry="$entry}"

        # Add to JSON array (simple append)
        if [[ "$json_entries" == "[]" ]]; then
            json_entries="[$entry"
        else
            json_entries="$json_entries,$entry"
        fi
    done <<< "$commits_data"

    json_entries="$json_entries]"

    local json="{\"version\":\"$version\",\"date\":\"$date\",\"changes\":$json_entries}"
    echo "$json"
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

# Function to show usage
show_usage() {
    cat << EOF
Enhanced Changelog Generator for CodeGuardian

USAGE:
    $0 [OPTIONS] --version VERSION

OPTIONS:
    --version VERSION       Version for the changelog entry (required)
    --date DATE            Date for the changelog entry (default: today)
    --since TAG            Generate changelog since this tag (default: latest tag)
    --output-format FORMAT Output format: markdown (default) or json
    --include-authors      Include author information in output
    --include-pr-info      Include PR information when available
    --config FILE          Configuration file (default: scripts/release/changelog-config.toml)
    --verbose              Enable verbose logging
    --dry-run              Show preview without updating files
    --help                 Show this help message

EXAMPLES:
    $0 --version 1.2.3
    $0 --version 1.2.3 --since v1.2.2 --include-authors --include-pr-info
    $0 --version 1.2.3 --output-format json --dry-run

CONFIGURATION:
    Create a changelog-config.toml file to set default options:
    output_format = "markdown"
    include_authors = true
    include_pr_info = true
    verbose = false

CONVENTIONAL COMMITS:
    The script supports conventional commits with the following types:
    - feat: Features
    - fix: Bug fixes
    - docs: Documentation
    - style: Code style changes
    - refactor: Code refactoring
    - perf: Performance improvements
    - test: Testing
    - chore: Maintenance
    - ci: CI/CD changes
    - build: Build system changes
    - security: Security fixes
    - revert: Reverts

    Breaking changes are detected by:
    - ! suffix in commit type: feat(api)!: breaking change
    - BREAKING CHANGE: footer in commit message

    Scopes are supported: feat(api): add new endpoint
EOF
}

# Main function
main() {
    local version=""
    local date=""
    local since_tag=""
    local dry_run=false

    # Load configuration first
    load_config

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
            --output-format)
                OUTPUT_FORMAT="$2"
                shift 2
                ;;
            --include-authors)
                INCLUDE_AUTHORS=true
                shift
                ;;
            --include-pr-info)
                INCLUDE_PR_INFO=true
                shift
                ;;
            --config)
                CONFIG_FILE="$2"
                load_config
                shift 2
                ;;
            --verbose)
                VERBOSE=true
                shift
                ;;
            --dry-run)
                dry_run=true
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

    # Validate required arguments
    if [[ -z "$version" ]]; then
        log_error "Version is required. Use --version"
        echo "Use --help for usage information"
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
    log_info "Output format: $OUTPUT_FORMAT"
    if [[ -n "$since_tag" ]]; then
        log_info "Since tag: $since_tag"
    else
        log_info "Since: beginning of repository"
    fi

    if [[ "$INCLUDE_AUTHORS" == "true" ]]; then
        log_info "Including author information"
    fi

    if [[ "$INCLUDE_PR_INFO" == "true" ]]; then
        log_info "Including PR information"
    fi

    # Get commits
    local commits=$(get_commits_since_tag "$since_tag")

    if [[ -z "$commits" ]]; then
        log_warning "No commits found since $since_tag"
        exit 0
    fi

    # Generate changelog entry
    local changelog_entry=""
    case $OUTPUT_FORMAT in
        markdown)
            changelog_entry=$(generate_markdown_changelog "$version" "$date" "$commits")
            ;;
        json)
            changelog_entry=$(generate_json_changelog "$version" "$date" "$commits")
            ;;
        *)
            log_error "Unsupported output format: $OUTPUT_FORMAT"
            exit 1
            ;;
    esac

    # Show preview
    echo "=== Changelog Entry Preview ==="
    if [[ "$OUTPUT_FORMAT" == "json" ]]; then
        echo "$changelog_entry" | jq . 2>/dev/null || echo "$changelog_entry"
    else
        echo "$changelog_entry"
    fi
    echo "================================="

    if [[ "$dry_run" == "true" ]]; then
        log_info "Dry run mode - not updating files"
        exit 0
    fi

    # Confirm (only for markdown format)
    if [[ "$OUTPUT_FORMAT" == "markdown" ]]; then
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
    else
        # For JSON, just output to stdout or save to file
        local json_file="changelog-${version}.json"
        echo "$changelog_entry" > "$json_file"
        log_success "JSON changelog saved to $json_file"
    fi
}

# Run main function with all arguments
main "$@"