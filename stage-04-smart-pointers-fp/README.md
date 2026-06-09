# Stage 4：智能指针、闭包、迭代器

> 目标：掌握 Rust 突破"栈 / 静态借用"边界的工具箱，写出**函数式风格 + 零分配**的代码。

## 📍 你将学到

- **智能指针**：`Box<T>` / `Rc<T>` / `Arc<T>` / `RefCell<T>` / `Mutex<T>`
- **闭包**：`Fn` / `FnMut` / `FnOnce` 三种 trait，捕获机制
- **迭代器深入**：`Iterator` trait、惰性求值、组合子、自定义迭代器
- **零分配迭代**、**状态机式解析**
- **内部可变性**：编译器检查 → 运行时检查

## 🗺️ 章节导航

| # | 文档 | 主题 | 预计时间 |
|---|------|------|----------|
| 1 | [docs/01-box-rc.md](./docs/01-box-rc.md) | `Box<T>` 堆分配与递归类型 / `Rc<T>` 单线程共享 | 60 min |
| 2 | [docs/02-refcell.md](./docs/02-refcell.md) | `RefCell<T>` 内部可变性 | 45 min |
| 3 | [docs/03-closures.md](./docs/03-closures.md) | 闭包、捕获、`Fn` / `FnMut` / `FnOnce` | 60 min |
| 4 | [docs/04-iterators.md](./docs/04-iterators.md) | 迭代器、组合子、自定义迭代器 | 75 min |
| 5 | [docs/05-smart-pointers-advanced.md](./docs/05-smart-pointers-advanced.md) | `Arc` / `Mutex` / `Weak` / `Cow` | 60 min |
| 6 | [docs/06-stage-review.md](./docs/06-stage-review.md) | 知识地图、自测 10 题 | 30 min |

## 🛠️ 练习

| 练习 | 主题 | 难度 |
|------|------|------|
| [ex01_box](./exercises/ex01_box) | 递归类型、DST、Box 用于 trait object | ⭐ |
| [ex02_rc](./exercises/ex02_rc) | 单线程共享所有权、引用计数 | ⭐⭐ |
| [ex03_refcell](./exercises/ex03_refcell) | 内部可变性、RefCell、Cell | ⭐⭐ |
| [ex04_closures](./exercises/ex04_closures) | 闭包语法、捕获、trait 三态 | ⭐⭐ |
| [ex05_iterators](./exercises/ex05_iterators) | 迭代器组合子、惰性求值 | ⭐⭐ |
| [ex06_custom_iterator](./exercises/ex06_custom_iterator) | 实现自定义 Iterator | ⭐⭐⭐ |
| [ex07_smart_pointers_advanced](./exercises/ex07_smart_pointers_advanced) | Arc / Mutex / Cow | ⭐⭐⭐ |

## 🎯 综合项目

[`project-04-text-parser/`](./project-04-text-parser) — **文本解析器（手写 JSON 简化版）**。

- 字符级迭代器（自定义 `Iterator`）
- 状态机：跳空白、读字符串、读数字、读布尔/null
- 闭包驱动 dispatch
- 支持类型化输出 `Json` enum
- 完整测试用例

## 📖 推荐节奏

1. 按顺序读 6 篇文档
2. 边读边敲示例
3. 完成 [`exercises/`](./exercises) 7 个练习
4. 做 [`project-04-text-parser/`](./project-04-text-parser)（预计 1.5–2.5 天）
5. 读 [docs/06-stage-review.md](./docs/06-stage-review.md) 自测
6. 提交 git，进入 Stage 5

## 🔗 对照官方资料

本教程对应 [The Rust Book](https://doc.rust-lang.org/book/) 第 13 章（闭包）、第 15 章（智能指针）。

## ⚠️ 心态提示

> Stage 4 是 Rust **函数式编程 + 内存管理** 的进阶。三个"看起来"的概念其实互相关联：
>
> - **闭包** = 可以捕获环境的匿名函数
> - **迭代器** = 闭包的"流水线化"
> - **智能指针** = 在运行时扩展"借用检查"的能力
>
> 学完本章你应该能：
> 1. 写自定义 `Iterator` 处理流式数据
> 2. 组合 `Rc<RefCell<T>>` 表达"共享 + 可变"图结构
> 3. 区分"什么时候需要内部可变性"

---

回到 [项目总览](../README.md)
