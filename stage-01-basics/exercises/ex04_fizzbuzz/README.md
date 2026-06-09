# Exercise 04 · FizzBuzz

> 难度：⭐  
> 涉及：第 4 章（控制流 / `match` 入门）

## 任务

打印 1 到 100（**含** 100）：
- 3 的倍数打印 `Fizz`
- 5 的倍数打印 `Buzz`
- **15** 的倍数打印 `FizzBuzz`（**注意顺序**）
- 其他打印数字本身

## 验收

```bash
cargo run
```

输出前 20 行（**注意 15 那一行是 FizzBuzz 不是 Fizz**）：

```
1
2
Fizz
4
Buzz
Fizz
7
8
Fizz
Buzz
11
Fizz
13
14
FizzBuzz
16
17
Fizz
19
Buzz
```

## 提示

```rust
for n in 1..=100 {
    if n % 15 == 0 {
        println!("FizzBuzz");
    } else if n % 3 == 0 {
        println!("Fizz");
    } else if n % 5 == 0 {
        println!("Buzz");
    } else {
        println!("{n}");
    }
}
```

## 进阶（可选）

用 `match` 重写一遍（感受 Rust `match` 的表达力）：

```rust
match (n % 3, n % 5) {
    (0, 0) => println!("FizzBuzz"),
    (0, _) => println!("Fizz"),
    (_, 0) => println!("Buzz"),
    _      => println!("{n}"),
}
```

完成后进 [ex05_ownership_quiz](../ex05_ownership_quiz)
