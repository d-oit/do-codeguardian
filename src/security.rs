//! # Security Analysis Module
//!
//! This module provides comprehensive security analysis capabilities for
//! code files, checking for common vulnerabilities, insecure patterns,
//! and security best practices.
//!
//! ## Analysis Features
//!
//! - File size validation
//! - Path traversal checks
//! - Insecure function usage detection
//! - Credential exposure detection
//! - Code quality checks
//!
//! ## Supported Languages
//!
//! - Rust
//! - Python
//! - JavaScript/TypeScript
//! - Go
//! - General patterns

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;
use tracing::{debug, warn};

use crate::config::Config;
use crate::error::Result;

/// Results from security analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResults {
    /// Number of files analyzed
    pub files_analyzed: usize,
    /// List of security issues found
    pub issues: Vec<SecurityIssue>,
    /// Analysis duration in milliseconds
    pub duration_ms: u128,
}

/// A security issue found during analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityIssue {
    /// File where the issue was found
    pub file: PathBuf,
    /// Line number where the issue occurs
    pub line: usize,
    /// Severity level (low, medium, high, critical)
    pub severity: String,
    /// Category of the security issue
    pub category: String,
    /// Description of the issue
    pub message: String,
    /// Suggested fix or mitigation
    pub suggestion: String,
}

/// Analyze a list of files for security issues
///
/// This function performs comprehensive security analysis on the provided files,
/// checking for common vulnerabilities and insecure patterns.
///
/// # Arguments
///
/// * `files` - List of file paths to analyze
/// * `config` - Configuration settings for analysis
///
/// # Returns
///
/// Returns `AnalysisResults` containing any issues found
///
/// # Errors
///
/// This function will return an error if:
/// - Any file cannot be read
/// - Analysis fails due to I/O issues
pub async fn analyze_files(files: &[PathBuf], config: &Config) -> Result<AnalysisResults> {
    let start_time = std::time::Instant::now();
    let mut issues = Vec::new();
    let mut files_analyzed = 0;

    // Collect all files to analyze, recursing into directories
    let all_files = collect_files_to_analyze(files, config).await?;

    for file_path in all_files {
        debug!("Analyzing file: {}", file_path.display());

        // Check if file should be analyzed
        if !should_analyze_file(&file_path, config) {
            continue;
        }

        // Read file content
        match fs::read_to_string(&file_path).await {
            Ok(content) => {
                let file_issues = analyze_file_content(file_path.as_path(), &content).await?;
                issues.extend(file_issues);
                files_analyzed += 1;
            }
            Err(e) => {
                warn!("Failed to read file {}: {}", file_path.display(), e);
                issues.push(SecurityIssue {
                    file: file_path.clone(),
                    line: 0,
                    severity: "medium".to_string(),
                    category: "file_access".to_string(),
                    message: format!("Failed to read file: {}", e),
                    suggestion: "Ensure file is readable and not corrupted".to_string(),
                });
            }
        }
    }

    let duration = start_time.elapsed().as_millis();

    // Filter issues based on minimum severity
    let min_severity_level = severity_level(&config.security.min_severity);
    let filtered_issues: Vec<SecurityIssue> = issues
        .into_iter()
        .filter(|issue| severity_level(&issue.severity) >= min_severity_level)
        .collect();

    Ok(AnalysisResults {
        files_analyzed,
        issues: filtered_issues,
        duration_ms: duration,
    })
}

/// Collect all files to analyze, recursing into directories
async fn collect_files_to_analyze(paths: &[PathBuf], config: &Config) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for path in paths {
        if path.is_dir() {
            // Recurse into directory
            let mut dir_files = collect_files_from_dir(path, config).await?;
            files.append(&mut dir_files);
        } else {
            files.push(path.clone());
        }
    }

    Ok(files)
}

/// Recursively collect files from a directory
async fn collect_files_from_dir(dir: &PathBuf, config: &Config) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    let mut entries = fs::read_dir(dir).await?;

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.is_dir() {
            // Recurse with Box::pin to handle async recursion
            let mut sub_files = Box::pin(collect_files_from_dir(&path, config)).await?;
            files.append(&mut sub_files);
        } else {
            files.push(path);
        }
    }

    Ok(files)
}

/// Check if a file should be analyzed based on security criteria
///
/// # Arguments
///
/// * `path` - Path to the file to check
/// * `config` - Configuration settings
///
/// # Returns
///
/// Returns `true` if the file should be analyzed, `false` otherwise
fn should_analyze_file(path: &std::path::Path, config: &Config) -> bool {
    let path_str = path.to_string_lossy();

    // Check exclude patterns
    for pattern in &config.analysis.exclude_patterns {
        // Simple substring match for now; could be improved with glob matching
        if path_str.contains(pattern) {
            return false;
        }
    }

    // Skip hidden files (except specific ones)
    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
        if name.starts_with('.') && name != ".gitignore" {
            // Allow temp files that start with .tmp
            if !name.starts_with(".tmp") {
                return false;
            }
        }
    }

    // Check file size limits (security: prevent huge files)
    if let Ok(metadata) = path.metadata() {
        if metadata.len() > config.security.max_file_size {
            return false;
        }
    }

    // Skip binary files if not configured to analyze them
    if !config.analysis.analyze_binaries {
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            if matches!(
                ext,
                "jpg"
                    | "jpeg"
                    | "png"
                    | "gif"
                    | "bmp"
                    | "tiff"
                    | "ico"
                    | "svg"
                    | "pdf"
                    | "doc"
                    | "docx"
                    | "xls"
                    | "xlsx"
                    | "ppt"
                    | "pptx"
                    | "zip"
                    | "tar"
                    | "gz"
                    | "bz2"
                    | "7z"
                    | "rar"
                    | "exe"
                    | "dll"
                    | "so"
                    | "dylib"
            ) {
                return false;
            }
        }
    }

    true
}

/// Analyze the content of a single file for security issues
///
/// # Arguments
///
/// * `file_path` - Path to the file being analyzed
/// * `content` - Content of the file
///
/// # Returns
///
/// Returns a vector of security issues found in the file
async fn analyze_file_content(
    file_path: &std::path::Path,
    content: &str,
) -> Result<Vec<SecurityIssue>> {
    let mut issues = Vec::new();
    let lines: Vec<&str> = content.lines().collect();

    // Common security patterns to check
    let patterns = vec![
        // Hardcoded secrets
        (
            r#"(?i)(api[_-]?key|secret|password|token|apikey)\s*[=:]\s*['"]([^'"]{10,})['"]"#,
            "high",
            "hardcoded_secret",
            "Security issue: Hardcoded secret detected",
        ),
        // SQL injection patterns
        (
            r"(?i)SELECT.*\+.*FROM",
            "high",
            "sql_injection",
            "Potential SQL injection vulnerability",
        ),
        // Path traversal
        (
            r"\.\./",
            "medium",
            "path_traversal",
            "Potential path traversal vulnerability",
        ),
        // Insecure random
        (
            r"(?i)rand::random|Math\.random",
            "medium",
            "weak_random",
            "Using weak random number generator",
        ),
        // Debug information
        (
            r"(?i)println!.*password|console\.log.*password",
            "medium",
            "debug_info",
            "Debug information may expose sensitive data",
        ),
        // TODO comments with security implications
        (
            r"(?i)//\s*TODO.*(?:security|auth|encrypt)",
            "low",
            "todo_security",
            "TODO comment mentions security",
        ),
    ];

    for (line_num, line) in lines.iter().enumerate() {
        for (pattern, severity, category, message) in &patterns {
            if let Ok(regex) = Regex::new(pattern) {
                if regex.is_match(line) {
                    issues.push(SecurityIssue {
                        file: file_path.to_path_buf(),
                        line: line_num + 1,
                        severity: severity.to_string(),
                        category: category.to_string(),
                        message: message.to_string(),
                        suggestion: get_suggestion_for_category(category),
                    });
                }
            }
        }
    }

    // Language-specific analysis
    if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
        match ext {
            "rs" => issues.extend(analyze_rust_file(file_path, content).await?),
            "py" => issues.extend(analyze_python_file(file_path, content).await?),
            "js" | "ts" => issues.extend(analyze_javascript_file(file_path, content).await?),
            _ => {}
        }
    }

    Ok(issues)
}

/// Get a suggestion for fixing a security issue based on its category
fn get_suggestion_for_category(category: &str) -> String {
    match category {
        "hardcoded_secret" => "Use environment variables or secure key management".to_string(),
        "sql_injection" => "Use parameterized queries or prepared statements".to_string(),
        "path_traversal" => "Validate and sanitize file paths".to_string(),
        "weak_random" => "Use cryptographically secure random number generators".to_string(),
        "debug_info" => "Remove debug statements before production deployment".to_string(),
        "todo_security" => "Address security TODO items before release".to_string(),
        _ => "Review and fix the security issue".to_string(),
    }
}

/// Get the numeric level for a severity string
fn severity_level(severity: &str) -> i32 {
    match severity {
        "low" => 1,
        "medium" => 2,
        "high" => 3,
        "critical" => 4,
        _ => 0,
    }
}

/// Analyze Rust-specific security patterns
async fn analyze_rust_file(
    _file_path: &std::path::Path,
    content: &str,
) -> Result<Vec<SecurityIssue>> {
    let mut issues = Vec::new();

    // Check for unsafe code usage
    if content.contains("unsafe") {
        issues.push(SecurityIssue {
            file: _file_path.to_path_buf(),
            line: 0, // Would need line number tracking
            severity: "medium".to_string(),
            category: "unsafe_code".to_string(),
            message: "Unsafe code block detected".to_string(),
            suggestion: "Review unsafe code for security implications".to_string(),
        });
    }

    Ok(issues)
}

/// Analyze Python-specific security patterns
async fn analyze_python_file(
    _file_path: &std::path::Path,
    content: &str,
) -> Result<Vec<SecurityIssue>> {
    let mut issues = Vec::new();

    // Check for eval usage
    if content.contains("eval(") {
        issues.push(SecurityIssue {
            file: _file_path.to_path_buf(),
            line: 0,
            severity: "high".to_string(),
            category: "code_injection".to_string(),
            message: "Use of eval() detected - potential code injection".to_string(),
            suggestion: "Avoid eval() or use ast.literal_eval() for safe evaluation".to_string(),
        });
    }

    Ok(issues)
}

/// Analyze JavaScript/TypeScript-specific security patterns
async fn analyze_javascript_file(
    _file_path: &std::path::Path,
    content: &str,
) -> Result<Vec<SecurityIssue>> {
    let mut issues = Vec::new();

    // Check for innerHTML usage
    if content.contains("innerHTML") {
        issues.push(SecurityIssue {
            file: _file_path.to_path_buf(),
            line: 0,
            severity: "medium".to_string(),
            category: "xss".to_string(),
            message: "Use of innerHTML detected - potential XSS vulnerability".to_string(),
            suggestion: "Use textContent or createElement for safe DOM manipulation".to_string(),
        });
    }

    Ok(issues)
}
