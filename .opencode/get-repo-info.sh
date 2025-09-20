#!/bin/bash

# Script to get repository information dynamically
# This prevents hardcoded URLs and ensures agents get correct repository info

# Cache the repo URL to avoid multiple git calls
REPO_URL=""
GITHUB_URL=""
ORG_REPO=""

get_repo_url() {
    if [ -z "$REPO_URL" ]; then
        REPO_URL=$(git remote get-url origin 2>/dev/null || echo "https://github.com/d-oit/do-codeguardian")
    fi
    echo "$REPO_URL"
}

get_org_repo() {
    if [ -z "$ORG_REPO" ]; then
        local url=$(get_repo_url)
        # Handle both HTTPS and SSH formats
        if [[ $url == https://github.com/* ]]; then
            ORG_REPO=$(echo "$url" | sed 's|https://github.com/||' | sed 's|\.git$||')
        elif [[ $url == git@github.com:* ]]; then
            ORG_REPO=$(echo "$url" | sed 's|git@github.com:||' | sed 's|\.git$||')
        else
            ORG_REPO="d-oit/do-codeguardian"  # fallback
        fi
    fi
    echo "$ORG_REPO"
}

get_repo_name() {
    basename "$(get_repo_url)" .git
}

get_github_url() {
    if [ -z "$GITHUB_URL" ]; then
        local url=$(get_repo_url)
        if [[ $url == https://github.com/* ]]; then
            GITHUB_URL=$url
        else
            GITHUB_URL=$(echo "$url" | sed 's|git@github.com:|https://github.com/|' | sed 's|\.git$||')
        fi
    fi
    echo "$GITHUB_URL"
}

get_issues_url() {
    echo "$(get_github_url)/issues"
}

get_docs_url() {
    echo "$(get_github_url)/blob/main/docs/"
}

get_ci_badge_url() {
    echo "$(get_github_url)/workflows/CI/badge.svg"
}

get_actions_url() {
    echo "$(get_github_url)/actions"
}

get_codecov_badge_url() {
    local org_repo=$(get_org_repo)
    echo "https://codecov.io/gh/$org_repo/branch/main/graph/badge.svg"
}

get_downloads_badge_url() {
    local org_repo=$(get_org_repo)
    echo "https://img.shields.io/github/downloads/$org_repo/total.svg"
}

get_contributors_badge_url() {
    local org_repo=$(get_org_repo)
    echo "https://img.shields.io/github/contributors/$org_repo.svg"
}

get_last_commit_badge_url() {
    local org_repo=$(get_org_repo)
    echo "https://img.shields.io/github/last-commit/$org_repo.svg"
}

# Export functions for use by agents
case "$1" in
    "url")
        get_repo_url
        ;;
    "name")
        get_repo_name
        ;;
    "github")
        get_github_url
        ;;
    "issues")
        get_issues_url
        ;;
    "docs")
        get_docs_url
        ;;
    "ci-badge")
        get_ci_badge_url
        ;;
    "actions")
        get_actions_url
        ;;
    "codecov-badge")
        get_codecov_badge_url
        ;;
    "downloads-badge")
        get_downloads_badge_url
        ;;
    "contributors-badge")
        get_contributors_badge_url
        ;;
    "last-commit-badge")
        get_last_commit_badge_url
        ;;
    "all")
        echo "Repository URL: $(get_repo_url)"
        echo "Repository Name: $(get_repo_name)"
        echo "GitHub URL: $(get_github_url)"
        echo "Issues URL: $(get_issues_url)"
        echo "Documentation URL: $(get_docs_url)"
        echo "CI Badge URL: $(get_ci_badge_url)"
        echo "Actions URL: $(get_actions_url)"
        echo "Codecov Badge URL: $(get_codecov_badge_url)"
        echo "Downloads Badge URL: $(get_downloads_badge_url)"
        echo "Contributors Badge URL: $(get_contributors_badge_url)"
        echo "Last Commit Badge URL: $(get_last_commit_badge_url)"
        ;;
    *)
        echo "Usage: $0 {url|name|github|issues|docs|ci-badge|actions|codecov-badge|downloads-badge|contributors-badge|last-commit-badge|all}"
        echo "Example: $0 github  # Returns GitHub URL"
        ;;
esac
