#![allow(dead_code)]

use serde::{Deserialize, Serialize};

/// Performance configuration for CodeGuardian
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Enable optimized analyzers (uses combined patterns and caching)
    pub enable_optimized_analyzers: bool,

    /// Enable file caching to avoid re-analyzing unchanged files
    pub enable_file_caching: bool,

    /// Maximum number of parallel workers for analysis
    pub max_parallel_workers: usize,

    /// Maximum file size for in-memory analysis (bytes)
    pub max_memory_file_size: u64,

    /// Streaming analysis chunk size (bytes)
    pub streaming_chunk_size: usize,

    /// Maximum findings per file (for performance limiting)
    pub max_findings_per_file: usize,

    /// Pattern cache size (number of cached regex matches)
    pub pattern_cache_size: usize,

    /// Cache cleanup settings
    pub cache_cleanup: CacheCleanupConfig,

    /// Early termination settings
    pub early_termination: EarlyTerminationConfig,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CacheCleanupConfig {
    /// Enable automatic cache cleanup
    pub enabled: bool,

    /// Maximum age of cache entries in days
    pub max_age_days: u64,

    /// Maximum cache size in MB
    pub max_size_mb: usize,

    /// Cleanup frequency (every N runs)
    pub cleanup_frequency: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EarlyTerminationConfig {
    /// Enable early termination for performance
    pub enabled: bool,

    /// Maximum analysis time per file in seconds
    pub max_analysis_time_seconds: u64,

    /// Maximum lines to analyze per file
    pub max_lines_per_file: usize,

    /// Skip files larger than this size (bytes)
    pub skip_large_files_bytes: u64,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            enable_optimized_analyzers: true,
            enable_file_caching: true,
            max_parallel_workers: num_cpus::get().min(8),
            max_memory_file_size: 10 * 1024 * 1024, // 10MB
            streaming_chunk_size: 64 * 1024,        // 64KB
            max_findings_per_file: 50,
            pattern_cache_size: 1000,
            cache_cleanup: CacheCleanupConfig::default(),
            early_termination: EarlyTerminationConfig::default(),
        }
    }
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

impl Default for EarlyTerminationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_analysis_time_seconds: 30,
            max_lines_per_file: 10000,
            skip_large_files_bytes: 50 * 1024 * 1024, // 50MB
        }
    }
}

impl PerformanceConfig {
    /// Create a high-performance configuration for CI/CD environments
    pub fn ci_optimized() -> Self {
        Self {
            enable_optimized_analyzers: true,
            enable_file_caching: true,
            max_parallel_workers: num_cpus::get().min(12), // Optimized for CI
            max_memory_file_size: 3 * 1024 * 1024,         // 3MB - more conservative
            streaming_chunk_size: 64 * 1024,               // 64KB - larger chunks for speed
            max_findings_per_file: 25,                     // Reduced for faster processing
            pattern_cache_size: 1000,                      // Increased cache size
            cache_cleanup: CacheCleanupConfig {
                enabled: true,
                max_age_days: 1,
                max_size_mb: 50,
                cleanup_frequency: 5,
            },
            early_termination: EarlyTerminationConfig {
                enabled: true,
                max_analysis_time_seconds: 5, // Faster timeout
                max_lines_per_file: 3000,     // Reduced for speed
                skip_large_files_bytes: 10 * 1024 * 1024, // 10MB - more aggressive
            },
        }
    }

    /// Create a thorough configuration for comprehensive analysis
    pub fn thorough() -> Self {
        Self {
            enable_optimized_analyzers: false, // Use all analyzers
            enable_file_caching: true,
            max_parallel_workers: num_cpus::get().min(4),
            max_memory_file_size: 50 * 1024 * 1024, // 50MB
            streaming_chunk_size: 128 * 1024,       // 128KB
            max_findings_per_file: 200,
            pattern_cache_size: 2000,
            cache_cleanup: CacheCleanupConfig {
                enabled: true,
                max_age_days: 30,
                max_size_mb: 500,
                cleanup_frequency: 20,
            },
            early_termination: EarlyTerminationConfig {
                enabled: false, // No early termination
                max_analysis_time_seconds: 300,
                max_lines_per_file: 100000,
                skip_large_files_bytes: 500 * 1024 * 1024, // 500MB
            },
        }
    }

    /// Create a maximum performance configuration for large codebases
    pub fn maximum_performance() -> Self {
        Self {
            enable_optimized_analyzers: true,
            enable_file_caching: true,
            max_parallel_workers: num_cpus::get().min(16), // Higher parallelism
            max_memory_file_size: 2 * 1024 * 1024,         // 2MB - very conservative
            streaming_chunk_size: 128 * 1024,              // 128KB - largest chunks
            max_findings_per_file: 15,                     // Minimal findings for speed
            pattern_cache_size: 2000,                      // Maximum cache size
            cache_cleanup: CacheCleanupConfig {
                enabled: false, // Disable cleanup for speed
                max_age_days: 7,
                max_size_mb: 200,
                cleanup_frequency: 20,
            },
            early_termination: EarlyTerminationConfig {
                enabled: true,
                max_analysis_time_seconds: 3, // Very fast timeout
                max_lines_per_file: 2000,     // Minimal lines
                skip_large_files_bytes: 5 * 1024 * 1024, // 5MB - very aggressive
            },
        }
    }

    /// Create an adaptive configuration based on system resources and file count
    pub fn adaptive(file_count: usize, total_size_mb: u64) -> Self {
        let _system_cores = num_cpus::get();

        // Adaptive logic based on workload characteristics
        if file_count > 10000 || total_size_mb > 1000 {
            // Large codebase - prioritize speed over thoroughness
            Self::maximum_performance()
        } else if file_count > 1000 || total_size_mb > 100 {
            // Medium codebase - balanced approach
            Self::ci_optimized()
        } else if file_count < 100 && total_size_mb < 10 {
            // Small codebase - thorough analysis
            Self::thorough()
        } else {
            // Default balanced configuration
            Self::default()
        }
    }

    /// Create a memory-optimized configuration for systems with limited RAM
    pub fn memory_optimized() -> Self {
        Self {
            enable_optimized_analyzers: true,
            enable_file_caching: true,
            max_parallel_workers: num_cpus::get().min(4), // Reduced parallelism
            max_memory_file_size: 1024 * 1024,            // 1MB - very conservative
            streaming_chunk_size: 32 * 1024,              // 32KB - smaller chunks
            max_findings_per_file: 30,
            pattern_cache_size: 500, // Reduced cache size
            cache_cleanup: CacheCleanupConfig {
                enabled: true,
                max_age_days: 3,
                max_size_mb: 50,
                cleanup_frequency: 5,
            },
            early_termination: EarlyTerminationConfig {
                enabled: true,
                max_analysis_time_seconds: 10,
                max_lines_per_file: 5000,
                skip_large_files_bytes: 20 * 1024 * 1024, // 20MB
            },
        }
    }

    /// Create a CPU-optimized configuration for systems with many cores
    pub fn cpu_optimized() -> Self {
        let cores = num_cpus::get();
        Self {
            enable_optimized_analyzers: true,
            enable_file_caching: true,
            max_parallel_workers: cores.min(16), // High parallelism
            max_memory_file_size: 5 * 1024 * 1024, // 5MB
            streaming_chunk_size: 128 * 1024,    // 128KB
            max_findings_per_file: 25,
            pattern_cache_size: 1500,
            cache_cleanup: CacheCleanupConfig {
                enabled: true,
                max_age_days: 7,
                max_size_mb: 150,
                cleanup_frequency: 15,
            },
            early_termination: EarlyTerminationConfig {
                enabled: true,
                max_analysis_time_seconds: 15,
                max_lines_per_file: 8000,
                skip_large_files_bytes: 30 * 1024 * 1024, // 30MB
            },
        }
    }

    /// Validate configuration values
    #[allow(dead_code)]
    pub fn validate(&self) -> Result<(), String> {
        if self.max_parallel_workers == 0 {
            return Err("max_parallel_workers must be greater than 0".to_string());
        }

        if self.max_memory_file_size == 0 {
            return Err("max_memory_file_size must be greater than 0".to_string());
        }

        if self.streaming_chunk_size < 1024 {
            return Err("streaming_chunk_size should be at least 1KB".to_string());
        }

        if self.max_findings_per_file == 0 {
            return Err("max_findings_per_file must be greater than 0".to_string());
        }

        if self.pattern_cache_size == 0 {
            return Err("pattern_cache_size must be greater than 0".to_string());
        }

        Ok(())
    }

    /// Get performance recommendations based on current configuration
    pub fn get_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();

        if self.max_parallel_workers > num_cpus::get() {
            recommendations.push(format!(
                "Consider reducing max_parallel_workers from {} to {} to match system cores",
                self.max_parallel_workers,
                num_cpus::get()
            ));
        }

        if self.max_memory_file_size > 50 * 1024 * 1024 {
            recommendations.push("Large max_memory_file_size may cause memory issues".to_string());
        }

        if self.pattern_cache_size > 2000 {
            recommendations.push("Large pattern_cache_size may use significant memory".to_string());
        }

        if !self.enable_file_caching {
            recommendations
                .push("Enable file caching for better performance on repeated runs".to_string());
        }

        if self.early_termination.max_analysis_time_seconds < 10 {
            recommendations
                .push("Very short analysis timeout may miss important findings".to_string());
        }

        recommendations
    }

    /// Estimate memory usage for the current configuration
    pub fn estimate_memory_usage_mb(&self) -> u64 {
        let base_memory = 50; // Base memory usage in MB
        let cache_memory = (self.pattern_cache_size as u64 * 100) / 1024 / 1024; // Rough estimate
        let worker_memory = self.max_parallel_workers as u64 * 10; // Per worker memory

        base_memory + cache_memory + worker_memory
    }

    /// Check if a file should be skipped based on size limits
    #[allow(dead_code)]
    pub fn should_skip_file(&self, file_size: u64) -> bool {
        self.early_termination.enabled && file_size > self.early_termination.skip_large_files_bytes
    }

    /// Check if streaming analysis should be used for a file
    #[allow(dead_code)]
    pub fn should_use_streaming(&self, file_size: u64) -> bool {
        file_size > self.max_memory_file_size
    }

    /// Get timeout for file analysis
    #[allow(dead_code)]
    pub fn get_analysis_timeout(&self) -> std::time::Duration {
        std::time::Duration::from_secs(self.early_termination.max_analysis_time_seconds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = PerformanceConfig::default();
        assert!(config.validate().is_ok());
        assert!(config.enable_optimized_analyzers);
        assert!(config.enable_file_caching);
    }

    #[test]
    fn test_ci_optimized_config() {
        let config = PerformanceConfig::ci_optimized();
        assert!(config.validate().is_ok());
        assert_eq!(config.max_findings_per_file, 25);
        assert_eq!(config.cache_cleanup.max_age_days, 1);
    }

    #[test]
    fn test_thorough_config() {
        let config = PerformanceConfig::thorough();
        assert!(config.validate().is_ok());
        assert!(!config.enable_optimized_analyzers);
        assert!(!config.early_termination.enabled);
    }

    #[test]
    fn test_file_size_checks() {
        let config = PerformanceConfig::default();

        // Small file - no streaming, no skip
        assert!(!config.should_skip_file(1024));
        assert!(!config.should_use_streaming(1024));

        // Medium file - streaming but no skip
        let medium_size = 20 * 1024 * 1024; // 20MB
        assert!(!config.should_skip_file(medium_size));
        assert!(config.should_use_streaming(medium_size));

        // Large file - skip
        let large_size = 100 * 1024 * 1024; // 100MB
        assert!(config.should_skip_file(large_size));
    }

    #[test]
    fn test_validation() {
        let mut config = PerformanceConfig::default();

        // Valid config
        assert!(config.validate().is_ok());

        // Invalid parallel workers
        config.max_parallel_workers = 0;
        assert!(config.validate().is_err());

        // Reset and test invalid chunk size
        config = PerformanceConfig::default();
        config.streaming_chunk_size = 512; // Too small
        assert!(config.validate().is_err());
    }
}
