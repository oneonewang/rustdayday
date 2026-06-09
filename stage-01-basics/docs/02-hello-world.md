# 02 · Hello World 与第一个 Cargo 项目

> **本章目标**：建起第一个 Rust 项目，能 `cargo run` 出 "Hello, world!"，理解 `Cargo.toml` 和源文件的关系。

## 2.1 建项目

```bash
cargo new hello-rust
cd hello-rust
```

`cargo new` 默认会建一个 **二进制**（可执行）项目，目录结构：

```
hello-rust/
├── .git/             # 自动 git init
├── .gitignore
├── Cargo.toml
└── src/
    └── main.rs
```

如果当前目录已有同名文件夹，`cargo new` 会**报错**让你换个名字。

> 第二个常用参数：`cargo new --lib my-lib` 建**库**项目（产出 `src/lib.rs`，没有 `main`，不能直接 `run`）。

## 2.2 看一眼生成的文件

### `Cargo.toml`（项目清单）

```toml
[package]
name = "hello-rust"
version = "0.1.0"
edition = "2021"   # ⭐ Rust 版本（"edition"），影响语法和 stdlib 行为

[dependencies]
# 这里写依赖的 crate 名和版本
```

| 字段 | 含义 |
|------|------|
| `name` | 包名，会作为 crate 名（导入时 `use hello_rust::...`） |
| `version` | 遵循 [semver](https://semver.org/) |
| `edition` | Rust 的"语法大版本"：`2015` / `2018` / `2021` / **`2024`**。新项目用最新（cargo new 默认 2021，2024 已经在主流编译器支持） |
| `dependencies` | 项目依赖的第三方 crate 列表 |

> 💡 **对比**：类似 `package.json`（Node）/ `pyproject.toml`（Python）/ `go.mod`（Go）。

### `src/main.rs`（入口）

```rust
fn main() {
    println!("Hello, world!");
}
```

- `fn main()` 是程序的入口，和 C / Go 一样。每个**二进制 crate** 必须有且只能有一个 `main` 函数。
- `println!` 是**宏**（macro），不是函数。`!` 结尾是 Rust 宏的标志。宏是元编程，编译期展开；函数是运行时调用。
- 语句以分号 `;` 结尾（注意：Rust 中表达式也可以没有 `;`，见第 5 章）。

## 2.3 编译并运行

```bash
cargo run
```

输出：

```
   Compiling hello-rust v0.1.0 (/path/to/hello-rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.5s
     Running `target/debug/hello-rust`
Hello, world!
```

> 第一次会下载依赖、稍慢（即使没依赖也会编译 std）。之后增量编译非常快（亚秒级）。

## 2.4 Cargo 常用子命令（再列一遍）

| 命令 | 等价的两个步骤 |
|------|----------------|
| `cargo run` | `cargo build` + 运行产物 |
| `cargo check` | 只做类型检查，不生成可执行文件（CI 首选，比 `build` 快） |
| `cargo build --release` | 用 release 模式（开优化）编译，产物在 `target/release/` |
| `cargo clean` | 删 `target/` 目录（出问题时的万能办法） |
| `cargo fmt` | 自动格式化（要先 `rustup component add rustfmt`） |
| `cargo clippy` | Lint，建议装（`rustup component add clippy`） |

## 2.5 项目内文件改动会怎样？

打开 `src/main.rs` 把消息改成中文，**再 `cargo run`**：

```rust
fn main() {
    println!("你好，Rust！");
}
```

Cargo 会**自动检测**源码变化并重编译。这就是增量编译的力量。

## 2.6 看一眼 target/ 目录

```bash
ls target/debug/
```

会有 `hello-rust`（可执行）、`build/`、`deps/`、`incremental/` 等。`target/` 目录**不要提交**到 git（`.gitignore` 已经帮你忽略了）。

## 2.7 一个常见的迷惑

如果你看到类似这样的报错：

```
error: linker `cc` not found
```

意思是系统没装 C 编译器。Linux 上 `sudo apt install build-essential`（Debian/Ubuntu）或 `sudo dnf install gcc`（Fedora）。macOS 装 Xcode Command Line Tools：`xcode-select --install`。Windows 装 MSVC 构建工具（`rustup-init` 时会引导）。

---

## 🏋️ 本章小练习

**练习 2.1**：建一个 `hello-rust` 项目，让它输出：
```
🦀 Hello, Rustaceans!
我的第一个 Rust 程序
```

**练习 2.2**：故意把 `main` 改名成别的（比如 `Main`），`cargo run` 看报什么错。然后改回来。**目的：熟悉 Rust 编译器的错误风格**——它通常会建议你可能想写什么。

**练习 2.3**：把 `println!` 写错成 `println`（没有 `!`），编译看错误。然后改回来。

> 提示：`println!` 后面跟 `!` 因为它是宏。`println`（没有 `!`）会被解释成"把 `println` 当成一个标识符"——但 std 没有这个名字，会报 cannot find value `println` in this scope。

---

下一章：[03 · 变量与类型 →](./03-variables-and-types.md)
