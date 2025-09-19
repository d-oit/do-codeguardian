//! # Base Configuration Module
//!
//! This module contains the core configuration structures and basic functionality
//! for loading and managing CodeGuardian configuration.

use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Default configuration for CodeGuardian
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    /// Output configuration
    pub output: super::output::OutputConfig,
    /// Security-related configuration
    pub security: super::security::SecurityConfig,
    /// Git-related configuration
    pub git: GitConfig,
    /// Performance optimization settings
    pub performance: PerformanceConfig,
    /// Analyzer-specific configurations
    pub analyzers: super::analysis::AnalyzerConfigs,
    /// File inclusion/exclusion patterns
    pub files: FileConfig,
    /// Severity level customizations
    pub severity: SeverityConfig,
    /// Integration settings
    pub integrations: IntegrationConfig,
    /// General analysis settings
    pub analysis: AnalysisConfig,
    /// AI enhancement configuration
    pub ai: AIConfig,
    /// Optimization settings
    pub optimization: OptimizationConfig,
    /// Retention policy configuration
    pub retention: super::retention::RetentionConfig,
}

impl Config {
    /// Load configuration from a TOML file
    pub fn from_file(path: &Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;
        let processed_content = Self::process_env_vars(&content)?;

        // First, try to parse as TOML to get better error messages
        let toml_value: toml::Value = toml::from_str(&processed_content).with_context(|| {
            format!(
                "Failed to parse config file: {} - TOML syntax error",
                path.display()
            )
        })?;

        // Then try to deserialize into Config struct with better error handling
        let config: Config = toml_value.try_into().map_err(|e| {
            anyhow::anyhow!(
                "Failed to parse config file: {} - configuration structure error: {}. \
                 This may be due to missing required fields or type mismatches. \
                 Consider using 'codeguardian init --default' to generate a valid config.",
                path.display(),
                e
            )
        })?;

        config.validate()?;
        Ok(config)
    }

    /// Process environment variable substitution in configuration content
    pub fn process_env_vars(content: &str) -> anyhow::Result<String> {
        let mut result = content.to_string();
        let env_var_pattern = regex::Regex::new(r"\$\{([^}]+)\}").unwrap();

        for capture in env_var_pattern.captures_iter(content) {
            let placeholder = &capture[0];
            let var_name = &capture[1];

            match std::env::var(var_name) {
                Ok(value) => {
                    result = result.replace(placeholder, &value);
                }
                Err(_) => {
                    // Keep the placeholder if environment variable is not set
                    // This maintains backward compatibility
                }
            }
        }

        Ok(result)
    }

    /// Validate configuration settings
    pub fn validate(&self) -> anyhow::Result<()> {
        // Validate GitHub configuration
        if self.integrations.github.enabled {
            if self.integrations.github.repository.is_empty() {
                return Err(anyhow::anyhow!(
                    "GitHub integration is enabled but repository is not configured. \
                     Please set 'integrations.github.repository' in your configuration file."
                ));
            }

            if self.integrations.github.token.is_empty()
                || self.integrations.github.token.starts_with("${")
            {
                return Err(anyhow::anyhow!(
                    "GitHub integration is enabled but token is not configured. \
                     Please set the CODEGUARDIAN_GITHUB_TOKEN environment variable or configure 'integrations.github.token'."
                ));
            }

            // Validate repository format (owner/repo)
            if !self.integrations.github.repository.contains('/') {
                return Err(anyhow::anyhow!(
                    "GitHub repository must be in 'owner/repo' format, got: {}",
                    self.integrations.github.repository
                ));
            }
        }

        // Validate GitLab configuration
        if self.integrations.gitlab.enabled {
            if self.integrations.gitlab.project.is_empty() {
                return Err(anyhow::anyhow!(
                    "GitLab integration is enabled but project is not configured. \
                     Please set 'integrations.gitlab.project' in your configuration file."
                ));
            }

            if self.integrations.gitlab.token.is_empty()
                || self.integrations.gitlab.token.starts_with("${")
            {
                return Err(anyhow::anyhow!(
                    "GitLab integration is enabled but token is not configured. \
                     Please set the CODEGUARDIAN_GITLAB_TOKEN environment variable or configure 'integrations.gitlab.token'."
                ));
            }
        }

        Ok(())
    }

    /// Load configuration from a TOML file
    ///
    /// Async version of from_file for use in async contexts
    pub async fn from_file_async(path: &Path) -> anyhow::Result<Self> {
        let content = tokio::fs::read_to_string(path)
            .await
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;
        let processed_content = Self::process_env_vars(&content)?;
        let config: Config = toml::from_str(&processed_content)
            .with_context(|| format!("Failed to parse config file: {}", path.display()))?;
        config.validate()?;
        Ok(config)
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
    /// Regex cache settings
    pub regex_cache: RegexCacheConfig,
    /// Memory pool settings
    pub memory_pools: MemoryPoolConfig,
    /// Enhanced cache settings
    pub enhanced_cache: EnhancedCacheConfig,
    /// Performance monitoring settings
    pub monitoring: PerformanceMonitoringConfig,
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
            regex_cache: RegexCacheConfig::default(),
            memory_pools: MemoryPoolConfig::default(),
            enhanced_cache: EnhancedCacheConfig::default(),
            monitoring: PerformanceMonitoringConfig::default(),
        }
    }
}

/// Regex cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegexCacheConfig {
    /// Enable regex caching
    pub enabled: bool,
    /// Maximum capacity of the regex cache
    pub capacity: usize,
    /// Cache expiration time in seconds
    pub expiration_seconds: u64,
    /// Cache eviction policy
    pub eviction_policy: String,
}

impl Default for RegexCacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            capacity: 1000,
            expiration_seconds: 3600,
            eviction_policy: "lru".to_string(),
        }
    }
}

/// Memory pool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryPoolConfig {
    /// Enable memory pools
    pub enabled: bool,
    /// Pool size for findings
    pub findings_pool_size: usize,
    /// Pool size for strings
    pub strings_pool_size: usize,
    /// Pool size for PathBuf objects
    pub pathbuf_pool_size: usize,
    /// Pool size for HashMap objects
    pub hashmap_pool_size: usize,
}

impl Default for MemoryPoolConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            findings_pool_size: 1000,
            strings_pool_size: 5000,
            pathbuf_pool_size: 2000,
            hashmap_pool_size: 500,
        }
    }
}

/// Enhanced cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedCacheConfig {
    /// Enable enhanced caching
    pub enabled: bool,
    /// Memory limit for cache in MB
    pub memory_limit_mb: usize,
    /// Enable pool integration
    pub pool_integration: bool,
    /// Enable compression for cached data
    pub compression_enabled: bool,
}

impl Default for EnhancedCacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            memory_limit_mb: 256,
            pool_integration: true,
            compression_enabled: false,
        }
    }
}

/// Performance monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMonitoringConfig {
    /// Enable performance monitoring
    pub enabled: bool,
    /// Enable metrics collection
    pub metrics_collection: bool,
    /// Reporting interval in seconds
    pub reporting_interval_seconds: u64,
}

impl Default for PerformanceMonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            metrics_collection: true,
            reporting_interval_seconds: 60,
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
    /// Baseline file path for comparison
    pub baseline_file: Option<PathBuf>,
    /// ML threshold for anomaly detection (0.0-1.0)
    pub ml_threshold: Option<f64>,
    /// Enable AI enhancement
    pub enable_ai_enhancement: bool,
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
            baseline_file: None,
            ml_threshold: None,
            enable_ai_enhancement: false,
        }
    }
}

/// AI enhancement configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConfig {
    /// Enable AI enhancement
    pub enabled: bool,
    /// Enable semantic enrichment
    pub enable_semantic_enrichment: bool,
    /// Enable relationship detection
    pub enable_relationship_detection: bool,
    /// Enable insight generation
    pub enable_insight_generation: bool,
    /// Enable context analysis
    pub enable_context_analysis: bool,
    /// Minimum confidence threshold for insights
    pub min_confidence_threshold: f64,
    /// Maximum processing time in seconds
    pub max_processing_time: u32,
    /// Enable historical analysis
    pub enable_historical_analysis: bool,
    /// Model cache directory
    pub model_cache_directory: Option<String>,
}

impl Default for AIConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            enable_semantic_enrichment: true,
            enable_relationship_detection: true,
            enable_insight_generation: true,
            enable_context_analysis: false,
            min_confidence_threshold: 0.7,
            max_processing_time: 300,
            enable_historical_analysis: false,
            model_cache_directory: Some(".codeguardian/models".to_string()),
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
