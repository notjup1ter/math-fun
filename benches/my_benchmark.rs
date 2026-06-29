use criterion::{black_box, criterion_group, criterion_main, Criterion};
use math_fun::multiples;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("sum_multiples_100000", |b| b.iter(|| multiples::sum_multiples_3or5(black_box(100000))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
