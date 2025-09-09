#!/bin/bash
# GitHub Issue Duplicate Prevention Utilities
# This script provides duplicate detection and prevention for GitHub issues

set -euo pipefail

# Default configuration
GITHUB_API_MAX_RETRIES=${GITHUB_API_MAX_RETRIES:-3}
GITHUB_API_RETRY_DELAY=${GITHUB_API_RETRY_DELAY:-2}
GITHUB_ISSUE_CACHE_DIR=${GITHUB_ISSUE_CACHE_DIR:-/tmp/github-issue-cache}
GITHUB_ISSUE_CACHE_TTL=${GITHUB_ISSUE_CACHE_TTL:-3600} # 1 hour

# Initialize cache directory
mkdir -p "$GITHUB_API_CACHE_DIR"

# Function to execute GitHub CLI with retry logic
exec_gh_with_retry() {
    local args=("$@")
    local attempt=1
    local max_attempts=$GITHUB_API_MAX_RETRIES
    local delay=$GITHUB_API_RETRY_DELAY
    
    while [ $attempt -le $max_attempts ]; do
        if gh "${args[@]}" 2>/dev/null; then
            return 0
        fi
        
        if [ $attempt -lt $max_attempts ]; then
            echo "Attempt $attempt failed, retrying in ${delay}s..." >&2
            sleep $delay
            delay=$((delay * 2))
        fi
        
        attempt=$((attempt + 1))
    done
    
    echo "All attempts failed for command: gh ${args[*]}" >&2
    return 1
}

# Function to search for existing issues with enhanced duplicate detection
find_existing_issues() {
    local repo="$1"
    local search_query="$2"
    local fields="${3:-number,title,body}"
    
    local cache_key="$(echo "$repo-$search_query" | sha256sum | cut -d' ' -f1)"
    local cache_file="$GITHUB_ISSUE_CACHE_DIR/$cache_key"
    
    # Check cache first
    if [ -f "$cache_file" ] && [ $(($(date +%s) - $(stat -c %Y "$cache_file"))) -lt $GITHUB_ISSUE_CACHE_TTL ]; then
        cat "$cache_file"
        return 0
    fi
    
    # Execute GitHub search
    local result
    result=$(exec_gh_with_retry issue list \
        --repo "$repo" \
        --state "open" \
        --search "$search_query" \
        --json "$fields" 2>/dev/null || echo '[]')
    
    # Cache the result
    echo "$result" > "$cache_file"
    echo "$result"
}

# Function to check for duplicate issues using multiple strategies
detect_duplicate_issue() {
    local repo="$1"
    local title="$2"
    local body="$3"
    local commit_hash="${4:-}"
    
    echo "🔍 Checking for duplicate issues in $repo..." >&2
    
    # Strategy 1: Exact title match
    local title_matches
    title_matches=$(find_existing_issues "$repo" "\"$title\" in:title" "number,title")
    
    if [ "$(echo "$title_matches" | jq 'length')" -gt 0 ]; then
        local issue_number=$(echo "$title_matches" | jq -r '.[0].number')
        echo "📌 Found exact title match: #$issue_number" >&2
        echo "$issue_number"
        return 0
    fi
    
    # Strategy 2: Commit hash in title or body
    if [ -n "$commit_hash" ]; then
        local commit_matches
        commit_matches=$(find_existing_issues "$repo" "$commit_hash in:title,body" "number,title")
        
        if [ "$(echo "$commit_matches" | jq 'length')" -gt 0 ]; then
            local issue_number=$(echo "$commit_matches" | jq -r '.[0].number')
            echo "📌 Found commit hash match: #$issue_number" >&2
            echo "$issue_number"
            return 0
        fi
    fi
    
    # Strategy 3: Semantic matching (basic keyword extraction)
    local keywords=$(extract_keywords "$title" "$body")
    if [ -n "$keywords" ]; then
        local keyword_matches
        keyword_matches=$(find_existing_issues "$repo" "$keywords in:title,body" "number,title")
        
        if [ "$(echo "$keyword_matches" | jq 'length')" -gt 0 ]; then
            local issue_number=$(echo "$keyword_matches" | jq -r '.[0].number')
            echo "📌 Found semantic keyword match: #$issue_number" >&2
            echo "$issue_number"
            return 0
        fi
    fi
    
    echo "✅ No duplicates found" >&2
    echo ""
    return 0
}

# Function to extract keywords for semantic matching
extract_keywords() {
    local title="$1"
    local body="$2"
    
    # Extract important keywords (security terms, error types, file patterns)
    echo "$title $body" | tr '[:upper:]' '[:lower:]' | \
        grep -o -E '\b(security|vulnerability|error|warning|issue|bug|leak|injection|xss|sql|broken|corrupt|malicious)\b' | \
        sort | uniq | head -5 | tr '\n' ' ' | sed 's/ $//'
}

# Function to create or update issue with duplicate prevention
create_or_update_issue() {
    local repo="$1"
    local title="$2"
    local body_file="$3"
    local labels="$4"
    local commit_hash="${5:-}"
    
    # Check for duplicates
    local existing_issue
    existing_issue=$(detect_duplicate_issue "$repo" "$title" "$(cat "$body_file")" "$commit_hash")
    
    if [ -n "$existing_issue" ]; then
        echo "🔄 Updating existing issue #$existing_issue" >&2
        
        # Update existing issue
        exec_gh_with_retry issue edit "$existing_issue" \
            --repo "$repo" \
            --body-file "$body_file" \
            --add-label "$labels"
        
        echo "$existing_issue"
        return 0
    else
        echo "🆕 Creating new issue" >&2
        
        # Create new issue
        local issue_output
        issue_output=$(exec_gh_with_retry issue create \
            --repo "$repo" \
            --title "$title" \
            --body-file "$body_file" \
            --label "$labels")
        
        # Extract issue number from output
        local issue_number=$(echo "$issue_output" | grep -o '#[0-9]\+' | head -1 | tr -d '#')
        echo "$issue_number"
        return 0
    fi
}

# Function to get current commit hash
get_commit_hash() {
    git rev-parse --short HEAD 2>/dev/null || echo ""
}

# Function to generate unique issue title
generate_issue_title() {
    local prefix="$1"
    local repo="$2"
    local commit_hash=$(get_commit_hash)
    
    if [ -n "$commit_hash" ]; then
        echo "$prefix - Commit $commit_hash"
    elif [ -n "${GITHUB_PR_NUMBER:-}" ]; then
        echo "$prefix - PR #$GITHUB_PR_NUMBER"
    else
        echo "$prefix - $(date +%Y%m%d-%H%M%S)"
    fi
}

# Function to clean up cache
cleanup_cache() {
    find "$GITHUB_ISSUE_CACHE_DIR" -type f -mmin +$((GITHUB_ISSUE_CACHE_TTL / 60)) -delete 2>/dev/null || true
}

# Main execution
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    case "${1:-}" in
        "detect-duplicate")
            detect_duplicate_issue "$2" "$3" "$4" "${5:-}"
            ;;
        "create-or-update")
            create_or_update_issue "$2" "$3" "$4" "$5" "${6:-}"
            ;;
        "generate-title")
            generate_issue_title "$2" "$3"
            ;;
        "cleanup-cache")
            cleanup_cache
            ;;
        *)
            echo "Usage: $0 [detect-duplicate|create-or-update|generate-title|cleanup-cache]"
            exit 1
            ;;
    esac
fi