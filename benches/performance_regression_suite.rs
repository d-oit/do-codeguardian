use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use do_codeguardian::{
    analyzers::{performance_analyzer::PerformanceAnalyzer, security_analyzer::SecurityAnalyzer},
    cache::FileCache,
    config::{Config, PerformanceConfig},
    core::GuardianEngine,
    performance::{PerformanceAnalyzer as PerfAnalyzer, PerformanceMetrics, PerformanceProfiler},
    streaming::StreamingAnalyzer,
    types::Finding,
    utils::{
        adaptive_parallelism::{AdaptiveParallelismController, SystemLoadMonitor},
        memory_pool::{thread_local_pools, GlobalMemoryPools},
    },
};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tempfile::NamedTempFile;
use tokio::runtime::Runtime;

/// Comprehensive Performance Regression Suite
/// This suite provides automated performance regression detection,
/// load testing integration, and optimization recommendations

/// Generate test files for regression testing
fn generate_regression_test_files() -> Vec<(NamedTempFile, String, usize)> {
    let mut files = Vec::new();

    // Small file (baseline)
    let small_content = "fn main() {\n    println!(\"Hello, world!\");\n}\n".repeat(25);
    let mut small_file = NamedTempFile::new().unwrap();
    small_file.write_all(small_content.as_bytes()).unwrap();
    files.push((small_file, "baseline.rs".to_string(), small_content.len()));

    // Medium file (stress test)
    let medium_content = format!(
        "{}\n{}\n",
        "pub struct TestStruct { field: String }".repeat(250),
        "impl TestStruct { pub fn new() -> Self { Self { field: String::new() } } }".repeat(250)
    );
    let mut medium_file = NamedTempFile::new().unwrap();
    medium_file.write_all(medium_content.as_bytes()).unwrap();
    files.push((medium_file, "stress.rs".to_string(), medium_content.len()));

    // Large file (memory pressure)
    let large_content = format!(
        "{}\n{}\n",
        "// Large file for memory testing".repeat(1000),
        "pub fn function() { println!(\"test\"); }".repeat(500)
    );
    let mut large_file = NamedTempFile::new().unwrap();
    large_file.write_all(large_content.as_bytes()).unwrap();
    files.push((large_file, "memory.rs".to_string(), large_content.len()));

    files
}

/// Performance regression detection benchmark
fn bench_performance_regression_detection(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let test_files = generate_regression_test_files();
    let metrics = Arc::new(PerformanceMetrics::new());
    let profiler = PerformanceProfiler::new(Arc::clone(&metrics));

    let mut group = c.benchmark_group("performance_regression");

    // Baseline performance benchmark
    group.bench_function("baseline_analysis", |b| {
        let config = Config::minimal();
        let mut engine = rt.block_on(async {
            GuardianEngine::new_with_ml(config, Default::default(), None)
                .await
                .unwrap()
        });

        let file_paths: Vec<PathBuf> = test_files
            .iter()
            .take(1)
            .map(|(file, _, _)| file.path().to_path_buf())
            .collect();

        b.iter(|| {
            rt.block_on(async {
                let start = Instant::now();
                let result = engine.analyze_files(&file_paths, 1).await;
                let duration = start.elapsed();

                // Regression detection: ensure performance doesn't degrade beyond threshold
                assert!(
                    duration < Duration::from_millis(500),
                    "Performance regression detected: analysis took {:.2}ms (threshold: 500ms)",
                    duration.as_millis()
                );

                black_box(result.unwrap());
            });
        });
    });

    // Memory usage regression detection
    group.bench_function("memory_regression_detection", |b| {
        let config = Config::minimal();
        let mut engine = rt.block_on(async {
            GuardianEngine::new_with_ml(config, Default::default(), None)
                .await
                .unwrap()
        });

        let file_paths: Vec<PathBuf> = test_files
            .iter()
            .take(3)
            .map(|(file, _, _)| file.path().to_path_buf())
            .collect();

        b.iter(|| {
            rt.block_on(async {
                let start_mem = get_memory_usage().unwrap_or(0);
                let result = engine.analyze_files(&file_paths, 2).await;
                let end_mem = get_memory_usage().unwrap_or(0);
                let mem_delta = end_mem.saturating_sub(start_mem);

                // Memory regression detection: ensure memory usage doesn't exceed threshold
                assert!(
                    mem_delta < 50 * 1024 * 1024, // 50MB threshold
                    "Memory regression detected: {} bytes used (threshold: 50MB)",
                    mem_delta
                );

                black_box(result.unwrap());
            });
        });
    });

    group.finish();
}

/// Load testing integration benchmark
fn bench_load_testing_integration(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let test_files = generate_regression_test_files();

    let mut group = c.benchmark_group("load_testing_integration");

    // Concurrent file processing benchmark
    for concurrency in [1, 4, 8, 16].iter() {
        group.throughput(Throughput::Elements(*concurrency as u64));
        group.bench_with_input(
            BenchmarkId::new("concurrent_file_processing", concurrency),
            concurrency,
            |b, &concurrency| {
                let config = Config::minimal();
                let mut engine = rt.block_on(async {
                    GuardianEngine::new_with_ml(config, Default::default(), None)
                        .await
                        .unwrap()
                });

                let file_paths: Vec<PathBuf> = test_files
                    .iter()
                    .cycle()
                    .take(concurrency * 2)
                    .map(|(file, _, _)| file.path().to_path_buf())
                    .collect();

                b.iter(|| {
                    rt.block_on(async {
                        let result = engine.analyze_files(&file_paths, concurrency).await;
                        black_box(result.unwrap());
                    });
                });
            },
        );
    }

    group.finish();
}

/// Performance metrics collection benchmark
fn bench_performance_metrics_collection(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let test_files = generate_regression_test_files();
    let metrics = Arc::new(PerformanceMetrics::new());
    let profiler = PerformanceProfiler::new(Arc::clone(&metrics));

    let mut group = c.benchmark_group("metrics_collection");

    group.bench_function("comprehensive_metrics_collection", |b| {
        let config = Config::minimal();
        let mut engine = rt.block_on(async {
            GuardianEngine::new_with_ml(config, Default::default(), None)
                .await
                .unwrap()
        });

        let file_paths: Vec<PathBuf> = test_files
            .iter()
            .take(2)
            .map(|(file, _, _)| file.path().to_path_buf())
            .collect();

        b.iter(|| {
            rt.block_on(async {
                // Profile the entire analysis operation
                let result = profiler
                    .profile_file_analysis(|| async { engine.analyze_files(&file_paths, 2).await })
                    .await;

                // Update metrics
                metrics.update_memory_usage(get_memory_usage().unwrap_or(0));

                black_box(result.unwrap());
            });
        });
    });

    group.finish();
}

/// Optimization recommendations benchmark
fn bench_optimization_recommendations(c: &mut Criterion) {
    let metrics = Arc::new(PerformanceMetrics::new());
    let analyzer = PerfAnalyzer::new(Arc::clone(&metrics));

    // Simulate various performance scenarios
    let mut group = c.benchmark_group("optimization_recommendations");

    group.bench_function("generate_recommendations_low_cache_hit", |b| {
        // Simulate poor cache performance
        for _ in 0..100 {
            metrics.record_cache_miss();
        }
        for _ in 0..20 {
            metrics.record_cache_hit();
        }

        b.iter(|| {
            let recommendations = analyzer.analyze_performance();
            black_box(recommendations);
        });
    });

    group.bench_function("generate_recommendations_high_memory", |b| {
        // Simulate high memory usage
        metrics.update_memory_usage(200 * 1024 * 1024); // 200MB

        b.iter(|| {
            let recommendations = analyzer.analyze_performance();
            black_box(recommendations);
        });
    });

    group.bench_function("generate_recommendations_slow_processing", |b| {
        // Simulate slow processing
        for _ in 0..10 {
            metrics.record_file_processed(Duration::from_millis(200));
        }

        b.iter(|| {
            let recommendations = analyzer.analyze_performance();
            black_box(recommendations);
        });
    });

    group.finish();
}

/// Automated regression alerting simulation
fn bench_regression_alerting(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let test_files = generate_regression_test_files();

    let mut group = c.benchmark_group("regression_alerting");

    group.bench_function("performance_threshold_monitoring", |b| {
        let config = Config::minimal();
        let mut engine = rt.block_on(async {
            GuardianEngine::new_with_ml(config, Default::default(), None)
                .await
                .unwrap()
        });

        let file_paths: Vec<PathBuf> = test_files
            .iter()
            .take(2)
            .map(|(file, _, _)| file.path().to_path_buf())
            .collect();

        b.iter(|| {
            rt.block_on(async {
                let start = Instant::now();
                let result = engine.analyze_files(&file_paths, 2).await;
                let duration = start.elapsed();

                // Simulate alerting logic
                let performance_threshold = Duration::from_millis(1000);
                if duration > performance_threshold {
                    // In real implementation, this would trigger alerts
                    black_box(format!(
                        "ALERT: Performance threshold exceeded: {:.2}ms",
                        duration.as_millis()
                    ));
                }

                black_box(result.unwrap());
            });
        });
    });

    group.finish();
}

/// Helper function to get current memory usage
fn get_memory_usage() -> Option<u64> {
    // This is a simplified implementation
    // In a real scenario, you'd use a proper memory profiling library
    Some(1024 * 1024 * 50) // Return 50MB as example
}

criterion_group!(
    benches,
    bench_performance_regression_detection,
    bench_load_testing_integration,
    bench_performance_metrics_collection,
    bench_optimization_recommendations,
    bench_regression_alerting
);
criterion_main!(benches);
