# Stage 2：结构化代码与错误处理

> 目标：学会用 `struct` / `enum` 描述数据，用 `match` 消灭 `if/else`，用 `Result` 消灭异常，让代码分模块组织。

## 📍 你将学到

- `struct`：自定义数据结构
- 方法与关联函数（Rust 没有 `class`，但有更好的"特质多态"）
- **`enum` 与模式匹配**——Rust 最具表达力的特性
- **`Option<T>` / `Result<T, E>`**——消灭 `null` 和 `try/catch`
- 错误处理：`?` 运算符、`From` trait、`main` 返回 `Result`
- 模块系统：`mod` / `use` / `pub` / 文件拆分
- 标准库常用集合：`Vec` / `HashMap` / `String` 深入

## 🗺️ 章节导航

| # | 文档 | 主题 | 预计时间 |
|---|------|------|----------|
| 1 | [docs/01-structs.md](./docs/01-structs.md) | struct 三种形态、字段简写 | 45 min |
| 2 | [docs/02-methods.md](./docs/02-methods.md) | `impl` 块、`self` 三态、关联函数 | 45 min |
| 3 | **[docs/03-enums-and-pattern-matching.md](./docs/03-enums-and-pattern-matching.md)** ⭐ | **enum 携带数据、`match` 穷尽性、`if let`** | **90 min** |
| 4 | **[docs/04-option-result.md](./docs/04-option-result.md)** ⭐ | **`Option` 替 `null`，`Result` 替异常** | **60 min** |
| 5 | [docs/05-error-handling.md](./docs/05-error-handling.md) | `?` 运算符、`From`、自定义错误 | 60 min |
| 6 | [docs/06-modules-crates.md](./docs/06-modules-crates.md) | `mod` / `use` / `pub`、文件拆分 | 45 min |
| 7 | [docs/07-collections.md](./docs/07-collections.md) | `Vec` / `String` / `HashMap` | 60 min |
| 8 | [docs/08-stage-review.md](./docs/08-stage-review.md) | 阶段知识地图、自测 | 30 min |

## 🛠️ 练习

| 练习 | 主题 | 难度 |
|------|------|------|
| [ex01_struct_basics](./exercises/ex01_struct_basics) | 定义 struct、方法、字段访问 | ⭐ |
| [ex02_methods](./exercises/ex02_methods) | `&self` / `&mut self` / 关联函数 | ⭐⭐ |
| [ex03_enum_matching](./exercises/ex03_enum_matching) | enum 携带数据、match | ⭐⭐ |
| [ex04_option_result](./exercises/ex04_option_result) | `Option` / `Result` 组合子 | ⭐⭐ |
| [ex05_error_handling](./exercises/ex05_error_handling) | `?` 运算符、自定义错误 | ⭐⭐ |
| [ex06_modules](./exercises/ex06_modules) | 拆文件、写 `mod` | ⭐⭐ |
| [ex07_collections](./exercises/ex07_collections) | `Vec` / `HashMap` 综合 | ⭐⭐ |

## 🎯 综合项目

[`project-02-cli-todo/`](./project-02-cli-todo) — 命令行 Todo 工具。

- `add` / `list` / `done` / `remove` 四个子命令
- JSON 文件持久化（用 `serde` + `serde_json`）
- 完整错误处理（`Result` + `?`）
- 跨多个文件组织代码

## 📖 推荐节奏

1. 按顺序读 8 篇文档（每章 30/45/60/90 min）
2. 边读边在本地敲示例代码
3. 完成 [`exercises/`](./exercises) 7 个练习
4. 做 [`project-02-cli-todo/`](./project-02-cli-todo)（预计 1.5–2.5 天）
5. 读 [docs/08-stage-review.md](./docs/08-stage-review.md) 自测
6. 提交 git，进入 Stage 3

## 🔗 对照官方资料

本教程章节对应 [The Rust Book](https://doc.rust-lang.org/book/) 第 5–8 章 + 第 9 章（错误处理）。

## ⚠️ 心态提示

> 第一次看到 `enum` 可以"携带数据"会觉得反直觉——但这是 Rust 把"和类型 / 代数数据类型"直接做到了语言层面的设计。**理解它之后，你会用 `enum` 表达几乎所有"分类 + 每类带不同信息"的问题**。
>
> `Result` 取代 `try/catch` 是 Rust 错误处理的核心思路：**错误是值，不是控制流**。一开始会觉得啰嗦，写多了会发现"显式胜过隐式"。

---

回到 [项目总览](../README.md)
