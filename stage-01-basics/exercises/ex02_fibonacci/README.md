# Exercise 02 · 斐波那契

> 难度：⭐  
> 涉及：第 3、4、5 章（变量 / 循环 / 函数）

## 任务

实现一个**返回第 n 个斐波那契数**的函数 `fib(n: u32) -> u64`：

```
fib(0)  == 0
fib(1)  == 1
fib(2)  == 1
fib(3)  == 2
fib(4)  == 3
fib(5)  == 5
fib(10) == 55
fib(20) == 6765
```

## 要求

1. **必须**用循环实现（**不要**递归——后面会专门写递归版本来感受栈溢出）
2. 写一个 `main`，打印 `fib(0)` 到 `fib(20)` 的结果

## 验收

```bash
cargo run
# 输出形如：
# fib(0) = 0
# fib(1) = 1
# ...
# fib(20) = 6765
```

## 提示

```rust
fn fib(n: u32) -> u64 {
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    for _ in 0..n {
        let c = a + b;
        a = b;
        b = c;
    }
    a   // 循环 n 次后 a 是 fib(n)
}
```

## 进阶（可选）

- 不用循环，用**递归**实现一遍 `fib_rec`，然后试 `fib_rec(50)`——会看到栈溢出
- 加一行 `if n > 30 { panic!("n too large for recursion") }` 防爆栈

完成后进 [ex03_temperature](../ex03_temperature)
