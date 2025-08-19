use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;
use crate::error::GuardianError;

pub mod performance;
pub use performance::PerformanceConfig as OptimizationConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub max_file_size: u64,
    pub max_memory_mb: u64,
    pub parallel_workers: usize,
    pub timeout_seconds: u64,
    pub exclude_patterns: Vec<String>,
    pub include_patterns: Vec<String>,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            max_file_size: 10 * 1024 * 1024, // 10MB
            max_memory_mb: 512,
            parallel_workers: num_cpus::get(),
            timeout_seconds: 300,
            exclude_patterns: vec![
                "target/**".to_string(),
                "node_modules/**".to_string(),
                ".git/**".to_string(),
                "*.min.js".to_string(),
                "*.min.css".to_string(),
            ],
            include_patterns: vec!["**/*.rs".to_string(), "**/*.toml".to_string()],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HashAlgorithm {
    Blake3,
    Sha256,
}

impl Default for HashAlgorithm {
    fn default() -> Self {
        HashAlgorithm::Blake3
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrityConfig {
    pub enabled: bool,
    pub algorithm: HashAlgorithm,
    pub baseline_file: String,
    pub auto_update_baseline: bool,
    pub check_permissions: bool,
}

impl Default for IntegrityConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            algorithm: HashAlgorithm::Blake3,
            baseline_file: ".codeguardian/integrity.baseline".to_string(),
            auto_update_baseline: false,
            check_permissions: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LintDriftConfig {
    pub enabled: bool,
    pub config_files: Vec<String>,
    pub baseline_file: String,
    pub auto_update_baseline: bool,
    pub strict_mode: bool,
}

impl Default for LintDriftConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            config_files: vec![
                "Cargo.toml".to_string(),
                ".clippy.toml".to_string(),
                "rustfmt.toml".to_string(),
                ".rustfmt.toml".to_string(),
            ],
            baseline_file: ".codeguardian/lint_drift.baseline".to_string(),
            auto_update_baseline: false,
            strict_mode: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NonProdPattern {
    pub pattern: String,
    pub description: String,
    pub severity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NonProductionConfig {
    pub enabled: bool,
    pub patterns: Vec<NonProdPattern>,
    pub exclude_test_files: bool,
    pub exclude_example_files: bool,
}

impl Default for NonProductionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            patterns: vec![
                NonProdPattern {
                    pattern: r"(?i)\b(todo|fixme|hack|xxx)\b".to_string(),
                    description: "Non-production code markers".to_string(),
                    severity: "medium".to_string(),
                },
                NonProdPattern {
                    pattern: r"(?i)\bdebug\s*!".to_string(),
                    description: "Debug print statements".to_string(),
                    severity: "low".to_string(),
                },
                NonProdPattern {
                    pattern: r"(?i)\bprintln\s*!".to_string(),
                    description: "Print statements in production code".to_string(),
                    severity: "low".to_string(),
                },
            ],
            exclude_test_files: true,
            exclude_example_files: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub enabled: bool,
    pub check_secrets: bool,
    pub check_unsafe_code: bool,
    pub check_dependencies: bool,
    pub secret_patterns: Vec<String>,
    pub entropy_threshold: f64,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            check_secrets: true,
            check_unsafe_code: true,
            check_dependencies: true,
            secret_patterns: vec![
                r"(?i)(password|passwd|pwd)\s*[:=]\s*['\x22][^'\x22]{8,}['\x22]".to_string(),
                r"(?i)(api[_-]?key|apikey)\s*[:=]\s*['\x22][^'\x22]{16,}['\x22]".to_string(),
                r"(?i)(secret|token)\s*[:=]\s*['\x22][^'\x22]{16,}['\x22]".to_string(),
            ],
            entropy_threshold: 4.5,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    pub enabled: bool,
    pub check_allocations: bool,
    pub check_async_blocking: bool,
    pub max_complexity: usize,
    pub max_function_length: usize,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            check_allocations: true,
            check_async_blocking: true,
            max_complexity: 10,
            max_function_length: 100,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyAnalyzerConfig {
    /// Enable dependency analysis
    pub enabled: bool,
    /// Check for outdated dependencies
    pub check_outdated: bool,
    /// Check for vulnerable dependencies
    pub check_vulnerabilities: bool,
    /// Check for unused dependencies
    pub check_unused: bool,
    /// Check for duplicate dependencies
    pub check_duplicates: bool,
    /// Maximum allowed dependency age in days
    pub max_age_days: u32,
    /// Severity threshold for vulnerabilities
    pub vulnerability_threshold: String,
}

impl Default for DependencyAnalyzerConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            check_outdated: true,
            check_vulnerabilities: true,
            check_unused: true,
            check_duplicates: true,
            max_age_days: 365,
            vulnerability_threshold: "medium".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAnalyzerConfig {
    /// Enable performance analysis
    pub enabled: bool,
    /// Check for nested loops
    pub check_nested_loops: bool,
    /// Check for inefficient string operations
    pub check_string_operations: bool,
    /// Check for blocking I/O
    pub check_blocking_io: bool,
    /// Maximum acceptable cyclomatic complexity
    pub max_complexity: usize,
    /// Maximum acceptable function length
    pub max_function_length: usize,
}

impl Default for PerformanceAnalyzerConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            check_nested_loops: true,
            check_string_operations: true,
            check_blocking_io: true,
            max_complexity: 10,
            max_function_length: 50,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAnalyzerConfig {
    /// Enable security analysis
    pub enabled: bool,
    /// Check for SQL injection vulnerabilities
    pub check_sql_injection: bool,
    /// Check for XSS vulnerabilities
    pub check_xss: bool,
    /// Check for command injection
    pub check_command_injection: bool,
    /// Check for hardcoded secrets
    pub check_hardcoded_secrets: bool,
    /// Minimum entropy threshold for secret detection
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
            min_entropy_threshold: 3.5,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeQualityConfig {
    /// Enable code quality analysis
    pub enabled: bool,
    /// Check for magic numbers
    pub check_magic_numbers: bool,
    /// Check for complex conditions
    pub check_complex_conditions: bool,
    /// Check for deep nesting
    pub check_deep_nesting: bool,
    /// Check for commented-out code
    pub check_commented_code: bool,
    /// Maximum acceptable nesting depth
    pub max_nesting_depth: usize,
    /// Maximum acceptable file size (lines)
    pub max_file_size: usize,
}

impl Default for CodeQualityConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            check_magic_numbers: true,
            check_complex_conditions: true,
            check_deep_nesting: true,
            check_commented_code: true,
            max_nesting_depth: 6,
            max_file_size: 500,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub general: GeneralConfig,
    pub integrity: IntegrityConfig,
    pub lint_drift: LintDriftConfig,
    pub non_production: NonProductionConfig,
    pub dependency: DependencyAnalyzerConfig,
    pub performance_analyzer: PerformanceAnalyzerConfig,
    pub security_analyzer: SecurityAnalyzerConfig,
    pub code_quality: CodeQualityConfig,
    pub security: SecurityConfig,
    pub performance: PerformanceConfig,
    pub optimization: OptimizationConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self::minimal()
    }
}

impl Config {
    /// Load configuration from file
    pub fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::minimal());
        }

        let content = std::fs::read_to_string(path)
            .map_err(|e| anyhow::Error::from(GuardianError::io("Failed to read config file", e)))?;

        let config: Config = toml::from_str(&content)
            .map_err(|e| anyhow::Error::from(GuardianError::config(
                format!("Failed to parse config file: {}", e),
                Some(path.to_path_buf()),
            )))?;

        config.validate()?;
        Ok(config)
    }

    /// Save configuration to file
    pub fn save(&self, path: &Path) -> Result<()> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)
            .map_err(|e| anyhow::Error::from(GuardianError::io("Failed to write config file", e)))?;
        Ok(())
    }

    /// Create default configuration file
    pub fn create_default_config() -> Result<()> {
        let config = Self::default();
        config.save(Path::new("codeguardian.toml"))?;
        Ok(())
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        // Validate general config
        if self.general.max_file_size == 0 {
            return Err(anyhow::Error::from(GuardianError::config(
                "max_file_size must be greater than 0",
                None,
            )));
        }

        if self.general.max_memory_mb == 0 {
            return Err(anyhow::Error::from(GuardianError::config(
                "max_memory_mb must be greater than 0",
                None,
            )));
        }

        // Validate patterns
        for pattern in &self.non_production.patterns {
            if pattern.pattern.is_empty() {
                return Err(anyhow::Error::from(GuardianError::config(
                    "Non-production pattern cannot be empty",
                    None,
                )));
            }
        }

        Ok(())
    }

    /// Minimal configuration for basic usage
    pub fn minimal() -> Self {
        Self {
            general: GeneralConfig {
                max_file_size: 5 * 1024 * 1024, // 5MB
                max_memory_mb: 256,
                parallel_workers: 2,
                timeout_seconds: 120,
                exclude_patterns: vec![
                    "target/**".to_string(),
                    ".git/**".to_string(),
                ],
                include_patterns: vec!["**/*.rs".to_string()],
            },
            integrity: IntegrityConfig {
                algorithm: HashAlgorithm::Blake3,
                enabled: true,
                baseline_file: ".codeguardian/integrity.baseline".to_string(),
                auto_update_baseline: false,
                check_permissions: false,
            },
            lint_drift: LintDriftConfig {
                enabled: false,
                config_files: vec!["Cargo.toml".to_string()],
                baseline_file: ".codeguardian/lint_drift.baseline".to_string(),
                auto_update_baseline: false,
                strict_mode: false,
            },
            non_production: NonProductionConfig {
                enabled: true,
                patterns: vec![
                    NonProdPattern {
                        pattern: r"(?i)\b(todo|fixme|hack|xxx)\b".to_string(),
                        description: "Non-production code markers".to_string(),
                        severity: "medium".to_string(),
                    },
                ],
                exclude_test_files: true,
                exclude_example_files: true,
            },
            dependency: DependencyAnalyzerConfig::default(),
            performance_analyzer: PerformanceAnalyzerConfig::default(),
            security_analyzer: SecurityAnalyzerConfig::default(),
            code_quality: CodeQualityConfig::default(),
            security: SecurityConfig::default(),
            performance: PerformanceConfig {
                enabled: false,
                check_allocations: false,
                check_async_blocking: false,
                max_complexity: 15,
                max_function_length: 150,
            },
            optimization: OptimizationConfig::default(),
        }
    }

    /// Security-focused configuration
    pub fn security_focused() -> Self {
        Self {
            general: GeneralConfig::default(),
            integrity: IntegrityConfig::default(),
            lint_drift: LintDriftConfig::default(),
            non_production: NonProductionConfig::default(),
            dependency: DependencyAnalyzerConfig::default(),
            performance_analyzer: PerformanceAnalyzerConfig::default(),
            security_analyzer: SecurityAnalyzerConfig::default(),
            code_quality: CodeQualityConfig::default(),
            security: SecurityConfig::default(),
            performance: PerformanceConfig::default(),
            optimization: OptimizationConfig::thorough(),
        }
    }

    /// CI-optimized configuration
    pub fn ci_optimized() -> Self {
        Self {
            general: GeneralConfig::default(),
            integrity: IntegrityConfig::default(),
            lint_drift: LintDriftConfig::default(),
            non_production: NonProductionConfig::default(),
            dependency: DependencyAnalyzerConfig::default(),
            performance_analyzer: PerformanceAnalyzerConfig::default(),
            security_analyzer: SecurityAnalyzerConfig::default(),
            code_quality: CodeQualityConfig::default(),
            security: SecurityConfig::default(),
            performance: PerformanceConfig::default(),
            optimization: OptimizationConfig::ci_optimized(),
        }
    }
}