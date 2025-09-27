#!/bin/bash
# GitHub Issue Duplicate Prevention Utilities
# This script provides duplicate detection and prevention for GitHub issues

set -euo pipefail

# Default configuration - now configurable
GITHUB_API_MAX_RETRIES=${GITHUB_API_MAX_RETRIES:-3}
GITHUB_API_RETRY_DELAY=${GITHUB_API_RETRY_DELAY:-2}
GITHUB_ISSUE_CACHE_DIR=${GITHUB_ISSUE_CACHE_DIR:-${XDG_CACHE_HOME:-$HOME/.cache}/codeguardian/github-issues}
GITHUB_ISSUE_CACHE_TTL=${GITHUB_ISSUE_CACHE_TTL:-3600} # 1 hour
GITHUB_SIMILARITY_THRESHOLD=${GITHUB_SIMILARITY_THRESHOLD:-0.8} # Semantic matching threshold
GITHUB_MAX_KEYWORDS=${GITHUB_MAX_KEYWORDS:-5} # Maximum keywords for semantic matching
GITHUB_CACHE_INVALIDATION=${GITHUB_CACHE_INVALIDATION:-true} # Enable smart cache invalidation

# Initialize cache directory
mkdir -p "$GITHUB_ISSUE_CACHE_DIR"

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

# Function to validate inputs
validate_duplicate_detection_inputs() {
    local repo="$1"
    local title="$2"
    local body="$3"
    
    if [ -z "$repo" ]; then
        echo "❌ Error: Repository parameter is required" >&2
        return 1
    fi
    
    if [ -z "$title" ]; then
        echo "❌ Error: Title parameter is required" >&2
        return 1
    fi
    
    if [ -z "$body" ]; then
        echo "⚠️ Warning: Body parameter is empty" >&2
    fi
    
    # Validate repository format (owner/repo)
    if [[ ! "$repo" =~ ^[a-zA-Z0-9_.-]+/[a-zA-Z0-9_.-]+$ ]]; then
        echo "❌ Error: Repository must be in format 'owner/repo'" >&2
        return 1
    fi
    
    return 0
}

# Function to check for duplicate issues using multiple strategies
detect_duplicate_issue() {
    local repo="$1"
    local title="$2"
    local body="$3"
    local commit_hash="${4:-}"

    # Validate inputs
    if ! validate_duplicate_detection_inputs "$repo" "$title" "$body"; then
        return 1
    fi

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
        grep -o -E '\b(security|vulnerability|error|warning|issue|bug|leak|injection|xss|sql|broken|corrupt|malicious|performance|regression|optimization|memory|cpu|slow|timeout|bottleneck)\b' | \
        sort | uniq | head -"$GITHUB_MAX_KEYWORDS" | tr '\n' ' ' | sed 's/ $//'
}

# Function to validate create/update issue inputs
validate_create_update_inputs() {
    local repo="$1"
    local title="$2"
    local body_file="$3"
    local labels="$4"
    
    if ! validate_duplicate_detection_inputs "$repo" "$title" "$(cat "$body_file" 2>/dev/null || echo '')"; then
        return 1
    fi
    
    if [ ! -f "$body_file" ]; then
        echo "❌ Error: Body file '$body_file' does not exist" >&2
        return 1
    fi
    
    if [ ! -r "$body_file" ]; then
        echo "❌ Error: Body file '$body_file' is not readable" >&2
        return 1
    fi
    
    if [ -z "$labels" ]; then
        echo "⚠️ Warning: No labels specified" >&2
    fi
    
    return 0
}

# Function to create or update issue with duplicate prevention
create_or_update_issue() {
    local repo="$1"
    local title="$2"
    local body_file="$3"
    local labels="$4"
    local commit_hash="${5:-}"

    # Validate inputs
    if ! validate_create_update_inputs "$repo" "$title" "$body_file" "$labels"; then
        return 1
    fi

    # Check for duplicates
    local existing_issue
    existing_issue=$(detect_duplicate_issue "$repo" "$title" "$(cat "$body_file")" "$commit_hash")

    if [ -n "$existing_issue" ]; then
        echo "🔄 Updating existing issue #$existing_issue" >&2

        # Record metrics
        echo "$(date -u +%Y-%m-%dT%H:%M:%SZ),duplicate_update,$repo,$existing_issue" >> "${GITHUB_ISSUE_CACHE_DIR}/metrics.log" 2>/dev/null || true

        # Update existing issue
        exec_gh_with_retry issue edit "$existing_issue" \
            --repo "$repo" \
            --body-file "$body_file" \
            --add-label "$labels"

        # Invalidate cache after update
        invalidate_cache_for_issue "$repo" "$existing_issue"

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
        
        # Record metrics
        echo "$(date -u +%Y-%m-%dT%H:%M:%SZ),new_issue,$repo,$issue_number" >> "${GITHUB_ISSUE_CACHE_DIR}/metrics.log" 2>/dev/null || true
        
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

# Function to invalidate cache for specific issue
invalidate_cache_for_issue() {
    local repo="$1"
    local issue_number="$2"
    
    if [ "$GITHUB_CACHE_INVALIDATION" = "true" ]; then
        echo "🗑️ Invalidating cache for issue #$issue_number in $repo..." >&2
        # Remove cache entries that might be affected by this issue update
        find "$GITHUB_ISSUE_CACHE_DIR" -name "*$(echo "$repo" | tr '/' '-')*" -delete 2>/dev/null || true
    fi
}

# Function to clean up cache
cleanup_cache() {
    echo "🧹 Cleaning up expired cache entries..." >&2
    local deleted_count=0
    
    # Remove files older than TTL
    while IFS= read -r -d '' file; do
        rm -f "$file" 2>/dev/null && ((deleted_count++))
    done < <(find "$GITHUB_ISSUE_CACHE_DIR" -type f -mmin +"$((GITHUB_ISSUE_CACHE_TTL / 60))" -print0 2>/dev/null)
    
    if [ "$deleted_count" -gt 0 ]; then
        echo "🗑️ Cleaned up $deleted_count expired cache entries" >&2
    fi
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
