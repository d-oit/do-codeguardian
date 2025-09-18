//! Command Security Utilities
//!
//! This module provides secure command execution with input validation
//! and sanitization to prevent command injection attacks.

use anyhow::{anyhow, Result};
use regex::Regex;
use std::path::Path;

/// Validate and sanitize git branch names
pub fn validate_git_branch_name(branch_name: &str) -> Result<()> {
    // Git branch name rules:
    // - Cannot start with . or -
    // - Cannot contain .. or @{
    // - Cannot contain control characters
    // - Cannot end with .lock
    // - Cannot contain spaces or special shell characters

    if branch_name.is_empty() {
        return Err(anyhow!("Branch name cannot be empty"));
    }

    if branch_name.len() > 250 {
        return Err(anyhow!("Branch name too long (max 250 characters)"));
    }

    // Check for dangerous characters that could be used for injection
    let dangerous_chars = &[
        '`', '$', '(', ')', ';', '|', '&', '<', '>', ' ', '\t', '\n', '\r',
    ];
    if branch_name.chars().any(|c| dangerous_chars.contains(&c)) {
        return Err(anyhow!("Branch name contains invalid characters"));
    }

    // Check git-specific rules
    if branch_name.starts_with('.') || branch_name.starts_with('-') {
        return Err(anyhow!("Branch name cannot start with . or -"));
    }

    if branch_name.contains("..") || branch_name.contains("@{") {
        return Err(anyhow!("Branch name cannot contain .. or @{{"));
    }

    if branch_name.ends_with(".lock") {
        return Err(anyhow!("Branch name cannot end with .lock"));
    }

    // Only allow alphanumeric, hyphens, underscores, and forward slashes
    let valid_pattern = Regex::new(r"^[a-zA-Z0-9/_-]+$").unwrap();
    if !valid_pattern.is_match(branch_name) {
        return Err(anyhow!("Branch name contains invalid characters"));
    }

    Ok(())
}

/// Validate file paths for git operations
pub fn validate_file_path(file_path: &str) -> Result<()> {
    if file_path.is_empty() {
        return Err(anyhow!("File path cannot be empty"));
    }

    if file_path.len() > 4096 {
        return Err(anyhow!("File path too long"));
    }

    // Check for dangerous characters
    let dangerous_chars = &['`', '$', ';', '|', '&', '<', '>'];
    if file_path.chars().any(|c| dangerous_chars.contains(&c)) {
        return Err(anyhow!("File path contains dangerous characters"));
    }

    // Prevent directory traversal
    if file_path.contains("..") {
        return Err(anyhow!("File path cannot contain .."));
    }

    // Ensure the path is relative and doesn't try to escape the project
    let path = Path::new(file_path);
    if path.is_absolute() {
        return Err(anyhow!("File path must be relative"));
    }

    Ok(())
}

/// Validate git diff specifications
pub fn validate_git_diff_spec(diff_spec: &str) -> Result<()> {
    if diff_spec.is_empty() {
        return Err(anyhow!("Diff spec cannot be empty"));
    }

    if diff_spec.len() > 100 {
        return Err(anyhow!("Diff spec too long"));
    }

    // Check for command injection characters
    let dangerous_chars = &['`', '$', '(', ')', ';', '|', '&', '<', '>'];
    if diff_spec.chars().any(|c| dangerous_chars.contains(&c)) {
        return Err(anyhow!("Diff spec contains dangerous characters"));
    }

    // Allow only safe git diff specs: commit hashes, branch names, HEAD~N, etc.
    let valid_patterns = vec![
        Regex::new(r"^[a-fA-F0-9]{7,40}$").unwrap(), // Commit hash
        Regex::new(r"^HEAD~\d+$").unwrap(),          // HEAD~N
        Regex::new(r"^HEAD\^\d*$").unwrap(),         // HEAD^N
        Regex::new(r"^[a-zA-Z0-9/_-]+$").unwrap(),   // Branch name
        Regex::new(r"^[a-fA-F0-9]{7,40}\.\.[a-fA-F0-9]{7,40}$").unwrap(), // Range with dots
        Regex::new(r"^[a-zA-Z0-9/_-]+\.\.[a-zA-Z0-9/_-]+$").unwrap(), // Branch range
    ];

    if !valid_patterns
        .iter()
        .any(|pattern| pattern.is_match(diff_spec))
    {
        return Err(anyhow!("Invalid diff specification format"));
    }

    Ok(())
}

/// Validate GitHub CLI arguments
pub fn validate_gh_args(args: &[&str]) -> Result<()> {
    for arg in args {
        if arg.is_empty() {
            return Err(anyhow!("GitHub CLI argument cannot be empty"));
        }

        if arg.len() > 1000 {
            return Err(anyhow!("GitHub CLI argument too long"));
        }

        // Check for command injection characters
        let dangerous_chars = &['`', '$', ';', '|', '&', '<', '>'];
        if arg.chars().any(|c| dangerous_chars.contains(&c)) {
            return Err(anyhow!("GitHub CLI argument contains dangerous characters"));
        }

        // Prevent arguments that start with - unless they're known safe flags
        if arg.starts_with('-') && !is_safe_gh_flag(arg) {
            return Err(anyhow!("Potentially unsafe GitHub CLI flag: {}", arg));
        }
    }

    Ok(())
}

/// Check if a GitHub CLI flag is known to be safe
fn is_safe_gh_flag(flag: &str) -> bool {
    let safe_flags = &[
        "--help",
        "-h",
        "--version",
        "--json",
        "--template",
        "--limit",
        "--state",
        "--assignee",
        "--author",
        "--milestone",
        "--label",
        "--repo",
        "-R",
        "--output",
        "-o",
        "--title",
        "-t",
        "--body",
        "-b",
    ];

    safe_flags.contains(&flag) || flag.starts_with("--limit=") || flag.starts_with("--repo=")
}

/// Sanitize commit messages
pub fn sanitize_commit_message(message: &str) -> Result<String> {
    if message.is_empty() {
        return Err(anyhow!("Commit message cannot be empty"));
    }

    if message.len() > 500 {
        return Err(anyhow!("Commit message too long (max 500 characters)"));
    }

    // Remove dangerous characters but allow normal punctuation
    let dangerous_chars = &['`', '$', ';', '|', '&', '<', '>'];
    if message.chars().any(|c| dangerous_chars.contains(&c)) {
        return Err(anyhow!("Commit message contains dangerous characters"));
    }

    // Trim and normalize whitespace
    let sanitized = message.trim().to_string();

    if sanitized.is_empty() {
        return Err(anyhow!("Commit message cannot be empty after sanitization"));
    }

    Ok(sanitized)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_git_branch_name() {
        assert!(validate_git_branch_name("feature/fix-bug").is_ok());
        assert!(validate_git_branch_name("main").is_ok());
        assert!(validate_git_branch_name("user/feature-123").is_ok());

        assert!(validate_git_branch_name("").is_err());
        assert!(validate_git_branch_name("branch with spaces").is_err());
        assert!(validate_git_branch_name("branch;rm -rf /").is_err());
        assert!(validate_git_branch_name("branch`ls`").is_err());
        assert!(validate_git_branch_name(".dotfile").is_err());
        assert!(validate_git_branch_name("-hyphen").is_err());
        assert!(validate_git_branch_name("branch..master").is_err());
    }

    #[test]
    fn test_validate_file_path() {
        assert!(validate_file_path("src/main.rs").is_ok());
        assert!(validate_file_path("docs/README.md").is_ok());

        assert!(validate_file_path("").is_err());
        assert!(validate_file_path("../etc/passwd").is_err());
        assert!(validate_file_path("/etc/passwd").is_err());
        assert!(validate_file_path("file;rm -rf /").is_err());
        assert!(validate_file_path("file`ls`").is_err());
    }

    #[test]
    fn test_validate_git_diff_spec() {
        assert!(validate_git_diff_spec("HEAD~1").is_ok());
        assert!(validate_git_diff_spec("abc123def").is_ok());
        assert!(validate_git_diff_spec("main").is_ok());
        assert!(validate_git_diff_spec("abc123..def456").is_ok());

        assert!(validate_git_diff_spec("").is_err());
        assert!(validate_git_diff_spec("HEAD;rm -rf /").is_err());
        assert!(validate_git_diff_spec("HEAD`ls`").is_err());
    }

    #[test]
    fn test_sanitize_commit_message() {
        assert_eq!(
            sanitize_commit_message("Fix bug in parser").unwrap(),
            "Fix bug in parser"
        );
        assert_eq!(sanitize_commit_message("  Fix bug  ").unwrap(), "Fix bug");

        assert!(sanitize_commit_message("").is_err());
        assert!(sanitize_commit_message("Fix`ls`").is_err());
        assert!(sanitize_commit_message("Fix;rm -rf /").is_err());
    }
}
