#!/bin/bash

# CodeGuardian Release Management - Release Notes Generation Script
# This script generates detailed release notes from PRs and commits

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
GIT_REPO_ROOT="$(git rev-parse --show-toplevel)"
GITHUB_REPO="${GITHUB_REPO:-d-oit/do-codeguardian}"

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

# Function to get PRs merged between two commits/tags
get_merged_prs() {
    local since=$1
    local until=$2

    # Use GitHub CLI if available
    if command -v gh >/dev/null 2>&1; then
        log_info "Using GitHub CLI to fetch PRs"
        gh pr list --state merged --json number,title,author,labels,mergedAt \
            --search "merged:$since..$until" \
            --limit 100
    else
        log_warning "GitHub CLI not found. Using git log to extract PR information."
        # Fallback to git log parsing
        git log --oneline --grep="Merge pull request" --since="$since" --until="$until" | \
        grep -o "#[0-9]\+" | tr -d "#" | sort -u
    fi
}

# Function to categorize PR based on labels
categorize_pr() {
    local pr_data=$1

    # Extract labels from PR data
    local labels=""
    if command -v jq >/dev/null 2>&1 && [[ "$pr_data" == "{"* ]]; then
        labels=$(echo "$pr_data" | jq -r '.labels[].name' 2>/dev/null || echo "")
    fi

    # Default category
    local category="🔧 Maintenance"

    # Check labels for categorization
    if echo "$labels" | grep -qi "feature\|enhancement"; then
        category="🚀 Features"
    elif echo "$labels" | grep -qi "bug\|fix"; then
        category="🐛 Bug Fixes"
    elif echo "$labels" | grep -qi "security"; then
        category="🔒 Security"
    elif echo "$labels" | grep -qi "documentation\|docs"; then
        category="📚 Documentation"
    elif echo "$labels" | grep -qi "performance\|perf"; then
        category="⚡ Performance"
    elif echo "$labels" | grep -qi "test"; then
        category="🧪 Testing"
    elif echo "$labels" | grep -qi "refactor"; then
        category="🔄 Refactoring"
    elif echo "$labels" | grep -qi "dependencies\|deps"; then
        category="📦 Dependencies"
    fi

    echo "$category"
}

# Function to format PR entry
format_pr_entry() {
    local pr_data=$1

    if command -v jq >/dev/null 2>&1 && [[ "$pr_data" == "{"* ]]; then
        # Parse JSON data from GitHub CLI
        local number=$(echo "$pr_data" | jq -r '.number')
        local title=$(echo "$pr_data" | jq -r '.title')
        local author=$(echo "$pr_data" | jq -r '.author.login')
        local category=$(categorize_pr "$pr_data")

        echo "$category|$title (#$number) by @$author"
    else
        # Fallback for simple PR number
        local pr_number=$pr_data
        echo "🔧 Maintenance|Pull request #$pr_number"
    fi
}

# Function to generate release notes
generate_release_notes() {
    local version=$1
    local since=$2
    local until=$3

    log_info "Generating release notes for $version"
    log_info "From: $since"
    log_info "To: $until"

    # Get merged PRs
    local prs_data=$(get_merged_prs "$since" "$until")

    if [[ -z "$prs_data" ]]; then
        log_warning "No merged PRs found between $since and $until"
        return 1
    fi

    # Initialize categories
    declare -A categories=(
        ["🚀 Features"]=""
        ["🐛 Bug Fixes"]=""
        ["🔒 Security"]=""
        ["📚 Documentation"]=""
        ["⚡ Performance"]=""
        ["🧪 Testing"]=""
        ["🔄 Refactoring"]=""
        ["📦 Dependencies"]=""
        ["🔧 Maintenance"]=""
    )

    # Process PRs
    if command -v jq >/dev/null 2>&1 && [[ "$prs_data" == "["* ]]; then
        # Process JSON array from GitHub CLI
        local pr_count=$(echo "$prs_data" | jq '. | length')
        log_info "Found $pr_count merged PRs"

        for ((i=0; i<pr_count; i++)); do
            local pr_data=$(echo "$prs_data" | jq ".[$i]")
            local formatted=$(format_pr_entry "$pr_data")
            IFS='|' read -r category description <<< "$formatted"

            if [[ -n "${categories[$category]}" ]]; then
                categories[$category]="${categories[$category]}\n$description"
            else
                categories[$category]="$description"
            fi
        done
    else
        # Fallback processing
        log_info "Processing PRs in fallback mode"
        while IFS= read -r pr_number; do
            if [[ -n "$pr_number" ]]; then
                local formatted=$(format_pr_entry "$pr_number")
                IFS='|' read -r category description <<< "$formatted"
                categories[$category]="${categories[$category]}\n$description"
            fi
        done <<< "$prs_data"
    fi

    # Generate release notes
    local notes="# Release ${version}\n\n"
    notes="${notes}**Full Changelog**: https://github.com/${GITHUB_REPO}/compare/${since}...${until}\n\n"

    # Add categorized PRs
    local has_content=false
    for category in "${!categories[@]}"; do
        if [[ -n "${categories[$category]}" ]]; then
            notes="${notes}## ${category}\n\n"
            notes="${notes}${categories[$category]}\n\n"
            has_content=true
        fi
    done

    if [[ "$has_content" == false ]]; then
        log_warning "No categorized content found"
        return 1
    fi

    echo -e "$notes"
}

# Function to save release notes to file
save_release_notes() {
    local notes=$1
    local output_file=$2

    echo -e "$notes" > "$output_file"
    log_success "Release notes saved to $output_file"
}

# Main function
main() {
    local version=""
    local since=""
    local until=""
    local output_file=""

    # Parse command line arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            --version)
                version="$2"
                shift 2
                ;;
            --since)
                since="$2"
                shift 2
                ;;
            --until)
                until="$2"
                shift 2
                ;;
            --output)
                output_file="$2"
                shift 2
                ;;
            --help)
                echo "Usage: $0 --version VERSION --since REF --until REF [--output FILE]"
                echo ""
                echo "Options:"
                echo "  --version   Release version"
                echo "  --since     Starting reference (tag/commit)"
                echo "  --until     Ending reference (tag/commit)"
                echo "  --output    Output file (default: stdout)"
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
    if [[ -z "$version" || -z "$since" || -z "$until" ]]; then
        log_error "Version, since, and until are required"
        exit 1
    fi

    # Change to repository root
    cd "$GIT_REPO_ROOT"

    # Generate release notes
    local notes=$(generate_release_notes "$version" "$since" "$until")

    if [[ $? -ne 0 ]]; then
        log_error "Failed to generate release notes"
        exit 1
    fi

    # Output or save
    if [[ -n "$output_file" ]]; then
        save_release_notes "$notes" "$output_file"
    else
        echo "$notes"
    fi
}

# Run main function with all arguments
main "$@"
