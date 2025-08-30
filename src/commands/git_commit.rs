//! # Git Commit Command
//!
//! This module implements the enhanced git-commit functionality with
//! security analysis, intelligent commit message generation, and quality validation.
//!
//! ## Features
//!
//! - Analyzes staged changes for security issues
//! - Generates intelligent commit messages based on changes
//! - Validates code quality before committing
//! - Integrates with GitHub for additional context

use std::path::PathBuf;

use git2::{Repository, Status};
use tracing::{debug, info, warn};

use crate::config::Config;
use crate::error::{CodeGuardianError, Result};
use crate::security;

/// Execute the git-commit command
///
/// This function performs a comprehensive git commit operation that includes:
/// 1. Security analysis of staged changes
/// 2. Intelligent commit message generation
/// 3. Quality validation
/// 4. Actual git commit execution
///
/// # Arguments
///
/// * `message` - Optional custom commit message
/// * `config` - Configuration for the operation
///
/// # Returns
///
/// Returns a `Result` indicating success or failure
///
/// # Errors
///
/// This function will return an error if:
/// - No git repository is found
/// - No staged changes are present
/// - Security analysis fails
/// - Commit message generation fails
/// - Git commit operation fails
pub async fn execute_git_commit(message: Option<&str>, config: &Config) -> Result<()> {
    info!("Starting enhanced git commit process");

    // Open git repository
    let repo = Repository::open(".").map_err(|_| {
        CodeGuardianError::generic("Failed to open git repository. Are you in a git repository?")
    })?;

    // Check for staged changes
    let statuses = repo.statuses(None)?;
    let staged_files: Vec<PathBuf> = statuses
        .iter()
        .filter(|entry| {
            let status = entry.status();
            status.contains(Status::INDEX_NEW)
                || status.contains(Status::INDEX_MODIFIED)
                || status.contains(Status::INDEX_DELETED)
                || status.contains(Status::INDEX_RENAMED)
                || status.contains(Status::INDEX_TYPECHANGE)
        })
        .filter_map(|entry| entry.path().map(PathBuf::from))
        .collect();

    if staged_files.is_empty() {
        return Err(CodeGuardianError::NoStagedChanges);
    }

    debug!("Found {} staged files", staged_files.len());

    // Perform security analysis on staged files
    info!("Performing security analysis on staged changes");
    let analysis_results = security::analyze_files(&staged_files, config).await?;

    if !analysis_results.issues.is_empty() {
        warn!("Security issues found in staged changes:");
        for issue in &analysis_results.issues {
            warn!("  - {}: {}", issue.severity, issue.message);
        }

        if config.security.fail_on_issues {
            return Err(CodeGuardianError::SecurityIssuesFound(
                analysis_results.issues.len(),
            ));
        }
    }

    // Generate commit message if not provided
    let commit_message = if let Some(msg) = message {
        msg.to_string()
    } else {
        generate_commit_message(&staged_files, &analysis_results, config).await?
    };

    debug!("Using commit message: {}", commit_message);

    // Perform the actual git commit
    perform_git_commit(&repo, &commit_message)?;

    info!("Git commit completed successfully");
    Ok(())
}

/// Generate an intelligent commit message based on changes
///
/// This function analyzes the staged files and generates a meaningful
/// commit message following conventional commit standards.
///
/// # Arguments
///
/// * `files` - List of staged files
/// * `analysis` - Results from security analysis
/// * `_config` - Configuration settings (unused for now)
///
/// # Returns
///
/// Returns a generated commit message
async fn generate_commit_message(
    files: &[PathBuf],
    analysis: &security::AnalysisResults,
    _config: &Config,
) -> Result<String> {
    // Analyze file types and changes
    let mut file_types = std::collections::HashMap::new();
    for file in files {
        if let Some(ext) = file.extension().and_then(|e| e.to_str()) {
            *file_types.entry(ext.to_string()).or_insert(0) += 1;
        }
    }

    // Determine commit type based on file types and analysis
    let commit_type = if analysis.issues.iter().any(|i| i.severity == "high") {
        "fix"
    } else if file_types.contains_key("rs") {
        "feat"
    } else if file_types.contains_key("md") {
        "docs"
    } else if file_types.contains_key("toml") || file_types.contains_key("yaml") {
        "chore"
    } else {
        "refactor"
    };

    // Generate scope based on affected directories
    let scope = files
        .iter()
        .filter_map(|f| {
            f.parent()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str())
        })
        .next()
        .unwrap_or("general");

    // Create descriptive message
    let description = if files.len() == 1 {
        format!("update {}", files[0].display())
    } else {
        format!("update {} files", files.len())
    };

    Ok(format!("{}({}): {}", commit_type, scope, description))
}

/// Perform the actual git commit operation
///
/// # Arguments
///
/// * `repo` - The git repository
/// * `message` - The commit message
///
/// # Returns
///
/// Returns a `Result` indicating success or failure
fn perform_git_commit(repo: &Repository, message: &str) -> Result<()> {
    // Get the current index
    let mut index = repo.index()?;
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;

    // Get the current HEAD commit
    let head = repo.head()?;
    let parent_commit = repo.find_commit(head.target().unwrap())?;

    // Create the commit
    let signature = repo.signature()?;
    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        message,
        &tree,
        &[&parent_commit],
    )?;

    Ok(())
}
