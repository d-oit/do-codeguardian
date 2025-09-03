//! Performance optimizer for CodeGuardian
//!
//! This module provides runtime performance optimizations and tuning

use crate::performance::PerformanceMetrics;
use std::sync::Arc;
use std::time::Duration;

/// Runtime performance optimizer
pub struct PerformanceOptimizer {
    metrics: Arc<PerformanceMetrics>,
    optimization_level: OptimizationLevel,
}

#[derive(Debug, Clone, Copy)]
pub enum OptimizationLevel {
    Conservative,
    Balanced,
    Aggressive,
}

impl PerformanceOptimizer {
    pub fn new(metrics: Arc<PerformanceMetrics>, level: OptimizationLevel) -> Self {
        Self {
            metrics,
            optimization_level: level,
        }
    }

    /// Determine optimal worker count based on system and workload
    pub fn optimal_worker_count(&self, file_count: usize) -> usize {
        let cpu_count = num_cpus::get();

        match self.optimization_level {
            OptimizationLevel::Conservative => {
                // Use fewer workers to avoid overwhelming system
                std::cmp::min(cpu_count / 2, file_count)
            }
            OptimizationLevel::Balanced => {
                // Use most CPUs but leave some headroom
                std::cmp::min(cpu_count.saturating_sub(1), file_count)
            }
            OptimizationLevel::Aggressive => {
                // Use all available CPUs
                std::cmp::min(cpu_count * 2, file_count)
            }
        }
    }

    /// Determine optimal chunk size for parallel processing
    pub fn optimal_chunk_size(&self, total_files: usize, worker_count: usize) -> usize {
        let base_chunk_size = total_files / worker_count;

        match self.optimization_level {
            OptimizationLevel::Conservative => {
                // Larger chunks to reduce coordination overhead
                std::cmp::max(base_chunk_size * 2, 10)
            }
            OptimizationLevel::Balanced => {
                // Balanced chunk size
                std::cmp::max(base_chunk_size, 5)
            }
            OptimizationLevel::Aggressive => {
                // Smaller chunks for better load balancing
                std::cmp::max(base_chunk_size / 2, 1)
            }
        }
    }

    /// Suggest memory optimization strategies
    pub fn memory_optimization_strategy(&self) -> MemoryStrategy {
        let cache_hit_rate = self.metrics.get_cache_hit_rate();
        let avg_processing_time = self.metrics.get_average_processing_time();

        if cache_hit_rate < 0.5 {
            MemoryStrategy::ExpandCache
        } else if avg_processing_time > Duration::from_millis(100) {
            MemoryStrategy::OptimizeAlgorithms
        } else {
            MemoryStrategy::Maintain
        }
    }

    /// Auto-tune performance based on runtime metrics
    pub fn auto_tune(&mut self) -> OptimizationRecommendations {
        let mut recommendations = OptimizationRecommendations::new();

        // Analyze cache performance
        let cache_hit_rate = self.metrics.get_cache_hit_rate();
        if cache_hit_rate < 0.8 {
            recommendations.add_cache_optimization(cache_hit_rate);
        }

        // Analyze processing speed
        let avg_time = self.metrics.get_average_processing_time();
        if avg_time > Duration::from_millis(50) {
            recommendations.add_speed_optimization(avg_time);
        }

        // Analyze throughput
        let throughput = self.metrics.get_throughput_files_per_second();
        if throughput < 10.0 {
            recommendations.add_throughput_optimization(throughput);
        }

        recommendations
    }
}

#[derive(Debug)]
pub enum MemoryStrategy {
    ExpandCache,
    OptimizeAlgorithms,
    Maintain,
}

#[derive(Debug)]
pub struct OptimizationRecommendations {
    pub recommendations: Vec<String>,
    pub estimated_improvement: f64, // percentage
}

impl OptimizationRecommendations {
    pub fn new() -> Self {
        Self {
            recommendations: Vec::new(),
            estimated_improvement: 0.0,
        }
    }

    pub fn add_cache_optimization(&mut self, current_rate: f64) {
        self.recommendations.push(format!(
            "Improve cache hit rate from {:.1}% to 85%+ by optimizing cache size and eviction policy",
            current_rate * 100.0
        ));
        self.estimated_improvement += 20.0;
    }

    pub fn add_speed_optimization(&mut self, current_time: Duration) {
        self.recommendations.push(format!(
            "Optimize processing speed from {:.1}ms to <50ms per file through algorithm improvements",
            current_time.as_millis()
        ));
        self.estimated_improvement += 30.0;
    }

    pub fn add_throughput_optimization(&mut self, current_throughput: f64) {
        self.recommendations.push(format!(
            "Increase throughput from {:.1} to 20+ files/second through parallel optimization",
            current_throughput
        ));
        self.estimated_improvement += 25.0;
    }
}

impl Default for OptimizationRecommendations {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimal_worker_count() {
        let metrics = Arc::new(PerformanceMetrics::new());
        let optimizer = PerformanceOptimizer::new(metrics, OptimizationLevel::Balanced);

        let worker_count = optimizer.optimal_worker_count(100);
        assert!(worker_count > 0);
        assert!(worker_count <= num_cpus::get());
    }

    #[test]
    fn test_optimization_recommendations() {
        let metrics = Arc::new(PerformanceMetrics::new());
        let mut optimizer = PerformanceOptimizer::new(metrics, OptimizationLevel::Balanced);

        let recommendations = optimizer.auto_tune();
        assert!(recommendations.estimated_improvement >= 0.0);
    }
}
