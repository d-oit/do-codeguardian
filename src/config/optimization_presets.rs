use crate::config::{Config, PerformanceConfig};
use std::collections::HashMap;

// Constants for configuration values
const DEFAULT_TIMEOUT_SECONDS: u64 = 120;

/// Optimization presets for different scenarios
#[derive(Debug, Clone)]
pub enum OptimizationScenario {
    /// Development environment with fast feedback needed
    Development,
    /// CI/CD environment with consistent performance
    CI,
    /// Large codebase analysis with maximum throughput
    LargeCodebase,
    /// Memory-constrained environment
    MemoryConstrained,
    /// CPU-intensive environment with many cores
    CPUIntensive,
    /// Security-focused analysis with thorough checking
    SecurityAudit,
    /// Performance regression testing
    PerformanceTesting,
    /// Custom configuration
    Custom,
}

impl OptimizationScenario {
    /// Get recommended configuration for this scenario
    pub fn get_config(&self) -> Config {
        match self {
            OptimizationScenario::Development => Config::development_optimized(),
            OptimizationScenario::CI => Config::ci_optimized(),
            OptimizationScenario::LargeCodebase => Config::large_codebase_optimized(),
            OptimizationScenario::MemoryConstrained => Config::memory_constrained_optimized(),
            OptimizationScenario::CPUIntensive => Config::cpu_intensive_optimized(),
            OptimizationScenario::SecurityAudit => Config::security_audit_optimized(),
            OptimizationScenario::PerformanceTesting => Config::performance_testing_optimized(),
            OptimizationScenario::Custom => Config::minimal(),
        }
    }

    /// Get description of the scenario
    pub fn description(&self) -> &'static str {
        match self {
            OptimizationScenario::Development => "Development environment - fast feedback, moderate analysis",
            OptimizationScenario::CI => "CI/CD environment - consistent performance, automated analysis",
            OptimizationScenario::LargeCodebase => "Large codebase - maximum throughput, optimized for scale",
            OptimizationScenario::MemoryConstrained => "Memory-constrained - reduced memory usage, streaming analysis",
            OptimizationScenario::CPUIntensive => "CPU-intensive - parallel processing, high throughput",
            OptimizationScenario::SecurityAudit => "Security audit - thorough analysis, all security checks enabled",
            OptimizationScenario::PerformanceTesting => "Performance testing - detailed metrics, regression detection",
            OptimizationScenario::Custom => "Custom configuration - user-defined settings",
        }
    }

    /// Get performance characteristics for this scenario
    pub fn performance_characteristics(&self) -> PerformanceCharacteristics {
        match self {
            OptimizationScenario::Development => PerformanceCharacteristics {
                expected_scan_speed: ScanSpeed::Fast,
                memory_usage: MemoryUsage::Moderate,
                analysis_depth: AnalysisDepth::Standard,
                parallel_processing: ParallelProcessing::Adaptive,
            },
            OptimizationScenario::CI => PerformanceCharacteristics {
                expected_scan_speed: ScanSpeed::Balanced,
                memory_usage: MemoryUsage::Moderate,
                analysis_depth: AnalysisDepth::Standard,
                parallel_processing: ParallelProcessing::Fixed,
            },
            OptimizationScenario::LargeCodebase => PerformanceCharacteristics {
                expected_scan_speed: ScanSpeed::Fast,
                memory_usage: MemoryUsage::High,
                analysis_depth: AnalysisDepth::Optimized,
                parallel_processing: ParallelProcessing::Maximum,
            },
            OptimizationScenario::MemoryConstrained => PerformanceCharacteristics {
                expected_scan_speed: ScanSpeed::Slow,
                memory_usage: MemoryUsage::Low,
                analysis_depth: AnalysisDepth::Standard,
                parallel_processing: ParallelProcessing::Minimal,
            },
            OptimizationScenario::CPUIntensive => PerformanceCharacteristics {
                expected_scan_speed: ScanSpeed::Fast,
                memory_usage: MemoryUsage::High,
                analysis_depth: AnalysisDepth::Standard,
                parallel_processing: ParallelProcessing::Maximum,
            },
            OptimizationScenario::SecurityAudit => PerformanceCharacteristics {
                expected_scan_speed: ScanSpeed::Slow,
                memory_usage: MemoryUsage::High,
                analysis_depth: AnalysisDepth::Maximum,
                parallel_processing: ParallelProcessing::Adaptive,
            },
            OptimizationScenario::PerformanceTesting => PerformanceCharacteristics {
                expected_scan_speed: ScanSpeed::Balanced,
                memory_usage: MemoryUsage::High,
                analysis_depth: AnalysisDepth::Maximum,
                parallel_processing: ParallelProcessing::Adaptive,
            },
            OptimizationScenario::Custom => PerformanceCharacteristics {
                expected_scan_speed: ScanSpeed::Balanced,
                memory_usage: MemoryUsage::Moderate,
                analysis_depth: AnalysisDepth::Standard,
                parallel_processing: ParallelProcessing::Adaptive,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct PerformanceCharacteristics {
    pub expected_scan_speed: ScanSpeed,
    pub memory_usage: MemoryUsage,
    pub analysis_depth: AnalysisDepth,
    pub parallel_processing: ParallelProcessing,
}

#[derive(Debug, Clone)]
pub enum ScanSpeed {
    Slow,      // Thorough analysis, comprehensive checks
    Balanced,  // Good balance of speed and thoroughness
    Fast,      // Optimized for speed, may miss some issues
}

#[derive(Debug, Clone)]
pub enum MemoryUsage {
    Low,       // Minimal memory usage, streaming analysis
    Moderate,  // Standard memory usage
    High,      // High memory usage for performance
}

#[derive(Debug, Clone)]
pub enum AnalysisDepth {
    Optimized, // Use optimized analyzers only
    Standard,  // Standard analysis depth
    Maximum,   // All analyzers, maximum depth
}

#[derive(Debug, Clone)]
pub enum ParallelProcessing {
    Minimal,   // 1-2 workers
    Fixed,     // Fixed number of workers
    Adaptive,  // Adaptive based on system load
    Maximum,   // Maximum available cores
}

/// Configuration optimizer for automatic optimization
pub struct ConfigurationOptimizer {
    system_info: SystemInfo,
    optimization_history: HashMap<OptimizationScenario, OptimizationResult>,
}

#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub cpu_cores: usize,
    pub total_memory_gb: usize,
    pub available_memory_gb: usize,
    pub os_type: String,
    pub is_ci_environment: bool,
}

#[derive(Debug, Clone)]
pub struct OptimizationResult {
    pub scenario: OptimizationScenario,
    pub performance_score: f64,
    pub memory_efficiency: f64,
    pub analysis_completeness: f64,
    pub applied_at: std::time::Instant,
}

impl ConfigurationOptimizer {
    pub fn new() -> Self {
        Self {
            system_info: Self::detect_system_info(),
            optimization_history: HashMap::new(),
        }
    }

    /// Automatically detect optimal configuration based on system and environment
    pub fn auto_detect_scenario(&self) -> OptimizationScenario {
        // Check for CI environment
        if self.system_info.is_ci_environment {
            return OptimizationScenario::CI;
        }

        // Check memory constraints
        if self.system_info.available_memory_gb < 4 {
            return OptimizationScenario::MemoryConstrained;
        }

        // Check CPU resources
        if self.system_info.cpu_cores >= 8 {
            return OptimizationScenario::CPUIntensive;
        }

        // Default to development for most cases
        OptimizationScenario::Development
    }

    /// Get optimized configuration for current system
    pub fn get_optimized_config(&self) -> Config {
        let scenario = self.auto_detect_scenario();
        scenario.get_config()
    }

    /// Analyze codebase and recommend optimization scenario
    pub fn analyze_codebase(&self, file_count: usize, total_size_mb: u64) -> OptimizationScenario {
        // Large codebase
        if file_count > 10000 || total_size_mb > 1000 {
            return OptimizationScenario::LargeCodebase;
        }

        // Medium codebase
        if file_count > 1000 || total_size_mb > 100 {
            return OptimizationScenario::CI;
        }

        // Small codebase - thorough analysis
        if file_count < 100 && total_size_mb < 10 {
            return OptimizationScenario::SecurityAudit;
        }

        // Default based on system
        self.auto_detect_scenario()
    }

    /// Record optimization result for learning
    pub fn record_optimization_result(&mut self, scenario: OptimizationScenario, result: OptimizationResult) {
        self.optimization_history.insert(scenario, result);
    }

    /// Get optimization recommendations
    pub fn get_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();

        let scenario = self.auto_detect_scenario();
        let characteristics = scenario.performance_characteristics();

        recommendations.push(format!("Recommended scenario: {}", scenario.description()));
        recommendations.push(format!("Expected scan speed: {:?}", characteristics.expected_scan_speed));
        recommendations.push(format!("Memory usage: {:?}", characteristics.memory_usage));
        recommendations.push(format!("Analysis depth: {:?}", characteristics.analysis_depth));
        recommendations.push(format!("Parallel processing: {:?}", characteristics.parallel_processing));

        // System-specific recommendations
        if self.system_info.cpu_cores < 4 {
            recommendations.push("Consider using fewer parallel workers for better stability".to_string());
        }

        if self.system_info.available_memory_gb < 8 {
            recommendations.push("Memory-constrained environment detected, consider using streaming analysis".to_string());
        }

        if self.system_info.is_ci_environment {
            recommendations.push("CI environment detected, using optimized CI configuration".to_string());
        }

        recommendations
    }

    fn detect_system_info() -> SystemInfo {
        let cpu_cores = num_cpus::get();

        // Estimate memory (simplified - in production would use system APIs)
        let total_memory_gb = (sys_info::mem_info()
            .map(|m| m.total / 1024 / 1024 / 1024)
            .unwrap_or(8)) as usize;

        let available_memory_gb = total_memory_gb * 3 / 4; // Assume 75% available

        let os_type = std::env::consts::OS.to_string();

        // Detect CI environment
        let is_ci_environment = std::env::var("CI").is_ok() ||
                              std::env::var("CONTINUOUS_INTEGRATION").is_ok() ||
                              std::env::var("BUILD_NUMBER").is_ok();

        SystemInfo {
            cpu_cores,
            total_memory_gb,
            available_memory_gb,
            os_type,
            is_ci_environment,
        }
    }
}

/// Helper functions for creating optimized configurations
pub mod config_helpers {
    use super::*;

    /// Create a development-optimized configuration
    pub fn development_optimized() -> Config {
        let mut config = Config::minimal();
        config.general.parallel_workers = 2;
        config.general.timeout_seconds = 30;
        config.performance = PerformanceConfig::default();
        config
    }

    /// Create a CI-optimized configuration
    pub fn ci_optimized() -> Config {
        let mut config = Config::minimal();
        config.general.parallel_workers = num_cpus::get().min(8);
        config.general.timeout_seconds = DEFAULT_TIMEOUT_SECONDS;
        config.performance = PerformanceConfig::ci_optimized();
        config
    }

    /// Create a large codebase optimized configuration
    pub fn large_codebase_optimized() -> Config {
        let mut config = Config::minimal();
        config.general.parallel_workers = num_cpus::get().min(16);
        config.general.timeout_seconds = 300;
        config.performance = PerformanceConfig::maximum_performance();
        config
    }

    /// Create a memory-constrained optimized configuration
    pub fn memory_constrained_optimized() -> Config {
        let mut config = Config::minimal();
        config.general.parallel_workers = 1;
        config.general.max_memory_mb = 256;
        config.performance = PerformanceConfig::memory_optimized();
        config
    }

    /// Create a CPU-intensive optimized configuration
    pub fn cpu_intensive_optimized() -> Config {
        let mut config = Config::minimal();
        config.general.parallel_workers = num_cpus::get();
        config.performance = PerformanceConfig::cpu_optimized();
        config
    }

    /// Create a security audit optimized configuration
    pub fn security_audit_optimized() -> Config {
        let mut config = Config::security_focused();
        config.general.timeout_seconds = 600; // Longer timeout for thorough analysis
        config.performance = PerformanceConfig::thorough();
        config
    }

    /// Create a performance testing optimized configuration
    pub fn performance_testing_optimized() -> Config {
        let mut config = Config::minimal();
        config.general.parallel_workers = num_cpus::get().min(8);
        config.performance = PerformanceConfig::default();
        // Enable all performance tracking
        config.performance_analyzer.enabled = true;
        config.performance_analyzer.max_complexity = 15;
        config
    }
}

// Extend Config with the new optimization methods
impl Config {
    pub fn development_optimized() -> Self {
        config_helpers::development_optimized()
    }

    pub fn large_codebase_optimized() -> Self {
        config_helpers::large_codebase_optimized()
    }

    pub fn memory_constrained_optimized() -> Self {
        config_helpers::memory_constrained_optimized()
    }

    pub fn cpu_intensive_optimized() -> Self {
        config_helpers::cpu_intensive_optimized()
    }

    pub fn security_audit_optimized() -> Self {
        config_helpers::security_audit_optimized()
    }

    pub fn performance_testing_optimized() -> Self {
        config_helpers::performance_testing_optimized()
    }
}