use crate::error::GuardianError;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

pub mod performance;

// Constants for default configuration values
const KB: u64 = 1024;
const MB: u64 = KB * 1024;
#[allow(dead_code)]
const GB: u64 = MB * 1024;

const DEFAULT_MAX_FILE_SIZE: u64 = 5 * MB; // 5MB
const DEFAULT_MAX_MEMORY_MB: u64 = 256;
const DEFAULT_PARALLEL_WORKERS: usize = 2;
const DEFAULT_TIMEOUT_SECONDS: u64 = 120;
pub use performance::PerformanceConfig as OptimizationConfig;

/// General configuration settings that apply across all analyzers.
///
/// This struct contains global settings that control the overall behavior
/// of the CodeGuardian analysis engine, including resource limits and
/// file processing rules.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    /// Maximum file size to analyze (in bytes)
    pub max_file_size: u64,
    /// Maximum memory usage limit (in MB)
    pub max_memory_mb: u64,
    /// Number of parallel worker threads
    pub parallel_workers: usize,
    /// Analysis timeout in seconds
    pub timeout_seconds: u64,
    /// File patterns to exclude from analysis
    pub exclude_patterns: Vec<String>,
    /// File patterns to include in analysis
    pub include_patterns: Vec<String>,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            max_file_size: 10 * MB, // 10MB
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

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub enum HashAlgorithm {
    #[default]
    Blake3,
    Sha256,
    Sha512,
}

impl std::fmt::Display for HashAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HashAlgorithm::Blake3 => write!(f, "blake3"),
            HashAlgorithm::Sha256 => write!(f, "sha256"),
            HashAlgorithm::Sha512 => write!(f, "sha512"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrityConfig {
    pub enabled: bool,
    pub hash_algorithm: HashAlgorithm,
    pub baseline_file: String,
    pub auto_update_baseline: bool,
    pub check_permissions: bool,
    pub check_binary_files: bool,
    pub verify_checksums: bool,
    pub max_file_size: u64,
}

impl Default for IntegrityConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            hash_algorithm: HashAlgorithm::Blake3,
            baseline_file: ".codeguardian/integrity.baseline".to_string(),
            auto_update_baseline: false,
            check_permissions: true,
            check_binary_files: true,
            verify_checksums: true,
            max_file_size: 100 * MB,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NonProdPattern {
    pub pattern: String,
    pub description: String,
    pub severity: String,
}

impl NonProdPattern {
    /// Create a new NonProdPattern with validation
    pub fn new(pattern: String, description: String, severity: String) -> Result<Self, String> {
        if pattern.trim().is_empty() {
            return Err("Pattern cannot be empty".to_string());
        }
        if !matches!(severity.as_str(), "low" | "medium" | "high" | "critical") {
            return Err("Severity must be one of: low, medium, high, critical".to_string());
        }
        Ok(Self {
            pattern,
            description,
            severity,
        })
    }

    /// Check if this pattern matches the given text
    #[allow(dead_code)]
    pub fn matches(&self, text: &str) -> bool {
        regex::Regex::new(&self.pattern)
            .map(|re| re.is_match(text))
            .unwrap_or(false)
    }

    /// Get the pattern as a string slice
    #[allow(dead_code)]
    pub fn as_str(&self) -> &str {
        &self.pattern
    }
}

impl AsRef<str> for NonProdPattern {
    fn as_ref(&self) -> &str {
        &self.pattern
    }
}

impl std::fmt::Display for NonProdPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.pattern, self.severity)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NonProductionConfig {
    pub enabled: bool,
    pub patterns: Vec<NonProdPattern>,
    pub exclude_test_files: bool,
    pub exclude_example_files: bool,
}

impl NonProductionConfig {
    /// Check if any pattern matches the given text
    #[allow(dead_code)]
    pub fn matches_any(&self, text: &str) -> bool {
        self.patterns.iter().any(|pattern| pattern.matches(text))
    }

    /// Find patterns that match the given text
    #[allow(dead_code)]
    pub fn find_matches(&self, text: &str) -> Vec<&NonProdPattern> {
        self.patterns
            .iter()
            .filter(|pattern| pattern.matches(text))
            .collect()
    }

    /// Add a new pattern with validation
    #[allow(dead_code)]
    pub fn add_pattern(
        &mut self,
        pattern: String,
        description: String,
        severity: String,
    ) -> Result<(), String> {
        let new_pattern = NonProdPattern::new(pattern, description, severity)?;
        self.patterns.push(new_pattern);
        Ok(())
    }

    /// Remove patterns containing the given substring
    #[allow(dead_code)]
    pub fn remove_patterns_containing(&mut self, substring: &str) {
        self.patterns
            .retain(|pattern| !pattern.pattern.contains(substring));
    }

    /// Get all pattern strings as a Vec<String>
    #[allow(dead_code)]
    pub fn pattern_strings(&self) -> Vec<String> {
        self.patterns.iter().map(|p| p.pattern.clone()).collect()
    }
}

impl Default for NonProductionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            patterns: vec![
                NonProdPattern::new(
                    r"(?i)\b(todo|fixme|hack|xxx)\b".to_string(),
                    "Non-production code markers".to_string(),
                    "medium".to_string(),
                )
                .unwrap_or_else(|_| panic!("Invalid default pattern")),
                NonProdPattern::new(
                    r"(?i)\bdebug\s*!".to_string(),
                    "Debug print statements".to_string(),
                    "low".to_string(),
                )
                .unwrap_or_else(|_| panic!("Invalid default pattern")),
                NonProdPattern::new(
                    r"(?i)\bprintln\s*!".to_string(),
                    "Print statements in production code".to_string(),
                    "low".to_string(),
                )
                .unwrap_or_else(|_| panic!("Invalid default pattern")),
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
    /// Check for license compliance
    pub check_licenses: bool,
    /// Maximum allowed dependency age in days
    pub max_age_days: u32,
    /// Severity threshold for vulnerabilities
    pub vulnerability_threshold: String,
    /// List of vulnerability databases to check
    pub vulnerability_databases: Vec<String>,
}

impl Default for DependencyAnalyzerConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            check_outdated: true,
            check_vulnerabilities: true,
            check_unused: true,
            check_duplicates: true,
            check_licenses: true,
            max_age_days: 365,
            vulnerability_threshold: "medium".to_string(),
            vulnerability_databases: vec![
                "https://cve.mitre.org".to_string(),
                "https://nvd.nist.gov".to_string(),
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
    /// Check for algorithmic inefficiencies
    pub check_algorithms: bool,
    /// Check for memory usage issues
    pub check_memory_usage: bool,
    /// Check for I/O operation inefficiencies
    pub check_io_operations: bool,
    /// Maximum acceptable cyclomatic complexity
    pub max_complexity: usize,
    /// Maximum acceptable function length
    pub max_function_length: usize,
    /// Maximum acceptable loop depth
    pub max_loop_depth: usize,
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
    /// Check for vulnerabilities in dependencies
    pub check_vulnerabilities: bool,
    /// Check for file permission issues
    pub check_permissions: bool,
    /// Check for secrets in code
    pub check_secrets: bool,
    /// Minimum entropy threshold for secret detection
    pub min_entropy_threshold: f64,
    /// Patterns for secret detection
    pub secret_patterns: Vec<String>,
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
            secret_patterns: vec![
                r"(?i)(password|passwd|pwd)\s*[:=]\s*['\x22][^'\x22]{8,}['\x22]".to_string(),
                r"(?i)(api[_-]?key|apikey)\s*[:=]\s*['\x22][^'\x22]{16,}['\x22]".to_string(),
                r"(?i)(secret|token)\s*[:=]\s*['\x22][^'\x22]{16,}['\x22]".to_string(),
            ],
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
    /// Check for cyclomatic complexity
    pub check_complexity: bool,
    /// Check for code duplication
    pub check_duplication: bool,
    /// Check for naming convention violations
    pub check_naming: bool,
    /// Maximum acceptable cyclomatic complexity
    pub max_complexity: usize,
    /// Maximum acceptable nesting depth
    pub max_nesting_depth: usize,
    /// Maximum acceptable file size (lines)
    pub max_file_size: usize,
    /// Maximum acceptable line length
    pub max_line_length: usize,
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

/// Main configuration structure for CodeGuardian.
///
/// This struct contains all configuration settings organized by analyzer type.
/// It provides methods for loading, saving, and validating configuration files,
/// as well as preset configurations for different use cases.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// General settings that apply to all analyzers
    pub general: GeneralConfig,
    /// Integrity checking configuration
    pub integrity: IntegrityConfig,
    /// Lint configuration drift detection settings
    pub lint_drift: LintDriftConfig,
    /// Non-production code detection settings
    pub non_production: NonProductionConfig,
    /// Dependency analysis configuration
    pub dependency: DependencyAnalyzerConfig,
    /// Performance analyzer settings
    pub performance_analyzer: PerformanceAnalyzerConfig,
    /// Security analyzer configuration
    pub security_analyzer: SecurityAnalyzerConfig,
    /// Code quality analysis settings
    pub code_quality: CodeQualityConfig,
    /// Security analysis configuration
    pub security: SecurityConfig,
    /// Performance analysis settings
    pub performance: PerformanceConfig,
    /// Optimization configuration
    pub optimization: OptimizationConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self::minimal()
    }
}

impl Config {
    /// Load configuration from a TOML file.
    ///
    /// Reads and parses a configuration file. If the file doesn't exist,
    /// returns a minimal configuration. The loaded configuration is validated
    /// before being returned.
    ///
    /// # Arguments
    /// * `path` - Path to the configuration file
    ///
    /// # Returns
    /// A validated Config instance or an error if loading/parsing fails
    pub fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::minimal());
        }

        let content = std::fs::read_to_string(path)
            .map_err(|e| anyhow::Error::from(GuardianError::io("Failed to read config file", e)))?;

        let config: Config = toml::from_str(&content).map_err(|e| {
            anyhow::Error::from(GuardianError::config(
                format!("Failed to parse config file: {}", e),
                Some(path.to_path_buf()),
            ))
        })?;

        config.validate()?;
        Ok(config)
    }

    /// Load configuration by searching for codeguardian.toml in the current directory
    /// and parent directories. This is the recommended method for loading configuration
    /// as it handles cases where the tool is run from subdirectories.
    ///
    /// # Returns
    /// A validated Config instance or an error if loading/parsing fails
    pub fn load_from_project_root() -> Result<Self> {
        let config_path = Self::find_config_file()?;
        match config_path {
            Some(path) => {
                eprintln!("Using configuration file: {}", path.display());
                match Self::load(&path) {
                    Ok(config) => Ok(config),
                    Err(e) => {
                        eprintln!("Error loading configuration from {}: {}", path.display(), e);
                        eprintln!("Falling back to defaults");
                        Ok(Self::minimal())
                    }
                }
            }
            None => {
                eprintln!("No configuration file found, using defaults");
                Ok(Self::minimal())
            }
        }
    }

    /// Search for codeguardian.toml starting from the current directory
    /// and moving up the directory tree until found or root is reached.
    ///
    /// # Returns
    /// The path to the config file if found, None otherwise
    pub fn find_config_file() -> Result<Option<std::path::PathBuf>> {
        let mut current_dir = std::env::current_dir()?;

        loop {
            let config_path = current_dir.join("codeguardian.toml");
            if config_path.exists() {
                return Ok(Some(config_path));
            }

            // Move up one directory
            if let Some(parent) = current_dir.parent() {
                current_dir = parent.to_path_buf();
            } else {
                // Reached the root directory
                break;
            }
        }

        Ok(None)
    }

    /// Save configuration to file
    #[allow(dead_code)]
    pub fn save(&self, path: &Path) -> Result<()> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content).map_err(|e| {
            anyhow::Error::from(GuardianError::io("Failed to write config file", e))
        })?;
        Ok(())
    }

    /// Create default configuration file
    #[allow(dead_code)]
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

        if self.general.parallel_workers == 0 {
            return Err(anyhow::Error::from(GuardianError::config(
                "parallel_workers must be greater than 0",
                None,
            )));
        }

        if self.general.timeout_seconds == 0 {
            return Err(anyhow::Error::from(GuardianError::config(
                "timeout_seconds must be greater than 0",
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

    /// Creates a minimal configuration for basic usage.
    ///
    /// This configuration enables only essential analyzers with conservative
    /// settings, suitable for initial setup or environments with limited resources.
    /// It focuses on core security and integrity checks while keeping
    /// performance overhead low.
    pub fn minimal() -> Self {
        Self {
            general: GeneralConfig {
                max_file_size: DEFAULT_MAX_FILE_SIZE,
                max_memory_mb: DEFAULT_MAX_MEMORY_MB,
                parallel_workers: DEFAULT_PARALLEL_WORKERS,
                timeout_seconds: DEFAULT_TIMEOUT_SECONDS,
                exclude_patterns: vec!["target/**".to_string(), ".git/**".to_string()],
                include_patterns: vec!["**/*.rs".to_string()],
            },
            integrity: IntegrityConfig {
                hash_algorithm: HashAlgorithm::Blake3,
                enabled: true,
                baseline_file: ".codeguardian/integrity.baseline".to_string(),
                auto_update_baseline: false,
                check_permissions: false,
                check_binary_files: true,
                verify_checksums: false,
                max_file_size: DEFAULT_MAX_FILE_SIZE,
            },
            lint_drift: LintDriftConfig {
                enabled: true,
                config_files: vec!["Cargo.toml".to_string()],
                baseline_file: ".codeguardian/lint_drift.baseline".to_string(),
                auto_update_baseline: false,
                strict_mode: false,
            },
            non_production: NonProductionConfig {
                enabled: true,
                patterns: vec![NonProdPattern::new(
                    r"(?i)\b(todo|fixme|hack|xxx)\b".to_string(),
                    "Non-production code markers".to_string(),
                    "medium".to_string(),
                )
                .unwrap_or_else(|_| panic!("Invalid default pattern"))],
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

    /// Creates a security-focused configuration.
    ///
    /// This configuration enables all security-related analyzers with
    /// thorough settings, optimized for maximum security coverage.
    /// It includes comprehensive vulnerability detection, secret scanning,
    /// and security best practice enforcement.
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

    /// Creates a CI-optimized configuration.
    ///
    /// This configuration is tuned for continuous integration environments,
    /// balancing thorough analysis with fast execution times. It enables
    /// all analyzers but uses settings optimized for CI performance and
    /// reliability.
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::TempDir;

    #[test]
    fn test_general_config_default() {
        let config = GeneralConfig::default();
        assert_eq!(config.max_file_size, 10 * MB);
        assert_eq!(config.max_memory_mb, 512);
        assert_eq!(config.parallel_workers, num_cpus::get());
        assert_eq!(config.timeout_seconds, 300);
        assert!(!config.exclude_patterns.is_empty());
        assert!(!config.include_patterns.is_empty());
    }

    #[test]
    fn test_integrity_config_default() {
        let config = IntegrityConfig::default();
        assert_eq!(config.hash_algorithm, HashAlgorithm::Blake3);
        assert!(config.check_binary_files);
        assert!(config.verify_checksums);
        assert_eq!(config.max_file_size, 100 * MB);
    }

    #[test]
    fn test_hash_algorithm_display() {
        assert_eq!(HashAlgorithm::Blake3.to_string(), "blake3");
        assert_eq!(HashAlgorithm::Sha256.to_string(), "sha256");
        assert_eq!(HashAlgorithm::Sha512.to_string(), "sha512");
    }

    #[test]
    fn test_config_minimal() {
        let config = Config::minimal();
        assert_eq!(config.general.max_file_size, DEFAULT_MAX_FILE_SIZE);
        assert_eq!(config.general.max_memory_mb, DEFAULT_MAX_MEMORY_MB);
        assert_eq!(config.general.parallel_workers, DEFAULT_PARALLEL_WORKERS);
        assert_eq!(config.general.timeout_seconds, DEFAULT_TIMEOUT_SECONDS);
    }

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.general.max_file_size, DEFAULT_MAX_FILE_SIZE);
        assert!(config.integrity.check_binary_files);
        assert!(config.lint_drift.enabled);
        assert!(config.non_production.enabled);
    }

    #[test]
    fn test_config_security_focused() {
        let config = Config::security_focused();
        // Should use thorough optimization for security
        assert_eq!(config.optimization, OptimizationConfig::thorough());
    }

    #[test]
    fn test_config_ci_optimized() {
        let config = Config::ci_optimized();
        // Should use CI-optimized settings
        assert_eq!(config.optimization, OptimizationConfig::ci_optimized());
    }

    #[tokio::test]
    async fn test_config_load_from_file() {
        let temp_dir = TempDir::new().unwrap();
        let config_file = temp_dir.path().join("test_config.toml");

        // Create a test config by modifying the minimal config
        let mut test_config = Config::minimal();
        test_config.general.max_file_size = 10 * MB; // 10MB
        test_config.general.max_memory_mb = 256;
        test_config.general.parallel_workers = 4;
        test_config.integrity.hash_algorithm = HashAlgorithm::Sha256;

        // Save the config to file
        test_config.save(&config_file).unwrap();

        // Load it back
        let loaded_config = Config::load(&config_file).unwrap();

        // Verify the loaded config matches what we saved
        assert_eq!(loaded_config.general.max_file_size, 10 * MB);
        assert_eq!(loaded_config.general.max_memory_mb, 256);
        assert_eq!(loaded_config.general.parallel_workers, 4);
        assert_eq!(
            loaded_config.integrity.hash_algorithm,
            HashAlgorithm::Sha256
        );
    }

    #[tokio::test]
    async fn test_config_load_nonexistent_file() {
        let nonexistent_path = PathBuf::from("/nonexistent/config.toml");
        let result = Config::load(&nonexistent_path);
        // Should return minimal config when file doesn't exist
        assert!(result.is_ok());
        let config = result.unwrap();
        assert_eq!(config.general.max_file_size, DEFAULT_MAX_FILE_SIZE);
        assert_eq!(config.general.parallel_workers, DEFAULT_PARALLEL_WORKERS);
    }

    #[test]
    fn test_config_validation() {
        let mut config = Config::default();

        // Valid config should pass validation
        assert!(config.validate().is_ok());

        // Invalid max_file_size should fail
        config.general.max_file_size = 0;
        assert!(config.validate().is_err());

        // Reset and test invalid parallel_workers
        config.general.max_file_size = DEFAULT_MAX_FILE_SIZE;
        config.general.parallel_workers = 0;
        assert!(config.validate().is_err());

        // Reset and test invalid timeout
        config.general.parallel_workers = DEFAULT_PARALLEL_WORKERS;
        config.general.timeout_seconds = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_non_production_config_default() {
        let config = NonProductionConfig::default();
        assert!(config.enabled);
        assert!(!config.patterns.is_empty());
        assert!(config.patterns.iter().any(|p| p.pattern.contains("todo")));
        assert!(config.patterns.iter().any(|p| p.pattern.contains("fixme")));
        assert!(config.patterns.iter().any(|p| p.pattern.contains("debug")));
    }

    #[test]
    fn test_non_prod_pattern_new() {
        // Valid pattern
        let pattern = NonProdPattern::new(
            r"(?i)\btest\b".to_string(),
            "Test pattern".to_string(),
            "low".to_string(),
        );
        assert!(pattern.is_ok());

        // Invalid pattern - empty
        let empty_pattern = NonProdPattern::new(
            "".to_string(),
            "Empty pattern".to_string(),
            "low".to_string(),
        );
        assert!(empty_pattern.is_err());

        // Invalid severity
        let invalid_severity = NonProdPattern::new(
            r"(?i)\btest\b".to_string(),
            "Test pattern".to_string(),
            "invalid".to_string(),
        );
        assert!(invalid_severity.is_err());
    }

    #[test]
    fn test_non_prod_pattern_matches() {
        let pattern = NonProdPattern::new(
            r"(?i)\btest\b".to_string(),
            "Test pattern".to_string(),
            "low".to_string(),
        )
        .unwrap();

        assert!(pattern.matches("This is a test"));
        assert!(pattern.matches("TEST"));
        assert!(!pattern.matches("testing"));
        assert!(!pattern.matches("hello world"));
    }

    #[test]
    fn test_non_prod_pattern_as_ref() {
        let pattern = NonProdPattern::new(
            r"(?i)\btest\b".to_string(),
            "Test pattern".to_string(),
            "low".to_string(),
        )
        .unwrap();

        assert_eq!(pattern.as_ref(), r"(?i)\btest\b");
        assert_eq!(pattern.as_str(), r"(?i)\btest\b");
    }

    #[test]
    fn test_non_production_config_methods() {
        let mut config = NonProductionConfig::default();

        // Test matches_any
        assert!(config.matches_any("TODO: fix this"));

        // Test find_matches
        let matches = config.find_matches("FIXME: handle this");
        assert!(!matches.is_empty());

        // Test add_pattern
        let result = config.add_pattern(
            r"(?i)\brefactor\b".to_string(),
            "Refactor markers".to_string(),
            "medium".to_string(),
        );
        assert!(result.is_ok());
        assert!(config.matches_any("REFACTOR: improve this"));

        // Test pattern_strings
        let pattern_strings = config.pattern_strings();
        assert!(pattern_strings.iter().any(|p| p.contains("refactor")));

        // Test remove_patterns_containing
        config.remove_patterns_containing("refactor");
        assert!(!config.matches_any("REFACTOR: improve this"));
    }

    #[test]
    fn test_dependency_analyzer_config_default() {
        let config = DependencyAnalyzerConfig::default();
        assert!(config.enabled);
        assert!(!config.vulnerability_databases.is_empty());
        assert!(config.check_licenses);
        assert!(config.check_outdated);
    }

    #[test]
    fn test_security_analyzer_config_default() {
        let config = SecurityAnalyzerConfig::default();
        assert!(config.enabled);
        assert!(config.check_secrets);
        assert!(config.check_vulnerabilities);
        assert!(config.check_permissions);
        assert!(!config.secret_patterns.is_empty());
    }

    #[test]
    fn test_code_quality_config_default() {
        let config = CodeQualityConfig::default();
        assert!(config.enabled);
        assert!(config.check_complexity);
        assert!(config.check_duplication);
        assert!(config.check_naming);
        assert!(config.max_complexity > 0);
        assert!(config.max_line_length > 0);
    }

    #[test]
    fn test_performance_analyzer_config_default() {
        let config = PerformanceAnalyzerConfig::default();
        assert!(config.enabled);
        assert!(config.check_algorithms);
        assert!(config.check_memory_usage);
        assert!(config.check_io_operations);
        assert!(config.max_loop_depth > 0);
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let serialized = toml::to_string(&config).unwrap();
        assert!(serialized.contains("[general]"));
        assert!(serialized.contains("[integrity]"));
        assert!(serialized.contains("[lint_drift]"));

        // Should be able to deserialize back
        let deserialized: Config = toml::from_str(&serialized).unwrap();
        assert_eq!(
            config.general.max_file_size,
            deserialized.general.max_file_size
        );
    }
}
