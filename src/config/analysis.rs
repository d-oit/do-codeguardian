//! # Analysis Configuration Module
//!
//! This module contains configuration structures for all analyzers,
//! including integrity, lint drift, non-production code, dependency analysis,
//! performance analysis, security analysis, and code quality analysis.

use serde::{Deserialize, Serialize};

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
    /// Broken files detection
    pub broken_files: BrokenFilesConfig,
    /// Duplicate code detection
    pub duplicate_analyzer: DuplicateAnalyzerConfig,
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
    /// Custom test directory patterns
    pub custom_test_directories: Vec<String>,
    /// Custom test file extensions
    pub custom_test_extensions: Vec<String>,
    /// Fuzzy test patterns (regex)
    pub fuzzy_test_patterns: Vec<String>,
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
            custom_test_directories: vec![
                "tests".to_string(),
                "test".to_string(),
                "spec".to_string(),
                "specs".to_string(),
                "__tests__".to_string(),
                "testdata".to_string(),
                "fixtures".to_string(),
                "mocks".to_string(),
            ],
            custom_test_extensions: vec![
                ".test.rs".to_string(),
                ".spec.rs".to_string(),
                ".integration.rs".to_string(),
                ".e2e.rs".to_string(),
            ],
            fuzzy_test_patterns: vec![
                r"(?i)test.*\.rs$".to_string(),
                r"(?i)spec.*\.rs$".to_string(),
                r"(?i).*test.*".to_string(),
                r"(?i).*spec.*".to_string(),
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

/// Broken files detection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrokenFilesConfig {
    /// Enable broken files detection
    pub enabled: bool,
    /// Detect merge conflicts
    pub detect_merge_conflicts: bool,
    /// Detect AI placeholders
    pub detect_ai_placeholders: bool,
    /// Detect duplicates
    pub detect_duplicates: bool,
    /// Git conflict detection settings
    pub conflicts: ConflictDetectionConfig,
    /// AI placeholder detection settings
    pub placeholders: PlaceholderDetectionConfig,
    /// Duplicate detection settings
    pub duplicates: DuplicateAnalyzerConfig,
}

impl Default for BrokenFilesConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            detect_merge_conflicts: true,
            detect_ai_placeholders: true,
            detect_duplicates: false, // Opt-in for performance
            conflicts: ConflictDetectionConfig::default(),
            placeholders: PlaceholderDetectionConfig::default(),
            duplicates: DuplicateAnalyzerConfig::default(),
        }
    }
}

/// Git conflict detection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictDetectionConfig {
    /// Fail on conflicts
    pub fail_on_conflicts: bool,
    /// Validate syntax
    pub validate_syntax: bool,
    /// Check git status
    pub check_git_status: bool,
}

impl Default for ConflictDetectionConfig {
    fn default() -> Self {
        Self {
            fail_on_conflicts: true,
            validate_syntax: true,
            check_git_status: true,
        }
    }
}

/// AI placeholder detection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaceholderDetectionConfig {
    /// Severity level for placeholder findings
    pub severity: String,
    /// Built-in patterns to detect
    pub patterns: Vec<String>,
    /// Custom patterns to detect
    pub custom_patterns: Vec<String>,
}

impl Default for PlaceholderDetectionConfig {
    fn default() -> Self {
        Self {
            severity: "medium".to_string(),
            patterns: vec![
                "add content here".to_string(),
                "implement this".to_string(),
                "your code here".to_string(),
                "placeholder".to_string(),
                "todo: implement".to_string(),
                "fill in the details".to_string(),
                "complete this".to_string(),
                "add your logic".to_string(),
            ],
            custom_patterns: Vec::new(),
        }
    }
}

/// Duplicate analyzer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateAnalyzerConfig {
    /// Enable duplicate analysis
    pub enabled: bool,
    /// Minimum lines for duplicate detection
    pub min_lines: usize,
    /// Focus on security-relevant code
    pub focus_security: bool,
    /// Ignore test files
    pub ignore_test_files: bool,
    /// Maximum files to compare (performance limit)
    pub max_files_to_compare: usize,
    /// Enable ML-enhanced similarity detection
    pub enable_ml_similarity: bool,
    /// ML model path for similarity detection
    pub ml_model_path: Option<String>,
    /// Similarity threshold for duplicate detection (0.0-1.0)
    pub similarity_threshold: f64,
    /// Enable GitHub duplicate issue prevention
    pub enable_github_prevention: bool,
    /// Cache settings for duplicate analysis
    pub cache: DuplicateCacheConfig,
}

impl Default for DuplicateAnalyzerConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            min_lines: 10,
            focus_security: true,
            ignore_test_files: true,
            max_files_to_compare: 1000,
            enable_ml_similarity: false,
            ml_model_path: None,
            similarity_threshold: 0.8,
            enable_github_prevention: false,
            cache: DuplicateCacheConfig::default(),
        }
    }
}

/// Cache configuration for duplicate analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateCacheConfig {
    /// Enable caching for duplicate analysis
    pub enabled: bool,
    /// Maximum cache size in MB
    pub max_size_mb: usize,
    /// Cache expiration time in hours
    pub expiration_hours: u32,
    /// Maximum entries in cache
    pub max_entries: usize,
}

impl Default for DuplicateCacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_size_mb: 100,
            expiration_hours: 24,
            max_entries: 1000,
        }
    }
}
