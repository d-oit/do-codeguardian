use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use do_do_codeguardian::{
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
use tempfile::NamedTempFile;
use tokio::runtime::Runtime;

/// Generate test files of various sizes and types
fn generate_test_files() -> Vec<(NamedTempFile, String, usize)> {
    let mut files = Vec::new();

    // Small file (1KB)
    let small_content = "fn main() {\n    println!(\"Hello, world!\");\n}\n".repeat(50);
    let mut small_file = NamedTempFile::new().unwrap();
    small_file.write_all(small_content.as_bytes()).unwrap();
    files.push((small_file, "small.rs".to_string(), small_content.len()));

    // Medium file (100KB)
    let medium_content = format!(
        "{}\n{}\n{}\n",
        "#[derive(Debug, Clone)]".repeat(1000),
        "pub struct TestStruct { field: String }".repeat(1000),
        "impl TestStruct { pub fn new() -> Self { Self { field: String::new() } } }".repeat(1000)
    );
    let mut medium_file = NamedTempFile::new().unwrap();
    medium_file.write_all(medium_content.as_bytes()).unwrap();
    files.push((medium_file, "medium.rs".to_string(), medium_content.len()));

    // Large file (1MB)
    let large_content = format!(
        "{}\n{}\n{}\n{}\n",
        "// Large file with many functions".repeat(5000),
        "pub fn function_1() { println!(\"test\"); }".repeat(2000),
        "pub fn function_2() { println!(\"test\"); }".repeat(2000),
        "pub fn function_3() { println!(\"test\"); }".repeat(2000)
    );
    let mut large_file = NamedTempFile::new().unwrap();
    large_file.write_all(large_content.as_bytes()).unwrap();
    files.push((large_file, "large.rs".to_string(), large_content.len()));

    // JSON file
    let json_content = format!(
        r#"{{
            "users": [
                {}
            ],
            "config": {{
                "debug": true,
                "timeout": 30
            }}
        }}"#,
        r#"{"id": 1, "name": "test", "email": "test@example.com"}"#.repeat(1000)
    );
    let mut json_file = NamedTempFile::new().unwrap();
    json_file.write_all(json_content.as_bytes()).unwrap();
    files.push((json_file, "data.json".to_string(), json_content.len()));

    files
}

/// Benchmark cache operations
fn bench_cache_operations(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("cache_load_empty", |b| {
        let cache_path = tempfile::NamedTempFile::new().unwrap().path().to_path_buf();
        b.iter(|| {
            rt.block_on(async {
                black_box(FileCache::load(cache_path.clone()).await.unwrap());
            });
        });
    });

    c.bench_function("cache_save_empty", |b| {
        let cache_path = tempfile::NamedTempFile::new().unwrap().path().to_path_buf();
        b.iter(|| {
            rt.block_on(async {
                let cache = FileCache::new(cache_path.clone());
                black_box(cache.save().await.unwrap());
            });
        });
    });
}

/// Benchmark streaming operations
fn bench_streaming_operations(c: &mut Criterion) {
    let test_files = generate_test_files();
    let rt = Runtime::new().unwrap();

    c.bench_function("streaming_analyzer_creation", |b| {
        b.iter(|| {
            black_box(StreamingAnalyzer::new());
        });
    });

    c.bench_function("streaming_text_analysis_small", |b| {
        let small_file = &test_files[0];
        let analyzer = StreamingAnalyzer::new();

        b.iter(|| {
            let result = analyzer.analyze_text_streaming(small_file.0.path(), |line, _line_num| {
                if line.contains("fn") {
                    Ok(vec![Finding::new(
                        "test",
                        "function_found",
                        do_do_codeguardian::types::Severity::Info,
                        small_file.0.path().to_path_buf(),
                        1,
                        "Function found".to_string(),
                    )])
                } else {
                    Ok(vec![])
                }
            });
            black_box(result.unwrap());
        });
    });

    c.bench_function("streaming_async_analysis_medium", |b| {
        let medium_file = &test_files[1];
        let analyzer = StreamingAnalyzer::new();

        b.iter(|| {
            rt.block_on(async {
                let result = analyzer
                    .analyze_large_file(medium_file.0.path(), |line, _line_num| {
                        if line.contains("struct") {
                            Ok(vec![Finding::new(
                                "test",
                                "struct_found",
                                do_do_codeguardian::types::Severity::Info,
                                medium_file.0.path().to_path_buf(),
                                1,
                                "Struct found".to_string(),
                            )])
                        } else {
                            Ok(vec![])
                        }
                    })
                    .await;
                black_box(result.unwrap());
            });
        });
    });
}

/// Benchmark memory pool operations
fn bench_memory_pool_operations(c: &mut Criterion) {
    c.bench_function("memory_pool_string_operations", |b| {
        thread_local_pools::init();
        b.iter(|| {
            let mut buffer = thread_local_pools::get_string_buffer();
            buffer.push_str("test content");
            black_box(buffer.len());
            thread_local_pools::put_string_buffer(buffer);
        });
    });

    c.bench_function("memory_pool_findings_operations", |b| {
        thread_local_pools::init();
        b.iter(|| {
            let mut findings = thread_local_pools::get_findings_vec();
            findings.push(Finding::new(
                "test",
                "test_finding",
                do_codeguardian::types::Severity::Info,
                PathBuf::from("test.rs"),
                1,
                "Test finding".to_string(),
            ));
            black_box(findings.len());
            thread_local_pools::put_findings_vec(findings);
        });
    });
}

/// Benchmark adaptive parallelism
fn bench_adaptive_parallelism(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("adaptive_parallelism_controller_creation", |b| {
        b.iter(|| {
            black_box(AdaptiveParallelismController::new(1, 8, 4));
        });
    });

    c.bench_function("adaptive_parallelism_load_update", |b| {
        let controller = AdaptiveParallelismController::new(1, 8, 4);
        let load = do_codeguardian::utils::adaptive_parallelism::SystemLoad::new();

        b.iter(|| {
            rt.block_on(async {
                black_box(controller.update_load(load.clone()).await.unwrap());
            });
        });
    });
}

/// Benchmark analyzer registry operations
fn bench_analyzer_registry(c: &mut Criterion) {
    c.bench_function("analyzer_registry_creation", |b| {
        b.iter(|| {
            black_box(AnalyzerRegistry::new());
        });
    });

    c.bench_function("analyzer_registry_analyze_file", |b| {
        let registry = AnalyzerRegistry::new();
        let content = "fn main() { println!(\"Hello, world!\"); }";

        b.iter(|| {
            let result =
                registry.analyze_file(PathBuf::from("test.rs").as_path(), content.as_bytes());
            black_box(result.unwrap());
        });
    });
}

/// Benchmark configuration operations
fn bench_configuration_operations(c: &mut Criterion) {
    c.bench_function("performance_config_creation", |b| {
        b.iter(|| {
            black_box(PerformanceConfig::default());
        });
    });
}

/// Benchmark full engine operations with baselines and regression detection
fn bench_full_engine_operations(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let test_files = generate_test_files();

    c.bench_function("guardian_engine_creation", |b| {
        b.iter(|| {
            rt.block_on(async {
                let config = Config::default();
                black_box(
                    GuardianEngine::new(config, Default::default())
                        .await
                        .unwrap(),
                );
            });
        });
    });

    c.bench_function("guardian_engine_analyze_small_files", |b| {
        let config = Config::default();
        let mut engine = rt.block_on(async {
            GuardianEngine::new(config, Default::default())
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
                let result = engine.analyze_files(&file_paths, 2).await;
                black_box(result.unwrap());
            });
        });
    });

    // Baseline comparison: Sequential vs Parallel analysis
    let mut group = c.benchmark_group("analysis_performance_comparison");
    group.bench_function("baseline_sequential_analysis", |b| {
        let config = Config::default();
        let mut engine = rt.block_on(async {
            GuardianEngine::new(config, Default::default())
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
                // Force sequential processing by setting parallelism to 1
                let result = engine.analyze_files(&file_paths, 1).await;
                black_box(result.unwrap());
            });
        });
    });

    group.bench_function("optimized_parallel_analysis", |b| {
        let config = Config::default();
        let mut engine = rt.block_on(async {
            GuardianEngine::new(config, Default::default())
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
                // Use adaptive parallelism (should be 2x+ faster)
                let result = engine.analyze_files(&file_paths, 4).await;
                black_box(result.unwrap());
            });
        });
    });
    group.finish();

    // Memory usage benchmark with regression detection
    c.bench_function("memory_usage_during_analysis", |b| {
        let config = Config::default();
        let mut engine = rt.block_on(async {
            GuardianEngine::new(config, Default::default())
                .await
                .unwrap()
        });

        let file_paths: Vec<PathBuf> = test_files
            .iter()
            .take(4) // Include larger files for memory stress
            .map(|(file, _, _)| file.path().to_path_buf())
            .collect();

        b.iter(|| {
            rt.block_on(async {
                // Measure peak memory usage during analysis
                let start_mem = get_current_memory_usage().unwrap_or(0);
                let result = engine.analyze_files(&file_paths, 4).await;
                let end_mem = get_current_memory_usage().unwrap_or(0);

                // Ensure we don't exceed reasonable memory limits (regression test)
                assert!(
                    end_mem - start_mem < 100 * 1024 * 1024, // 100MB limit
                    "Memory usage regression detected: {} bytes",
                    end_mem - start_mem
                );

                black_box(result.unwrap());
            });
        });
    });
}

/// Helper function to get current memory usage (approximate)
fn get_current_memory_usage() -> anyhow::Result<u64> {
    // Use sysinfo for memory tracking (already in dependencies)
    let mut system = sysinfo::System::new_all();
    system.refresh_all();

    let process = system
        .process(sysinfo::Pid::from(std::process::id() as usize))
        .ok_or_else(|| anyhow::anyhow!("Could not find current process"))?;

    Ok(process.memory())
}

criterion_group!(
    benches,
    bench_cache_operations,
    bench_streaming_operations,
    bench_memory_pool_operations,
    bench_adaptive_parallelism,
    bench_analyzer_registry,
    bench_configuration_operations,
    bench_full_engine_operations
);
criterion_main!(benches);
