use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use do_codeguardian::analyzers::Analyzer;
use do_codeguardian::analyzers::{
    performance_analyzer::PerformanceAnalyzer, security_analyzer::SecurityAnalyzer,
};
use std::hint::black_box;
use std::path::PathBuf;
use tempfile::tempdir;

/// Comprehensive performance benchmarks for CodeGuardian
/// These benchmarks help identify performance regressions and optimization opportunities

/// Generate test content with hardcoded secrets for security analyzer benchmarks
fn generate_test_content_with_secrets(size: usize) -> String {
    let mut content = String::with_capacity(size);

    // Add some realistic Rust code structure
    content.push_str("use std::collections::HashMap;\n\n");
    content.push_str("pub struct Config {\n");
    content.push_str("    pub api_key: String,\n");
    content.push_str("    pub database_url: String,\n");
    content.push_str("    pub secret_token: String,\n");
    content.push_str("}\n\n");

    content.push_str("impl Config {\n");
    content.push_str("    pub fn new() -> Self {\n");
    content.push_str("        Self {\n");

    // Add hardcoded secrets that should be detected - ALL ARE TEST DATA ONLY
    content.push_str("            // TEST DATA: Fake API key for security analyzer testing\n");
    content.push_str("            api_key: \"sk-1234567890abcdef1234567890abcdef\".to_string(),\n");
    content.push_str("            // TEST DATA: Fake database URL with test credentials for security analyzer testing\n");
    content.push_str(
        "            database_url: \"postgres://user:password123@localhost/db\".to_string(),\n",
    );
    content.push_str("            // TEST DATA: Fake GitHub token for security analyzer testing\n");
    content.push_str(
        "            secret_token: \"ghp_1234567890abcdef1234567890abcdef12345678\".to_string(),\n",
    );

    content.push_str("        }\n");
    content.push_str("    }\n");
    content.push_str("}\n\n");

    // Add more content to reach desired size
    while content.len() < size {
        content.push_str(&format!("/// This is a comment line {}\n", content.len()));
        content.push_str(&format!("pub fn function_{}() {{}}\n", content.len()));
    }

    content
}

/// Generate test content with performance issues for performance analyzer benchmarks
fn generate_test_content_with_performance_issues(size: usize) -> String {
    let mut content = String::with_capacity(size);

    // Add some realistic Rust code with performance issues
    content.push_str("use std::collections::HashMap;\n\n");

    // Add nested loops (performance issue)
    content.push_str("pub fn nested_loops_example(data: &[Vec<i32>]) {\n");
    content.push_str("    for i in 0..100 {\n");
    content.push_str("        for j in 0..100 {\n");
    content.push_str("            println!(\"{} {}\", i, j);\n");
    content.push_str("        }\n");
    content.push_str("    }\n");
    content.push_str("}\n\n");

    // Add inefficient string operations
    content.push_str("pub fn inefficient_string_ops() {\n");
    content.push_str("    let mut result = String::new();\n");
    content.push_str("    for i in 0..1000 {\n");
    content.push_str("        result += &format!(\"item_{} \", i);\n");
    content.push_str("    }\n");
    content.push_str("}\n\n");

    // Add blocking I/O in async context
    content.push_str("#[tokio::main]\n");
    content.push_str("async fn blocking_io_example() {\n");
    content.push_str("    let content = std::fs::read_to_string(\"file.txt\").unwrap();\n");
    content.push_str("    println!(\"{}\", content);\n");
    content.push_str("}\n\n");

    // Add algorithmic inefficiency
    content.push_str("pub fn algorithmic_inefficiency(data: &[i32]) {\n");
    content.push_str("    let collected: Vec<_> = data.iter().collect();\n");
    content.push_str("    let result: Vec<_> = collected.iter().map(|x| x * 2).collect();\n");
    content.push_str("}\n\n");

    // Add more content to reach desired size
    while content.len() < size {
        content.push_str(&format!("/// Performance comment {}\n", content.len()));
        content.push_str(&format!(
            "pub fn perf_function_{}() -> i32 {{ {} }}\n",
            content.len(),
            content.len()
        ));
    }

    content
}

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

    let analyzer = PerformanceAnalyzer::new().unwrap();
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

    // Benchmark regex compilation - using test patterns for security analysis performance
    group.bench_function("compile_secret_regex", |b| {
        b.iter(|| regex::Regex::new(black_box(r"password\s*=\s*[^;]+")))
    });

    // Benchmark regex matching - test pattern for security analysis benchmarks
    let regex = regex::Regex::new(r"password\s*=\s*[^;]+").unwrap();
    group.bench_function("match_secrets", |b| {
        b.iter(|| regex.find_iter(black_box(&test_content)).count())
    });

    group.finish();
}
criterion_group!(
    benches,
    benchmark_security_analyzer,
    benchmark_performance_analyzer,
    benchmark_file_operations,
    benchmark_hashing_algorithms,
    benchmark_regex_performance
);
criterion_main!(benches);
