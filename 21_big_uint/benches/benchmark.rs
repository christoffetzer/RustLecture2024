use big_uint::collatz_big_uint;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    const N: u128 = 29303209;
    c.bench_function(&format!("collatz_big_uint {N}"), |b| {
        b.iter(|| collatz_big_uint(black_box(N)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
