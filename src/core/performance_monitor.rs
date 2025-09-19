//! Performance monitoring system for tracking swarm performance and identifying bottlenecks

use crate::core::swarm_types::{SwarmError, SwarmPerformanceMetrics, TaskMetrics, TaskResult};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

/// Performance monitor for tracking swarm execution metrics
pub struct SwarmPerformanceMonitor {
    start_time: Instant,
    task_metrics: Arc<Mutex<HashMap<String, TaskPerformanceData>>>,
    system_metrics: Arc<Mutex<SystemPerformanceData>>,
    performance_history: Arc<Mutex<Vec<PerformanceSnapshot>>>,
    config: PerformanceConfig,
}

impl SwarmPerformanceMonitor {
    pub fn new(config: PerformanceConfig) -> Self {
        Self {
            start_time: Instant::now(),
            task_metrics: Arc::new(Mutex::new(HashMap::new())),
            system_metrics: Arc::new(Mutex::new(SystemPerformanceData::default())),
            performance_history: Arc::new(Mutex::new(Vec::new())),
            config,
        }
    }

    /// Start monitoring a task
    pub async fn start_task_monitoring(&self, task_id: &str) -> Result<(), SwarmError> {
        let mut metrics = self.task_metrics.lock().await;
        metrics.insert(
            task_id.to_string(),
            TaskPerformanceData {
                end_time: None,
                final_metrics: None,
                task_id: task_id.to_string(),
                start_time: Instant::now(),
                cpu_samples: Vec::new(),
                memory_samples: Vec::new(),
                io_samples: Vec::new(),
                status: TaskStatus::Running,
            },
        );
        Ok(())
    }

    /// Update metrics for a running task
    pub async fn update_task_metrics(
        &self,
        task_id: &str,
        metrics: TaskMetrics,
    ) -> Result<(), SwarmError> {
        let mut task_data = self.task_metrics.lock().await;
        if let Some(data) = task_data.get_mut(task_id) {
            data.cpu_samples
                .push((Instant::now(), metrics.cpu_usage_percent));
            data.memory_samples
                .push((Instant::now(), metrics.memory_usage_mb as f64));
            data.io_samples
                .push((Instant::now(), metrics.io_operations as f64));
        }
        Ok(())
    }

    /// Complete monitoring for a task
    pub async fn complete_task_monitoring(
        &self,
        task_id: &str,
        result: &TaskResult,
    ) -> Result<(), SwarmError> {
        let mut metrics = self.task_metrics.lock().await;
        if let Some(data) = metrics.get_mut(task_id) {
            data.end_time = Some(Instant::now());
            data.status = TaskStatus::Completed;
            data.final_metrics = Some(result.metrics.clone());
        }

        // Update system-wide metrics
        self.update_system_metrics(result).await?;

        Ok(())
    }

    /// Get current performance snapshot
    pub async fn get_performance_snapshot(&self) -> Result<PerformanceSnapshot, SwarmError> {
        let task_data = self.task_metrics.lock().await;
        let system_data = self.system_metrics.lock().await;

        let active_tasks = task_data
            .values()
            .filter(|t| matches!(t.status, TaskStatus::Running))
            .count();

        let completed_tasks = task_data
            .values()
            .filter(|t| matches!(t.status, TaskStatus::Completed))
            .count();

        let failed_tasks = task_data
            .values()
            .filter(|t| matches!(t.status, TaskStatus::Failed))
            .count();

        Ok(PerformanceSnapshot {
            timestamp: Instant::now(),
            active_tasks,
            completed_tasks,
            failed_tasks,
            current_cpu_usage: system_data.current_cpu_percent,
            current_memory_usage: system_data.current_memory_mb as f64,
            total_io_operations: system_data.total_io_operations,
            average_task_time: self.calculate_average_task_time(&task_data).await,
            throughput_tasks_per_second: self.calculate_throughput(&task_data).await,
        })
    }

    /// Record a performance snapshot
    pub async fn record_snapshot(&self) -> Result<(), SwarmError> {
        let snapshot = self.get_performance_snapshot().await?;
        let mut history = self.performance_history.lock().await;
        history.push(snapshot);

        // Maintain history size limit
        if history.len() > self.config.max_history_size {
            history.remove(0);
        }

        Ok(())
    }

    /// Get performance statistics
    pub async fn get_performance_stats(&self) -> Result<PerformanceStats, SwarmError> {
        let task_data = self.task_metrics.lock().await;
        let history = self.performance_history.lock().await;

        let total_tasks = task_data.len();
        let completed_tasks = task_data
            .values()
            .filter(|t| matches!(t.status, TaskStatus::Completed))
            .count();

        let total_execution_time = self.start_time.elapsed();

        let mut task_times = Vec::new();
        for data in task_data.values() {
            if let (Some(start), Some(end)) = (Some(data.start_time), data.end_time) {
                task_times.push(end.duration_since(start));
            }
        }

        let average_task_time = if !task_times.is_empty() {
            task_times.iter().sum::<Duration>() / task_times.len() as u32
        } else {
            Duration::from_secs(0)
        };

        let throughput = if total_execution_time.as_secs_f64() > 0.0 {
            completed_tasks as f64 / total_execution_time.as_secs_f64()
        } else {
            0.0
        };

        // Calculate percentiles
        task_times.sort();
        let p50_task_time = self.calculate_percentile(&task_times, 50.0);
        let p95_task_time = self.calculate_percentile(&task_times, 95.0);
        let p99_task_time = self.calculate_percentile(&task_times, 99.0);

        // Calculate resource utilization trends
        let cpu_trend = self.calculate_resource_trend(&history, |s| s.current_cpu_usage);
        let memory_trend =
            self.calculate_resource_trend(&history, |s| s.current_memory_usage as f64);

        Ok(PerformanceStats {
            total_execution_time,
            total_tasks,
            completed_tasks,
            failed_tasks: total_tasks - completed_tasks,
            average_task_time,
            throughput_tasks_per_second: throughput,
            p50_task_time,
            p95_task_time,
            p99_task_time,
            cpu_utilization_trend: cpu_trend,
            memory_utilization_trend: memory_trend,
            bottleneck_analysis: self.analyze_bottlenecks(&task_data).await,
        })
    }

    /// Identify performance bottlenecks
    pub async fn identify_bottlenecks(&self) -> Result<Vec<Bottleneck>, SwarmError> {
        let task_data = self.task_metrics.lock().await;
        let mut bottlenecks = Vec::new();

        // Analyze CPU bottlenecks
        let high_cpu_tasks: Vec<_> = task_data
            .values()
            .filter(|t| {
                t.cpu_samples
                    .iter()
                    .any(|(_, cpu)| *cpu > self.config.cpu_threshold_percent)
            })
            .collect();

        if !high_cpu_tasks.is_empty() {
            bottlenecks.push(Bottleneck {
                resource_type: "CPU".to_string(),
                severity: BottleneckSeverity::High,
                description: format!("{} tasks exceeding CPU threshold", high_cpu_tasks.len()),
                affected_tasks: high_cpu_tasks.iter().map(|t| t.task_id.clone()).collect(),
                recommendation: "Consider reducing concurrency or optimizing CPU-intensive tasks"
                    .to_string(),
            });
        }

        // Analyze memory bottlenecks
        let high_memory_tasks: Vec<_> = task_data
            .values()
            .filter(|t| {
                t.memory_samples
                    .iter()
                    .any(|(_, mem)| *mem > self.config.memory_threshold_mb as f64)
            })
            .collect();

        if !high_memory_tasks.is_empty() {
            bottlenecks.push(Bottleneck {
                resource_type: "Memory".to_string(),
                severity: BottleneckSeverity::High,
                description: format!(
                    "{} tasks exceeding memory threshold",
                    high_memory_tasks.len()
                ),
                affected_tasks: high_memory_tasks
                    .iter()
                    .map(|t| t.task_id.clone())
                    .collect(),
                recommendation: "Consider increasing memory limits or optimizing memory usage"
                    .to_string(),
            });
        }

        // Analyze I/O bottlenecks
        let high_io_tasks: Vec<_> = task_data
            .values()
            .filter(|t| {
                t.io_samples
                    .iter()
                    .any(|(_, io)| *io > self.config.io_threshold_operations as f64)
            })
            .collect();

        if !high_io_tasks.is_empty() {
            bottlenecks.push(Bottleneck {
                resource_type: "I/O".to_string(),
                severity: BottleneckSeverity::Medium,
                description: format!("{} tasks with high I/O operations", high_io_tasks.len()),
                affected_tasks: high_io_tasks.iter().map(|t| t.task_id.clone()).collect(),
                recommendation: "Consider batching I/O operations or using async I/O".to_string(),
            });
        }

        Ok(bottlenecks)
    }

    /// Generate performance report
    pub async fn generate_performance_report(&self) -> Result<PerformanceReport, SwarmError> {
        let stats = self.get_performance_stats().await?;
        let bottlenecks = self.identify_bottlenecks().await?;
        let recommendations = self.generate_recommendations(&stats, &bottlenecks).await;

        Ok(PerformanceReport {
            summary: stats,
            bottlenecks,
            recommendations,
            generated_at: std::time::SystemTime::now(),
        })
    }

    /// Update system-wide performance metrics
    async fn update_system_metrics(&self, result: &TaskResult) -> Result<(), SwarmError> {
        let mut system_data = self.system_metrics.lock().await;

        system_data.total_tasks_completed += 1;
        system_data.total_io_operations += result.metrics.io_operations;
        system_data.total_network_requests += result.metrics.network_requests;
        system_data.total_cpu_time += result.execution_time;

        // Update current values (simplified)
        system_data.current_cpu_percent = result.metrics.cpu_usage_percent;
        system_data.current_memory_mb = result.metrics.memory_usage_mb as u64;

        Ok(())
    }

    /// Calculate average task time
    async fn calculate_average_task_time(
        &self,
        task_data: &HashMap<String, TaskPerformanceData>,
    ) -> Duration {
        let mut total_time = Duration::from_secs(0);
        let mut count = 0;

        for data in task_data.values() {
            if let (Some(start), Some(end)) = (Some(data.start_time), data.end_time) {
                total_time += end.duration_since(start);
                count += 1;
            }
        }

        if count > 0 {
            total_time / count as u32
        } else {
            Duration::from_secs(0)
        }
    }

    /// Calculate throughput
    async fn calculate_throughput(&self, task_data: &HashMap<String, TaskPerformanceData>) -> f64 {
        let total_time = self.start_time.elapsed().as_secs_f64();
        let completed_count = task_data
            .values()
            .filter(|t| matches!(t.status, TaskStatus::Completed))
            .count();

        if total_time > 0.0 {
            completed_count as f64 / total_time
        } else {
            0.0
        }
    }

    /// Calculate percentile from sorted durations
    fn calculate_percentile(&self, sorted_times: &[Duration], percentile: f64) -> Duration {
        if sorted_times.is_empty() {
            return Duration::from_secs(0);
        }

        let index = ((percentile / 100.0) * (sorted_times.len() - 1) as f64) as usize;
        sorted_times[index]
    }

    /// Calculate resource utilization trend
    fn calculate_resource_trend<F>(
        &self,
        history: &[PerformanceSnapshot],
        extractor: F,
    ) -> ResourceTrend
    where
        F: Fn(&PerformanceSnapshot) -> f64,
    {
        if history.len() < 2 {
            return ResourceTrend::Stable;
        }

        let recent = history.iter().rev().take(10).collect::<Vec<_>>();
        let values: Vec<f64> = recent.iter().map(|s| extractor(s)).collect();

        let avg_first_half =
            values.iter().take(values.len() / 2).sum::<f64>() / (values.len() / 2) as f64;
        let avg_second_half =
            values.iter().rev().take(values.len() / 2).sum::<f64>() / (values.len() / 2) as f64;

        let change_percent = ((avg_second_half - avg_first_half) / avg_first_half) * 100.0;

        if change_percent > 10.0 {
            ResourceTrend::Increasing
        } else if change_percent < -10.0 {
            ResourceTrend::Decreasing
        } else {
            ResourceTrend::Stable
        }
    }

    /// Analyze bottlenecks from task data
    async fn analyze_bottlenecks(
        &self,
        task_data: &HashMap<String, TaskPerformanceData>,
    ) -> BottleneckAnalysis {
        let mut cpu_bound_tasks = 0;
        let mut memory_bound_tasks = 0;
        let mut io_bound_tasks = 0;

        for data in task_data.values() {
            if data.cpu_samples.iter().any(|(_, cpu)| *cpu > 80.0) {
                cpu_bound_tasks += 1;
            }
            if data.memory_samples.iter().any(|(_, mem)| *mem > 500.0) {
                memory_bound_tasks += 1;
            }
            if data.io_samples.iter().any(|(_, io)| *io > 1000.0) {
                io_bound_tasks += 1;
            }
        }

        BottleneckAnalysis {
            cpu_bound_tasks,
            memory_bound_tasks,
            io_bound_tasks,
            total_tasks: task_data.len(),
        }
    }

    /// Generate performance recommendations
    async fn generate_recommendations(
        &self,
        stats: &PerformanceStats,
        bottlenecks: &[Bottleneck],
    ) -> Vec<String> {
        let mut recommendations = Vec::new();

        if stats.throughput_tasks_per_second < 1.0 {
            recommendations
                .push("Consider increasing parallelism for better throughput".to_string());
        }

        if stats.average_task_time > Duration::from_secs(30) {
            recommendations
                .push("Task execution times are high - consider optimization".to_string());
        }

        for bottleneck in bottlenecks {
            recommendations.push(bottleneck.recommendation.clone());
        }

        if recommendations.is_empty() {
            recommendations.push("Performance is within acceptable ranges".to_string());
        }

        recommendations
    }
}

/// Configuration for performance monitoring
#[derive(Debug, Clone)]
pub struct PerformanceConfig {
    pub cpu_threshold_percent: f64,
    pub memory_threshold_mb: f64,
    pub io_threshold_operations: u64,
    pub max_history_size: usize,
    pub monitoring_interval_ms: u64,
}

/// Task performance data
#[derive(Debug, Clone)]
pub struct TaskPerformanceData {
    pub task_id: String,
    pub start_time: Instant,
    pub end_time: Option<Instant>,
    pub cpu_samples: Vec<(Instant, f64)>,
    pub memory_samples: Vec<(Instant, f64)>,
    pub io_samples: Vec<(Instant, f64)>,
    pub status: TaskStatus,
    pub final_metrics: Option<TaskMetrics>,
}

/// Task status for monitoring
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    Running,
    Completed,
    Failed,
}

/// System performance data
#[derive(Debug, Clone)]
pub struct SystemPerformanceData {
    pub current_cpu_percent: f64,
    pub current_memory_mb: u64,
    pub total_io_operations: u64,
    pub total_network_requests: u64,
    pub total_cpu_time: Duration,
    pub total_tasks_completed: u64,
}

impl Default for SystemPerformanceData {
    fn default() -> Self {
        Self {
            current_cpu_percent: 0.0,
            current_memory_mb: 0,
            total_io_operations: 0,
            total_network_requests: 0,
            total_cpu_time: Duration::from_secs(0),
            total_tasks_completed: 0,
        }
    }
}

/// Performance snapshot
#[derive(Debug, Clone)]
pub struct PerformanceSnapshot {
    pub timestamp: Instant,
    pub active_tasks: usize,
    pub completed_tasks: usize,
    pub failed_tasks: usize,
    pub current_cpu_usage: f64,
    pub current_memory_usage: f64,
    pub total_io_operations: u64,
    pub average_task_time: Duration,
    pub throughput_tasks_per_second: f64,
}

/// Performance statistics
#[derive(Debug, Clone)]
pub struct PerformanceStats {
    pub total_execution_time: Duration,
    pub total_tasks: usize,
    pub completed_tasks: usize,
    pub failed_tasks: usize,
    pub average_task_time: Duration,
    pub throughput_tasks_per_second: f64,
    pub p50_task_time: Duration,
    pub p95_task_time: Duration,
    pub p99_task_time: Duration,
    pub cpu_utilization_trend: ResourceTrend,
    pub memory_utilization_trend: ResourceTrend,
    pub bottleneck_analysis: BottleneckAnalysis,
}

/// Resource utilization trend
#[derive(Debug, Clone)]
pub enum ResourceTrend {
    Increasing,
    Decreasing,
    Stable,
}

/// Bottleneck information
#[derive(Debug, Clone)]
pub struct Bottleneck {
    pub resource_type: String,
    pub severity: BottleneckSeverity,
    pub description: String,
    pub affected_tasks: Vec<String>,
    pub recommendation: String,
}

/// Bottleneck severity levels
#[derive(Debug, Clone)]
pub enum BottleneckSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Bottleneck analysis results
#[derive(Debug, Clone)]
pub struct BottleneckAnalysis {
    pub cpu_bound_tasks: usize,
    pub memory_bound_tasks: usize,
    pub io_bound_tasks: usize,
    pub total_tasks: usize,
}

/// Performance report
#[derive(Debug, Clone)]
pub struct PerformanceReport {
    pub summary: PerformanceStats,
    pub bottlenecks: Vec<Bottleneck>,
    pub recommendations: Vec<String>,
    pub generated_at: std::time::SystemTime,
}
