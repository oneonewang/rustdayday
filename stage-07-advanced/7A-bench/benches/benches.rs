//! 性能基准：用 criterion 对比实现
//!
//! 运行：cargo bench
//! 报告：target/criterion/ 里有 HTML 报告

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use stage_07A_bench::*;

fn bench_sum(c: &mut Criterion) {
    let v: Vec<u64> = (0..1_000_000).collect();

    let mut group = c.benchmark_group("sum_1M");
    group.bench_function("serial", |b| b.iter(|| sum_serial(black_box(&v))));
    group.bench_function("parallel", |b| b.iter(|| sum_parallel(black_box(&v))));
    group.finish();
}

fn bench_sort(c: &mut Criterion) {
    let mut group = c.benchmark_group("sort_100k");
    group.bench_function("serial", |b| {
        b.iter(|| {
            let mut v: Vec<u64> = (0..100_000).rev().collect();
            sort_serial(black_box(&mut v));
        });
    });
    group.bench_function("parallel", |b| {
        b.iter(|| {
            let mut v: Vec<u64> = (0..100_000).rev().collect();
            sort_parallel(black_box(&mut v));
        });
    });
    group.finish();
}

fn bench_primes(c: &mut Criterion) {
    let mut group = c.benchmark_group("primes_100k");
    group.bench_function("serial", |b| b.iter(|| primes_serial(black_box(100_000))));
    group.bench_function("parallel", |b| b.iter(|| primes_parallel(black_box(100_000))));
    group.bench_function("sieve", |b| b.iter(|| primes_sieve(black_box(100_000))));
    group.finish();
}

fn bench_string(c: &mut Criterion) {
    let s: String = "Hello, World! ".repeat(100);

    let mut group = c.benchmark_group("string_upper_100x");
    group.bench_function("to_uppercase", |b| b.iter(|| string_upper(black_box(&s))));
    group.bench_function("ascii_only", |b| b.iter(|| string_upper_ascii(black_box(&s))));
    group.finish();
}

fn bench_word_count(c: &mut Criterion) {
    let text: String = "the quick brown fox jumps over the lazy dog the quick brown fox ".repeat(100);

    let mut group = c.benchmark_group("word_count");
    group.bench_function("hashmap", |b| b.iter(|| word_count_hashmap(black_box(&text))));
    group.bench_function("fold",   |b| b.iter(|| word_count_fold(black_box(&text))));
    group.finish();
}

fn bench_matmul(c: &mut Criterion) {
    let n = 50;
    let a: Vec<Vec<f64>> = (0..n).map(|i| (0..n).map(|j| ((i + j) as f64) * 0.1).collect()).collect();
    let b: Vec<Vec<f64>> = (0..n).map(|i| (0..n).map(|j| ((i * j + 1) as f64) * 0.1).collect()).collect();

    let mut group = c.benchmark_group("matmul_50x50");
    group.bench_function("naive", |bench| {
        bench.iter(|| matmul_naive(black_box(&a), black_box(&b)))
    });
    group.bench_function("transposed", |bench| {
        bench.iter(|| matmul_transposed(black_box(&a), black_box(&b)))
    });
    group.finish();
}

fn bench_fib(c: &mut Criterion) {
    let mut group = c.benchmark_group("fib_30");
    group.bench_function("recursive", |b| b.iter(|| fib_recursive(black_box(30))));
    group.bench_function("iterative", |b| b.iter(|| fib_iterative(black_box(30))));
    group.finish();
}

criterion_group!(
    benches,
    bench_sum,
    bench_sort,
    bench_primes,
    bench_string,
    bench_word_count,
    bench_matmul,
    bench_fib,
);
criterion_main!(benches);
