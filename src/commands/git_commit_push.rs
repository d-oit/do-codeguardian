//! # Git Commit Push Command
//!
//! This module implements the enhanced git-commit-push functionality with
//! security analysis, intelligent commit message generation, quality validation,
//! conflict prevention, and flexible push options.
//!
//! ## Features
//!
//! - Analyzes staged changes for security issues (unless skipped)
//! - Generates intelligent commit messages based on changes
//! - Validates code quality before committing (unless skipped)
//! - Supports amend commits and skipping hooks
//! - Checks for potential merge conflicts before pushing (unless skipped)
//! - Supports dry-run mode for testing operations
//! - Allows custom remote and branch specification
//! - Supports force push with appropriate warnings
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
/// 1. Security analysis of staged changes (unless skipped)
/// 2. Intelligent commit message generation (if not provided)
/// 3. Quality validation (unless skipped)
/// 4. Conflict prevention checks (unless skipped)
/// 5. Actual git commit execution (unless dry-run)
/// 6. Push to remote repository (unless dry-run)
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
/// - Security analysis fails (unless skipped)
/// - Quality validation fails (unless skipped)
/// - Conflict prevention fails (unless skipped)
/// - Commit message generation fails
/// - Git commit operation fails
/// - Push operation fails
pub async fn execute_git_commit_push(args: GitCommitPushArgs, config: &Config) -> Result<()> {
    if args.dry_run {
        info!("Starting dry-run of enhanced git commit and push process");
    } else {
        info!("Starting enhanced git commit and push process");
    }

    // Open git repository
    let repo = Repository::open(".").map_err(|_| {
        CodeGuardianError::generic("Failed to open git repository. Are you in a git repository?")
    })?;

    // Determine target branch
    let target_branch = args.branch.clone().unwrap_or_else(|| {
        repo.head()
            .ok()
            .and_then(|head| head.name().map(|s| s.to_string()))
            .and_then(|name| name.strip_prefix("refs/heads/").map(|s| s.to_string()))
            .unwrap_or_else(|| "main".to_string())
    });

    debug!("Target branch: {}", target_branch);

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

        // Perform comprehensive analysis on staged files (unless skipped)
        let analysis_results = if !args.skip_security {
            info!("Performing security analysis on staged changes");
            let progress = ProgressReporter::new(false);
            let mut engine = GuardianEngine::new(config.clone(), progress).await?;
            let results = engine.analyze_files(&staged_files, 1).await?;

            if results.has_issues() {
                warn!("Issues found in staged changes:");
                for finding in &results.findings {
                    warn!("  - {}: {}", finding.severity, finding.message);
                }

                if config.security.fail_on_issues && results.has_high_severity_issues() {
                    return Err(CodeGuardianError::SecurityIssuesFound(
                        results.findings.len(),
                    ));
                }
            }
            results
        } else {
            warn!("Skipping security analysis as requested");
            AnalysisResults::new("skipped".to_string())
        };

        // Perform quality validation (unless skipped)
        if !args.skip_quality {
            info!("Performing quality validation");
            perform_quality_validation(&staged_files)?;
        } else {
            warn!("Skipping quality validation as requested");
        }

        // Generate commit message if not provided
        let commit_message = if let Some(ref msg) = args.message {
            msg.clone()
        } else {
            generate_commit_message(&staged_files, &analysis_results, config).await?
        };

        debug!("Using commit message: {}", commit_message);

        // Perform conflict prevention checks (unless skipped)
        if !args.skip_conflicts && args.push {
            info!("Checking for potential merge conflicts");
            check_for_conflicts(&repo, &args.remote, &target_branch)?;
        } else if args.skip_conflicts && args.push {
            warn!("Skipping conflict prevention checks as requested");
        }

        // Perform the actual git commit (unless dry-run)
        if !args.dry_run {
            perform_git_commit(&repo, &commit_message, args.amend, args.no_verify)?;
        } else {
            info!("Dry-run: Would commit with message: {}", commit_message);
        }
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

        // Perform amend commit (unless dry-run)
        if !args.dry_run {
            perform_git_commit(&repo, &commit_message, true, args.no_verify)?;
        } else {
            info!(
                "Dry-run: Would amend commit with message: {}",
                commit_message
            );
        }
    }

    // Push to remote if requested (unless dry-run)
    if args.push {
        if !args.dry_run {
            info!("Pushing changes to remote repository");
            perform_git_push(&repo, &args.remote, &target_branch, args.force)?;
        } else {
            info!(
                "Dry-run: Would push to remote '{}' branch '{}'",
                args.remote, target_branch
            );
        }
    }

    if args.dry_run {
        info!("Dry-run completed successfully - no changes made");
    } else {
        info!("Git commit and push completed successfully");
    }
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
    no_verify: bool,
) -> Result<()> {
    if no_verify {
        // Use command line git to skip hooks
        warn!("Skipping pre-commit hooks as requested");
        let mut cmd = std::process::Command::new("git");
        cmd.arg("commit");

        if amend {
            cmd.arg("--amend");
        }

        cmd.arg("--no-verify");
        cmd.arg("-m");
        cmd.arg(message);

        let output = cmd.output().map_err(|e| {
            CodeGuardianError::generic(format!("Failed to execute git commit: {}", e))
        })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(CodeGuardianError::generic(format!(
                "Git commit failed:\n{}",
                stderr
            )));
        }
    } else {
        // Use git2 for normal commits
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
    }

    Ok(())
}

/// Perform quality validation on staged files
///
/// This function runs code quality checks like formatting and linting
/// before allowing the commit.
///
/// # Arguments
///
/// * `files` - List of staged files to validate
///
/// # Returns
///
/// Returns a `Result` indicating success or failure
fn perform_quality_validation(files: &[PathBuf]) -> Result<()> {
    // Check if any Rust files are present
    let has_rust_files = files
        .iter()
        .any(|f| f.extension().and_then(|e| e.to_str()) == Some("rs"));

    if has_rust_files {
        // Run cargo fmt --check
        info!("Running cargo fmt check");
        let output = std::process::Command::new("cargo")
            .args(["fmt", "--check"])
            .output()
            .map_err(|e| CodeGuardianError::generic(format!("Failed to run cargo fmt: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(CodeGuardianError::generic(format!(
                "Code formatting issues found:\n{}",
                stderr
            )));
        }

        // Run cargo clippy
        info!("Running cargo clippy");
        let output = std::process::Command::new("cargo")
            .args(["clippy", "--", "-D", "warnings"])
            .output()
            .map_err(|e| {
                CodeGuardianError::generic(format!("Failed to run cargo clippy: {}", e))
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(CodeGuardianError::generic(format!(
                "Code quality issues found:\n{}",
                stderr
            )));
        }
    }

    Ok(())
}

/// Check for potential merge conflicts before pushing
///
/// This function performs checks to identify potential merge conflicts
/// that could occur when pushing to the remote branch.
///
/// # Arguments
///
/// * `repo` - The git repository
/// * `remote_name` - Name of the remote repository
/// * `branch_name` - Name of the branch to check
///
/// # Returns
///
/// Returns a `Result` indicating success or failure
fn check_for_conflicts(repo: &Repository, remote_name: &str, branch_name: &str) -> Result<()> {
    // Fetch latest changes from remote
    let mut remote = repo
        .find_remote(remote_name)
        .map_err(|_| CodeGuardianError::generic(format!("No remote '{}' found", remote_name)))?;

    // Fetch the remote branch
    remote.fetch(&[branch_name], None, None)?;

    // Get the remote branch reference
    let remote_ref_name = format!("refs/remotes/{}/{}", remote_name, branch_name);
    let remote_commit = match repo.find_reference(&remote_ref_name) {
        Ok(reference) => {
            let oid = reference
                .target()
                .ok_or_else(|| CodeGuardianError::generic("Remote reference has no target"))?;
            repo.find_commit(oid)?
        }
        Err(_) => {
            // Remote branch doesn't exist yet, no conflict possible
            debug!("Remote branch '{}' does not exist yet", branch_name);
            return Ok(());
        }
    };

    // Get the local branch commit
    let local_ref_name = format!("refs/heads/{}", branch_name);
    let local_commit = repo
        .find_reference(&local_ref_name)
        .and_then(|r| r.target().ok_or_else(|| git2::Error::from_str("No target")))
        .and_then(|oid| repo.find_commit(oid))?;

    // Check if local is ahead of remote (fast-forward possible)
    if repo.graph_descendant_of(local_commit.id(), remote_commit.id())? {
        debug!("Local branch is ahead of remote - fast-forward push possible");
        return Ok(());
    }

    // Check if remote is ahead of local (would need merge)
    if repo.graph_descendant_of(remote_commit.id(), local_commit.id())? {
        warn!("Remote branch has new commits - consider pulling first");
        return Err(CodeGuardianError::generic(
            "Remote branch has diverged. Please pull and resolve conflicts locally before pushing.",
        ));
    }

    // Branches have diverged - potential conflict
    warn!("Local and remote branches have diverged - potential merge conflict detected");
    Err(CodeGuardianError::generic(
        "Local and remote branches have diverged. Force push would overwrite remote history. \
         Consider pulling and resolving conflicts locally, or use --force if you know what you're doing."
    ))
}

/// Perform git push to remote repository
///
/// # Arguments
///
/// * `repo` - The git repository
/// * `remote_name` - Name of the remote repository
/// * `branch_name` - Name of the branch to push
/// * `force` - Whether to force push
///
/// # Returns
///
/// Returns a `Result` indicating success or failure
fn perform_git_push(
    repo: &Repository,
    remote_name: &str,
    branch_name: &str,
    force: bool,
) -> Result<()> {
    // Get the remote
    let mut remote = repo.find_remote(remote_name).map_err(|_| {
        CodeGuardianError::generic(format!(
            "No remote '{}' found. Please add the remote repository.",
            remote_name
        ))
    })?;

    // Push the specified branch
    let refspec = if force {
        warn!("Performing force push - this will overwrite remote history!");
        format!("+refs/heads/{}:refs/heads/{}", branch_name, branch_name)
    } else {
        format!("refs/heads/{}:refs/heads/{}", branch_name, branch_name)
    };

    let mut push_options = git2::PushOptions::new();
    remote.push(&[&refspec], Some(&mut push_options))?;

    Ok(())
}
