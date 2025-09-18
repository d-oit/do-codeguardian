//! # Git Commit Push Command
//!
//! This module implements the enhanced git-commit-push functionality with
//! security analysis, intelligent commit message generation, quality validation,
//! and automatic push to remote repository.
//!
//! ## Features
//!
//! - Analyzes staged changes for security issues
//! - Generates intelligent commit messages based on changes
//! - Validates code quality before committing
//! - Supports amend commits and skipping hooks
//! - Automatically pushes to remote after successful commit
//! - Integrates with GitHub for additional context

use std::path::PathBuf;

use git2::{Repository, Status};
use tracing::{debug, info, warn};

use crate::cli::GitCommitPushArgs;
use crate::config::Config;
use crate::core::GuardianEngine;
use crate::error::{CodeGuardianError, Result};
use crate::types::{AnalysisResults, Severity};
use crate::utils::progress::ProgressReporter;

/// Execute the git-commit-push command
///
/// This function performs a comprehensive git commit and push operation that includes:
/// 1. Security analysis of staged changes
/// 2. Intelligent commit message generation (if not provided)
/// 3. Quality validation
/// 4. Actual git commit execution
/// 5. Push to remote repository
///
/// # Arguments
///
/// * `args` - Command line arguments for the operation
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
/// - No staged changes are present (unless amending)
/// - Security analysis fails
/// - Commit message generation fails
/// - Git commit operation fails
/// - Push operation fails
pub async fn execute_git_commit_push(args: GitCommitPushArgs, config: &Config) -> Result<()> {
    info!("Starting enhanced git commit and push process");

    // Open git repository
    let repo = Repository::open(".").map_err(|_| {
        CodeGuardianError::generic("Failed to open git repository. Are you in a git repository?")
    })?;

    // Check for staged changes (unless amending)
    if !args.amend {
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

        // Perform comprehensive analysis on staged files
        info!("Performing security analysis on staged changes");
        let progress = ProgressReporter::new(false);
        let mut engine = GuardianEngine::new(config.clone(), progress).await?;
        let analysis_results = engine.analyze_files(&staged_files, 1).await?;

        if analysis_results.has_issues() {
            warn!("Issues found in staged changes:");
            for finding in &analysis_results.findings {
                warn!("  - {}: {}", finding.severity, finding.message);
            }

            if config.security.fail_on_issues && analysis_results.has_high_severity_issues() {
                return Err(CodeGuardianError::SecurityIssuesFound(
                    analysis_results.findings.len(),
                ));
            }
        }

        // Generate commit message if not provided
        let commit_message = if let Some(ref msg) = args.message {
            msg.clone()
        } else {
            generate_commit_message(&staged_files, &analysis_results, config).await?
        };

        debug!("Using commit message: {}", commit_message);

        // Perform the actual git commit
        perform_git_commit(&repo, &commit_message, args.amend, args.no_verify)?;
    } else {
        // For amend, use existing message or provided one
        let commit_message = args.message.unwrap_or_else(|| {
            // Get the last commit message
            if let Ok(head) = repo.head() {
                if let Some(target) = head.target() {
                    if let Ok(commit) = repo.find_commit(target) {
                        if let Some(msg) = commit.message() {
                            return msg.to_string();
                        }
                    }
                }
            }
            "amend".to_string()
        });

        debug!("Amending with commit message: {}", commit_message);

        // Perform amend commit
        perform_git_commit(&repo, &commit_message, true, args.no_verify)?;
    }

    // Push to remote if requested
    if args.push {
        info!("Pushing changes to remote repository");
        perform_git_push(&repo)?;
    }

    info!("Git commit and push completed successfully");
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
    analysis: &AnalysisResults,
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
    let commit_type = if analysis
        .findings
        .iter()
        .any(|f| matches!(f.severity, Severity::High | Severity::Critical))
    {
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
/// * `amend` - Whether to amend the last commit
/// * `no_verify` - Whether to skip pre-commit hooks
///
/// # Returns
///
/// Returns a `Result` indicating success or failure
fn perform_git_commit(
    repo: &Repository,
    message: &str,
    amend: bool,
    _no_verify: bool,
) -> Result<()> {
    // Get the current index
    let mut index = repo.index()?;
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;

    // Get the current HEAD commit (for amend or new commit)
    let head = repo.head()?;
    let target = head
        .target()
        .ok_or_else(|| CodeGuardianError::generic("HEAD has no target (unborn repository)"))?;
    let parent_commit = repo.find_commit(target)?;

    // Create the commit
    let signature = repo.signature()?;
    if amend {
        // For amend, include all parents of the current HEAD
        let commit = repo.find_commit(target)?;
        let parent_commits: Vec<git2::Commit> = commit.parents().collect();
        let parent_refs: Vec<&git2::Commit> = parent_commits.iter().collect();
        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            message,
            &tree,
            &parent_refs,
        )?;
    } else {
        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            message,
            &tree,
            &[&parent_commit],
        )?;
    };

    Ok(())
}

/// Perform git push to remote repository
///
/// # Arguments
///
/// * `repo` - The git repository
///
/// # Returns
///
/// Returns a `Result` indicating success or failure
fn perform_git_push(repo: &Repository) -> Result<()> {
    // Get the current branch
    let head = repo.head()?;
    let branch_name = head
        .name()
        .and_then(|name| name.strip_prefix("refs/heads/"))
        .ok_or_else(|| CodeGuardianError::generic("Unable to determine current branch"))?;

    // Get the remote
    let mut remote = repo.find_remote("origin").map_err(|_| {
        CodeGuardianError::generic("No remote 'origin' found. Please add a remote repository.")
    })?;

    // Push the current branch
    let refspec = format!("refs/heads/{}:refs/heads/{}", branch_name, branch_name);
    remote.push(&[&refspec], None)?;

    Ok(())
}
