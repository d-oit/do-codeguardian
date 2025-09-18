//! Integration tests for security fixes
//!
//! Tests to verify that command injection vulnerabilities have been properly fixed

use do_codeguardian::utils::command_security;

#[test]
fn test_command_injection_prevention() {
    // Test branch name validation prevents injection
    assert!(command_security::validate_git_branch_name("valid-branch").is_ok());
    assert!(command_security::validate_git_branch_name("branch;rm -rf /").is_err());
    assert!(command_security::validate_git_branch_name("branch`ls`").is_err());
    assert!(command_security::validate_git_branch_name("branch$(whoami)").is_err());
}

#[test]
fn test_file_path_validation_prevents_injection() {
    // Test file path validation prevents injection
    assert!(command_security::validate_file_path("src/main.rs").is_ok());
    assert!(command_security::validate_file_path("../etc/passwd").is_err());
    assert!(command_security::validate_file_path("file;rm -rf /").is_err());
    assert!(command_security::validate_file_path("file`ls`").is_err());
}

#[test]
fn test_git_diff_spec_validation() {
    // Test git diff spec validation prevents injection
    assert!(command_security::validate_git_diff_spec("HEAD~1").is_ok());
    assert!(command_security::validate_git_diff_spec("abc123def").is_ok());
    assert!(command_security::validate_git_diff_spec("main").is_ok());
    assert!(command_security::validate_git_diff_spec("HEAD;rm -rf /").is_err());
    assert!(command_security::validate_git_diff_spec("HEAD`ls`").is_err());
}

#[test]
fn test_github_cli_args_validation() {
    // Test GitHub CLI args validation prevents injection
    let safe_args = ["issue", "list", "--limit", "10"];
    assert!(command_security::validate_gh_args(&safe_args).is_ok());
    
    let dangerous_args = ["issue", "list;rm -rf /"];
    assert!(command_security::validate_gh_args(&dangerous_args).is_err());
    
    let injection_args = ["issue", "list`ls`"];
    assert!(command_security::validate_gh_args(&injection_args).is_err());
}

#[test]
fn test_commit_message_sanitization() {
    // Test commit message sanitization
    assert_eq!(
        command_security::sanitize_commit_message("Fix bug in parser").unwrap(),
        "Fix bug in parser"
    );
    assert_eq!(
        command_security::sanitize_commit_message("  Fix bug  ").unwrap(),
        "Fix bug"
    );
    
    assert!(command_security::sanitize_commit_message("Fix`ls`").is_err());
    assert!(command_security::sanitize_commit_message("Fix;rm -rf /").is_err());
    assert!(command_security::sanitize_commit_message("").is_err());
}

/// Test edge cases and boundary conditions
#[test]
fn test_security_edge_cases() {
    // Test very long inputs
    let long_branch = "a".repeat(300);
    assert!(command_security::validate_git_branch_name(&long_branch).is_err());
    
    let long_file = "a".repeat(5000);
    assert!(command_security::validate_file_path(&long_file).is_err());
    
    let long_commit = "a".repeat(600);
    assert!(command_security::sanitize_commit_message(&long_commit).is_err());
    
    // Test unicode and special characters
    assert!(command_security::validate_git_branch_name("branch-with-émojis").is_err());
    assert!(command_security::validate_git_branch_name("branch\n\r\t").is_err());
    
    // Test null bytes and control characters
    assert!(command_security::validate_git_branch_name("branch\0").is_err());
    assert!(command_security::validate_file_path("file\0").is_err());
}