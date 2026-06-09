# Exercise 06 · 模块化

> 难度：⭐⭐  涉及：第 6 章

## 任务

把一个单文件的"小项目"拆成多文件模块结构。

### 起步代码（先全部放 `main.rs` 跑通）

```rust
// ---- 形状 ----
#[derive(Debug)]
pub struct Circle { pub radius: f64 }
impl Circle {
    pub fn new(r: f64) -> Self { Self { radius: r } }
    pub fn area(&self) -> f64 { std::f64::consts::PI * self.radius * self.radius }
    pub fn perimeter(&self) -> f64 { 2.0 * std::f64::consts::PI * self.radius }
}

#[derive(Debug)]
pub struct Rectangle { pub width: f64, pub height: f64 }
impl Rectangle {
    pub fn new(w: f64, h: f64) -> Self { Self { width: w, height: h } }
    pub fn area(&self) -> f64 { self.width * self.height }
    pub fn perimeter(&self) -> f64 { 2.0 * (self.width + self.height) }
}

// ---- 工具函数 ----
pub fn describe(s: &str) -> String {
    format!("这是一个 {s} 模块")
}

pub fn version() -> &'static str { "0.1.0" }
```

### 拆分成

```
src/
├── main.rs
├── lib.rs
└── geometry/
    ├── mod.rs
    ├── shape.rs           # Circle + Rectangle
    └── utils.rs           # describe + version
```

要求：
- `lib.rs` 暴露 `pub mod geometry;` 和 `pub use geometry::{Circle, Rectangle};`
- `geometry/mod.rs` 暴露 `pub mod shape; pub mod utils;`
- `main.rs` 用 `use mycrate::{Circle, Rectangle, version};`

## 验收

- `cargo run` 跑通
- `cargo doc --open` 能看到所有 public 项都有文档
- 故意把 `Circle` 改成私有，验证 main 编译错

## 提示

- 在 binary crate 里拆文件也是合法的：写 `mod geometry;` 在 `main.rs` 里
- 但这样所有东西都是**私有**（只在 main 内部用），不能 `pub use` 给其他 crate
- 想要更接近"库"的方式：`cargo new --lib` 单独建一个 `lib` crate，main 引用它

## 进阶

把整个项目改成 **library + binary** 模式：

```
src/
├── lib.rs                  # 库入口
├── main.rs                 # binary 入口，引用库
└── ...（按上面组织）
```

`Cargo.toml` 不需要改——`cargo new` 时同时有 `lib.rs` 和 `main.rs` 就是这个模式。

完成 → [ex07_collections](../ex07_collections)
