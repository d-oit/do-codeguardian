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
//! - `update-docs`: Update and maintain documentation

pub mod git_commit;
pub mod git_commit_push;
pub mod turbo;
pub mod update_docs;

// Re-export command functions for easier access
pub use git_commit::execute_git_commit;
pub use git_commit_push::execute_git_commit_push;
pub use turbo::execute_turbo;
pub use update_docs::execute_update_docs;
