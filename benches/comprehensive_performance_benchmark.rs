use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use do_codeguardian::analyzers::{
    performance_analyzer::PerformanceAnalyzer, security_analyzer::SecurityAnalyzer,
};
use std::path::PathBuf;
use tempfile::tempdir;

/// Comprehensive performance benchmarks for CodeGuardian
/// These benchmarks help identify performance regressions and optimization opportunities

fn benchmark_security_analyzer(c: &mut Criterion) {
    let mut group = c.benchmark_group("security_analyzer");

    let analyzer = SecurityAnalyzer::new();
    let file_path = PathBuf::from("test.rs");

    // Test different content sizes
    for size in [1024, 10_240, 102_400].iter() {
        let content = generate_test_content_with_secrets(*size);

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(
            BenchmarkId::new("analyze_content", size),
            &content,
            |b, content| {
                b.iter(|| analyzer.analyze(black_box(&file_path), black_box(content.as_bytes())))
            },
        );
    }

    group.finish();
}

fn benchmark_performance_analyzer(c: &mut Criterion) {
    let mut group = c.benchmark_group("performance_analyzer");

    let analyzer = PerformanceAnalyzer::new();
    let file_path = PathBuf::from("test.rs");

    // Test different content sizes with performance patterns
    for size in [1024, 10_240, 102_400].iter() {
        let content = generate_test_content_with_performance_issues(*size);

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(
            BenchmarkId::new("analyze_performance", size),
            &content,
            |b, content| {
                b.iter(|| analyzer.analyze(black_box(&file_path), black_box(content.as_bytes())))
            },
        );
    }

    group.finish();
}

fn benchmark_file_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("file_operations");

    // Benchmark file reading performance
    for size in [1024, 10_240, 102_400, 1_024_000].iter() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join(format!("test_{}.rs", size));
        let content = "x".repeat(*size);
        std::fs::write(&file_path, &content).unwrap();

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(
            BenchmarkId::new("read_file", size),
            &file_path,
            |b, path| b.iter(|| std::fs::read(black_box(path))),
        );
    }

    group.finish();
}

fn benchmark_hashing_algorithms(c: &mut Criterion) {
    let mut group = c.benchmark_group("hashing");

    for size in [1024, 10_240, 102_400].iter() {
        let data = vec![0u8; *size];

        group.throughput(Throughput::Bytes(*size as u64));

        // BLAKE3 hashing
        group.bench_with_input(BenchmarkId::new("blake3", size), &data, |b, data| {
            b.iter(|| {
                let mut hasher = blake3::Hasher::new();
                hasher.update(black_box(data));
                hasher.finalize()
            })
        });

        // SHA256 hashing
        group.bench_with_input(BenchmarkId::new("sha256", size), &data, |b, data| {
            b.iter(|| {
                use sha2::{Digest, Sha256};
                let mut hasher = Sha256::new();
                hasher.update(black_box(data));
                hasher.finalize()
            })
        });
    }

    group.finish();
}

fn benchmark_regex_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("regex");

    let test_content = generate_test_content_with_secrets(10_240);

    // Benchmark regex compilation
    group.bench_function("compile_secret_regex", |b| {
        b.iter(|| regex::Regex::new(black_box(r"password\s*=\s*[^;]+")))
    });

    // Benchmark regex matching
    let regex = regex::Regex::new(r"password\s*=\s*[^;]+").unwrap();
    group.bench_function("match_secrets", |b| {
        b.iter(|| regex.find_iter(black_box(&test_content)).count())
    });

    group.finish();
}
