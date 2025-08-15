use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use crate::error::GuardianError;

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
}

impl Config {
    /// Load configuration from file
    pub fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }

        let content = std::fs::read_to_string(path)
            .map_err(|e| GuardianError::io("Failed to read config file", e).into())?;

        let config: Config = toml::from_str(&content)
            .map_err(|e| GuardianError::config(
                format!("Failed to parse config file: {}", e),
                Some(path.to_path_buf()),
            ).into())?;

        config.validate()?;
        Ok(config)
    }

    /// Save configuration to file
    pub fn save(&self, path: &Path) -> Result<()> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)
            .map_err(|e| GuardianError::io("Failed to write config file", e).into())?;
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
            return Err(GuardianError::config(
                "max_file_size must be greater than 0",
                None,
            ));
        }

        // Validate security config
        if self.security.max_memory_mb == 0 {
            return Err(GuardianError::config(
                "max_memory_mb must be greater than 0",
                None,
            ));
        }

        if self.security.operation_timeout == 0 {
            return Err(GuardianError::config(
                "operation_timeout must be greater than 0",
                None,
            ).into());
        }

        // Validate performance config
        if self.performance.buffer_size_kb == 0 {
            return Err(GuardianError::config(
                "buffer_size_kb must be greater than 0",
                None,
            ).into());
        }

        // Validate patterns
        for pattern in &self.non_production.patterns {
            if pattern.pattern.is_empty() {
                return Err(GuardianError::config(
                    "Non-production pattern cannot be empty",
                    None,
                ).into());
            }
        }

        Ok(())
    }

    /// Get effective worker thread count
    pub fn effective_worker_threads(&self) -> usize {
        if self.performance.worker_threads == 0 {
            num_cpus::get()
        } else {
            self.performance.worker_threads
        }
    }

    /// Create minimal configuration for basic usage
    pub fn minimal() -> Self {
        Self {
            general: GeneralConfig {
                max_file_size: 10 * 1024 * 1024, // 10MB
                include_patterns: vec![
                    "**/*.rs".to_string(),
                    "**/*.py".to_string(),
                    "**/*.js".to_string(),
                ],
                exclude_patterns: vec![
                    "**/target/**".to_string(),
                    "**/node_modules/**".to_string(),
                    "**/.git/**".to_string(),
                ],
                follow_symlinks: false,
                max_depth: Some(10),
            },
            code_quality: CodeQualityConfig::default(),
            dependency: DependencyAnalyzerConfig::default(),
            performance_analyzer: PerformanceAnalyzerConfig::default(),
            security_analyzer: SecurityAnalyzerConfig::default(),
            integrity: IntegrityConfig {
                algorithm: HashAlgorithm::Blake3,
                use_xattr: false,
                baseline_file: None,
                verify_signatures: false,
            },
            lint_drift: LintDriftConfig {
                config_files: vec![
                    ".eslintrc*".to_string(),
                    "pyproject.toml".to_string(),
                ],
                git_repo: None,
                baseline_ref: "main".to_string(),
                check_missing: false,
                custom_rules: HashMap::new(),
            },
            non_production: NonProductionConfig {
                patterns: vec![
                    NonProdPattern {
                        pattern: r"todo!\s*\(".to_string(),
                        description: "TODO macro".to_string(),
                        severity: "high".to_string(),
                        exclude_paths: vec![],
                    },
                ],
                file_extensions: vec!["rs".to_string(), "py".to_string()],
                include_test_dirs: false,
                custom_rules: vec![],
            },
            security: SecurityConfig::default(),
            performance: PerformanceConfig {
                worker_threads: 2,
                buffer_size_kb: 32,
                use_mmap: false,
                memory_pool_mb: 128,
            },
        }
    }

    /// Create security-focused configuration
    pub fn security_focused() -> Self {
        Self {
            general: GeneralConfig {
                max_file_size: 50 * 1024 * 1024, // 50MB
                include_patterns: vec![
                    "**/*.rs".to_string(),
                    "**/*.py".to_string(),
                    "**/*.js".to_string(),
                    "**/*.ts".to_string(),
                    "**/*.go".to_string(),
                    "**/*.java".to_string(),
                    "**/*.c".to_string(),
                    "**/*.cpp".to_string(),
                    "**/*.sh".to_string(),
                    "**/*.yml".to_string(),
                    "**/*.yaml".to_string(),
                    "**/*.json".to_string(),
                ],
                exclude_patterns: vec![
                    "**/target/**".to_string(),
                    "**/node_modules/**".to_string(),
                    "**/.git/**".to_string(),
                    "**/build/**".to_string(),
                ],
                follow_symlinks: false,
                max_depth: Some(15),
            },
            code_quality: CodeQualityConfig::default(),
            dependency: DependencyAnalyzerConfig::default(),
            performance_analyzer: PerformanceAnalyzerConfig::default(),
            security_analyzer: SecurityAnalyzerConfig::default(),
            integrity: IntegrityConfig {
                algorithm: HashAlgorithm::Blake3,
                use_xattr: true,
                baseline_file: Some("security-baseline.json".to_string().into()),
                verify_signatures: true,
            },
            lint_drift: LintDriftConfig::default(),
            non_production: NonProductionConfig {
                patterns: vec![
                    NonProdPattern {
                        pattern: r"todo!\s*\(".to_string(),
                        description: "TODO macro".to_string(),
                        severity: "high".to_string(),
                        exclude_paths: vec![],
                    },
                    NonProdPattern {
                        pattern: r"unimplemented!\s*\(".to_string(),
                        description: "Unimplemented macro".to_string(),
                        severity: "critical".to_string(),
                        exclude_paths: vec![],
                    },
                    NonProdPattern {
                        pattern: r"panic!\s*\(".to_string(),
                        description: "Panic macro".to_string(),
                        severity: "high".to_string(),
                        exclude_paths: vec!["**/tests/**".to_string()],
                    },
                    NonProdPattern {
                        pattern: r#"(?i)(password|secret|key|token)\s*=\s*["'][^"']+["']"#.to_string(),
                        description: "Hardcoded credentials".to_string(),
                        severity: "critical".to_string(),
                        exclude_paths: vec!["**/tests/**".to_string(), "**/examples/**".to_string()],
                    },
                    NonProdPattern {
                        pattern: r#"(?i)api[_-]?key\s*[:=]\s*["'][^"']+["']"#.to_string(),
                        description: "Hardcoded API key".to_string(),
                        severity: "critical".to_string(),
                        exclude_paths: vec!["**/tests/**".to_string()],
                    },
                ],
                file_extensions: vec![
                    "rs".to_string(),
                    "py".to_string(),
                    "js".to_string(),
                    "ts".to_string(),
                    "go".to_string(),
                    "java".to_string(),
                    "sh".to_string(),
                ],
                include_test_dirs: false,
                custom_rules: vec![],
            },
            security: SecurityConfig {
                sandbox_enabled: true,
                max_memory_mb: 512,
                operation_timeout: 180,
                redact_sensitive_data: true,
                allowed_paths: vec![],
            },
            performance: PerformanceConfig::default(),
        }
    }

    /// Create CI-optimized configuration
    pub fn ci_optimized() -> Self {
        let mut config = Self {
            general: GeneralConfig {
                max_file_size: 20 * 1024 * 1024, // 20MB
                include_patterns: vec![
                    "**/*.rs".to_string(),
                    "**/*.py".to_string(),
                    "**/*.js".to_string(),
                    "**/*.ts".to_string(),
                    "**/*.go".to_string(),
                    "**/*.java".to_string(),
                ],
                exclude_patterns: vec![
                    "**/target/**".to_string(),
                    "**/node_modules/**".to_string(),
                    "**/.git/**".to_string(),
                ],
                follow_symlinks: false,
                max_depth: Some(12),
            },
            code_quality: CodeQualityConfig::default(),
            dependency: DependencyAnalyzerConfig::default(),
            performance_analyzer: PerformanceAnalyzerConfig::default(),
            security_analyzer: SecurityAnalyzerConfig::default(),
            security: SecurityConfig::default(),
            performance: PerformanceConfig::default(),
            non_production: NonProductionConfig::default(),
            integrity: IntegrityConfig::default(),
            lint_drift: LintDriftConfig::default(),
        };

        config.security.sandbox_enabled = true;
        config.security.max_memory_mb = 512;
        config.performance.worker_threads = 4;

        config
    }
}

// New analyzer configurations

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyAnalyzerConfig {
    /// Enable dependency analysis
    pub enabled: bool,
    /// Check for vulnerable packages
    pub check_vulnerabilities: bool,
    /// Check for outdated dependencies
    pub check_outdated: bool,
    /// Check for license compliance
    pub check_licenses: bool,
    /// Allowed licenses (empty = allow all)
    pub allowed_licenses: Vec<String>,
}

impl Default for DependencyAnalyzerConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            check_vulnerabilities: true,
            check_outdated: true,
            check_licenses: false,
            allowed_licenses: vec![
                "MIT".to_string(),
                "Apache-2.0".to_string(),
                "BSD-3-Clause".to_string(),
                "ISC".to_string(),
            ],
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
