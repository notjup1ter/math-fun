use criterion::{black_box, criterion_group, criterion_main, Criterion};
use math_fun::multiples;

fn mutliples_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sum of Multiples (Limit 100,000)");

    group.bench_function(
        "sum of multiples up to 100000, iteratively", 
        |b| 
        b.iter(|| multiples::sum_multiples_3or5_iter(black_box(100000)))
    );
    group.bench_function(
        "sum of multiples up to 100000, guass' estimate", 
        |b| 
        b.iter(|| multiples::sum_multiples_3or5_guass(black_box(100000)))
    );

    group.finish();
}

criterion_group!(benches, mutliples_benchmark);
criterion_main!(benches);
