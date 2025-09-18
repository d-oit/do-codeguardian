//! # Git Operations Module
//!
//! This module provides enhanced git operations for the CodeGuardian tool,
//! including commit functionality with security analysis and intelligent
//! message generation.
//!
//! ## Features
//!
//! - Enhanced commit with security checks
//! - Intelligent commit message generation
//! - Repository status analysis
//! - Integration with security analysis

use git2::{Repository, Status};
use std::path::PathBuf;
use tracing::{debug, info};

use crate::config::Config;
use crate::core::GuardianEngine;
use crate::error::{CodeGuardianError, Result};
use crate::types::{AnalysisResults, Severity};
use crate::utils::progress::ProgressReporter;

/// Perform an enhanced git commit with security analysis
///
/// This function performs a comprehensive git commit operation that includes:
/// 1. Analysis of staged changes
/// 2. Security validation
/// 3. Intelligent commit message generation
/// 4. Actual commit execution
///
/// # Arguments
///
/// * `message` - Optional custom commit message
/// * `config` - Configuration for the commit operation
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
/// - Commit operation fails
pub async fn commit(message: Option<&str>, config: &Config) -> Result<()> {
    info!("Starting enhanced git commit process");

    // Open git repository
    let repo = Repository::open(".").map_err(|_| {
        CodeGuardianError::generic("Failed to open git repository. Are you in a git repository?")
    })?;

    // Get staged files
    let staged_files = get_staged_files(&repo)?;

    if staged_files.is_empty() {
        return Err(CodeGuardianError::NoStagedChanges);
    }

    debug!("Found {} staged files", staged_files.len());

    // Perform comprehensive analysis
    info!("Performing security analysis on staged changes");
    let progress = ProgressReporter::new(false);
    let mut engine = GuardianEngine::new(config.clone(), progress).await?;
    let analysis_results = engine.analyze_files(&staged_files, 1).await?;

    // Check for issues
    if analysis_results.has_issues() {
        let high_severity_count = analysis_results
            .findings
            .iter()
            .filter(|f| matches!(f.severity, Severity::High | Severity::Critical))
            .count();

        if high_severity_count > 0 && config.security.fail_on_issues {
            return Err(CodeGuardianError::SecurityIssuesFound(high_severity_count));
        }
    }

    // Generate commit message if not provided
    let commit_message = if let Some(msg) = message {
        msg.to_string()
    } else {
        generate_commit_message(&staged_files, &analysis_results, config).await?
    };

    debug!("Using commit message: {}", commit_message);

    // Perform the commit
    perform_commit(&repo, &commit_message)?;

    info!("Git commit completed successfully");
    Ok(())
}

/// Get list of staged files in the repository
///
/// # Arguments
///
/// * `repo` - The git repository
///
/// # Returns
///
/// Returns a vector of staged file paths
fn get_staged_files(repo: &Repository) -> Result<Vec<PathBuf>> {
    let statuses = repo.statuses(None)?;
    let mut staged_files = Vec::new();

    for entry in statuses.iter() {
        let status = entry.status();
        if status.contains(Status::INDEX_NEW)
            || status.contains(Status::INDEX_MODIFIED)
            || status.contains(Status::INDEX_DELETED)
            || status.contains(Status::INDEX_RENAMED)
            || status.contains(Status::INDEX_TYPECHANGE)
        {
            if let Some(path) = entry.path() {
                staged_files.push(PathBuf::from(path));
            }
        }
    }

    Ok(staged_files)
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
/// * `config` - Configuration settings
///
/// # Returns
///
/// Returns a generated commit message
async fn generate_commit_message(
    files: &[PathBuf],
    analysis: &AnalysisResults,
    config: &Config,
) -> Result<String> {
    if !config.git.conventional_commits {
        return Ok("Update files".to_string());
    }

    // Analyze file types
    let mut file_types = std::collections::HashMap::new();
    for file in files {
        if let Some(ext) = file.extension().and_then(|e| e.to_str()) {
            *file_types.entry(ext.to_string()).or_insert(0) += 1;
        }
    }

    // Determine commit type
    let commit_type = determine_commit_type(&file_types, analysis);

    // Determine scope
    let scope = determine_scope(files);

    // Create description
    let description = create_description(files, analysis);

    Ok(format!("{}({}): {}", commit_type, scope, description))
}

/// Determine the commit type based on file types and analysis results
fn determine_commit_type(
    file_types: &std::collections::HashMap<String, i32>,
    analysis: &AnalysisResults,
) -> String {
    // Check for security fixes
    if analysis
        .findings
        .iter()
        .any(|f| matches!(f.severity, Severity::High | Severity::Critical))
    {
        return "fix".to_string();
    }

    // Check file types
    if file_types.contains_key("rs")
        || file_types.contains_key("py")
        || file_types.contains_key("js")
    {
        "feat".to_string()
    } else if file_types.contains_key("md") {
        "docs".to_string()
    } else if file_types.contains_key("toml")
        || file_types.contains_key("yaml")
        || file_types.contains_key("json")
        || file_types.contains_key("lock")
    {
        "chore".to_string()
    } else {
        "refactor".to_string()
    }
}

/// Determine the scope based on affected files
fn determine_scope(files: &[PathBuf]) -> String {
    let mut directories = std::collections::HashSet::new();

    for file in files {
        if let Some(parent) = file.parent() {
            if let Some(dir_name) = parent.file_name().and_then(|n| n.to_str()) {
                directories.insert(dir_name.to_string());
            }
        }
    }

    if directories.len() == 1 {
        directories
            .into_iter()
            .next()
            .unwrap_or("general".to_string())
    } else {
        "general".to_string()
    }
}

/// Create a description for the commit message
fn create_description(files: &[PathBuf], analysis: &AnalysisResults) -> String {
    if files.len() == 1 {
        if let Some(file_name) = files[0].file_name().and_then(|n| n.to_str()) {
            format!("update {}", file_name)
        } else {
            "update file".to_string()
        }
    } else if analysis.findings.is_empty() {
        format!("update {} files", files.len())
    } else {
        format!("update {} files with security improvements", files.len())
    }
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
fn perform_commit(repo: &Repository, message: &str) -> Result<()> {
    // Get the current index
    let mut index = repo.index()?;
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;

    // Get the current HEAD
    let head = repo.head()?;
    let target = head
        .target()
        .ok_or_else(|| CodeGuardianError::generic("HEAD has no target (unborn repository)"))?;
    let parent_commit = repo.find_commit(target)?;

    // Create signature
    let signature = repo.signature()?;

    // Create the commit
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
