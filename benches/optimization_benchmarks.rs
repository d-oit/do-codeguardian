use codeguardian::{
    analyzers::AnalyzerRegistry,
    cache::FileCache,
    config::{Config, PerformanceConfig},
    core::GuardianEngine,
    streaming::StreamingAnalyzer,
    types::Finding,
    utils::{
        adaptive_parallelism::{AdaptiveParallelismController, SystemLoadMonitor},
        memory_pool::{thread_local_pools, GlobalMemoryPools},
    },
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;
use tempfile::NamedTempFile;
use tokio::runtime::Runtime;

/// Generate test files for optimization benchmarks
fn generate_optimization_test_files() -> Vec<(NamedTempFile, String, usize)> {
    let mut files = Vec::new();

    // Small file for cache testing
    let small_content = "fn main() {\n    println!(\"Hello, world!\");\n}\n".repeat(10);
    let mut small_file = NamedTempFile::new().unwrap();
    small_file.write_all(small_content.as_bytes()).unwrap();
    files.push((small_file, "cache_test.rs".to_string(), small_content.len()));

    // Medium file for memory pool testing
    let medium_content = format!(
        "{}\n{}\n",
        "pub struct TestStruct { field: String }".repeat(500),
        "impl TestStruct { pub fn new() -> Self { Self { field: String::new() } } }".repeat(500)
    );
    let mut medium_file = NamedTempFile::new().unwrap();
    medium_file.write_all(medium_content.as_bytes()).unwrap();
    files.push((
        medium_file,
        "memory_test.rs".to_string(),
        medium_content.len(),
    ));

    // Large file for parallelism testing
    let large_content = format!(
        "{}\n{}\n",
        "// Large file for parallel processing".repeat(2000),
        "pub fn function() { println!(\"test\"); }".repeat(1000)
    );
    let mut large_file = NamedTempFile::new().unwrap();
    large_file.write_all(large_content.as_bytes()).unwrap();
    files.push((
        large_file,
        "parallel_test.rs".to_string(),
        large_content.len(),
    ));

    files
}

/// Benchmark memory pool optimizations with baseline comparison
fn bench_memory_pool_optimizations(c: &mut Criterion) {
    let test_files = generate_optimization_test_files();

    let mut group = c.benchmark_group("memory_pool_optimization");

    // Baseline: Standard allocation without memory pools
    group.bench_function("baseline_standard_allocation", |b| {
        b.iter(|| {
            let mut strings = Vec::new();
            let mut findings = Vec::new();

            // Simulate analysis without memory pools
            for _ in 0..1000 {
                strings.push("test content".to_string());
                findings.push(Finding::new(
                    "test",
                    "test_finding",
                    codeguardian::types::Severity::Info,
                    PathBuf::from("test.rs"),
                    1,
                    "Test finding".to_string(),
                ));
            }

            black_box((strings, findings));
        });
    });

    // Optimized: Using memory pools
    group.bench_function("optimized_memory_pool_allocation", |b| {
        thread_local_pools::init();

        b.iter(|| {
            let mut strings = Vec::new();
            let mut findings = Vec::new();

            // Simulate analysis with memory pools
            for _ in 0..1000 {
                let mut buffer = thread_local_pools::get_string_buffer();
                buffer.push_str("test content");
                strings.push(buffer);

                let mut finding_vec = thread_local_pools::get_findings_vec();
                finding_vec.push(Finding::new(
                    "test",
                    "test_finding",
                    codeguardian::types::Severity::Info,
                    PathBuf::from("test.rs"),
                    1,
                    "Test finding".to_string(),
                ));
                findings.push(finding_vec);
            }

            black_box((strings, findings));
        });
    });

    // Memory pool stress test for regression detection
    group.bench_function("memory_pool_stress_test", |b| {
        thread_local_pools::init();

        b.iter(|| {
            // Stress test with many allocations/deallocations
            let mut buffers = Vec::new();

            for i in 0..10000 {
                let mut buffer = thread_local_pools::get_string_buffer();
                buffer.push_str(&format!("test content {}", i));
                buffers.push(buffer);
            }

            // Verify all buffers are properly managed
            assert_eq!(buffers.len(), 10000);
            assert!(buffers[0].contains("test content"));

            black_box(buffers);
        });
    });

    group.finish();
}

/// Benchmark async caching optimizations with cache hit/miss scenarios
fn bench_async_caching_optimizations(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let test_files = generate_optimization_test_files();

    let mut group = c.benchmark_group("async_caching_optimization");

    // Baseline: No caching (direct file reads)
    group.bench_function("baseline_no_caching", |b| {
        b.iter(|| {
            rt.block_on(async {
                for (file, _, _) in &test_files {
                    let content = tokio::fs::read_to_string(file.path()).await.unwrap();
                    black_box(content.len());
                }
            });
        });
    });

    // Optimized: With async caching
    group.bench_function("optimized_async_caching", |b| {
        let cache = Arc::new(FileCache::new());

        b.iter(|| {
            rt.block_on(async {
                for (file, _, _) in &test_files {
                    // First access (cache miss)
                    let content1 = cache.get_or_load(file.path()).await.unwrap();
                    // Second access (cache hit - should be faster)
                    let content2 = cache.get_or_load(file.path()).await.unwrap();

                    black_box((content1.len(), content2.len()));
                }
            });
        });
    });

    // Cache performance under concurrent load
    group.bench_function("concurrent_cache_access", |b| {
        let cache = Arc::new(FileCache::new());
        let file_path = test_files[0].0.path().to_path_buf();

        b.iter(|| {
            rt.block_on(async {
                // Simulate concurrent cache access
                let mut handles = vec![];

                for _ in 0..10 {
                    let cache_clone = Arc::clone(&cache);
                    let path_clone = file_path.clone();

                    handles.push(tokio::spawn(async move {
                        cache_clone.get_or_load(&path_clone).await.unwrap()
                    }));
                }

                for handle in handles {
                    let content = handle.await.unwrap();
                    black_box(content.len());
                }
            });
        });
    });

    group.finish();
}

/// Benchmark parallelism optimizations with adaptive scaling
fn bench_parallelism_optimizations(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let test_files = generate_optimization_test_files();

    let mut group = c.benchmark_group("parallelism_optimization");

    // Baseline: Single-threaded analysis
    group.bench_function("baseline_single_threaded", |b| {
        let registry = AnalyzerRegistry::new();

        b.iter(|| {
            for (file, _, _) in &test_files {
                let content = std::fs::read_to_string(file.path()).unwrap();
                let result = registry.analyze_file(file.path(), content.as_bytes());
                black_box(result.unwrap());
            }
        });
    });

    // Optimized: Multi-threaded with rayon
    group.bench_function("optimized_parallel_rayon", |b| {
        let registry = AnalyzerRegistry::new();

        b.iter(|| {
            let results: Vec<_> = test_files
                .par_iter()
                .map(|(file, _, _)| {
                    let content = std::fs::read_to_string(file.path()).unwrap();
                    registry
                        .analyze_file(file.path(), content.as_bytes())
                        .unwrap()
                })
                .collect();

            black_box(results);
        });
    });

    // Adaptive parallelism with system load monitoring
    group.bench_function("adaptive_parallelism_with_monitoring", |b| {
        let controller = AdaptiveParallelismController::new(1, 8, 4);
        let registry = AnalyzerRegistry::new();

        b.iter(|| {
            rt.block_on(async {
                let current_load = SystemLoadMonitor::get_current_load().await.unwrap();
                let optimal_threads = controller.calculate_optimal_threads(&current_load);

                // Simulate adaptive analysis
                let results: Vec<_> = test_files
                    .par_iter()
                    .take(optimal_threads as usize)
                    .map(|(file, _, _)| {
                        let content = std::fs::read_to_string(file.path()).unwrap();
                        registry
                            .analyze_file(file.path(), content.as_bytes())
                            .unwrap()
                    })
                    .collect();

                black_box((results, optimal_threads));
            });
        });
    });

    group.finish();
}

/// Benchmark overall optimization impact with performance regression detection
fn bench_overall_optimization_impact(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let test_files = generate_optimization_test_files();

    let mut group = c.benchmark_group("overall_optimization_impact");

    // Baseline: Engine without optimizations
    group.bench_function("baseline_engine_no_optimizations", |b| {
        b.iter(|| {
            rt.block_on(async {
                let config = Config::minimal();
                let mut engine = GuardianEngine::new_with_ml(config, Default::default(), None)
                    .await
                    .unwrap();

                let file_paths: Vec<PathBuf> = test_files
                    .iter()
                    .take(2)
                    .map(|(file, _, _)| file.path().to_path_buf())
                    .collect();

                // Force minimal optimizations
                let result = engine.analyze_files(&file_paths, 1).await;
                black_box(result.unwrap());
            });
        });
    });

    // Optimized: Full engine with all optimizations enabled
    group.bench_function("optimized_engine_full_optimizations", |b| {
        b.iter(|| {
            rt.block_on(async {
                let config = Config::minimal();
                let mut engine = GuardianEngine::new_with_ml(config, Default::default(), None)
                    .await
                    .unwrap();

                let file_paths: Vec<PathBuf> = test_files
                    .iter()
                    .take(2)
                    .map(|(file, _, _)| file.path().to_path_buf())
                    .collect();

                // Use full optimizations (should be 2x+ faster)
                let result = engine.analyze_files(&file_paths, 4).await;
                black_box(result.unwrap());
            });
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_memory_pool_optimizations,
    bench_async_caching_optimizations,
    bench_parallelism_optimizations,
    bench_overall_optimization_impact
);
criterion_main!(benches);
