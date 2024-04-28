
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use termination_check::collatz_tc;



fn criterion_benchmark(c: &mut Criterion) {
    const N : u128 =29303209;
    c.bench_function(&format!("collatz_tc {N}"), |b| b.iter(|| collatz_tc(black_box(N))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);