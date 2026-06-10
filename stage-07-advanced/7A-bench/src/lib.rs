//! 一些"待测"的函数实现
//!
//! 这些是被基准测试的目标——故意写得直白，方便对比不同实现。

use rayon::prelude::*;

/// 求和：单线程迭代 vs 并行迭代
pub fn sum_serial(v: &[u64]) -> u64 {
    v.iter().sum()
}

pub fn sum_parallel(v: &[u64]) -> u64 {
    v.par_iter().sum()
}

/// 排序：std::sort vs 并行排序（rayon）
pub fn sort_serial(v: &mut [u64]) {
    v.sort();
}

pub fn sort_parallel(v: &mut [u64]) {
    use rayon::prelude::*;
    v.par_sort();
}

/// 找出 1..=n 里的所有质数：朴素 vs 埃氏筛
pub fn primes_serial(n: u32) -> Vec<u32> {
    (2..=n).filter(|&x| is_prime_naive(x)).collect()
}

pub fn primes_parallel(n: u32) -> Vec<u32> {
    (2..=n).into_par_iter().filter(|&x| is_prime_naive(x)).collect()
}

pub fn primes_sieve(n: u32) -> Vec<u32> {
    if n < 2 { return vec![]; }
    let mut sieve = vec![true; (n + 1) as usize];
    sieve[0] = false;
    sieve[1] = false;
    for i in 2..=((n as f64).sqrt() as u32) {
        if sieve[i as usize] {
            for j in (i*i..=n).step_by(i as usize) {
                sieve[j as usize] = false;
            }
        }
    }
    (2..=n).filter(|&i| sieve[i as usize]).collect()
}

fn is_prime_naive(n: u32) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 { return false; }
        i += 2;
    }
    true
}

/// 字符串处理：直接 vs 切片
pub fn string_upper(s: &str) -> String {
    s.to_uppercase()
}

pub fn string_upper_ascii(s: &str) -> String {
    s.chars().map(|c| c.to_ascii_uppercase()).collect()
}

/// 词频统计：HashMap vs 收集到 Vec
pub fn word_count_hashmap(text: &str) -> std::collections::HashMap<String, usize> {
    let mut map = std::collections::HashMap::new();
    for word in text.split_whitespace() {
        *map.entry(word.to_lowercase()).or_insert(0) += 1;
    }
    map
}

pub fn word_count_fold(text: &str) -> Vec<(String, usize)> {
    let mut pairs: Vec<(String, usize)> = vec![];
    for word in text.split_whitespace() {
        let w = word.to_lowercase();
        if let Some(p) = pairs.iter_mut().find(|(k, _)| k == &w) {
            p.1 += 1;
        } else {
            pairs.push((w, 1));
        }
    }
    pairs
}

/// 矩阵乘法：naive vs 转置优化
pub fn matmul_naive(a: &[Vec<f64>], b: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let n = a.len();
    let m = b[0].len();
    let p = b.len();
    let mut c = vec![vec![0.0; m]; n];
    for i in 0..n {
        for j in 0..m {
            for k in 0..p {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    c
}

pub fn matmul_transposed(a: &[Vec<f64>], b: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let n = a.len();
    let m = b[0].len();
    // 转置 b
    let p = b.len();
    let bt: Vec<Vec<f64>> = (0..m).map(|j| (0..p).map(|i| b[i][j]).collect()).collect();
    let mut c = vec![vec![0.0; m]; n];
    for i in 0..n {
        for j in 0..m {
            c[i][j] = a[i].iter().zip(bt[j].iter()).map(|(x, y)| x * y).sum();
        }
    }
    c
}

/// 斐波那契：递归 vs 迭代
pub fn fib_recursive(n: u32) -> u64 {
    if n < 2 { return n as u64; }
    fib_recursive(n - 1) + fib_recursive(n - 2)
}

pub fn fib_iterative(n: u32) -> u64 {
    if n == 0 { return 0; }
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 1..n {
        let c = a + b;
        a = b;
        b = c;
    }
    b
}
