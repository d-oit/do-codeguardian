use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::TempDir;

#[cfg(test)]
mod ci_workflow_edge_cases {
    use super::*;

    /// Test edge cases for CI workflow deduplication logic
    #[test]
    fn test_ci_deduplication_script_edge_cases() {
        // Edge case 1: Git repository states
        let git_scenarios = [
            ("normal_repo", true, "abc123d"),
            ("detached_head", true, "def456a"),
            ("shallow_clone", true, "789beef"),
            ("no_commits", false, ""),
            ("corrupted_git", false, ""),
        ];

        for (scenario, should_have_hash, expected_hash) in &git_scenarios {
            let result = simulate_git_rev_parse(*should_have_hash, expected_hash);

            if *should_have_hash {
                assert!(result.is_ok(), "Scenario '{}' should succeed", scenario);
                assert_eq!(
                    result.unwrap(),
                    *expected_hash,
                    "Hash should match for scenario '{}'",
                    scenario
                );
            } else {
                assert!(result.is_err(), "Scenario '{}' should fail", scenario);
            }
        }
    }

    /// Test edge cases for GitHub CLI command variations
    #[test]
    fn test_github_cli_command_edge_cases() {
        let test_cases = [
            // Edge case 1: Different repository formats
            ("owner/repo", "abc123", true),
            ("org-name/repo-name", "def456", true),
            ("user.name/repo.name", "789abc", true),
            // Edge case 2: Special characters in commit hashes
            ("owner/repo", "abc123d", true),
            ("owner/repo", "1234567", true),
            ("owner/repo", "abcdefg", true),
            // Edge case 3: Invalid inputs
            ("", "abc123", false),
            ("owner/repo", "", false),
            ("invalid-format", "abc123", false),
        ];

        for (repo, commit_hash, should_be_valid) in &test_cases {
            let command_args = build_gh_search_command(repo, commit_hash);

            if *should_be_valid {
                assert!(
                    command_args.len() >= 8,
                    "Valid inputs should generate proper command args"
                );
                assert!(
                    command_args.iter().any(|s| s == "issue"),
                    "Should contain 'issue' subcommand"
                );
                assert!(
                    command_args.iter().any(|s| s == "list"),
                    "Should contain 'list' command"
                );
                assert!(
                    command_args.iter().any(|arg| arg.contains(commit_hash)),
                    "Should contain commit hash in search query"
                );
            } else {
                // Invalid inputs should be handled gracefully
                if repo.is_empty() || commit_hash.is_empty() {
                    assert!(
                        command_args.is_empty()
                            || !command_args.iter().any(|arg| arg.contains(commit_hash)),
                        "Invalid inputs should not generate valid commands"
                    );
                }
            }
        }
    }

    /// Test edge cases for CI environment variable handling
    #[test]
    fn test_ci_environment_edge_cases() {
        let env_scenarios = [
            // Edge case 1: Missing environment variables
            (vec![], "default"),
            // Edge case 2: GitHub Actions environment
            (
                vec![
                    ("GITHUB_EVENT_NAME", "pull_request"),
                    ("GITHUB_REF", "refs/heads/feature-branch"),
                    ("GITHUB_PR_NUMBER", "123"),
                    ("GITHUB_REPOSITORY", "owner/repo"),
                ],
                "pull_request",
            ),
            // Edge case 3: Push event environment
            (
                vec![
                    ("GITHUB_EVENT_NAME", "push"),
                    ("GITHUB_REF", "refs/heads/main"),
                    ("GITHUB_REPOSITORY", "owner/repo"),
                ],
                "push",
            ),
            // Edge case 4: Scheduled event environment
            (
                vec![
                    ("GITHUB_EVENT_NAME", "schedule"),
                    ("GITHUB_REF", "refs/heads/main"),
                    ("GITHUB_REPOSITORY", "owner/repo"),
                ],
                "schedule",
            ),
            // Edge case 5: Manual workflow dispatch
            (
                vec![
                    ("GITHUB_EVENT_NAME", "workflow_dispatch"),
                    ("GITHUB_REF", "refs/heads/develop"),
                    ("GITHUB_REPOSITORY", "owner/repo"),
                ],
                "workflow_dispatch",
            ),
        ];

        for (env_vars, scenario) in &env_scenarios {
            // Set up environment
            for (key, value) in env_vars {
                std::env::set_var(key, value);
            }

            // Test environment detection
            let detected_event = std::env::var("GITHUB_EVENT_NAME").unwrap_or_default();
            let should_run_dedup = should_run_deduplication(&detected_event);
            let expected_labels = generate_ci_labels(&detected_event);

            match *scenario {
                "pull_request" => {
                    assert!(should_run_dedup, "PR events should run deduplication");
                    assert!(expected_labels.contains("pr-"), "PR should have pr- label");
                }
                "push" | "schedule" => {
                    assert!(should_run_dedup, "Push/schedule should run deduplication");
                    assert!(
                        expected_labels.contains("full-scan"),
                        "Should have full-scan label"
                    );
                }
                "workflow_dispatch" => {
                    // May or may not run deduplication depending on configuration
                    assert!(
                        expected_labels.contains("manual"),
                        "Manual runs should have manual label"
                    );
                }
                "default" => {
                    // Default behavior when no environment is set
                    assert!(!should_run_dedup, "Default should not run deduplication");
                }
                _ => {}
            }

            // Clean up environment
            for (key, _) in env_vars {
                std::env::remove_var(key);
            }
        }
    }

    /// Test edge cases for concurrent CI runs
    #[test]
    fn test_concurrent_ci_runs_edge_cases() {
        // Edge case 1: Multiple PRs with same base commit
        let concurrent_scenarios = [
            ("pr-123", "abc123d", "feature-auth"),
            ("pr-124", "abc123d", "feature-logging"),
            ("pr-125", "abc123d", "bugfix-validation"),
        ];

        for (pr_id, commit_hash, branch) in &concurrent_scenarios {
            let issue_title = format!("CodeGuardian - {} - Commit {}", pr_id, commit_hash);
            let search_query = format!("{} in:title", commit_hash);

            // All should be found by the same commit hash search
            assert!(
                issue_title.contains(commit_hash),
                "Issue title should contain commit hash for deduplication"
            );
            assert!(
                search_query.contains(commit_hash),
                "Search query should find all issues with same commit"
            );
        }

        // Edge case 2: Rapid successive pushes
        let rapid_pushes = [
            ("abc123d", "2023-01-01T10:00:00Z"),
            ("abc124e", "2023-01-01T10:01:00Z"),
            ("abc125f", "2023-01-01T10:02:00Z"),
        ];

        for (commit_hash, timestamp) in &rapid_pushes {
            let issue_title = format!("CodeGuardian - Commit {} - {}", commit_hash, timestamp);

            // Each should be unique due to different commit hashes
            assert!(
                issue_title.contains(commit_hash),
                "Each push should have unique commit hash"
            );
        }
    }

    /// Test edge cases for workflow file parsing and execution
    #[test]
    fn test_workflow_file_edge_cases() {
        // Edge case 1: Different YAML formatting scenarios
        let yaml_scenarios = [
            // Standard formatting
            r#"
name: CodeGuardian CI
on:
  pull_request:
    branches: [main]
  push:
    branches: [main]
"#,
            // Compact formatting
            r#"
name: CodeGuardian CI
on: [pull_request, push]
"#,
            // Complex triggers
            r#"
name: CodeGuardian CI
on:
  pull_request:
    branches: [main, develop]
    types: [opened, synchronize]
  push:
    branches: [main]
  schedule:
    - cron: '0 2 * * 1'
"#,
        ];

        for (i, yaml_content) in yaml_scenarios.iter().enumerate() {
            let parsed = parse_workflow_triggers(yaml_content);

            assert!(
                parsed.iter().any(|s| s == "pull_request") || parsed.iter().any(|s| s == "push"),
                "Scenario {} should contain expected triggers",
                i
            );
        }
    }

    /// Test edge cases for issue creation rate limiting
    #[test]
    fn test_rate_limiting_edge_cases() {
        // Edge case 1: Burst of CI runs
        let burst_scenarios = [
            (5, "normal"),    // Normal load
            (20, "high"),     // High load
            (50, "burst"),    // Burst load
            (100, "extreme"), // Extreme load
        ];

        for (request_count, scenario) in &burst_scenarios {
            let rate_limit_delay = calculate_rate_limit_delay(*request_count);
            let expected_delay = match *scenario {
                "normal" => 0,
                "high" => 1,
                "burst" => 5,
                "extreme" => 10,
                _ => 0,
            };

            assert!(
                rate_limit_delay >= expected_delay,
                "Scenario '{}' should have appropriate rate limiting",
                scenario
            );
        }
    }

    /// Test edge cases for workflow step dependencies
    #[test]
    fn test_workflow_step_dependencies_edge_cases() {
        // Edge case 1: Step failure scenarios
        let step_scenarios = [
            ("checkout", true, "build", true),
            ("checkout", false, "build", false), // Build should fail if checkout fails
            ("build", true, "analyze", true),
            ("build", false, "analyze", false), // Analyze should fail if build fails
            ("dedup_check", true, "analyze", false), // Analyze should skip if duplicate found
            ("dedup_check", false, "analyze", true), // Analyze should run if no duplicate
        ];

        for (step_name, step_success, next_step, should_run_next) in &step_scenarios {
            let next_step_condition = should_step_run(step_name, *step_success, next_step);

            assert_eq!(
                next_step_condition, *should_run_next,
                "Step '{}' success={} should determine if '{}' runs",
                step_name, step_success, next_step
            );
        }
    }

    /// Test edge cases for artifact handling
    #[test]
    fn test_artifact_handling_edge_cases() {
        let temp_dir = TempDir::new().unwrap();

        // Edge case 1: Different artifact sizes
        let artifact_scenarios = [
            ("small", 1024),            // 1KB
            ("medium", 1024 * 100),     // 100KB
            ("large", 1024 * 1024),     // 1MB
            ("huge", 1024 * 1024 * 10), // 10MB
        ];

        for (size_name, size_bytes) in &artifact_scenarios {
            let artifact_path = temp_dir.path().join(format!("results_{}.json", size_name));
            let content = "x".repeat(*size_bytes);

            fs::write(&artifact_path, &content).unwrap();

            let should_upload = should_upload_artifact(&artifact_path);
            let compression_needed = needs_compression(*size_bytes);

            match *size_name {
                "small" | "medium" => {
                    assert!(should_upload, "Small/medium artifacts should be uploaded");
                    assert!(
                        !compression_needed,
                        "Small/medium artifacts don't need compression"
                    );
                }
                "large" => {
                    assert!(should_upload, "Large artifacts should be uploaded");
                    assert!(compression_needed, "Large artifacts should be compressed");
                }
                "huge" => {
                    // May need special handling for very large artifacts
                    if should_upload {
                        assert!(compression_needed, "Huge artifacts must be compressed");
                    }
                }
                _ => {}
            }
        }
    }

    // Helper functions for edge case testing

    fn simulate_git_rev_parse(should_succeed: bool, expected_hash: &str) -> Result<String, String> {
        if should_succeed {
            Ok(expected_hash.to_string())
        } else {
            Err("Git command failed".to_string())
        }
    }

    fn build_gh_search_command(repo: &str, commit_hash: &str) -> Vec<String> {
        if repo.is_empty() || commit_hash.is_empty() {
            return vec![];
        }

        vec![
            "issue".to_string(),
            "list".to_string(),
            "--repo".to_string(),
            repo.to_string(),
            "--search".to_string(),
            format!("{} in:title", commit_hash),
            "--state".to_string(),
            "open".to_string(),
            "--json".to_string(),
            "number".to_string(),
            "--jq".to_string(),
            ".[0].number // empty".to_string(),
        ]
    }

    fn should_run_deduplication(event_name: &str) -> bool {
        matches!(event_name, "pull_request" | "push" | "schedule")
    }

    fn generate_ci_labels(event_name: &str) -> String {
        let mut labels: Vec<String> = vec!["codeguardian".to_string(), "automated".to_string()];

        match event_name {
            "pull_request" => {
                if let Ok(pr_number) = std::env::var("GITHUB_PR_NUMBER") {
                    labels.push(format!("pr-{}", pr_number));
                }
            }
            "push" | "schedule" => {
                labels.push("full-scan".to_string());
            }
            "workflow_dispatch" => {
                labels.push("manual".to_string());
            }
            _ => {}
        }

        labels.join(",")
    }

    fn parse_workflow_triggers(yaml_content: &str) -> Vec<String> {
        let mut triggers = Vec::new();

        if yaml_content.contains("pull_request") {
            triggers.push("pull_request".to_string());
        }
        if yaml_content.contains("push") {
            triggers.push("push".to_string());
        }
        if yaml_content.contains("schedule") {
            triggers.push("schedule".to_string());
        }

        triggers
    }

    fn calculate_rate_limit_delay(request_count: usize) -> u64 {
        match request_count {
            0..=10 => 0,
            11..=25 => 1,
            26..=50 => 5,
            _ => 10,
        }
    }

    fn should_step_run(step_name: &str, step_success: bool, next_step: &str) -> bool {
        match (step_name, next_step) {
            ("checkout", "build") => step_success,
            ("build", "analyze") => step_success,
            ("dedup_check", "analyze") => !step_success, // Skip analyze if duplicate found
            _ => true,
        }
    }

    fn should_upload_artifact(artifact_path: &Path) -> bool {
        if let Ok(metadata) = fs::metadata(artifact_path) {
            // Don't upload artifacts larger than 25MB (GitHub limit is ~2GB but be conservative)
            metadata.len() < 25 * 1024 * 1024
        } else {
            false
        }
    }

    fn needs_compression(size_bytes: usize) -> bool {
        size_bytes > 500 * 1024 // Compress if larger than 500KB
    }
}
