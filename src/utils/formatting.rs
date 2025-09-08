//! Common formatting utilities for consistent output across CLI commands

use crate::types::Severity;

/// Get emoji representation for severity levels
pub fn severity_emoji(severity: &Severity) -> &'static str {
    match severity {
        Severity::Critical => "🔴",
        Severity::High => "🟠",
        Severity::Medium => "🟡",
        Severity::Low => "🔵",
        Severity::Info => "ℹ️",
    }
}

/// Get color class for severity levels (for HTML output)
pub fn severity_css_class(severity: &Severity) -> &'static str {
    match severity {
        Severity::Critical => "critical",
        Severity::High => "high",
        Severity::Medium => "medium",
        Severity::Low => "low",
        Severity::Info => "info",
    }
}

/// Get log level string for severity
pub fn severity_log_level(severity: &Severity) -> &'static str {
    match severity {
        Severity::Critical => "error",
        Severity::High => "warn",
        Severity::Medium => "info",
        Severity::Low => "debug",
        Severity::Info => "trace",
    }
}

/// Format file location as string
pub fn format_file_location(file: &std::path::Path, line: usize, column: Option<usize>) -> String {
    if let Some(col) = column {
        format!("{}:{}:{}", file.display(), line, col)
    } else {
        format!("{}:{}", file.display(), line)
    }
}

/// Format finding ID with consistent styling
pub fn format_finding_id(id: &str) -> String {
    format!("`{}`", id)
}

/// Generate consistent summary header
pub fn summary_header() -> String {
    "📊 Analysis Summary\n==================".to_string()
}

/// Generate consistent findings header
pub fn findings_header() -> String {
    "## 🔍 Detailed Findings\n\n".to_string()
}

/// Generate no issues message
pub fn no_issues_message() -> String {
    "## ✅ No Issues Found\n\nGreat job! No issues were detected in the analyzed code.\n\n"
        .to_string()
}
