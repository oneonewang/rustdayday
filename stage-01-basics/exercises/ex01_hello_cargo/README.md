# Exercise 01 · Hello Cargo

> 难度：⭐  
> 涉及：第 1、2 章

## 任务

完成一个能打印以下内容的 Rust 程序：

```
🦀 Rust Stage 1
================
I'm <你的名字> and I'm learning Rust!
Today I will write my first cargo project.
```

## 要求

1. 使用 `cargo run` 跑通
2. 程序至少包含 3 行 `println!` 输出
3. **不要**修改 `Cargo.toml`

## 验收

```bash
cargo run
# 输出至少包含三行内容，且不是默认的 "Hello, world!"
```

## 提示

- 字符串内嵌变量：`println!("Hello, {name}");`
- 字符串内嵌表达式：`println!("2 + 3 = {}", 2 + 3);`

完成后进 [ex02_fibonacci](../ex02_fibonacci)
