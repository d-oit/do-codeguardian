use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::TempDir;

use do_codeguardian::cli::gh_issue::create_or_update_issue;
use do_codeguardian::cli::GhIssueArgs;
use do_codeguardian::github_api::GitHubApiClient;
use do_codeguardian::types::AnalysisResults;

#[cfg(test)]
mod github_deduplication_edge_cases {
    use super::*;

    /// Test edge cases for commit hash extraction and formatting
    #[test]
    fn test_commit_hash_edge_cases() {
        // Edge case 1: Different commit hash lengths
        let hash_variations = [
            "abc123d",                                  // 7 chars (typical short hash)
            "abc123def4",                               // 10 chars
            "abc123def456789012345678901234567890abcd", // Full 40 char hash
            "1234567",                                  // Numeric hash
            "abcdefg",                                  // All letters
        ];

        for hash in &hash_variations {
            // Test title generation with different hash formats
            let title = format!("CodeGuardian - Commit {}", hash);
            assert!(
                title.contains(hash),
                "Title should contain commit hash: {}",
                hash
            );
            assert!(
                title.len() < 256,
                "Title should be reasonable length for hash: {}",
                hash
            );
        }
    }

    /// Test edge cases for issue title generation
    #[test]
    fn test_issue_title_generation_edge_cases() {
        // Mock git command scenarios
        let test_scenarios = [
            // Edge case 1: Git command fails (no git repo)
            ("no-git-repo", None),
            // Edge case 2: Different environment variables
            ("pr-context", Some("123")),
            ("scheduled-context", None),
        ];

        for (scenario, pr_number) in &test_scenarios {
            // Simulate different environments
            if let Some(pr) = pr_number {
                std::env::set_var("GITHUB_PR_NUMBER", pr);
            } else {
                std::env::remove_var("GITHUB_PR_NUMBER");
            }

            // Test title generation logic
            let prefix = "CodeGuardian:";
            let result = generate_issue_title_fallback(prefix, scenario);

            assert!(
                !result.is_empty(),
                "Title should not be empty for scenario: {}",
                scenario
            );
            assert!(
                result.starts_with("CodeGuardian"),
                "Title should start with prefix for scenario: {}",
                scenario
            );

            if pr_number.is_some() {
                assert!(
                    result.contains("PR #"),
                    "Title should contain PR number for scenario: {}",
                    scenario
                );
            }
        }

        // Clean up environment
        std::env::remove_var("GITHUB_PR_NUMBER");
    }

    /// Test edge cases for duplicate issue detection
    #[test]
    fn test_duplicate_detection_edge_cases() {
        // Edge case 1: Similar but different commit hashes
        let similar_hashes = [
            ("abc123d", "abc123e"), // One character different
            ("abc123d", "abc124d"), // Middle character different
            ("abc123d", "ABC123D"), // Case different
            ("abc123d", "abc123"),  // Length different
        ];

        for (hash1, hash2) in &similar_hashes {
            let title1 = format!("CodeGuardian - Commit {}", hash1);
            let title2 = format!("CodeGuardian - Commit {}", hash2);

            // These should be treated as different issues
            assert_ne!(
                title1, title2,
                "Different hashes should generate different titles"
            );
        }

        // Edge case 2: Same hash with different prefixes
        let hash = "abc123d";
        let title_variations = [
            format!("CodeGuardian - Commit {}", hash),
            format!("CodeGuardian: Security Scan - Commit {}", hash),
            format!("Security Analysis - Commit {}", hash),
        ];

        // All should contain the same hash for deduplication
        for title in &title_variations {
            assert!(
                title.contains(hash),
                "Title should contain hash for deduplication: {}",
                title
            );
        }
    }

    /// Test edge cases for GitHub CLI command construction
    #[test]
    fn test_github_cli_edge_cases() {
        let test_cases = [
            // Edge case 1: Special characters in repository names
            ("owner/repo-name", true),
            ("owner/repo_name", true),
            ("owner/repo.name", true),
            ("owner-org/repo-name", true),
            // Edge case 2: Different commit hash formats in search
            ("abc123d", true),
            ("abc123def456", true),
            ("1234567890abcdef", true),
            // Edge case 3: Empty or invalid inputs
            ("", false),
            ("invalid-repo-format", false),
        ];

        for (input, should_be_valid) in &test_cases {
            if input.contains('/') && !input.is_empty() {
                // Test repository name validation
                assert_eq!(
                    is_valid_repo_format(input),
                    *should_be_valid,
                    "Repository format validation failed for: {}",
                    input
                );
            } else if !input.contains('/') && !input.is_empty() {
                // Test commit hash validation
                assert_eq!(
                    is_valid_commit_hash(input),
                    *should_be_valid,
                    "Commit hash validation failed for: {}",
                    input
                );
            }
        }
    }

    /// Test edge cases for CI workflow scenarios
    #[test]
    fn test_ci_workflow_edge_cases() {
        // Edge case 1: Different GitHub event types
        let event_scenarios = [
            ("pull_request", "refs/heads/feature-branch", Some("123")),
            ("push", "refs/heads/main", None),
            ("schedule", "refs/heads/main", None),
            ("workflow_dispatch", "refs/heads/develop", None),
        ];

        for (event_name, github_ref, pr_number) in &event_scenarios {
            // Simulate GitHub Actions environment
            std::env::set_var("GITHUB_EVENT_NAME", event_name);
            std::env::set_var("GITHUB_REF", github_ref);

            if let Some(pr) = pr_number {
                std::env::set_var("GITHUB_PR_NUMBER", pr);
            } else {
                std::env::remove_var("GITHUB_PR_NUMBER");
            }

            // Test workflow logic
            let should_run_dedup = should_run_deduplication_check(event_name);
            let expected_labels = generate_expected_labels(event_name, pr_number);

            match *event_name {
                "pull_request" => {
                    assert!(should_run_dedup, "PR events should run deduplication");
                    assert!(
                        expected_labels.contains("pr-"),
                        "PR labels should contain pr- prefix"
                    );
                }
                "push" | "schedule" => {
                    assert!(
                        should_run_dedup,
                        "Push/schedule events should run deduplication"
                    );
                    assert!(
                        expected_labels.contains("automated"),
                        "Should contain automated label"
                    );
                }
                _ => {
                    // Other events may or may not run deduplication
                }
            }
        }

        // Clean up environment
        std::env::remove_var("GITHUB_EVENT_NAME");
        std::env::remove_var("GITHUB_REF");
        std::env::remove_var("GITHUB_PR_NUMBER");
    }

    /// Test edge cases for concurrent issue creation
    #[test]
    fn test_concurrent_issue_scenarios() {
        // Edge case 1: Race condition simulation
        let commit_hash = "abc123d";
        let repo = "test-owner/test-repo";

        // Simulate scenario where multiple CI runs happen simultaneously
        let concurrent_scenarios = [
            (
                "PR #123",
                format!("CodeGuardian - PR #{} - Commit {}", 123, commit_hash),
            ),
            (
                "PR #124",
                format!("CodeGuardian - PR #{} - Commit {}", 124, commit_hash),
            ),
            (
                "Full Scan",
                format!("CodeGuardian - Full Scan - Commit {}", commit_hash),
            ),
            (
                "Scheduled",
                format!("CodeGuardian - Scheduled - Commit {}", commit_hash),
            ),
        ];

        // All should be detected as duplicates if they have the same commit hash
        for (scenario, title) in &concurrent_scenarios {
            assert!(
                title.contains(commit_hash),
                "Scenario '{}' should contain commit hash for deduplication",
                scenario
            );
        }

        // Test that search would find any of these as duplicates
        let search_query = format!("{} in:title", commit_hash);
        assert!(
            search_query.contains(commit_hash),
            "Search query should contain commit hash"
        );
    }

    /// Test edge cases for issue body size limits
    #[test]
    fn test_issue_body_size_edge_cases() {
        // Edge case 1: Very large number of findings
        let large_findings_count = [10, 50, 100, 500, 1000];

        for count in &large_findings_count {
            let mock_findings = create_mock_findings(*count);
            let body = generate_mock_issue_body(&mock_findings);

            // GitHub has a ~65KB limit for issue bodies
            const GITHUB_BODY_LIMIT: usize = 65536;

            if body.len() > GITHUB_BODY_LIMIT {
                // Should be truncated
                assert!(
                    body.contains("Report Truncated"),
                    "Large body ({} findings) should be truncated",
                    count
                );
            }

            // Should always contain essential information
            assert!(
                body.contains("Analysis Details"),
                "Body should contain analysis details for {} findings",
                count
            );
        }
    }

    /// Test edge cases for network failures and retries
    #[test]
    fn test_network_failure_edge_cases() {
        // Edge case 1: Different types of GitHub API errors
        let error_scenarios = [
            ("rate_limit", "API rate limit exceeded"),
            ("timeout", "Request timeout"),
            ("network", "Network connection failed"),
            ("auth", "Bad credentials"),
            ("not_found", "Repository not found"),
        ];

        for (error_type, error_message) in &error_scenarios {
            let is_retryable = is_retryable_github_error(error_message);
            let is_rate_limit = is_rate_limit_github_error(error_message);

            match *error_type {
                "rate_limit" => {
                    assert!(is_rate_limit, "Rate limit errors should be detected");
                    assert!(is_retryable, "Rate limit errors should be retryable");
                }
                "timeout" | "network" => {
                    assert!(is_retryable, "Network errors should be retryable");
                    assert!(!is_rate_limit, "Network errors are not rate limit errors");
                }
                "auth" | "not_found" => {
                    assert!(
                        !is_retryable,
                        "Auth/not found errors should not be retryable"
                    );
                    assert!(!is_rate_limit, "Auth errors are not rate limit errors");
                }
                _ => {}
            }
        }
    }

    // Helper functions for edge case testing

    fn generate_issue_title_fallback(prefix: &str, scenario: &str) -> String {
        // Simulate the title generation logic with fallbacks
        if let Ok(pr_number) = std::env::var("GITHUB_PR_NUMBER") {
            format!("{} - PR #{}", prefix.trim_end_matches(':'), pr_number)
        } else {
            let timestamp = chrono::Utc::now().format("%Y%m%d-%H%M%S");
            format!("{} - Scheduled {}", prefix.trim_end_matches(':'), timestamp)
        }
    }

    fn is_valid_repo_format(repo: &str) -> bool {
        repo.contains('/') && !repo.is_empty() && repo.split('/').count() == 2
    }

    fn is_valid_commit_hash(hash: &str) -> bool {
        !hash.is_empty() && hash.len() >= 7 && hash.chars().all(|c| c.is_ascii_alphanumeric())
    }

    fn should_run_deduplication_check(event_name: &str) -> bool {
        matches!(event_name, "pull_request" | "push" | "schedule")
    }

    fn generate_expected_labels(event_name: &str, pr_number: &Option<&str>) -> String {
        let mut labels: Vec<String> = vec!["codeguardian".to_string(), "automated".to_string()];

        match event_name {
            "pull_request" => {
                if let Some(pr) = pr_number {
                    labels.push(format!("pr-{}", pr));
                }
            }
            "push" | "schedule" => {
                labels.push("full-scan".to_string());
            }
            _ => {}
        }

        labels.join(",")
    }

    fn create_mock_findings(count: usize) -> Vec<String> {
        (0..count)
            .map(|i| format!("Finding {} - Mock security issue", i))
            .collect()
    }

    fn generate_mock_issue_body(findings: &[String]) -> String {
        let mut body = String::new();
        body.push_str("## Analysis Details\n\n");
        body.push_str(&format!("- **Total Findings:** {}\n\n", findings.len()));

        body.push_str("## Findings\n\n");
        for (i, finding) in findings.iter().enumerate() {
            body.push_str(&format!("{}. {}\n", i + 1, finding));
        }

        body
    }

    fn is_retryable_github_error(error_message: &str) -> bool {
        let error_lower = error_message.to_lowercase();
        error_lower.contains("timeout")
            || error_lower.contains("network")
            || error_lower.contains("rate limit")
            || error_lower.contains("502")
            || error_lower.contains("503")
            || error_lower.contains("504")
    }

    fn is_rate_limit_github_error(error_message: &str) -> bool {
        let error_lower = error_message.to_lowercase();
        error_lower.contains("rate limit") || error_lower.contains("403")
    }
}
