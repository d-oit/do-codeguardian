//! # Configuration Module
//!
//! This module provides the configuration system for CodeGuardian, allowing users
//! to customize analysis behavior, output formats, integrations, and performance settings.
//!
//! The configuration is organized into several sub-modules:
//!
//! - `base`: Core configuration structures and defaults
//! - `analysis`: Analysis-specific settings and analyzer configurations
//! - `output`: Output formatting and reporting options
//! - `security`: Security-related configuration options
//! - `retention`: Data retention and cleanup policies
//! - `checklist`: Checklist and validation configurations
//!
//! ## Usage
//!
//! Configurations can be loaded from TOML files or environment variables.
//! The `Config` struct in `base` serves as the main configuration entry point.

// Re-export all configuration structures for backward compatibility
pub use self::analysis::*;
pub use self::base::*;
pub use self::checklist::*;
pub use self::output::*;
pub use self::retention::*;
pub use self::security::*;

// Sub-modules
pub mod analysis;
pub mod base;
pub mod checklist;
pub mod output;
pub mod retention;
pub mod security;
