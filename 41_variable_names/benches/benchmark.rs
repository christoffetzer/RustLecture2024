
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use checked_collatz::checked_collatz;



fn criterion_benchmark(c: &mut Criterion) {
    const N : u64 =29303209;
    c.bench_function(&format!("collatz {N}"), |b| b.iter(|| checked_collatz(black_box(N))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);