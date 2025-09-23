use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::hint::black_box;
use do_do_codeguardian::{
    analyzers::{performance_analyzer::PerformanceAnalyzer, security_analyzer::SecurityAnalyzer},
    cache::{CacheConfig, FileCache},
    config::{Config, PerformanceConfig},
    core::GuardianEngine,
};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;
use tempfile::NamedTempFile;
use tokio::runtime::Runtime;

/// Comparative Analysis Benchmark Suite
/// Provides cross-version, cross-configuration, and cross-environment performance comparisons

/// Configuration variants for comparative analysis
#[derive(Clone, Debug)]
struct BenchmarkConfig {
    name: String,
    cache_enabled: bool,
    parallel_workers: usize,
    memory_limit_mb: usize,
    cache_size_mb: usize,
}

impl BenchmarkConfig {
    fn standard() -> Self {
        Self {
            name: "standard".to_string(),
            cache_enabled: true,
            parallel_workers: 4,
            memory_limit_mb: 200,
            cache_size_mb: 50,
        }
    }

    fn minimal() -> Self {
        Self {
            name: "minimal".to_string(),
            cache_enabled: false,
            parallel_workers: 1,
            memory_limit_mb: 100,
            cache_size_mb: 10,
        }
    }

    fn optimized() -> Self {
        Self {
            name: "optimized".to_string(),
            cache_enabled: true,
            parallel_workers: 8,
            memory_limit_mb: 500,
            cache_size_mb: 200,
        }
    }
}

/// Generate comparative test data
fn generate_comparative_test_data() -> Vec<(NamedTempFile, String)> {
    let mut files = Vec::new();

    // Small file for baseline comparison
    let small_content = r#"
use std::collections::HashMap;

fn main() {
    let mut data = HashMap::new();
    data.insert("key", "value");
    println!("{:?}", data);
}
"#;
    let mut small_file = NamedTempFile::new().unwrap();
    small_file.write_all(small_content.as_bytes()).unwrap();
    files.push((small_file, "baseline.rs".to_string()));

    // Medium file with security patterns
    let medium_content = format!(
        "{}\n{}\n{}",
        "use std::env;",
        "fn main() { let api_key = env::var(\"API_KEY\").unwrap_or(\"default\".to_string()); println!(\"{}\", api_key); }",
        "pub struct User { password: String } impl User { pub fn new(pwd: &str) -> Self { Self { password: pwd.to_string() } } }"
    );
    let mut medium_file = NamedTempFile::new().unwrap();
    medium_file.write_all(medium_content.as_bytes()).unwrap();
    files.push((medium_file, "security.rs".to_string()));

    files
}

/// Comparative performance analysis benchmark
fn bench_comparative_performance_analysis(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let test_files = generate_comparative_test_data();
    let configs = vec![
        BenchmarkConfig::default(),
        BenchmarkConfig::standard(),
        BenchmarkConfig::optimized(),
    ];

    let mut group = c.benchmark_group("comparative_performance");

    for config in configs {
        group.bench_with_input(
            BenchmarkId::new("configuration_comparison", &config.name),
            &config,
            |b, config| {
                let mut engine = rt.block_on(async {
                    let mut cfg = Config::default();
                    cfg.performance.parallel_workers = config.parallel_workers;
                    cfg.performance.memory_limit_mb = config.memory_limit_mb;

                    if config.cache_enabled {
                        cfg.cache.enabled = true;
                        cfg.cache.max_size_mb = config.cache_size_mb;
                    } else {
                        cfg.cache.enabled = false;
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
                        let result = engine
                            .analyze_files(&file_paths, config.parallel_workers)
                            .await;
                        let duration = start.elapsed();

                        // Record comparative metrics
                        black_box((result.unwrap(), duration));
                    });
                });
            },
        );
    }

    group.finish();
}

/// Cache performance comparison benchmark
fn bench_cache_performance_comparison(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let test_files = generate_comparative_test_data();

    let mut group = c.benchmark_group("cache_comparison");

    // Compare different cache configurations
    let cache_configs = vec![
        (
            "no_cache",
            CacheConfig {
                enabled: false,
                max_size_mb: 0,
                ..Default::default()
            },
        ),
        (
            "small_cache",
            CacheConfig {
                enabled: true,
                max_size_mb: 10,
                ..Default::default()
            },
        ),
        (
            "large_cache",
            CacheConfig {
                enabled: true,
                max_size_mb: 100,
                ..Default::default()
            },
        ),
    ];

    for (name, cache_config) in cache_configs {
        group.bench_with_input(
            BenchmarkId::new("cache_configuration", name),
            &(name, cache_config),
            |b, (name, cache_config)| {
                let mut engine = rt.block_on(async {
                    let mut cfg = Config::default();
                    cfg.cache = cache_config.clone();
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
                        // Run analysis twice to test cache effectiveness
                        let _first_run = engine.analyze_files(&file_paths, 2).await;
                        let start = Instant::now();
                        let result = engine.analyze_files(&file_paths, 2).await;
                        let duration = start.elapsed();

                        black_box((result.unwrap(), duration));
                    });
                });
            },
        );
    }

    group.finish();
}

/// Memory usage comparison benchmark
fn bench_memory_usage_comparison(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let test_files = generate_comparative_test_data();

    let mut group = c.benchmark_group("memory_comparison");

    // Compare memory usage across different configurations
    let memory_configs = vec![
        ("low_memory", 50),
        ("standard_memory", 200),
        ("high_memory", 500),
    ];

    for (name, memory_limit) in memory_configs {
        group.bench_with_input(
            BenchmarkId::new("memory_configuration", name),
            &(name, memory_limit),
            |b, (name, memory_limit)| {
                let mut engine = rt.block_on(async {
                    let mut cfg = Config::default();
                    cfg.performance.memory_limit_mb = *memory_limit;
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
                        let start_mem = get_memory_usage().unwrap_or(0);
                        let result = engine.analyze_files(&file_paths, 2).await;
                        let end_mem = get_memory_usage().unwrap_or(0);
                        let mem_delta = end_mem.saturating_sub(start_mem);

                        black_box((result.unwrap(), mem_delta));
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

criterion_group!(
    benches,
    bench_comparative_performance_analysis,
    bench_cache_performance_comparison,
    bench_memory_usage_comparison
);
criterion_main!(benches);
