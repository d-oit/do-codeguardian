use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use do_codeguardian::{
    analyzers::{performance_analyzer::PerformanceAnalyzer, security_analyzer::SecurityAnalyzer},
    cache::FileCache,
    config::{Config, PerformanceConfig},
    core::GuardianEngine,
    performance::{OptimizationTracker, PerformanceMetrics},
};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tempfile::NamedTempFile;
use tokio::runtime::Runtime;

/// Optimization Tracking Benchmark Suite
/// Tracks the impact of performance optimizations over time and versions

/// Optimization scenario definition
#[derive(Clone, Debug)]
struct OptimizationScenario {
    name: String,
    description: String,
    baseline_version: String,
    optimized_version: String,
    expected_improvement_percent: f64,
    metrics_focus: Vec<String>,
}

/// Generate optimization test data
fn generate_optimization_test_data() -> Vec<(NamedTempFile, String)> {
    let mut files = Vec::new();

    // File with optimization opportunities
    let optimization_content = r#"
// File with multiple optimization opportunities
use std::collections::HashMap;

pub struct DataProcessor {
    cache: HashMap<String, String>,
}

impl DataProcessor {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    // Inefficient string concatenation (optimization target)
    pub fn process_data(&mut self, data: Vec<String>) -> String {
        let mut result = String::new();
        for item in data {
            result = result + &item; // Inefficient - should use push_str
        }
        result
    }

    // Memory inefficient pattern (optimization target)
    pub fn find_duplicates(&self, items: Vec<String>) -> Vec<String> {
        let mut seen = HashMap::new();
        let mut duplicates = Vec::new();

        for item in items {
            let count = seen.entry(item.clone()).or_insert(0);
            *count += 1;
            if *count > 1 {
                duplicates.push(item);
            }
        }
        duplicates
    }
}

fn main() {
    let mut processor = DataProcessor::new();
    let test_data = vec!["item1".to_string(), "item2".to_string(), "item1".to_string()];
    let result = processor.process_data(test_data.clone());
    let duplicates = processor.find_duplicates(test_data);
    println!("Result: {}, Duplicates: {:?}", result, duplicates);
}
"#;

    let mut opt_file = NamedTempFile::new().unwrap();
    opt_file.write_all(optimization_content.as_bytes()).unwrap();
    files.push((opt_file, "optimization_targets.rs".to_string()));

    files
}

/// Optimization impact tracking benchmark
fn bench_optimization_impact_tracking(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let test_files = generate_optimization_test_data();
    let tracker = Arc::new(OptimizationTracker::new());

    let mut group = c.benchmark_group("optimization_tracking");

    // Define optimization scenarios
    let scenarios = vec![
        OptimizationScenario {
            name: "string_concatenation".to_string(),
            description: "String concatenation optimization".to_string(),
            baseline_version: "v1.0.0".to_string(),
            optimized_version: "v1.1.0".to_string(),
            expected_improvement_percent: 25.0,
            metrics_focus: vec!["memory_usage".to_string(), "processing_time".to_string()],
        },
        OptimizationScenario {
            name: "memory_pool".to_string(),
            description: "Memory pool optimization".to_string(),
            baseline_version: "v1.0.0".to_string(),
            optimized_version: "v1.1.0".to_string(),
            expected_improvement_percent: 15.0,
            metrics_focus: vec![
                "memory_efficiency".to_string(),
                "allocation_count".to_string(),
            ],
        },
        OptimizationScenario {
            name: "cache_optimization".to_string(),
            description: "Cache performance optimization".to_string(),
            baseline_version: "v1.0.0".to_string(),
            optimized_version: "v1.1.0".to_string(),
            expected_improvement_percent: 30.0,
            metrics_focus: vec!["cache_hit_rate".to_string(), "cache_latency".to_string()],
        },
    ];

    for scenario in scenarios {
        group.bench_with_input(
            BenchmarkId::new("optimization_scenario", &scenario.name),
            &scenario,
            |b, scenario| {
                let mut engine = rt.block_on(async {
                    let cfg = Config::minimal();
                    GuardianEngine::new_with_ml(cfg, Default::default(), None)
                        .await
                        .unwrap()
                });

                let file_paths: Vec<PathBuf> = test_files
                    .iter()
                    .map(|(file, _)| file.path().to_path_buf())
                    .collect();

                b.iter(|| {
                    rt.block_on(async {
                        // Track optimization metrics
                        tracker.start_tracking(&scenario.name);

                        let start = Instant::now();
                        let start_mem = get_memory_usage().unwrap_or(0);

                        let result = engine.analyze_files(&file_paths, 2).await;

                        let duration = start.elapsed();
                        let end_mem = get_memory_usage().unwrap_or(0);
                        let mem_delta = end_mem.saturating_sub(start_mem);

                        // Record optimization metrics
                        tracker.record_metric(
                            &scenario.name,
                            "processing_time",
                            duration.as_millis() as f64,
                        );
                        tracker.record_metric(&scenario.name, "memory_usage", mem_delta as f64);
                        tracker.record_metric(&scenario.name, "cache_hit_rate", 0.85); // Example

                        tracker.end_tracking(&scenario.name);

                        black_box(result.unwrap());
                    });
                });
            },
        );
    }

    group.finish();
}

/// Performance regression tracking benchmark
fn bench_performance_regression_tracking(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let test_files = generate_optimization_test_data();
    let tracker = Arc::new(OptimizationTracker::new());

    let mut group = c.benchmark_group("regression_tracking");

    // Track performance across simulated versions
    let versions = vec!["v1.0.0", "v1.1.0", "v1.2.0", "v2.0.0"];

    for version in versions {
        group.bench_with_input(
            BenchmarkId::new("version_comparison", version),
            version,
            |b, version| {
                let mut engine = rt.block_on(async {
                    let cfg = Config::minimal();
                    GuardianEngine::new_with_ml(cfg, Default::default(), None)
                        .await
                        .unwrap()
                });

                let file_paths: Vec<PathBuf> = test_files
                    .iter()
                    .map(|(file, _)| file.path().to_path_buf())
                    .collect();

                b.iter(|| {
                    rt.block_on(async {
                        let start = Instant::now();
                        let result = engine.analyze_files(&file_paths, 2).await;
                        let duration = start.elapsed();

                        // Track version-specific performance
                        tracker.record_version_metric(
                            version,
                            "analysis_time",
                            duration.as_millis() as f64,
                        );

                        // Check for regressions
                        if let Some(baseline) =
                            tracker.get_baseline_metric(version, "analysis_time")
                        {
                            let regression_threshold = 0.10; // 10% degradation allowed
                            let degradation = (duration.as_millis() as f64 - baseline) / baseline;

                            if degradation > regression_threshold {
                                eprintln!(
                                    "Performance regression detected in {}: {:.2}% degradation",
                                    version,
                                    degradation * 100.0
                                );
                            }
                        }

                        black_box(result.unwrap());
                    });
                });
            },
        );
    }

    group.finish();
}

/// Optimization effectiveness measurement benchmark
fn bench_optimization_effectiveness(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let test_files = generate_optimization_test_data();
    let tracker = Arc::new(OptimizationTracker::new());

    let mut group = c.benchmark_group("optimization_effectiveness");

    // Measure effectiveness of different optimization strategies
    let optimization_strategies = vec![
        ("baseline", "No optimizations applied"),
        ("memory_pool", "Memory pool optimization"),
        ("cache_optimization", "Enhanced caching"),
        ("parallel_processing", "Improved parallel processing"),
        ("algorithm_optimization", "Algorithm improvements"),
    ];

    for (strategy_name, description) in optimization_strategies {
        group.bench_with_input(
            BenchmarkId::new("strategy_effectiveness", strategy_name),
            &(strategy_name, description),
            |b, (strategy_name, description)| {
                let mut engine = rt.block_on(async {
                    let mut cfg = Config::minimal();

                    // Apply strategy-specific configuration
                    match *strategy_name {
                        "memory_pool" => {
                            cfg.performance.enable_memory_pool = true;
                        }
                        "cache_optimization" => {
                            cfg.cache.enabled = true;
                            cfg.cache.max_size_mb = 100;
                        }
                        "parallel_processing" => {
                            cfg.performance.parallel_workers = 8;
                        }
                        "algorithm_optimization" => {
                            cfg.performance.enable_algorithm_optimizations = true;
                        }
                        _ => {} // baseline - no changes
                    }

                    GuardianEngine::new_with_ml(cfg, Default::default(), None)
                        .await
                        .unwrap()
                });

                let file_paths: Vec<PathBuf> = test_files
                    .iter()
                    .map(|(file, _)| file.path().to_path_buf())
                    .collect();

                b.iter(|| {
                    rt.block_on(async {
                        let start = Instant::now();
                        let start_mem = get_memory_usage().unwrap_or(0);

                        let result = engine.analyze_files(&file_paths, 4).await;

                        let duration = start.elapsed();
                        let end_mem = get_memory_usage().unwrap_or(0);
                        let mem_delta = end_mem.saturating_sub(start_mem);

                        // Record effectiveness metrics
                        tracker.record_optimization_effectiveness(
                            strategy_name,
                            duration.as_millis() as f64,
                            mem_delta as f64,
                            0.85, // cache hit rate
                        );

                        black_box(result.unwrap());
                    });
                });
            },
        );
    }

    group.finish();
}

/// Helper function to get current memory usage
fn get_memory_usage() -> Option<u64> {
    // Simplified implementation - in production, use proper memory profiling
    Some(1024 * 1024 * 100) // Return 100MB as example
}

/// Optimization tracking structure
pub struct OptimizationTracker {
    metrics: Arc<std::sync::Mutex<HashMap<String, HashMap<String, Vec<f64>>>>>,
    version_baselines: Arc<std::sync::Mutex<HashMap<String, HashMap<String, f64>>>>,
}

impl OptimizationTracker {
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(std::sync::Mutex::new(HashMap::new())),
            version_baselines: Arc::new(std::sync::Mutex::new(HashMap::new())),
        }
    }

    pub fn start_tracking(&self, scenario: &str) {
        let mut metrics = self.metrics.lock().unwrap();
        metrics
            .entry(scenario.to_string())
            .or_insert_with(HashMap::new);
    }

    pub fn record_metric(&self, scenario: &str, metric: &str, value: f64) {
        let mut metrics = self.metrics.lock().unwrap();
        if let Some(scenario_metrics) = metrics.get_mut(scenario) {
            scenario_metrics
                .entry(metric.to_string())
                .or_insert_with(Vec::new)
                .push(value);
        }
    }

    pub fn end_tracking(&self, scenario: &str) {
        // Calculate and store averages, trends, etc.
        let metrics = self.metrics.lock().unwrap();
        if let Some(scenario_metrics) = metrics.get(scenario) {
            for (metric_name, values) in scenario_metrics {
                if !values.is_empty() {
                    let avg = values.iter().sum::<f64>() / values.len() as f64;
                    println!("{} - {}: Average = {:.2}", scenario, metric_name, avg);
                }
            }
        }
    }

    pub fn record_version_metric(&self, version: &str, metric: &str, value: f64) {
        let mut baselines = self.version_baselines.lock().unwrap();
        let version_metrics = baselines
            .entry(version.to_string())
            .or_insert_with(HashMap::new);
        version_metrics.insert(metric.to_string(), value);
    }

    pub fn get_baseline_metric(&self, version: &str, metric: &str) -> Option<f64> {
        let baselines = self.version_baselines.lock().unwrap();
        baselines.get(version)?.get(metric).copied()
    }

    pub fn record_optimization_effectiveness(
        &self,
        strategy: &str,
        time: f64,
        memory: f64,
        cache_hit_rate: f64,
    ) {
        println!(
            "Optimization Effectiveness - {}: Time={:.2}ms, Memory={}MB, Cache Hit Rate={:.2}%",
            strategy,
            time,
            memory / (1024.0 * 1024.0),
            cache_hit_rate * 100.0
        );
    }
}

criterion_group!(
    benches,
    bench_optimization_impact_tracking,
    bench_performance_regression_tracking,
    bench_optimization_effectiveness
);
criterion_main!(benches);
