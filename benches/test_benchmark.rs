use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use do_codeguardian::{
    analyzers::{security_analyzer::SecurityAnalyzer, Analyzer},
    config::Config,
    core::GuardianEngine,
    utils::progress::ProgressReporter,
};
use std::hint::black_box;
use std::path::PathBuf;
use tempfile::tempdir;
use tokio::runtime::Runtime;

/// Generate test files for basic benchmarking
fn generate_test_files(count: usize) -> Vec<PathBuf> {
    let temp_dir = tempdir().unwrap();
    let mut file_paths = Vec::new();

    for i in 0..count {
        let file_path = temp_dir.path().join(format!("test_file_{}.rs", i));
        let content = format!(
            r#"fn main() {{
    let password = "hardcoded_password_{}";
    let api_key = "sk-1234567890abcdef";
    println!("Password: {{}}", password);
    println!("API Key: {{}}", api_key);
}}

pub fn test_function() {{
    let mut data = vec![];
    for _ in 0..100 {{
        data.push("test_data".to_string());
    }}
}}"#,
            i
        );
        std::fs::write(&file_path, content).unwrap();
        file_paths.push(file_path);
    }

    file_paths
}

fn benchmark_basic_analysis(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let test_files = generate_test_files(5);

    let mut group = c.benchmark_group("basic_analysis");
    group.throughput(Throughput::Elements(5));

    group.bench_function("analyze_files", |b| {
        b.iter(|| {
            rt.block_on(async {
                let config = Config::default();
                let mut engine = GuardianEngine::new(config, ProgressReporter::new(false))
                    .await
                    .unwrap();
                let result = engine.analyze_files(&test_files, 2).await;
                black_box(result.unwrap());
            });
        });
    });

    group.finish();
}

fn benchmark_security_analyzer(c: &mut Criterion) {
    let analyzer = SecurityAnalyzer::new();
    let file_path = PathBuf::from("test.rs");

    let mut group = c.benchmark_group("security_analyzer");

    for size in [1024, 5120, 10240].iter() {
        let content = format!(
            r#"fn main() {{
    let password = "hardcoded_password";
    let api_key = "sk-1234567890abcdef";
    println!("Password: {{}}", password);
    println!("API Key: {{}}", api_key);
}}
{}
"#,
            "x".repeat(*size)
        );

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(
            BenchmarkId::new("analyze_content", size),
            &content,
            |b, content| {
                b.iter(|| {
                    let result = analyzer.analyze(&file_path, content.as_bytes());
                    black_box(result);
                });
            },
        );
    }

    group.finish();
}

fn benchmark_configuration_loading(c: &mut Criterion) {
    let mut group = c.benchmark_group("config_loading");

    group.bench_function("load_default_config", |b| {
        b.iter(|| {
            let config = Config::default();
            black_box(config);
        });
    });

    group.bench_function("create_engine", |b| {
        b.iter(|| {
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                let config = Config::default();
                let engine = GuardianEngine::new(config, ProgressReporter::new(false))
                    .await
                    .unwrap();
                black_box(engine);
            });
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_basic_analysis,
    benchmark_security_analyzer,
    benchmark_configuration_loading
);
criterion_main!(benches);
