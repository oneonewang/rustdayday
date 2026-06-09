# 06 · 模块、Crates 与代码组织

> **本章目标**：会拆文件、声明 `mod`、用 `use` 引入路径、用 `pub` 控制可见性，让项目从"一个 main.rs 越写越长"变成"清晰的多文件结构"。

## 6.1 基本概念

| 概念 | 类比 | 说明 |
|------|------|------|
| **Crate** | 一个完整的 Rust 项目 | 编译的最小单位 |
| **Package** | 一个或多个 crate 的集合（`Cargo.toml`） | `cargo new my-app` |
| **Module** | crate 内的命名空间 | 用 `mod` 声明 |
| **Path** | 模块内定位项的方式 | `crate::foo::bar` |

> **Crate 两种形态**：
> - `binary crate`：可执行（`src/main.rs`）
> - `library crate`：给别人用（`src/lib.rs`）
>
> 一个 package 可以**同时**含这两种：`src/lib.rs` + `src/main.rs`（这时 main 用 `use my_crate::...`）。

## 6.2 定义模块：`mod`

```rust
// src/lib.rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() { println!("waitlist!"); }
    }

    mod serving {
        fn take_order() {}     // 默认私有
    }
}
```

> - `mod` 声明一个**模块**（命名空间）
> - 嵌套用 `mod ... { mod ... { } }` 或 `mod` + 单独文件（见 6.4）
> - **`pub` 控制可见性**——默认**全私有**

## 6.3 `pub` 可见性规则

| 写法 | 含义 |
|------|------|
| 默认（不写 `pub`） | 仅当前模块 + 子模块可见 |
| `pub` | 任何能访问本模块的地方都能用 |
| `pub(crate)` | crate 内可见（不导出） |
| `pub(super)` | 父模块可见 |
| `pub(in path)` | 指定路径内可见 |
| `pub struct Foo` | struct 公开，但**字段默认私有** |
| `pub enum Foo` | enum 公开，**变体也公开** |
| `pub fn foo` | 函数公开 |

```rust
pub struct User {
    pub name: String,        // 字段也必须 pub，外面才能读
    age: u32,                // 私有
}
```

## 6.4 拆文件：模块在文件系统上的对应

| 代码声明 | 文件位置 |
|----------|----------|
| `mod garden;` 在 `src/lib.rs` | `src/garden.rs` 或 `src/garden/mod.rs` |
| `mod vegetables;` 在 `src/garden.rs` | `src/garden/vegetables.rs` |
| `mod foo;` 在 `src/main.rs` | `src/foo.rs` 或 `src/foo/mod.rs` |

> 现代 Rust 推荐 **`foo.rs`**（新风格），不用 `foo/mod.rs`（旧风格也能用）。

### 一个多文件项目

```
src/
├── main.rs
├── lib.rs                 # 我们的库入口
├── config.rs              # 对应 mod config;
├── models.rs              # 对应 mod models;
└── models/
    ├── user.rs            # 对应 mod user; (在 models.rs 里)
    └── product.rs         # 对应 mod product;
```

## 6.5 `use`：把路径引到本地

```rust
// src/lib.rs
mod front_of_house;

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();   // 用 hosting 而不是完整路径
}
```

> - `use` 只是创建**别名**，不会触发加载
> - 函数：通常 `use` 到**父模块**（`use foo::bar`，调用 `bar::baz()`），这样一眼能看出"不是本地函数"
> - Struct / Enum / Trait：通常 `use` 到**名字**（`use foo::Bar`）

### 同一名字导入多个

```rust
use std::fmt;
use std::io;

fn f() -> fmt::Result { ... }       // fmt::Result
fn g() -> io::Result<()> { ... }    // io::Result
```

> 用 `as` 重命名：

```rust
use std::fmt::Result;
use std::io::Result as IoResult;
```

## 6.6 路径：绝对 vs 相对

| 写法 | 起点 |
|------|------|
| `crate::foo::bar` | **当前 crate 根**（推荐） |
| `super::foo` | 父模块 |
| `self::foo` | 当前模块 |
| `foo::bar` | 当前模块（隐式相对） |

```rust
mod outer {
    pub mod inner {
        pub fn hello() { println!("hi"); }
    }

    pub fn call() {
        // 都可以：
        crate::outer::inner::hello();   // 绝对
        super::outer::inner::hello();   // 相对（这里 super 是 crate 根）
        self::inner::hello();           // 当前模块
    }
}
```

> 💡 **推荐用 `crate::`** 写绝对路径，避免重构时路径失效。

## 6.7 重导出：`pub use`

当你想"重新组织"对外 API：

```rust
mod internal;

pub use internal::deeply::nested::Type;   // 让外部用 crate::Type 而不是 crate::internal::deeply::nested::Type
```

> 这是 crate 维护者常用的"组织内部自由、对外稳定"。

## 6.8 一个"图书馆"完整例子

**文件结构**：

```
src/
├── main.rs
├── lib.rs
├── book.rs
└── library/
    ├── mod.rs
    ├── catalog.rs
    └── member.rs
```

**`src/lib.rs`**：

```rust
pub mod book;
pub mod library;

pub use book::Book;             // 重新导出
```

**`src/book.rs`**：

```rust
#[derive(Debug, Clone)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub year: u16,
}

impl Book {
    pub fn new(title: &str, author: &str, year: u16) -> Self {
        Self {
            title: title.to_string(),
            author: author.to_string(),
            year,
        }
    }
}
```

**`src/library/mod.rs`**：

```rust
pub mod catalog;
pub mod member;

pub struct Library {
    pub name: String,
}
```

**`src/library/catalog.rs`**：

```rust
use crate::book::Book;

pub struct Catalog {
    books: Vec<Book>,
}

impl Catalog {
    pub fn new() -> Self { Self { books: vec![] } }

    pub fn add(&mut self, book: Book) {
        self.books.push(book);
    }

    pub fn find_by_author(&self, author: &str) -> Vec<&Book> {
        self.books.iter().filter(|b| b.author == author).collect()
    }
}
```

**`src/library/member.rs`**：

```rust
pub struct Member {
    pub name: String,
    pub id: u32,
}

impl Member {
    pub fn new(name: &str, id: u32) -> Self {
        Self { name: name.to_string(), id }
    }
}
```

**`src/main.rs`**：

```rust
use rustdayday::book::Book;
use rustdayday::library::{catalog::Catalog, Library};

fn main() {
    let mut lib = Library { name: "市图".to_string() };
    let mut catalog = Catalog::new();

    catalog.add(Book::new("Rust 程序设计", "Steve", 2019));
    catalog.add(Book::new("Rust 实战", "Carol", 2022));

    for b in catalog.find_by_author("Steve") {
        println!("找到: {} ({})", b.title, b.year);
    }
    let _ = lib;
}
```

> 把项目改成 `cargo new --lib rustdayday_lib && cargo new --bin rustdayday_bin` 那种结构也行，但本教程保持单 crate。

## 6.9 在一个 binary crate 内拆文件

`src/main.rs`：

```rust
mod app;
mod config;
mod error;

use app::run;
use error::AppError;

fn main() -> Result<(), AppError> {
    run()
}
```

`src/app.rs` / `src/config.rs` / `src/error.rs` 都和 `main.rs` 平级。

> ⚠️ 这些模块**不是** public 给别的 crate（因为这是 binary crate），所以"私有"就是"只在 main 用"。

## 6.10 模块与可见性速查

```rust
// pub 控制
pub fn a() {}                     // 公开
fn b() {}                         // 私有（默认）

pub(crate) fn c() {}              // crate 内可见
pub(super) fn d() {}              // 父模块可见
pub(in crate::foo) fn e() {}      // 指定路径可见

// struct 字段
pub struct S {
    pub x: i32,                    // 公开字段
    y: i32,                        // 私有字段
}

// enum 变体自动跟 enum 一起
pub enum E {
    A,                             // 公开
    B,                             // 公开
}
```

## 6.11 对比其他语言

| 概念 | Rust | C# | Java | Python | Go |
|------|------|-----|------|--------|-----|
| 命名空间 | `mod` + `pub` | `namespace` | `package` + `class` | `module` / `package` | `package` |
| 文件对应 | 自由（一个 mod 一个或多个文件） | 一文件一类 | 一文件一公开类 | 自由 | 一目录一包 |
| 默认可见性 | 私有 | private | package-private | public | 大写 = 公开 |
| 重导出 | `pub use` | `using` 别名 | re-export | re-export | 别名 |

---

## 🏋️ 本章小练习

**练习 6.1**：把练习 2.1 的"几何"代码（`Rectangle` / `Circle`）拆成两个文件：`geometry/shape.rs` 和 `geometry/mod.rs`，用 `pub use` 在 crate 根直接暴露 `Rectangle` / `Circle`。

**练习 6.2**：写一个 `pub fn` 用 `pub(super)` 限制可见性，验证它在外面不可见。

**练习 6.3**：写一个内部模块 `internal_helpers` 用 `pub(crate)` 暴露辅助函数，证明 main 之外（比如单元测试）能访问。

**练习 6.4**（真实场景）：把练习 5.10 的 `load_config` 项目拆成：
```
src/
├── main.rs
├── lib.rs
├── config.rs
└── error.rs
```
外部 `main.rs` 只写 `run()` 入口和错误处理。

---

下一章：[07 · 集合：Vec / String / HashMap →](./07-collections.md)
