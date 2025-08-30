//! # Error Handling Module
//!
//! This module defines the error types and result aliases used throughout
//! the CodeGuardian application. It uses `thiserror` for library error types
//! and provides comprehensive error handling.
//!
//! ## Error Categories
//!
//! - `Io`: File system and I/O related errors
//! - `Git`: Git operation related errors
//! - `Security`: Security analysis related errors
//! - `Config`: Configuration related errors
//! - `Analysis`: Code analysis related errors
//! - `Network`: Network and HTTP related errors

use thiserror::Error;

/// Result type alias for CodeGuardian operations
pub type Result<T> = std::result::Result<T, CodeGuardianError>;

/// Comprehensive error type for the CodeGuardian application
#[derive(Error, Debug)]
pub enum CodeGuardianError {
    /// I/O related errors
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Git operation errors
    #[error("Git error: {0}")]
    Git(#[from] git2::Error),

    /// Security analysis errors
    #[error("Security analysis error: {0}")]
    Security(String),

    /// Configuration errors
    #[error("Configuration error: {0}")]
    Config(String),

    /// Code analysis errors
    #[error("Analysis error: {0}")]
    Analysis(String),

    /// Network and HTTP errors
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    /// TOML parsing errors
    #[error("TOML parsing error: {0}")]
    Toml(#[from] toml::de::Error),

    /// JSON parsing errors
    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    /// Generic errors
    #[error("Generic error: {0}")]
    Generic(String),

    /// No staged changes found for commit
    #[error("No staged changes found. Please stage your changes first.")]
    NoStagedChanges,

    /// Security issues found during analysis
    #[error("Security issues found: {0} issues detected")]
    SecurityIssuesFound(usize),

    /// Logging setup error
    #[error("Failed to setup logging")]
    LoggingSetup,

    /// Timeout error
    #[error("Operation timed out: {0}")]
    Timeout(String),
}

impl CodeGuardianError {
    /// Create a new security error with a custom message
    pub fn security<S: Into<String>>(message: S) -> Self {
        Self::Security(message.into())
    }

    /// Create a new configuration error with a custom message
    pub fn config<S: Into<String>>(message: S) -> Self {
        Self::Config(message.into())
    }

    /// Create a new analysis error with a custom message
    pub fn analysis<S: Into<String>>(message: S) -> Self {
        Self::Analysis(message.into())
    }

    /// Create a new generic error with a custom message
    pub fn generic<S: Into<String>>(message: S) -> Self {
        Self::Generic(message.into())
    }
}
