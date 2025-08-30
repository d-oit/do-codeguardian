//! # Commands Module
//!
//! This module contains the implementation of various CLI commands
//! for the CodeGuardian tool.
//!
//! ## Available Commands
//!
//! - `analyze`: Security analysis of files
//! - `git-commit`: Enhanced git commit with security checks

pub mod git_commit;

// Re-export command functions for easier access
pub use git_commit::execute_git_commit;
