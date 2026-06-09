# Stage 1：Rust 基础语法与工具链

> 目标：跑通 `cargo`，写出能编译的小程序，**吃透所有权 / 借用**。这一章是后面所有内容的基石，值得花双倍时间。

## 📍 你将学到

- Rust 工具链：`rustup` / `cargo` / `rustc`
- 基础语法：变量、类型、函数、控制流
- **核心概念**：所有权（ownership）、借用（borrowing）、切片（slice）
- 用 `cargo` 组织、构建、运行、测试一个小型项目

## 🗺️ 章节导航

| # | 文档 | 主题 | 预计时间 |
|---|------|------|----------|
| 1 | [docs/01-toolchain.md](./docs/01-toolchain.md) | 安装 rustup、cargo、rustc | 15 min |
| 2 | [docs/02-hello-world.md](./docs/02-hello-world.md) | `cargo new`、Hello World、编译运行 | 20 min |
| 3 | [docs/03-variables-and-types.md](./docs/03-variables-and-types.md) | `let` / `mut` / `shadowing`、标量与复合类型 | 45 min |
| 4 | [docs/04-control-flow.md](./docs/04-control-flow.md) | `if` / `loop` / `while` / `for` / `Range` | 30 min |
| 5 | [docs/05-functions.md](./docs/05-functions.md) | 函数定义、参数、返回值、表达式 | 30 min |
| 6 | **[docs/06-ownership.md](./docs/06-ownership.md)** ⭐ | **所有权三规则、Move、Copy、Clone** | **90 min** |
| 7 | **[docs/07-borrowing.md](./docs/07-borrowing.md)** ⭐ | **`&` / `&mut`、借用检查器** | **90 min** |
| 8 | [docs/08-slices.md](./docs/08-slices.md) | `&[T]` 与 `&str` | 30 min |
| 9 | [docs/09-stage-review.md](./docs/09-stage-review.md) | 阶段知识地图、错题回顾 | 30 min |

## 🛠️ 练习

每章文档末尾有"小练习"（边读边敲）。综合练习位于 [`exercises/`](./exercises)：

| 练习 | 主题 | 难度 |
|------|------|------|
| [01_hello_cargo](./exercises/01_hello_cargo) | 第一个 Cargo 项目 | ⭐ |
| [02_fibonacci](./exercises/02_fibonacci) | 函数 + 循环 | ⭐ |
| [03_temperature](./exercises/03_temperature) | 浮点 + 表达式返回值 | ⭐ |
| [04_fizzbuzz](./exercises/04_fizzbuzz) | 模式匹配的入门版 | ⭐ |
| [05_ownership_quiz](./exercises/05_ownership_quiz) | 修编译错，吃透所有权 | ⭐⭐ |
| [06_borrow_checker](./exercises/06_borrow_checker) | 修借用错误 | ⭐⭐ |
| [07_slices](./exercises/07_slices) | 用 slice 解决问题 | ⭐⭐ |

## 🎯 综合项目

[`project-01-guess-game/`](./project-01-guess-game) — 经典猜数字游戏。覆盖：标准输入、`loop`、`match`、外部 crate、错误处理。

## 📖 推荐节奏

1. 按顺序读完 9 篇文档（每篇按 30/60/90 min 分配时间）
2. 边读边在本地敲一遍示例代码
3. 完成 [`exercises/`](./exercises) 里 7 个练习
4. 做 [`project-01-guess-game/`](./project-01-guess-game)，提交 git
5. 读 [docs/09-stage-review.md](./docs/09-stage-review.md) 自测
6. **所有习题 / 项目必须能 `cargo build` 通过再进入 Stage 2**

## 🔗 对照官方资料

本教程的章节顺序与 [The Rust Book](https://doc.rust-lang.org/book/) 第 1–4 章对应，命名也尽量保持一致，方便你交叉阅读。

## ⚠️ 心态提示

> 如果你来自 GC 语言（Java/Go/Python/JS），所有权 / 借用 / 生命周期会"反直觉"。**别怕，编译器的错误信息非常友好**——它会精确告诉你哪一行、违反了哪条规则。前几次看到一堆红字是正常的，这不是你的问题，是 Rust 在保护你。
