use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn simple_benchmark(c: &mut Criterion) {
    c.bench_function("simple_test", |b| {
        b.iter(|| {
            let result = (0..1000).sum::<i32>();
            black_box(result);
        });
    });
}

criterion_group!(benches, simple_benchmark);
criterion_main!(benches);
