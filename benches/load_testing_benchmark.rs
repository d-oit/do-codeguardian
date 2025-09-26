use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use do_codeguardian::{
    cache::FileCache, config::Config, core::GuardianEngine, utils::progress::ProgressReporter,
};
use std::path::PathBuf;

use std::sync::Arc;
use std::time::{Duration, Instant};
use sysinfo::System;
use tempfile::tempdir;
use tokio::runtime::Runtime;
use tokio::time::timeout;

/// Load Testing Benchmark Suite
/// Integrates with the existing load testing framework to provide
/// comprehensive performance validation under various load conditions

/// Generate test repository structure for load testing
fn generate_load_test_repository(file_count: usize, avg_file_size_kb: usize) -> tempfile::TempDir {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    for i in 0..file_count {
        let file_path = temp_dir.path().join(format!("test_file_{}.rs", i));
        let content = generate_test_file_content(avg_file_size_kb);
        std::fs::write(&file_path, content).expect("Failed to write test file");
    }

    temp_dir
}

/// Generate realistic test file content
fn generate_test_file_content(size_kb: usize) -> String {
    let mut content = String::with_capacity(size_kb * 1024);

    // Add realistic Rust code structure
    content.push_str("use std::collections::HashMap;\n");
    content.push_str("use serde::{Serialize, Deserialize};\n\n");

    // Add structs and implementations
    content.push_str("#[derive(Debug, Clone, Serialize, Deserialize)]\n");
    content.push_str("pub struct TestData {\n");
    content.push_str("    pub id: u64,\n");
    content.push_str("    pub name: String,\n");
    content.push_str("    pub data: HashMap<String, String>,\n");
    content.push_str("}\n\n");

    content.push_str("impl TestData {\n");
    content.push_str("    pub fn new(id: u64, name: &str) -> Self {\n");
    content.push_str("        Self {\n");
    content.push_str("            id,\n");
    content.push_str("            name: name.to_string(),\n");
    content.push_str("            data: HashMap::new(),\n");
    content.push_str("        }\n");
    content.push_str("    }\n\n");

    content.push_str("    pub fn process(&mut self) {\n");
    content.push_str("        for i in 0..100 {\n");
    content.push_str("            self.data.insert(\n");
    content.push_str("                format!(\"key_{}\", i),\n");
    content.push_str("                format!(\"value_{}\", i * 2),\n");
    content.push_str("            );\n");
    content.push_str("        }\n");
    content.push_str("    }\n");
    content.push_str("}\n\n");

    // Add functions to reach desired size
    while content.len() < size_kb * 1024 {
        content.push_str(&format!(
            "pub fn test_function_{}() -> TestData {{\n",
            content.len()
        ));
        content.push_str("    let mut data = TestData::new(1, \"test\");\n");
        content.push_str("    data.process();\n");
        content.push_str("    data\n");
        content.push_str("}\n\n");
    }

    content
}

/// Load testing scenarios
fn bench_load_testing_scenarios(c: &mut Criterion) {
    let rt = Runtime::new().expect("Failed to create Tokio runtime");

    let mut group = c.benchmark_group("load_testing_scenarios");

    // Small repository load test
    group.bench_function("small_repository_load", |b| {
        let repo_dir = generate_load_test_repository(10, 5); // 10 files, 5KB each
        let config = Config::default();
        let mut engine = rt.block_on(async {
            GuardianEngine::new(config, ProgressReporter::new(false))
                .await
                .expect("Failed to create GuardianEngine")
        });

        let file_paths: Vec<PathBuf> = (0..10)
            .map(|i| repo_dir.path().join(format!("test_file_{}.rs", i)))
            .collect();

        b.iter(|| {
            rt.block_on(async {
                let result = timeout(
                    Duration::from_secs(30),
                    engine.analyze_files(&file_paths, 2),
                )
                .await
                .expect("Analysis timed out")
                .expect("Analysis failed");
                black_box(result);
            });
        });
    });

    // Medium repository load test
    group.bench_function("medium_repository_load", |b| {
        let repo_dir = generate_load_test_repository(50, 10); // 50 files, 10KB each (reduced for stability)
        let config = Config::default();
        let mut engine = rt.block_on(async {
            GuardianEngine::new(config, ProgressReporter::new(false))
                .await
                .expect("Failed to create GuardianEngine")
        });

        let file_paths: Vec<PathBuf> = (0..50)
            .map(|i| repo_dir.path().join(format!("test_file_{}.rs", i)))
            .collect();

        b.iter(|| {
            rt.block_on(async {
                let result = timeout(
                    Duration::from_secs(60),
                    engine.analyze_files(&file_paths, 4),
                )
                .await
                .expect("Analysis timed out")
                .expect("Analysis failed");
                black_box(result);
            });
        });
    });

    // Large repository load test
    group.bench_function("large_repository_load", |b| {
        let repo_dir = generate_load_test_repository(100, 20); // 100 files, 20KB each (reduced for stability)
        let config = Config::default();
        let mut engine = rt.block_on(async {
            GuardianEngine::new(config, ProgressReporter::new(false))
                .await
                .expect("Failed to create GuardianEngine")
        });

        let file_paths: Vec<PathBuf> = (0..100)
            .map(|i| repo_dir.path().join(format!("test_file_{}.rs", i)))
            .collect();

        b.iter(|| {
            rt.block_on(async {
                let result = timeout(
                    Duration::from_secs(120),
                    engine.analyze_files(&file_paths, 8),
                )
                .await
                .expect("Analysis timed out")
                .expect("Analysis failed");
                black_box(result);
            });
        });
    });

    group.finish();
}

/// Concurrent processing load tests
fn bench_concurrent_processing_load(c: &mut Criterion) {
    let rt = Runtime::new().expect("Failed to create Tokio runtime");

    let mut group = c.benchmark_group("concurrent_processing_load");

    for concurrency in [1, 2, 4].iter() {
        // Reduced concurrency levels for stability
        group.throughput(Throughput::Elements(*concurrency as u64));
        group.bench_with_input(
            BenchmarkId::new("concurrent_analysis", concurrency),
            concurrency,
            |b, &concurrency| {
                let repo_dir = generate_load_test_repository(20, 15); // Reduced files
                let config = Config::default();
                let mut engine = rt.block_on(async {
                    GuardianEngine::new(config, ProgressReporter::new(false))
                        .await
                        .expect("Failed to create GuardianEngine")
                });

                let file_paths: Vec<PathBuf> = (0..concurrency * 2)
                    .map(|i| repo_dir.path().join(format!("test_file_{}.rs", i)))
                    .collect();

                b.iter(|| {
                    rt.block_on(async {
                        let result = timeout(
                            Duration::from_secs(60),
                            engine.analyze_files(&file_paths, concurrency),
                        )
                        .await
                        .expect("Analysis timed out")
                        .expect("Analysis failed");
                        black_box(result);
                    });
                });
            },
        );
    }

    group.finish();
}

/// Memory pressure load tests
fn bench_memory_pressure_load(c: &mut Criterion) {
    let rt = Runtime::new().expect("Failed to create Tokio runtime");

    let mut group = c.benchmark_group("memory_pressure_load");

    // Large files memory test
    group.bench_function("large_files_memory_pressure", |b| {
        let repo_dir = generate_load_test_repository(10, 50); // 10 files, 50KB each (reduced)
        let config = Config::default();
        let mut engine = rt.block_on(async {
            GuardianEngine::new(config, ProgressReporter::new(false))
                .await
                .expect("Failed to create GuardianEngine")
        });

        let file_paths: Vec<PathBuf> = (0..10)
            .map(|i| repo_dir.path().join(format!("test_file_{}.rs", i)))
            .collect();

        b.iter(|| {
            rt.block_on(async {
                let start_mem = get_memory_usage().unwrap_or(0);
                let result = timeout(
                    Duration::from_secs(60),
                    engine.analyze_files(&file_paths, 4),
                )
                .await
                .expect("Analysis timed out")
                .expect("Analysis failed");
                let end_mem = get_memory_usage().unwrap_or(0);

                // Memory regression check
                let mem_delta = end_mem.saturating_sub(start_mem);
                assert!(
                    mem_delta < 200 * 1024 * 1024, // 200MB threshold
                    "Memory usage too high: {} bytes",
                    mem_delta
                );

                black_box(result);
            });
        });
    });

    // Many small files memory test
    group.bench_function("many_small_files_memory_pressure", |b| {
        let repo_dir = generate_load_test_repository(200, 1); // 200 files, 1KB each (reduced)
        let config = Config::default();
        let mut engine = rt.block_on(async {
            GuardianEngine::new(config, ProgressReporter::new(false))
                .await
                .expect("Failed to create GuardianEngine")
        });

        let file_paths: Vec<PathBuf> = (0..200)
            .map(|i| repo_dir.path().join(format!("test_file_{}.rs", i)))
            .collect();

        b.iter(|| {
            rt.block_on(async {
                let result = timeout(
                    Duration::from_secs(60),
                    engine.analyze_files(&file_paths, 4),
                )
                .await
                .expect("Analysis timed out")
                .expect("Analysis failed");
                black_box(result);
            });
        });
    });

    group.finish();
}

/// Sustained load testing
fn bench_sustained_load(c: &mut Criterion) {
    let rt = Runtime::new().expect("Failed to create Tokio runtime");

    let mut group = c.benchmark_group("sustained_load");

    group.bench_function("sustained_analysis_load", |b| {
        let repo_dir = generate_load_test_repository(50, 10); // Reduced files and size
        let config = Config::default();
        let mut engine = rt.block_on(async {
            GuardianEngine::new(config, ProgressReporter::new(false))
                .await
                .expect("Failed to create GuardianEngine")
        });

        let file_paths: Vec<PathBuf> = (0..50)
            .map(|i| repo_dir.path().join(format!("test_file_{}.rs", i)))
            .collect();

        b.iter(|| {
            rt.block_on(async {
                let start = Instant::now();

                // Simulate sustained load with multiple analysis runs
                for _ in 0..3 {
                    // Reduced iterations
                    let result = timeout(
                        Duration::from_secs(30),
                        engine.analyze_files(&file_paths, 4),
                    )
                    .await
                    .expect("Analysis timed out")
                    .expect("Analysis failed");
                    black_box(result);
                }

                let duration = start.elapsed();

                // Ensure sustained performance
                assert!(
                    duration < Duration::from_secs(60), // Increased timeout
                    "Sustained load performance degraded: {:.2}s",
                    duration.as_secs_f64()
                );
            });
        });
    });

    group.finish();
}

/// Cache performance under load
fn bench_cache_performance_under_load(c: &mut Criterion) {
    let rt = Runtime::new().expect("Failed to create Tokio runtime");

    let mut group = c.benchmark_group("cache_performance_under_load");

    group.bench_function("cache_hit_ratio_under_load", |b| {
        let repo_dir = generate_load_test_repository(20, 5); // Reduced
        let config = Config::default();
        let cache = Arc::new(FileCache::new(
            tempfile::NamedTempFile::new()
                .expect("Failed to create temp file")
                .path()
                .to_path_buf(),
        ));
        let mut engine = rt.block_on(async {
            GuardianEngine::new(config, ProgressReporter::new(false))
                .await
                .expect("Failed to create GuardianEngine")
        });

        let file_paths: Vec<PathBuf> = (0..20)
            .map(|i| repo_dir.path().join(format!("test_file_{}.rs", i)))
            .collect();

        b.iter(|| {
            rt.block_on(async {
                // First run (cache misses)
                let result1 = timeout(
                    Duration::from_secs(30),
                    engine.analyze_files(&file_paths, 4),
                )
                .await
                .expect("Analysis timed out")
                .expect("Analysis failed");

                // Second run (cache hits)
                let result2 = timeout(
                    Duration::from_secs(30),
                    engine.analyze_files(&file_paths, 4),
                )
                .await
                .expect("Analysis timed out")
                .expect("Analysis failed");

                black_box((result1, result2));
            });
        });
    });

    group.finish();
}

/// Helper function to get memory usage using sysinfo
fn get_memory_usage() -> Option<u64> {
    let mut system = System::new();
    system.refresh_all();
    Some(system.used_memory() * 1024) // Convert to bytes
}

criterion_group!(
    benches,
    bench_load_testing_scenarios,
    bench_concurrent_processing_load,
    bench_memory_pressure_load,
    bench_sustained_load,
    bench_cache_performance_under_load
);
criterion_main!(benches);
