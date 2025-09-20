//! # CodeGuardian Library
//!
//! This is the core library for the CodeGuardian CLI tool, providing
//! security analysis capabilities, git operations, and configuration management.

#![allow(dead_code)] // Temporarily allow dead code during development
#![allow(clippy::too_many_arguments)] // Allow complex function signatures
#![allow(clippy::type_complexity)] // Allow complex types during development
//!
//! ## Features
//!
//! - Security analysis of code files
//! - Git commit functionality with intelligent message generation
//! - Configuration management
//! - Error handling with detailed diagnostics
//!
//! ## Example
//!
//! ```rust,no_run
//! use do_codeguardian::{git_commit, Config};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let config = Config::default();
//!     git_commit(None, &config).await?;
//!     println!("Commit successful");
//!     Ok(())
//! }
//! ```

pub mod analyzers;
pub mod cache;
pub mod cli;
pub mod commands;
pub mod config;
pub mod core;
#[cfg(feature = "dashboard")]
pub mod dashboard;
pub mod error;
pub mod git;
pub mod github;
pub mod github_api;
pub mod indexing;
pub mod integrations;
#[cfg(feature = "ml")]
pub mod ml;
pub mod output;
pub mod performance;
pub mod relationships;
pub mod release_monitoring;
pub mod remediation;
pub mod report;
pub mod security;
pub mod streaming;
pub mod types;
pub mod utils;

pub use config::Config;
/// Re-export commonly used types
pub use error::{CodeGuardianError, Result};

/// Analyze a list of files for security issues
///
/// This function performs comprehensive security analysis on the provided files,
/// checking for common vulnerabilities, code quality issues, and best practices.
///
/// # Arguments
///
/// * `files` - A slice of file paths to analyze
/// * `config` - Configuration settings for analysis
///
/// # Returns
///
/// Returns a `Result` containing the analysis results or an error
///
/// # Errors
///
/// This function will return an error if:
/// - Any file cannot be read
/// - Security analysis fails
/// - Configuration is invalid
///
/// # Example
///
/// ```rust,no_run
/// use do_codeguardian::{analyze_files, Config};
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let config = Config::default();
///     let results = analyze_files(&["src/main.rs".into()], &config).await?;
///     println!("Found {} issues", results.issues.len());
///     Ok(())
/// }
/// ```
pub async fn analyze_files(
    files: &[std::path::PathBuf],
    config: &Config,
) -> Result<security::AnalysisResults> {
    use crate::core::GuardianEngine;
    use crate::utils::progress::ProgressReporter;
    use std::time::Instant;

    let start_time = Instant::now();
    let progress = ProgressReporter::new(false);
    let mut engine = GuardianEngine::new(config.clone(), progress).await?;
    let analysis_results = engine.analyze_files(files, 1).await?;

    // Convert from types::AnalysisResults to security::AnalysisResults
    let issues: Vec<security::SecurityIssue> = analysis_results
        .findings
        .into_iter()
        .map(|finding| security::SecurityIssue {
            file: finding.file,
            line: finding.line as usize,
            severity: format!("{:?}", finding.severity).to_lowercase(),
            category: finding.category.unwrap_or_else(|| "general".to_string()),
            message: finding.message,
            suggestion: finding.suggestion.unwrap_or_default(),
        })
        .collect();

    let duration = start_time.elapsed().as_millis();

    Ok(security::AnalysisResults {
        files_analyzed: analysis_results.summary.total_files_scanned,
        issues,
        duration_ms: duration,
    })
}

/// Perform a git commit with security analysis
///
/// This function analyzes the staged changes, generates an intelligent commit message,
/// and performs the commit while ensuring security best practices.
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
/// - Git repository is not found
/// - No staged changes are present
/// - Security analysis fails
/// - Commit operation fails
///
/// # Example
///
/// ```rust,no_run
/// use do_codeguardian::{git_commit, Config};
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let config = Config::default();
///     git_commit(None, &config).await?;
///     println!("Commit successful");
///     Ok(())
/// }
/// ```
pub async fn git_commit(message: Option<&str>, config: &Config) -> Result<()> {
    git::commit(message, config).await
}
