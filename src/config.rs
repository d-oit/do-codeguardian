//! # Configuration Module
//!
//! This module handles configuration management for the CodeGuardian tool.
//! It supports loading configuration from TOML files and provides default values.
//!
//! ## Configuration Options
//!
//! - Security settings (fail on issues, severity thresholds)
//! - Git settings (commit message templates, branch policies)
//! - Analysis settings (file size limits, excluded patterns)
//! - Logging settings (log level, output format)

use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Default configuration for CodeGuardian
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    /// Security-related configuration
    pub security: SecurityConfig,
    /// Git-related configuration
    pub git: GitConfig,
    /// Analysis-related configuration
    pub analysis: AnalysisConfig,
    /// Logging configuration
    pub logging: LoggingConfig,
}

impl Config {
    /// Load configuration from a TOML file
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the configuration file
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the loaded configuration or an error
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The file cannot be read
    /// - The TOML content is invalid
    pub fn from_file(path: &Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;
        let config: Config = toml::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {}", path.display()))?;
        Ok(config)
    }
}

/// Security configuration options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Whether to fail the commit if security issues are found
    pub fail_on_issues: bool,
    /// Minimum severity level to report (low, medium, high, critical)
    pub min_severity: String,
    /// Maximum file size to analyze (in bytes)
    pub max_file_size: u64,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            fail_on_issues: false,
            min_severity: "low".to_string(),
            max_file_size: 10 * 1024 * 1024, // 10MB
        }
    }
}

/// Git configuration options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitConfig {
    /// Whether to generate conventional commit messages
    pub conventional_commits: bool,
    /// Default commit message template
    pub commit_template: String,
    /// Whether to check for signed commits
    pub require_signed_commits: bool,
}

impl Default for GitConfig {
    fn default() -> Self {
        Self {
            conventional_commits: true,
            commit_template: "{type}({scope}): {description}".to_string(),
            require_signed_commits: false,
        }
    }
}

/// Analysis configuration options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisConfig {
    /// File patterns to exclude from analysis
    pub exclude_patterns: Vec<String>,
    /// Whether to analyze binary files
    pub analyze_binaries: bool,
    /// Timeout for analysis operations (in seconds)
    pub analysis_timeout: u64,
    /// Path to baseline file for comparison
    pub baseline_path: Option<std::path::PathBuf>,
    /// ML threshold for anomaly detection
    pub ml_threshold: Option<f64>,
    /// Enable streaming analysis
    pub streaming: bool,
    /// Default output directory for analysis results
    pub output_dir: String,
}

impl Default for AnalysisConfig {
    fn default() -> Self {
        Self {
            exclude_patterns: vec![
                "*.log".to_string(),
                "*.tmp".to_string(),
                "target/".to_string(),
                "node_modules/".to_string(),
            ],
            analyze_binaries: false,
            analysis_timeout: 300, // 5 minutes
            baseline_path: None,
            ml_threshold: None,
            streaming: false,
            output_dir: "codeguardian-results".to_string(),
        }
    }
}

/// Logging configuration options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level (error, warn, info, debug, trace)
    pub level: String,
    /// Whether to log to a file
    pub log_to_file: bool,
    /// Log file path (if log_to_file is true)
    pub log_file: Option<String>,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            log_to_file: false,
            log_file: None,
        }
    }
}
