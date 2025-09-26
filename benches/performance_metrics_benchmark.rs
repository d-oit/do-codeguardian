use criterion::{criterion_group, criterion_main, Criterion};
use do_codeguardian::{
    config::Config,
    core::GuardianEngine,
    performance::{PerformanceAnalyzer as PerfAnalyzer, PerformanceMetrics, PerformanceProfiler},
    utils::progress::ProgressReporter,
};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tempfile::tempdir;
use tokio::runtime::Runtime;

/// Performance Metrics Collection and Analysis Benchmark
/// This suite focuses on comprehensive metrics collection,
/// analysis, and automated performance insights

/// Generate comprehensive test data for metrics analysis
fn generate_metrics_test_data() -> tempfile::TempDir {
    let temp_dir = tempdir().unwrap();

    // Create various file types and sizes for comprehensive testing
    let file_types = vec![
        ("small.rs", 1),    // 1KB
        ("medium.rs", 50),  // 50KB
        ("large.rs", 500),  // 500KB
        ("config.toml", 5), // 5KB
        ("data.json", 100), // 100KB
    ];

    for (filename, size_kb) in file_types {
        let file_path = temp_dir.path().join(filename);
        let content = generate_file_content(filename, size_kb);
        std::fs::write(&file_path, content).unwrap();
    }

    temp_dir
}

/// Generate realistic file content based on type
fn generate_file_content(filename: &str, size_kb: usize) -> String {
    let mut content = String::with_capacity(size_kb * 1024);

    match filename {
        "small.rs" => {
            content.push_str("fn main() {\n    println!(\"Hello, world!\");\n}\n");
        }
        "medium.rs" => {
            content.push_str("use std::collections::HashMap;\n\n");
            content.push_str("#[derive(Debug)]\n");
            content.push_str("pub struct MediumStruct {\n");
            content.push_str("    pub data: HashMap<String, String>,\n");
            content.push_str("}\n\n");
            content.push_str("impl MediumStruct {\n");
            content.push_str("    pub fn process(&mut self) {\n");
            content.push_str("        // Processing logic here\n");
            content.push_str("    }\n");
            content.push_str("}\n");
        }
        "large.rs" => {
            content.push_str("// Large file with many components\n");
            for i in 0..100 {
                content.push_str(&format!("pub mod module_{} {{\n", i));
                content.push_str(&format!("    pub fn function_{}() {{}}\n", i));
                content.push_str("}\n\n");
            }
        }
        "config.toml" => {
            content.push_str("[package]\n");
            content.push_str("name = \"test-package\"\n");
            content.push_str("version = \"0.1.0\"\n");
            content.push_str("[dependencies]\n");
            content.push_str("serde = \"1.0\"\n");
        }
        "data.json" => {
            content.push_str("{\n");
            content.push_str("  \"users\": [\n");
            for i in 0..50 {
                content.push_str(&format!(
                    "    {{\"id\": {}, \"name\": \"user_{}\"}}{}",
                    i,
                    i,
                    if i < 49 { "," } else { "" }
                ));
            }
            content.push_str("  ]\n");
            content.push_str("}\n");
        }
        _ => {
            content.push_str("// Generic file content\n");
        }
    }

    // Pad to reach desired size
    while content.len() < size_kb * 1024 {
        content.push_str(&format!("// Padding line {}\n", content.len()));
    }

    content
}

/// Comprehensive metrics collection benchmark
fn bench_comprehensive_metrics_collection(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let test_dir = generate_metrics_test_data();
    let metrics = Arc::new(PerformanceMetrics::new());
    let profiler = PerformanceProfiler::new(Arc::clone(&metrics));

    let mut group = c.benchmark_group("comprehensive_metrics");

    group.bench_function("full_metrics_collection_run", |b| {
        let config = Config::default();
        let mut engine = rt.block_on(async {
            GuardianEngine::new(config, ProgressReporter::new(false))
                .await
                .unwrap()
        });

        let file_paths: Vec<PathBuf> = vec![
            test_dir.path().join("small.rs"),
            test_dir.path().join("medium.rs"),
            test_dir.path().join("large.rs"),
            test_dir.path().join("config.toml"),
            test_dir.path().join("data.json"),
        ];

        b.iter(|| {
            rt.block_on(async {
                let start = Instant::now();

                // Profile the analysis with comprehensive metrics
                let result = profiler
                    .profile_file_analysis(|| async { engine.analyze_files(&file_paths, 4).await })
                    .await;

                let duration = start.elapsed();

                // Update additional metrics
                metrics.update_memory_usage(get_memory_usage().unwrap_or(0) as usize);

                // Simulate cache operations
                for _ in 0..10 {
                    metrics.record_cache_hit();
                }
                for _ in 0..2 {
                    metrics.record_cache_miss();
                }

                black_box((result.unwrap(), duration));
            });
        });
    });

    group.finish();
}

/// Performance analysis and insights benchmark
fn bench_performance_analysis_insights(c: &mut Criterion) {
    let metrics = Arc::new(PerformanceMetrics::new());
    let analyzer = PerfAnalyzer::new(Arc::clone(&metrics));

    let mut group = c.benchmark_group("performance_analysis");

    // Simulate different performance scenarios
    group.bench_function("analyze_cache_performance", |b| {
        // Simulate cache performance data
        for _ in 0..100 {
            metrics.record_file_processed(Duration::from_millis(50));
            metrics.record_cache_hit();
        }
        for _ in 0..20 {
            metrics.record_cache_miss();
        }

        b.iter(|| {
            let cache_hit_rate = metrics.get_cache_hit_rate();
            let recommendations = analyzer.analyze_performance();
            black_box((cache_hit_rate, recommendations));
        });
    });

    group.bench_function("analyze_processing_speed", |b| {
        // Simulate processing speed data
        for _ in 0..50 {
            metrics.record_file_processed(Duration::from_millis(100));
        }

        b.iter(|| {
            let avg_time = metrics.get_average_processing_time();
            let throughput = metrics.get_throughput_files_per_second();
            let recommendations = analyzer.analyze_performance();
            black_box((avg_time, throughput, recommendations));
        });
    });

    group.bench_function("analyze_memory_usage", |b| {
        // Simulate memory usage data
        metrics.update_memory_usage(150 * 1024 * 1024); // 150MB

        b.iter(|| {
            let recommendations = analyzer.analyze_performance();
            black_box(recommendations);
        });
    });

    group.finish();
}

/// Automated performance reporting benchmark
fn bench_automated_reporting(c: &mut Criterion) {
    let metrics = Arc::new(PerformanceMetrics::new());
    let analyzer = PerfAnalyzer::new(Arc::clone(&metrics));

    let mut group = c.benchmark_group("automated_reporting");

    group.bench_function("generate_performance_report", |b| {
        // Populate metrics with realistic data
        for i in 0..100 {
            metrics.record_file_processed(Duration::from_millis(50 + (i % 20)));
            if i % 5 == 0 {
                metrics.record_cache_miss();
            } else {
                metrics.record_cache_hit();
            }
        }
        metrics.update_memory_usage(100 * 1024 * 1024);

        b.iter(|| {
            let report = analyzer.generate_performance_report();
            black_box(report.len());
        });
    });

    group.finish();
}

/// Regression detection benchmark
fn bench_regression_detection(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let test_dir = generate_metrics_test_data();
    let metrics = Arc::new(PerformanceMetrics::new());

    let mut group = c.benchmark_group("regression_detection");

    group.bench_function("performance_baseline_comparison", |b| {
        let config = Config::default();
        let mut engine = rt.block_on(async {
            GuardianEngine::new(config, ProgressReporter::new(false))
                .await
                .unwrap()
        });

        let file_paths: Vec<PathBuf> = vec![
            test_dir.path().join("small.rs"),
            test_dir.path().join("medium.rs"),
            test_dir.path().join("large.rs"),
            test_dir.path().join("config.toml"),
            test_dir.path().join("data.json"),
        ];

        b.iter(|| {
            rt.block_on(async {
                let start = Instant::now();
                let result = engine.analyze_files(&file_paths, 4).await;
                let duration = start.elapsed();

                // Baseline comparison (in real implementation, compare against stored baselines)
                let baseline_duration = Duration::from_millis(2000); // Example baseline
                let regression_threshold = 0.5; // 50% degradation allowed

                let degradation_ratio = duration.as_secs_f64() / baseline_duration.as_secs_f64();

                if degradation_ratio > (1.0 + regression_threshold) {
                    black_box(format!(
                        "PERFORMANCE REGRESSION: {:.2}x slower than baseline",
                        degradation_ratio
                    ));
                }

                black_box(result.unwrap());
            });
        });
    });

    group.finish();
}

/// Metrics aggregation and statistical analysis
fn bench_metrics_aggregation(c: &mut Criterion) {
    let metrics = Arc::new(PerformanceMetrics::new());

    let mut group = c.benchmark_group("metrics_aggregation");

    group.bench_function("aggregate_performance_statistics", |b| {
        // Simulate collecting metrics over time
        let mut durations = Vec::new();
        for i in 0..1000 {
            let duration = Duration::from_millis(50 + (i % 50));
            metrics.record_file_processed(duration);
            durations.push(duration);
        }

        b.iter(|| {
            let avg_time = metrics.get_average_processing_time();
            let throughput = metrics.get_throughput_files_per_second();
            let cache_hit_rate = metrics.get_cache_hit_rate();

            // Calculate additional statistics
            let total_time: Duration = durations.iter().sum();
            let min_time = durations.iter().min().unwrap();
            let max_time = durations.iter().max().unwrap();

            black_box((
                avg_time,
                throughput,
                cache_hit_rate,
                total_time,
                min_time,
                max_time,
            ));
        });
    });

    group.finish();
}

/// Performance alerting simulation
fn bench_performance_alerting(c: &mut Criterion) {
    let metrics = Arc::new(PerformanceMetrics::new());

    let mut group = c.benchmark_group("performance_alerting");

    group.bench_function("threshold_based_alerting", |b| {
        // Simulate various alerting scenarios
        b.iter(|| {
            let mut alerts = Vec::new();

            // Cache hit rate alert
            let cache_hit_rate = metrics.get_cache_hit_rate();
            if cache_hit_rate < 0.7 {
                alerts.push(format!(
                    "LOW CACHE HIT RATE: {:.1}%",
                    cache_hit_rate * 100.0
                ));
            }

            // Processing speed alert
            let avg_time = metrics.get_average_processing_time();
            if avg_time > Duration::from_millis(200) {
                alerts.push(format!(
                    "SLOW PROCESSING: {:.1}ms avg",
                    avg_time.as_millis()
                ));
            }

            // Memory usage alert
            let memory_usage = metrics
                .memory_peak_usage
                .load(std::sync::atomic::Ordering::Relaxed);
            if memory_usage > 200 * 1024 * 1024 {
                alerts.push(format!(
                    "HIGH MEMORY USAGE: {:.1}MB",
                    memory_usage as f64 / (1024.0 * 1024.0)
                ));
            }

            black_box(alerts);
        });
    });

    group.finish();
}

/// Helper function to get memory usage
fn get_memory_usage() -> Option<u64> {
    // Simplified implementation - in real scenarios use proper memory profiling
    Some(1024 * 1024 * 80) // 80MB example
}

criterion_group!(
    benches,
    bench_comprehensive_metrics_collection,
    bench_performance_analysis_insights,
    bench_automated_reporting,
    bench_regression_detection,
    bench_metrics_aggregation,
    bench_performance_alerting
);
criterion_main!(benches);
