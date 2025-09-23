use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use std::path::Path;
use walkdir::WalkDir;

fn scanning_benchmark(c: &mut Criterion) {
    let test_dir = ".";

    c.bench_function("walkdir scan", |b| {
        b.iter(|| {
            WalkDir::new(black_box(test_dir))
                .into_iter()
                .filter_map(|e| e.ok())
                .count()
        })
    });

    c.bench_function("ignore scan", |b| {
        b.iter(|| {
            ignore::Walk::new(black_box(test_dir))
                .filter_map(|e| e.ok())
                .count()
        })
    });
}

fn analysis_performance_benchmark(c: &mut Criterion) {
    let test_files = vec![
        Path::new("src/main.rs"),
        Path::new("src/core.rs"),
        Path::new("src/analyzers/performance_analyzer.rs"),
    ];

    c.bench_function("file analysis performance", |b| {
        b.iter(|| {
            for file_path in &test_files {
                if file_path.exists() {
                    let _content = std::fs::read_to_string(file_path);
                    // Simulate analysis workload
                    black_box(file_path);
                }
            }
        })
    });
}

criterion_group!(benches, scanning_benchmark, analysis_performance_benchmark);
criterion_main!(benches);
