//! # Security Analyzers Module
//!
//! This module contains specialized security analyzers for detecting various
//! types of security vulnerabilities and issues.

pub mod command_injection_analyzer;
pub mod secret_analyzer;
pub mod sql_injection_analyzer;
pub mod vulnerability_analyzer;
pub mod xss_analyzer;

// Re-export analyzers for easy access
pub use command_injection_analyzer::CommandInjectionAnalyzer;
pub use secret_analyzer::SecretAnalyzer;
pub use sql_injection_analyzer::SqlInjectionAnalyzer;
pub use vulnerability_analyzer::VulnerabilityAnalyzer;
pub use xss_analyzer::XssAnalyzer;
