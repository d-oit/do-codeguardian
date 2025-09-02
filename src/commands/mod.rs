//! # Commands Module
//!
//! This module contains the implementation of various CLI commands
//! for the CodeGuardian tool.
//!
//! ## Available Commands
//!
//! - `analyze`: Security analysis of files
//! - `git-commit`: Enhanced git commit with security checks
//! - `turbo`: High-performance parallel analysis

pub mod git_commit;
pub mod git_commit_push;
pub mod turbo;

// Re-export command functions for easier access
pub use git_commit::execute_git_commit;
pub use git_commit_push::execute_git_commit_push;
pub use turbo::execute_turbo;
