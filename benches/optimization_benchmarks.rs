use criterion::{black_box, criterion_group, criterion_main, Criterion};
use do_codeguardian::{
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
                let config = Config::default();
                let engine = GuardianEngine::new(config).unwrap();
                let _results = engine.analyze_files(&test_files, None).await.unwrap();
            });
        });
    });

    // Optimized: Engine with all optimizations enabled
    group.bench_function("optimized_engine_full_optimizations", |b| {
        b.iter(|| {
            rt.block_on(async {
                let mut config = Config::default();
                config.performance.enable_parallel_processing = true;
                config.performance.enable_memory_pool = true;
                config.performance.enable_caching = true;
                let engine = GuardianEngine::new(config).unwrap();
                let _results = engine.analyze_files(&test_files, None).await.unwrap();
            });
        });
    });

    group.finish();
}

/// Benchmark parallel output processing capabilities
fn bench_parallel_output_processing(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("parallel_output_processing");

    // Generate test analysis results
    let test_results = generate_test_analysis_results();

    // Sequential output generation
    group.bench_function("sequential_multiple_formats", |b| {
        b.iter(|| {
            rt.block_on(async {
                let formats = vec![
                    do_codeguardian::output::OutputFormat::Json,
                    do_codeguardian::output::OutputFormat::Html,
                    do_codeguardian::output::OutputFormat::Markdown,
                    do_codeguardian::output::OutputFormat::Sarif,
                ];

                let mut results = std::collections::HashMap::new();
                for format in &formats {
                    let output =
                        do_codeguardian::output::format_results(&test_results, *format).unwrap();
                    results.insert(*format, output);
                }

                black_box(results);
            });
        });
    });

    // Parallel output generation
    group.bench_function("parallel_multiple_formats", |b| {
        b.iter(|| {
            rt.block_on(async {
                use do_codeguardian::output::{OutputFormat, ParallelOutputProcessor};

                let processor = ParallelOutputProcessor::new().unwrap();
                let formats = vec![
                    OutputFormat::Json,
                    OutputFormat::Html,
                    OutputFormat::Markdown,
                    OutputFormat::Sarif,
                ];

                let results = processor
                    .process_multiple_formats(&test_results, formats)
                    .await
                    .unwrap();
                black_box(results);
            });
        });
    });

    group.finish();
}

/// Generate test analysis results for benchmarking
fn generate_test_analysis_results() -> do_codeguardian::types::AnalysisResults {
    use chrono::Utc;
    use do_codeguardian::types::{AnalysisResults, Finding, Severity};

    let mut results = AnalysisResults::new("benchmark_test".to_string());

    // Add some test findings
    for i in 0..100 {
        results.findings.push(Finding {
            id: format!("test-finding-{}", i),
            analyzer: "test_analyzer".to_string(),
            rule: "test_rule".to_string(),
            severity: Severity::Medium,
            file: std::path::PathBuf::from(format!("test_file_{}.rs", i)),
            line: i as u32 + 1,
            column: Some(10),
            message: format!("Test finding message {}", i),
            description: Some(format!("Test description {}", i)),
            suggestion: Some("Fix this issue".to_string()),
            category: Some("test".to_string()),
            metadata: std::collections::HashMap::new(),
        });
    }

    results.summary.total_findings = 100;
    results.summary.total_files_scanned = 10;
    results.timestamp = Utc::now();

    results
}

criterion_group!(
    benches,
    bench_memory_pool_optimizations,
    bench_async_caching_optimizations,
    bench_parallelism_optimizations,
    bench_overall_optimization_impact,
    bench_parallel_output_processing
);
criterion_main!(benches);
