# Exercise 01 · struct 基础

> 难度：⭐  涉及：第 1 章

## 任务

### 1) 定义一个 `Book` struct

```rust
#[derive(Debug, Clone, PartialEq)]
struct Book {
    title: String,
    author: String,
    pages: u32,
    price: f64,
}
```

### 2) 实现关联函数

```rust
impl Book {
    fn new(title: &str, author: &str, pages: u32, price: f64) -> Self {
        // 用字段初始化简写
    }

    fn free_sample(title: &str, author: &str) -> Self {
        // pages = 30, price = 0.0
    }
}
```

### 3) 在 main 中

- 创建 `b1 = Book::new("Rust 编程", "Alice", 400, 59.9)`
- 创建 `b2 = Book::free_sample("Rust 编程", "Alice")`
- 用 `{:?}` 和 `{:#?}` 两种方式打印两个 Book
- 验证 `b1 == b2` 为 `false`（靠 `PartialEq`），`b1.clone() == b1` 为 `true`

## 验收

```bash
cargo run
```

输出形如：

```
b1 = Book { title: "Rust 编程", author: "Alice", pages: 400, price: 59.9 }
b1 (pretty):
Book {
    title: "Rust 编程",
    author: "Alice",
    pages: 400,
    price: 59.9,
}
b2 = Book { title: "Rust 编程", author: "Alice", pages: 30, price: 0.0 }
b1 == b2 ? false
b1.clone() == b1 ? true
```

## 思考

- 字段顺序变了 `Book::new("...", "...", 400, 59.9)` 容易错位——用**命名参数风格**可以避免吗？
- 提示：命名参数风格靠结构体更新语法 `Book { title, author, .. }`。

## 进阶

写一个 `pub struct Config { pub host: String, pub port: u16 }`，
实现 `Config::from_env() -> Self`（从 `std::env::var("HOST")` / `PORT` 读，没设则用 `localhost` / `8080`）。

完成 7 个练习后 → [project-02-cli-todo](../project-02-cli-todo) 见！
