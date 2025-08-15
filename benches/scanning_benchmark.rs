use criterion::{black_box, criterion_group, criterion_main, Criterion};
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

criterion_group!(benches, scanning_benchmark);
criterion_main!(benches);
