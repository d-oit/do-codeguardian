use criterion::{black_box, criterion_group, criterion_main, Criterion};
use do_codeguardian::{
    analyze_files, utils::adaptive_parallelism::AdaptiveParallelismController, Config,
};
use std::io::Write;
use std::path::PathBuf;
use std::time::Duration;
use tempfile::NamedTempFile;
use tokio::runtime::Runtime;
use tokio::time::sleep;

/// Generate test files for chaos testing
fn generate_chaos_test_files(count: usize) -> Vec<(NamedTempFile, String, usize)> {
    let mut files = Vec::new();

    for i in 0..count {
        // Create files with potential security issues for testing
        let content = format!(
            r#"fn main() {{
    let password = "hardcoded_password_{}";
    let api_key = "sk-1234567890abcdef";
    println!("Password: {{}}", password);
    println!("API Key: {{}}", api_key);
}}

pub fn insecure_function() {{
    let mut data = vec![];
    // Potential buffer overflow simulation
    for _ in 0..100000 {{
        data.push("large_string_data".to_string());
    }}
}}"#,
            i
        );

        let mut file = NamedTempFile::new().unwrap();
        file.write_all(content.as_bytes()).unwrap();
        files.push((file, format!("chaos_test_{}.rs", i), content.len()));
    }

    files
}

/// Simulate network delays (chaos scenario)
async fn simulate_network_delay(delay_ms: u64) {
    sleep(Duration::from_millis(delay_ms)).await;
}

/// Simulate resource exhaustion (chaos scenario)
fn simulate_cpu_intensive_work(iterations: usize) -> u64 {
    let mut result = 0u64;
    for i in 0..iterations {
        result = result.wrapping_add(i as u64);
        result = result.wrapping_mul(31);
    }
    result
}

/// Simulate memory pressure (chaos scenario)
fn simulate_memory_pressure(size_mb: usize) -> Vec<u8> {
    vec![0u8; size_mb * 1024 * 1024]
}

/// Benchmark analysis under network chaos
fn bench_network_chaos(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let test_files = generate_chaos_test_files(10);
    let file_paths: Vec<PathBuf> = test_files
        .iter()
        .map(|(file, _, _)| file.path().to_path_buf())
        .collect();

    let mut group = c.benchmark_group("network_chaos");
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("normal_analysis", |b| {
        b.iter(|| {
            rt.block_on(async {
                let config = Config::default();
                let result = analyze_files(&file_paths, &config).await;
                black_box(result.unwrap());
            });
        });
    });

    group.bench_function("with_network_delays_50ms", |b| {
        b.iter(|| {
            rt.block_on(async {
                simulate_network_delay(50).await;
                let config = Config::default();
                let result = analyze_files(&file_paths, &config).await;
                black_box(result.unwrap());
            });
        });
    });

    group.bench_function("with_network_delays_200ms", |b| {
        b.iter(|| {
            rt.block_on(async {
                simulate_network_delay(200).await;
                let config = Config::default();
                let result = analyze_files(&file_paths, &config).await;
                black_box(result.unwrap());
            });
        });
    });

    group.finish();
}

/// Benchmark analysis under CPU chaos
fn bench_cpu_chaos(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let test_files = generate_chaos_test_files(5);
    let file_paths: Vec<PathBuf> = test_files
        .iter()
        .map(|(file, _, _)| file.path().to_path_buf())
        .collect();

    let mut group = c.benchmark_group("cpu_chaos");
    group.measurement_time(Duration::from_secs(15));

    group.bench_function("normal_analysis", |b| {
        b.iter(|| {
            rt.block_on(async {
                let config = Config::default();
                let result = analyze_files(&file_paths, &config).await;
                black_box(result.unwrap());
            });
        });
    });

    group.bench_function("with_cpu_load_light", |b| {
        b.iter(|| {
            // Simulate light CPU load
            black_box(simulate_cpu_intensive_work(100000));
            rt.block_on(async {
                let config = Config::default();
                let result = analyze_files(&file_paths, &config).await;
                black_box(result.unwrap());
            });
        });
    });

    group.bench_function("with_cpu_load_heavy", |b| {
        b.iter(|| {
            // Simulate heavy CPU load
            black_box(simulate_cpu_intensive_work(1000000));
            rt.block_on(async {
                let config = Config::default();
                let result = analyze_files(&file_paths, &config).await;
                black_box(result.unwrap());
            });
        });
    });

    group.finish();
}

/// Benchmark analysis under memory chaos
fn bench_memory_chaos(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let test_files = generate_chaos_test_files(3);
    let file_paths: Vec<PathBuf> = test_files
        .iter()
        .map(|(file, _, _)| file.path().to_path_buf())
        .collect();

    let mut group = c.benchmark_group("memory_chaos");
    group.measurement_time(Duration::from_secs(20));

    group.bench_function("normal_analysis", |b| {
        b.iter(|| {
            rt.block_on(async {
                let config = Config::default();
                let result = analyze_files(&file_paths, &config).await;
                black_box(result.unwrap());
            });
        });
    });

    group.bench_function("with_memory_pressure_50mb", |b| {
        b.iter(|| {
            // Simulate memory pressure
            let _memory_hog = simulate_memory_pressure(50);
            rt.block_on(async {
                let config = Config::default();
                let result = analyze_files(&file_paths, &config).await;
                black_box(result.unwrap());
            });
            drop(_memory_hog);
        });
    });

    group.bench_function("with_memory_pressure_100mb", |b| {
        b.iter(|| {
            // Simulate higher memory pressure
            let _memory_hog = simulate_memory_pressure(100);
            rt.block_on(async {
                let config = Config::default();
                let result = analyze_files(&file_paths, &config).await;
                black_box(result.unwrap());
            });
            drop(_memory_hog);
        });
    });

    group.finish();
}

/// Benchmark analysis under combined chaos scenarios
fn bench_combined_chaos(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let test_files = generate_chaos_test_files(5);
    let file_paths: Vec<PathBuf> = test_files
        .iter()
        .map(|(file, _, _)| file.path().to_path_buf())
        .collect();

    let mut group = c.benchmark_group("combined_chaos");
    group.measurement_time(Duration::from_secs(30));

    group.bench_function("normal_analysis", |b| {
        b.iter(|| {
            rt.block_on(async {
                let config = Config::default();
                let result = analyze_files(&file_paths, &config).await;
                black_box(result.unwrap());
            });
        });
    });

    group.bench_function("chaos_scenario_1", |b| {
        b.iter(|| {
            rt.block_on(async {
                // Network delay + CPU load
                simulate_network_delay(100).await;
                black_box(simulate_cpu_intensive_work(500000));
                let config = Config::default();
                let result = analyze_files(&file_paths, &config).await;
                black_box(result.unwrap());
            });
        });
    });

    group.bench_function("chaos_scenario_2", |b| {
        b.iter(|| {
            rt.block_on(async {
                // Memory pressure + network delay
                let _memory_hog = simulate_memory_pressure(75);
                simulate_network_delay(150).await;
                let config = Config::default();
                let result = analyze_files(&file_paths, &config).await;
                black_box(result.unwrap());
                drop(_memory_hog);
            });
        });
    });

    group.bench_function("chaos_scenario_3", |b| {
        b.iter(|| {
            rt.block_on(async {
                // All chaos factors combined
                let _memory_hog = simulate_memory_pressure(50);
                simulate_network_delay(75).await;
                black_box(simulate_cpu_intensive_work(250000));
                let config = Config::default();
                let result = analyze_files(&file_paths, &config).await;
                black_box(result.unwrap());
                drop(_memory_hog);
            });
        });
    });

    group.finish();
}

/// Benchmark adaptive parallelism under chaos
fn bench_adaptive_parallelism_chaos(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let test_files = generate_chaos_test_files(20);
    let file_paths: Vec<PathBuf> = test_files
        .iter()
        .map(|(file, _, _)| file.path().to_path_buf())
        .collect();

    let mut group = c.benchmark_group("adaptive_parallelism_chaos");
    group.measurement_time(Duration::from_secs(20));

    group.bench_function("adaptive_parallelism_normal", |b| {
        let controller = AdaptiveParallelismController::new(1, 8, 4);
        b.iter(|| {
            rt.block_on(async {
                let config = Config::default();
                let result = analyze_files(&file_paths, &config).await;
                black_box(result.unwrap());
                // Simulate adaptive adjustment
                black_box(controller.adjust_parallelism().await.unwrap());
            });
        });
    });

    group.bench_function("adaptive_parallelism_under_load", |b| {
        let controller = AdaptiveParallelismController::new(1, 8, 4);
        b.iter(|| {
            rt.block_on(async {
                // Simulate system under load
                black_box(simulate_cpu_intensive_work(200000));
                let config = Config::default();
                let result = analyze_files(&file_paths, &config).await;
                black_box(result.unwrap());
                // Adaptive adjustment under load
                black_box(controller.adjust_parallelism().await.unwrap());
            });
        });
    });

    group.finish();
}

criterion_group!(
    chaos_benches,
    bench_network_chaos,
    bench_cpu_chaos,
    bench_memory_chaos,
    bench_combined_chaos,
    bench_adaptive_parallelism_chaos
);
criterion_main!(chaos_benches);
