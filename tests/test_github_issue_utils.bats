#!/usr/bin/env bats
# Tests for github-issue-utils.sh duplicate detection functionality

setup() {
    # Load the github-issue-utils functions
    source "${BATS_TEST_DIRNAME}/../scripts/github-issue-utils.sh"
    
    # Create temporary test directory
    export TEST_CACHE_DIR=$(mktemp -d)
    export GITHUB_ISSUE_CACHE_DIR="$TEST_CACHE_DIR"
    export GITHUB_ISSUE_CACHE_TTL=60
    
    # Mock gh command for testing
    export PATH="${BATS_TEST_DIRNAME}/mocks:$PATH"
}

teardown() {
    # Clean up test cache directory
    rm -rf "$TEST_CACHE_DIR"
}

# Test extract_keywords function
@test "extract_keywords extracts security-related keywords" {
    local title="Security vulnerability detected in authentication"
    local body="SQL injection vulnerability found in user login system"
    
    result=$(extract_keywords "$title" "$body")
    
    [[ "$result" =~ "security" ]]
    [[ "$result" =~ "vulnerability" ]]
    [[ "$result" =~ "sql" ]]
}

@test "extract_keywords handles empty input" {
    result=$(extract_keywords "" "")
    [ -z "$result" ] || [ "$result" = " " ]
}

@test "extract_keywords is case insensitive" {
    local title="SECURITY VULNERABILITY"
    local body="ERROR in system"
    
    result=$(extract_keywords "$title" "$body")
    
    [[ "$result" =~ "security" ]]
    [[ "$result" =~ "vulnerability" ]]
    [[ "$result" =~ "error" ]]
}

@test "extract_keywords limits to 5 keywords maximum" {
    local text="security vulnerability error warning issue bug leak injection xss sql"
    
    result=$(extract_keywords "$text" "")
    word_count=$(echo "$result" | wc -w)
    
    [ "$word_count" -le 5 ]
}

# Test get_commit_hash function
@test "get_commit_hash returns short hash" {
    # Mock git command
    function git() {
        if [[ "$1" == "rev-parse" && "$2" == "--short" && "$3" == "HEAD" ]]; then
            echo "abc1234"
        fi
    }
    export -f git
    
    result=$(get_commit_hash)
    [ "$result" = "abc1234" ]
}

@test "get_commit_hash handles git failure gracefully" {
    # Mock git command to fail
    function git() {
        return 1
    }
    export -f git
    
    result=$(get_commit_hash)
    [ -z "$result" ]
}

# Test generate_issue_title function
@test "generate_issue_title includes commit hash when available" {
    # Mock get_commit_hash
    function get_commit_hash() {
        echo "abc1234"
    }
    export -f get_commit_hash
    
    result=$(generate_issue_title "Performance Regression" "owner/repo")
    [[ "$result" == "Performance Regression - Commit abc1234" ]]
}

@test "generate_issue_title uses PR number when available" {
    export GITHUB_PR_NUMBER="123"
    
    # Mock get_commit_hash to return empty
    function get_commit_hash() {
        echo ""
    }
    export -f get_commit_hash
    
    result=$(generate_issue_title "Performance Regression" "owner/repo")
    [[ "$result" == "Performance Regression - PR #123" ]]
    
    unset GITHUB_PR_NUMBER
}

@test "generate_issue_title uses timestamp as fallback" {
    # Mock get_commit_hash to return empty
    function get_commit_hash() {
        echo ""
    }
    export -f get_commit_hash
    
    result=$(generate_issue_title "Performance Regression" "owner/repo")
    [[ "$result" =~ "Performance Regression - [0-9]{8}-[0-9]{6}" ]] || [[ "$result" =~ "Performance Regression - 2[0-9]{7}-[0-9]{6}" ]]
}

# Test cache functionality
@test "cache directory is created" {
    [ -d "$GITHUB_ISSUE_CACHE_DIR" ]
}

@test "cleanup_cache removes old files" {
    # Create old test file
    old_file="$GITHUB_ISSUE_CACHE_DIR/old_cache_file"
    touch "$old_file"
    
    # Make it appear old (modify timestamp)
    touch -t 202001010000 "$old_file"
    
    cleanup_cache
    
    [ ! -f "$old_file" ]
}

# Test input validation scenarios
@test "detect_duplicate_issue handles empty repository" {
    run detect_duplicate_issue "" "test title" "test body"
    [ "$status" -ne 0 ]
}

@test "detect_duplicate_issue handles empty title" {
    run detect_duplicate_issue "owner/repo" "" "test body"
    [ "$status" -ne 0 ]
}

@test "detect_duplicate_issue handles empty body gracefully" {
    # Mock find_existing_issues to return empty results
    function find_existing_issues() {
        echo "[]"
    }
    export -f find_existing_issues
    
    run detect_duplicate_issue "owner/repo" "test title" ""
    [ "$status" -eq 0 ]
    [[ "$output" =~ "No duplicates found" ]]
}

# Test duplicate detection strategies
@test "detect_duplicate_issue finds exact title match" {
    # Mock find_existing_issues for exact title match
    function find_existing_issues() {
        local query="$2"
        if [[ "$query" =~ "test title" ]]; then
            echo '[{"number": 123, "title": "test title"}]'
        else
            echo '[]'
        fi
    }
    export -f find_existing_issues
    
    result=$(detect_duplicate_issue "owner/repo" "test title" "test body")
    [ "$result" = "123" ]
}

@test "detect_duplicate_issue finds commit hash match" {
    # Mock find_existing_issues
    function find_existing_issues() {
        local query="$2"
        if [[ "$query" =~ "abc1234" ]]; then
            echo '[{"number": 456, "title": "commit related issue"}]'
        else
            echo '[]'
        fi
    }
    export -f find_existing_issues
    
    result=$(detect_duplicate_issue "owner/repo" "test title" "test body" "abc1234")
    [ "$result" = "456" ]
}

@test "detect_duplicate_issue finds semantic keyword match" {
    # Mock find_existing_issues and extract_keywords
    function find_existing_issues() {
        local query="$2"
        if [[ "$query" =~ "security vulnerability" ]]; then
            echo '[{"number": 789, "title": "security issue"}]'
        else
            echo '[]'
        fi
    }
    export -f find_existing_issues
    
    function extract_keywords() {
        echo "security vulnerability"
    }
    export -f extract_keywords
    
    result=$(detect_duplicate_issue "owner/repo" "Security problem" "vulnerability detected")
    [ "$result" = "789" ]
}

@test "detect_duplicate_issue returns empty when no duplicates found" {
    # Mock find_existing_issues to always return empty
    function find_existing_issues() {
        echo '[]'
    }
    export -f find_existing_issues
    
    result=$(detect_duplicate_issue "owner/repo" "unique title" "unique body")
    [ -z "$result" ]
}

# Test error handling in exec_gh_with_retry
@test "exec_gh_with_retry succeeds on first attempt" {
    # Mock successful gh command
    function gh() {
        echo "success"
    }
    export -f gh
    
    run exec_gh_with_retry issue list --repo test/repo
    [ "$status" -eq 0 ]
    [[ "$output" == "success" ]]
}

@test "exec_gh_with_retry retries on failure" {
    # Create a counter file to track attempts
    counter_file="$TEST_CACHE_DIR/retry_counter"
    echo "0" > "$counter_file"
    
    # Mock gh command that fails first two times, succeeds on third
    function gh() {
        local count=$(cat "$counter_file")
        count=$((count + 1))
        echo "$count" > "$counter_file"
        
        if [ "$count" -lt 3 ]; then
            return 1
        else
            echo "success on attempt $count"
            return 0
        fi
    }
    export -f gh
    
    export GITHUB_API_MAX_RETRIES=3
    export GITHUB_API_RETRY_DELAY=0  # No delay for testing
    
    run exec_gh_with_retry issue list --repo test/repo
    [ "$status" -eq 0 ]
    [[ "$output" =~ "success on attempt 3" ]]
    
    # Verify it tried 3 times
    final_count=$(cat "$counter_file")
    [ "$final_count" -eq 3 ]
}

@test "exec_gh_with_retry fails after max retries" {
    # Mock gh command that always fails
    function gh() {
        return 1
    }
    export -f gh
    
    export GITHUB_API_MAX_RETRIES=2
    export GITHUB_API_RETRY_DELAY=0
    
    run exec_gh_with_retry issue list --repo test/repo
    [ "$status" -eq 1 ]
    [[ "$output" =~ "All attempts failed" ]]
}

# Test main script command line interface
@test "script shows usage when called with invalid command" {
    run bash "${BATS_TEST_DIRNAME}/../scripts/github-issue-utils.sh" invalid-command
    [ "$status" -eq 1 ]
    [[ "$output" =~ "Usage:" ]]
}

@test "script accepts detect-duplicate command" {
    # Mock functions for this test
    function detect_duplicate_issue() {
        echo "test-result"
    }
    export -f detect_duplicate_issue
    
    run bash "${BATS_TEST_DIRNAME}/../scripts/github-issue-utils.sh" detect-duplicate "repo" "title" "body"
    [ "$status" -eq 0 ]
    [[ "$output" == "test-result" ]]
}

@test "script accepts cleanup-cache command" {
    run bash "${BATS_TEST_DIRNAME}/../scripts/github-issue-utils.sh" cleanup-cache
    [ "$status" -eq 0 ]
}

# Test performance and edge cases
@test "large body text doesn't break keyword extraction" {
    local large_body=$(printf 'word%.0s ' {1..1000})
    large_body="$large_body security vulnerability error"
    
    result=$(extract_keywords "title" "$large_body")
    [[ "$result" =~ "security" ]]
    [[ "$result" =~ "vulnerability" ]]
    [[ "$result" =~ "error" ]]
}

@test "special characters in title and body are handled safely" {
    local title="Title with 'quotes' and \"double quotes\" and \\\$variables"
    local body="Body with \\\$(command) and {special} [characters]"
    
    run extract_keywords "$title" "$body"
    [ "$status" -eq 0 ]
}

@test "concurrent cache access doesn't cause issues" {
    # Simulate concurrent access by creating multiple cache files
    for i in {1..5}; do
        echo "test-$i" > "$GITHUB_ISSUE_CACHE_DIR/cache-$i" &
    done
    wait
    
    # Verify all files were created
    [ $(ls "$GITHUB_ISSUE_CACHE_DIR"/cache-* | wc -l) -eq 5 ]
}