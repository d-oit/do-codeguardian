//! # Security Configuration Module
//!
//! This module contains configuration structures related to security analysis,
//! including vulnerability thresholds, secret detection, and security patterns.

use serde::{Deserialize, Serialize};

/// Security configuration options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Enable comprehensive security analysis
    pub enabled: bool,
    /// Whether to fail the commit if security issues are found
    pub fail_on_issues: bool,
    /// Minimum severity level to report (low, medium, high, critical)
    pub min_severity: String,
    /// Maximum file size to analyze (in bytes)
    pub max_file_size: u64,
    /// Minimum entropy threshold for detecting secrets
    pub entropy_threshold: f64,
    /// Maximum file size to analyze for security issues
    pub max_file_size_bytes: u64,
    /// Security vulnerability severity thresholds
    pub vulnerability_threshold: String,
    /// Enable detection of hardcoded secrets
    pub check_hardcoded_secrets: bool,
    /// Enable detection of unsafe code patterns
    pub check_unsafe_code: bool,
    /// Enable dependency vulnerability scanning
    pub check_dependencies: bool,
    /// Custom secret detection patterns
    pub secret_patterns: Vec<String>,
    /// Enable SQL injection detection
    pub check_sql_injection: bool,
    /// Enable XSS detection
    pub check_xss: bool,
    /// Enable command injection detection
    pub check_command_injection: bool,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            fail_on_issues: false,
            min_severity: "low".to_string(),
            max_file_size: 10 * 1024 * 1024, // 10MB
            entropy_threshold: 4.5,
            max_file_size_bytes: 52428800, // 50MB
            vulnerability_threshold: "medium".to_string(),
            check_hardcoded_secrets: true,
            check_unsafe_code: true,
            check_dependencies: true,
            secret_patterns: vec![
                r"(?i)(password|passwd|pwd)\s*[:=]\s*['\x22][^'\x22]{8,}['\x22]".to_string(),
                r"(?i)(api[_-]?key|apikey)\s*[:=]\s*['\x22][^'\x22]{16,}['\x22]".to_string(),
                r"(?i)(secret|token)\s*[:=]\s*['\x22][^'\x22]{16,}['\x22]".to_string(),
                r"(?i)(private[_-]?key)\s*[:=]\s*['\x22][^'\x22]{32,}['\x22]".to_string(),
            ],
            check_sql_injection: true,
            check_xss: true,
            check_command_injection: true,
        }
    }
}
