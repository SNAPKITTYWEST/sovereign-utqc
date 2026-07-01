use criterion::{criterion_group, criterion_main, Criterion};
use sovereign_field_simd::SimdGoldilocks;

fn bench_add(c: &mut Criterion) {
    let a = SimdGoldilocks::new(12345);
    let b = SimdGoldilocks::new(67890);
    c.bench_function("goldilocks_add", |bench| {
        bench.iter(|| a.add(b))
    });
}

fn bench_mul(c: &mut Criterion) {
    let a = SimdGoldilocks::new(12345);
    let b = SimdGoldilocks::new(67890);
    c.bench_function("goldilocks_mul", |bench| {
        bench.iter(|| a.mul(b))
    });
}

fn bench_batch_mul(c: &mut Criterion) {
    let a: Vec<SimdGoldilocks> = (0..1024).map(|i| SimdGoldilocks::new(i)).collect();
    let b: Vec<SimdGoldilocks> = (0..1024).map(|i| SimdGoldilocks::new(i + 1)).collect();
    c.bench_function("goldilocks_batch_mul_1024", |bench| {
        bench.iter(|| SimdGoldilocks::batch_mul(&a, &b))
    });
}

criterion_group!(benches, bench_add, bench_mul, bench_batch_mul);
criterion_main!(benches);
