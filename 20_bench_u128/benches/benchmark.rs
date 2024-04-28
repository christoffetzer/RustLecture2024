
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use collatz_u128::collatz_u128;



fn criterion_benchmark(c: &mut Criterion) {
    const N : u128 =29303209;
    c.bench_function(&format!("collatz_u128 {N}"), |b| b.iter(|| collatz_u128(black_box(N))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);