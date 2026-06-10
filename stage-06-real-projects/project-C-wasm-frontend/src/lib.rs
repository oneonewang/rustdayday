//! Project C · Rust → WebAssembly 前端
//!
//! 用 wasm-bindgen 暴露纯计算函数到 JS。
//! 浏览器侧：JS 加载 .wasm + 调这些函数（用 `--target web` 自动生成 glue）。

use wasm_bindgen::prelude::*;

/// 阶乘
#[wasm_bindgen]
pub fn factorial(n: u32) -> u64 {
    (1..=n).map(|x| x as u64).product()
}

/// 斐波那契
#[wasm_bindgen]
pub fn fibonacci(n: u32) -> u64 {
    if n == 0 { return 0; }
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 1..n {
        let c = a + b;
        a = b;
        b = c;
    }
    b
}

/// 是不是质数
#[wasm_bindgen]
pub fn is_prime(n: u32) -> bool {
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

/// 反转字符串
#[wasm_bindgen]
pub fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

/// 求和数组（JS 调）
#[wasm_bindgen]
pub fn sum_array(values: &[f64]) -> f64 {
    values.iter().sum()
}

/// 版本号
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
