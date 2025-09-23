use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

fn hash_benchmark(c: &mut Criterion) {
    let data = b"hello world";

    c.bench_function("blake3 hash", |b| {
        b.iter(|| {
            let mut hasher = blake3::Hasher::new();
            hasher.update(black_box(data));
            hasher.finalize()
        })
    });

    c.bench_function("sha256 hash", |b| {
        b.iter(|| {
            use sha2::{Digest, Sha256};
            let mut hasher = Sha256::new();
            hasher.update(black_box(data));
            hasher.finalize()
        })
    });
}

criterion_group!(benches, hash_benchmark);
criterion_main!(benches);
