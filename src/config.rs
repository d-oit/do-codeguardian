//! # Configuration Module
//!
//! This module handles configuration management for the CodeGuardian tool.
//! It supports loading configuration from TOML files and provides default values.
//!
//! ## Configuration Options
//!
//! - Output settings (directory, format, compression, subdirectories, archiving)
//! - Security settings (vulnerability thresholds, secret detection)
//! - Performance settings (memory limits, complexity thresholds)
//! - Analyzer-specific configurations (integrity, lint drift, non-production code)
//! - File inclusion/exclusion patterns
//! - Severity level customizations
//! - Integration settings (GitHub, GitLab)
//! - Analysis settings (parallel processing, caching)
//! - Optimization settings (caching, early termination)

use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Default configuration for CodeGuardian
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    /// Output configuration
    pub output: OutputConfig,
    /// Security-related configuration
    pub security: SecurityConfig,
    /// Git-related configuration
    pub git: GitConfig,
    /// Performance optimization settings
    pub performance: PerformanceConfig,
    /// Analyzer-specific configurations
    pub analyzers: AnalyzerConfigs,
    /// File inclusion/exclusion patterns
    pub files: FileConfig,
    /// Severity level customizations
    pub severity: SeverityConfig,
    /// Integration settings
    pub integrations: IntegrationConfig,
    /// General analysis settings
    pub analysis: AnalysisConfig,
    /// Optimization settings
    pub optimization: OptimizationConfig,
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

/// Output configuration options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    /// Directory where analysis results will be saved
    pub directory: String,
    /// Output format options: "json", "html", "markdown", "sarif"
    pub format: String,
    /// Enable verbose output
    pub verbose: bool,
    /// Generate summary reports
    pub generate_summary: bool,
    /// Compress large output files
    pub compress_output: bool,
    /// Subdirectory for reports within the output directory
    pub reports_subdirectory: String,
    /// Subdirectory for data files within the output directory
    pub data_subdirectory: String,
    /// Subdirectory for temporary files within the output directory
    pub temp_subdirectory: String,
    /// Subdirectory for historical reports within the output directory
    pub historical_subdirectory: String,
    /// Automatically archive old reports
    pub auto_archive: bool,
    /// Maximum number of reports to keep before archiving
    pub max_reports_kept: u32,
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            directory: "codeguardian-results".to_string(),
            format: "sarif".to_string(),
            verbose: false,
            generate_summary: true,
            compress_output: true,
            reports_subdirectory: "reports".to_string(),
            data_subdirectory: "data".to_string(),
            temp_subdirectory: "temp".to_string(),
            historical_subdirectory: "historical".to_string(),
            auto_archive: true,
            max_reports_kept: 10,
        }
    }
}

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

/// Performance optimization settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Enable performance analysis
    pub enabled: bool,
    /// Check for excessive memory allocations
    pub check_allocations: bool,
    /// Check for blocking operations in async code
    pub check_async_blocking: bool,
    /// Maximum cyclomatic complexity allowed
    pub max_complexity: u32,
    /// Maximum function length (lines)
    pub max_function_length: u32,
    /// Enable performance profiling
    pub enable_profiling: bool,
    /// Memory usage limits (in MB)
    pub max_memory_usage_mb: u32,
    /// CPU usage limits (percentage)
    pub max_cpu_usage_percent: u32,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            check_allocations: true,
            check_async_blocking: true,
            max_complexity: 15,
            max_function_length: 150,
            enable_profiling: false,
            max_memory_usage_mb: 512,
            max_cpu_usage_percent: 80,
        }
    }
}

/// Analyzer-specific configurations
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AnalyzerConfigs {
    /// File integrity checking
    pub integrity: IntegrityConfig,
    /// Configuration drift detection
    pub lint_drift: LintDriftConfig,
    /// Non-production code detection
    pub non_production: NonProductionConfig,
    /// Dependency analysis
    pub dependency: DependencyConfig,
    /// Performance analysis
    pub performance_analyzer: PerformanceAnalyzerConfig,
    /// Security analysis
    pub security_analyzer: SecurityAnalyzerConfig,
    /// Code quality analysis
    pub code_quality: CodeQualityConfig,
}

/// File integrity configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrityConfig {
    /// Enable integrity checking
    pub enabled: bool,
    /// Hash algorithm to use
    pub hash_algorithm: String,
    /// Baseline file path
    pub baseline_file: String,
    /// Auto-update baseline
    pub auto_update_baseline: bool,
    /// Check file permissions
    pub check_permissions: bool,
    /// Check binary files
    pub check_binary_files: bool,
    /// Verify checksums
    pub verify_checksums: bool,
    /// Maximum file size for integrity checks
    pub max_file_size: u64,
}

impl Default for IntegrityConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            hash_algorithm: "Blake3".to_string(),
            baseline_file: ".codeguardian/integrity.baseline".to_string(),
            auto_update_baseline: false,
            check_permissions: true,
            check_binary_files: false,
            verify_checksums: true,
            max_file_size: 5242880, // 5MB
        }
    }
}

/// Configuration drift detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LintDriftConfig {
    /// Enable lint drift detection
    pub enabled: bool,
    /// Configuration files to monitor
    pub config_files: Vec<String>,
    /// Baseline file path
    pub baseline_file: String,
    /// Auto-update baseline
    pub auto_update_baseline: bool,
    /// Strict mode
    pub strict_mode: bool,
}

impl Default for LintDriftConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            config_files: vec![
                "Cargo.toml".to_string(),
                "package.json".to_string(),
                ".eslintrc.json".to_string(),
            ],
            baseline_file: ".codeguardian/lint_drift.baseline".to_string(),
            auto_update_baseline: false,
            strict_mode: false,
        }
    }
}

/// Non-production code detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NonProductionConfig {
    /// Enable non-production code detection
    pub enabled: bool,
    /// Exclude test files
    pub exclude_test_files: bool,
    /// Exclude example files
    pub exclude_example_files: bool,
    /// Detection patterns
    pub patterns: Vec<NonProductionPattern>,
}

impl Default for NonProductionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            exclude_test_files: true,
            exclude_example_files: true,
            patterns: vec![
                NonProductionPattern {
                    pattern: r"(?i)\b(todo|fixme|hack|xxx)\b".to_string(),
                    description: "Non-production code markers".to_string(),
                    severity: "medium".to_string(),
                },
                NonProductionPattern {
                    pattern: r"(?i)\bconsole\.log\b".to_string(),
                    description: "Debug logging statements".to_string(),
                    severity: "low".to_string(),
                },
            ],
        }
    }
}

/// Pattern for non-production code detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NonProductionPattern {
    /// Regex pattern to match
    pub pattern: String,
    /// Description of the pattern
    pub description: String,
    /// Severity level
    pub severity: String,
}

/// Dependency analysis configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyConfig {
    /// Enable dependency analysis
    pub enabled: bool,
    /// Check for outdated packages
    pub check_outdated: bool,
    /// Check for vulnerabilities
    pub check_vulnerabilities: bool,
    /// Check for unused dependencies
    pub check_unused: bool,
    /// Check for duplicate dependencies
    pub check_duplicates: bool,
    /// Check licenses
    pub check_licenses: bool,
    /// Maximum age in days for dependencies
    pub max_age_days: u32,
    /// Vulnerability databases
    pub vulnerability_databases: Vec<String>,
}

impl Default for DependencyConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            check_outdated: true,
            check_vulnerabilities: true,
            check_unused: true,
            check_duplicates: true,
            check_licenses: true,
            max_age_days: 365,
            vulnerability_databases: vec![
                "https://cve.mitre.org".to_string(),
                "https://nvd.nist.gov".to_string(),
            ],
        }
    }
}

/// Performance analyzer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAnalyzerConfig {
    /// Enable performance analysis
    pub enabled: bool,
    /// Check nested loops
    pub check_nested_loops: bool,
    /// Check string operations
    pub check_string_operations: bool,
    /// Check blocking I/O
    pub check_blocking_io: bool,
    /// Check algorithms
    pub check_algorithms: bool,
    /// Check memory usage
    pub check_memory_usage: bool,
    /// Check I/O operations
    pub check_io_operations: bool,
    /// Maximum complexity
    pub max_complexity: u32,
    /// Maximum function length
    pub max_function_length: u32,
    /// Maximum loop depth
    pub max_loop_depth: u32,
}

impl Default for PerformanceAnalyzerConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            check_nested_loops: true,
            check_string_operations: true,
            check_blocking_io: true,
            check_algorithms: true,
            check_memory_usage: true,
            check_io_operations: true,
            max_complexity: 10,
            max_function_length: 50,
            max_loop_depth: 3,
        }
    }
}

/// Security analyzer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAnalyzerConfig {
    /// Enable security analysis
    pub enabled: bool,
    /// Check SQL injection
    pub check_sql_injection: bool,
    /// Check XSS
    pub check_xss: bool,
    /// Check command injection
    pub check_command_injection: bool,
    /// Check hardcoded secrets
    pub check_hardcoded_secrets: bool,
    /// Check vulnerabilities
    pub check_vulnerabilities: bool,
    /// Check permissions
    pub check_permissions: bool,
    /// Check secrets
    pub check_secrets: bool,
    /// Minimum entropy threshold
    pub min_entropy_threshold: f64,
}

impl Default for SecurityAnalyzerConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            check_sql_injection: true,
            check_xss: true,
            check_command_injection: true,
            check_hardcoded_secrets: true,
            check_vulnerabilities: true,
            check_permissions: true,
            check_secrets: true,
            min_entropy_threshold: 3.5,
        }
    }
}

/// Code quality configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeQualityConfig {
    /// Enable code quality analysis
    pub enabled: bool,
    /// Check magic numbers
    pub check_magic_numbers: bool,
    /// Check complex conditions
    pub check_complex_conditions: bool,
    /// Check deep nesting
    pub check_deep_nesting: bool,
    /// Check commented code
    pub check_commented_code: bool,
    /// Check complexity
    pub check_complexity: bool,
    /// Check duplication
    pub check_duplication: bool,
    /// Check naming
    pub check_naming: bool,
    /// Maximum complexity
    pub max_complexity: u32,
    /// Maximum nesting depth
    pub max_nesting_depth: u32,
    /// Maximum file size
    pub max_file_size: u32,
    /// Maximum line length
    pub max_line_length: u32,
}

impl Default for CodeQualityConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            check_magic_numbers: true,
            check_complex_conditions: true,
            check_deep_nesting: true,
            check_commented_code: true,
            check_complexity: true,
            check_duplication: true,
            check_naming: true,
            max_complexity: 10,
            max_nesting_depth: 6,
            max_file_size: 500,
            max_line_length: 120,
        }
    }
}

/// File inclusion/exclusion patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileConfig {
    /// File patterns to include in analysis
    pub include_patterns: Vec<String>,
    /// File patterns to exclude from analysis
    pub exclude_patterns: Vec<String>,
    /// File extensions to analyze
    pub analyze_extensions: Vec<String>,
    /// Skip binary files
    pub skip_binaries: bool,
    /// Maximum file size to analyze
    pub max_file_size_bytes: u64,
    /// Skip files larger than specified size
    pub skip_large_files: bool,
}

impl Default for FileConfig {
    fn default() -> Self {
        Self {
            include_patterns: vec![],
            exclude_patterns: vec![
                "*.log".to_string(),
                "*.tmp".to_string(),
                "target/".to_string(),
                "node_modules/".to_string(),
                ".git/".to_string(),
                "dist/".to_string(),
                "build/".to_string(),
                "*.min.js".to_string(),
                "*.min.css".to_string(),
                "vendor/".to_string(),
                "third_party/".to_string(),
            ],
            analyze_extensions: vec![
                ".rs".to_string(),
                ".js".to_string(),
                ".ts".to_string(),
                ".py".to_string(),
                ".java".to_string(),
                ".cpp".to_string(),
                ".c".to_string(),
                ".h".to_string(),
                ".go".to_string(),
                ".php".to_string(),
                ".rb".to_string(),
                ".cs".to_string(),
                ".swift".to_string(),
                ".kt".to_string(),
                ".scala".to_string(),
                ".html".to_string(),
                ".css".to_string(),
                ".json".to_string(),
                ".xml".to_string(),
                ".yaml".to_string(),
                ".yml".to_string(),
                ".toml".to_string(),
                ".md".to_string(),
                ".txt".to_string(),
                ".sh".to_string(),
                ".bat".to_string(),
                ".ps1".to_string(),
            ],
            skip_binaries: true,
            max_file_size_bytes: 10485760, // 10MB
            skip_large_files: true,
        }
    }
}

/// Severity level customizations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeverityConfig {
    /// Custom severity mappings
    pub custom_levels: Vec<String>,
    /// Enable severity escalation
    pub enable_escalation: bool,
    /// Number of occurrences before escalating
    pub escalation_threshold: u32,
}

impl Default for SeverityConfig {
    fn default() -> Self {
        Self {
            custom_levels: vec![
                "info".to_string(),
                "low".to_string(),
                "medium".to_string(),
                "high".to_string(),
                "critical".to_string(),
            ],
            enable_escalation: true,
            escalation_threshold: 5,
        }
    }
}

/// Integration settings
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IntegrationConfig {
    /// GitHub integration
    pub github: GitHubConfig,
    /// GitLab integration
    pub gitlab: GitLabConfig,
}

/// GitHub integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubConfig {
    /// Enable GitHub integration
    pub enabled: bool,
    /// GitHub repository (owner/repo format)
    pub repository: String,
    /// GitHub token
    pub token: String,
    /// Create GitHub issues
    pub create_issues: bool,
    /// Issue labels
    pub issue_labels: Vec<String>,
    /// Comment on pull requests
    pub comment_prs: bool,
    /// Minimum severity to create issues/PR comments
    pub min_severity: String,
}

impl Default for GitHubConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            repository: "".to_string(),
            token: "${CODEGUARDIAN_GITHUB_TOKEN}".to_string(),
            create_issues: false,
            issue_labels: vec!["security".to_string(), "codeguardian".to_string()],
            comment_prs: false,
            min_severity: "high".to_string(),
        }
    }
}

/// GitLab integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitLabConfig {
    /// Enable GitLab integration
    pub enabled: bool,
    /// GitLab project ID or path
    pub project: String,
    /// GitLab token
    pub token: String,
    /// Create GitLab issues
    pub create_issues: bool,
    /// Issue labels
    pub issue_labels: Vec<String>,
    /// Comment on merge requests
    pub comment_mrs: bool,
    /// Minimum severity to create issues/MR comments
    pub min_severity: String,
}

impl Default for GitLabConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            project: "".to_string(),
            token: "${CODEGUARDIAN_GITLAB_TOKEN}".to_string(),
            create_issues: false,
            issue_labels: vec!["security".to_string(), "codeguardian".to_string()],
            comment_mrs: false,
            min_severity: "high".to_string(),
        }
    }
}

/// General analysis settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisConfig {
    /// Enable analysis
    pub enabled: bool,
    /// Analyze binary files
    pub analyze_binaries: bool,
    /// Analysis timeout in seconds
    pub timeout_seconds: u64,
    /// Enable parallel processing
    pub parallel_processing: bool,
    /// Number of parallel workers
    pub max_workers: u32,
    /// Enable caching
    pub enable_caching: bool,
    /// Cache directory
    pub cache_dir: String,
    /// Cache expiration in days
    pub cache_expiration_days: u32,
}

impl Default for AnalysisConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            analyze_binaries: false,
            timeout_seconds: 300,
            parallel_processing: true,
            max_workers: 4,
            enable_caching: true,
            cache_dir: ".codeguardian/cache".to_string(),
            cache_expiration_days: 7,
        }
    }
}

/// Optimization settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationConfig {
    /// Enable optimized analyzers
    pub enable_optimized_analyzers: bool,
    /// Enable file caching
    pub enable_file_caching: bool,
    /// Maximum parallel workers
    pub max_parallel_workers: u32,
    /// Maximum file size to load into memory
    pub max_memory_file_size: u64,
    /// Streaming chunk size
    pub streaming_chunk_size: u64,
    /// Maximum findings per file
    pub max_findings_per_file: u32,
    /// Pattern cache size
    pub pattern_cache_size: u32,
    /// Cache cleanup settings
    pub cache_cleanup: CacheCleanupConfig,
    /// Early termination settings
    pub early_termination: EarlyTerminationConfig,
}

impl Default for OptimizationConfig {
    fn default() -> Self {
        Self {
            enable_optimized_analyzers: true,
            enable_file_caching: true,
            max_parallel_workers: 4,
            max_memory_file_size: 10485760, // 10MB
            streaming_chunk_size: 65536,    // 64KB
            max_findings_per_file: 50,
            pattern_cache_size: 1000,
            cache_cleanup: CacheCleanupConfig::default(),
            early_termination: EarlyTerminationConfig::default(),
        }
    }
}

/// Cache cleanup configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheCleanupConfig {
    /// Enable automatic cache cleanup
    pub enabled: bool,
    /// Maximum cache age in days
    pub max_age_days: u32,
    /// Maximum cache size in MB
    pub max_size_mb: u32,
    /// Cleanup frequency
    pub cleanup_frequency: u32,
}

impl Default for CacheCleanupConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_age_days: 7,
            max_size_mb: 100,
            cleanup_frequency: 10,
        }
    }
}

/// Early termination configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarlyTerminationConfig {
    /// Enable early termination
    pub enabled: bool,
    /// Maximum analysis time per file
    pub max_analysis_time_seconds: u32,
    /// Maximum lines per file
    pub max_lines_per_file: u32,
    /// Skip files larger than this size
    pub skip_large_files_bytes: u64,
}

impl Default for EarlyTerminationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_analysis_time_seconds: 30,
            max_lines_per_file: 10000,
            skip_large_files_bytes: 52428800, // 50MB
        }
    }
}
