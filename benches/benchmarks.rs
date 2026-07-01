use criterion::{black_box, criterion_group, criterion_main, Criterion};
use math_fun::*;

fn mutliples_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sum of Multiples (Limit 100,000)");

    group.bench_function(
        "Iteratively", 
        |b| 
        b.iter(|| multiples::sum_multiples_3or5_iter(black_box(100000)))
    );
    group.bench_function(
        "Carl Guass' formula", 
        |b| 
        b.iter(|| multiples::sum_multiples_3or5_guass(black_box(100000)))
    );

    group.finish();
}

fn even_fibonacci_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sum of even Fibonacci vales < 4000000");

    group.bench_function("Double-iterative (Sequence creation and filtering)", 
        |b| 
        b.iter(|| even_fibonacci::even_fib_sum(4000000))
    );

    group.bench_function("Iterative method with reduced vector size",
        |b| 
        b.iter(|| even_fibonacci::even_fib_sum_v2(4000000))
    );

    group.finish();
}

criterion_group!(benches, mutliples_benchmark, even_fibonacci_benchmark);
criterion_main!(benches);
