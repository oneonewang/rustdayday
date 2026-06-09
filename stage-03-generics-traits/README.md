# Stage 3：泛型、Trait、生命周期

> 目标：写出**"对任何类型都能用"** 的代码、定义行为契约、用生命周期让借用检查器满意——这是 Rust **抽象能力**的核心。

## 📍 你将学到

- **泛型** `<T>`：写出"参数化类型"的代码
- **Trait**：定义行为契约，类似其他语言的 interface，但更强大
- **Trait bounds** 与 `where` 子句：约束泛型"能做什么"
- ⭐ **生命周期**：让编译器知道"哪个引用活得更久"
- **Trait object** `dyn Trait`：动态分发
- **高级 trait**：关联类型、运算符重载、supertrait

## 🗺️ 章节导航

| # | 文档 | 主题 | 预计时间 |
|---|------|------|----------|
| 1 | [docs/01-generics.md](./docs/01-generics.md) | 泛型函数 / struct / enum | 45 min |
| 2 | [docs/02-traits.md](./docs/02-traits.md) | trait 定义、impl、默认方法、trait bounds | 60 min |
| 3 | **[docs/03-lifetimes.md](./docs/03-lifetimes.md)** ⭐ | **生命周期标注、省略规则** | **120 min** |
| 4 | [docs/04-trait-objects.md](./docs/04-trait-objects.md) | `dyn Trait`、`Box<dyn Trait>` | 45 min |
| 5 | [docs/05-advanced-traits.md](./docs/05-advanced-traits.md) | 关联类型、运算符重载、supertrait | 60 min |
| 6 | [docs/06-stage-review.md](./docs/06-stage-review.md) | 知识地图、自测 10 题 | 30 min |

## 🛠️ 练习

| 练习 | 主题 | 难度 |
|------|------|------|
| [ex01_generics](./exercises/ex01_generics) | 泛型函数 / struct / enum | ⭐ |
| [ex02_traits](./exercises/ex02_traits) | trait 定义与实现 | ⭐⭐ |
| [ex03_trait_bounds](./exercises/ex03_trait_bounds) | bounds、`where` 子句 | ⭐⭐ |
| [ex04_lifetimes](./exercises/ex04_lifetimes) | 生命周期标注与省略 | ⭐⭐⭐ |
| [ex05_operator_overload](./exercises/ex05_operator_overload) | 实现 `Add` / `Display` / `From` | ⭐⭐ |
| [ex06_trait_objects](./exercises/ex06_trait_objects) | `Box<dyn Trait>` | ⭐⭐ |
| [ex07_advanced_traits](./exercises/ex07_advanced_traits) | 关联类型 / supertrait | ⭐⭐⭐ |

## 🎯 综合项目

[`project-03-lru-cache/`](./project-03-lru-cache) — 泛型 LRU 缓存。

- 泛型 `<K, V>` 支持任意 key/value 类型
- `K: Hash + Eq`、`V: Clone` 等 trait 约束
- `get` / `put` / `len` / `clear` 方法
- 内部用 `HashMap` + `VecDeque`（或自己实现的链表）
- 完整测试用例 + 性能基准

## 📖 推荐节奏

1. 按顺序读 6 篇文档（**第 3 章生命周期至少 2 小时，认真做**）
2. 边读边敲示例
3. 完成 [`exercises/`](./exercises) 7 个练习
4. 做 [`project-03-lru-cache/`](./project-03-lru-cache)（预计 1.5–2.5 天）
5. 读 [docs/06-stage-review.md](./docs/06-stage-review.md) 自测
6. 提交 git，进入 Stage 4

## 🔗 对照官方资料

本教程对应 [The Rust Book](https://doc.rust-lang.org/book/) 第 10 章（泛型 / Trait / 生命周期）。第 17 章（trait object）会涉及一点。

## ⚠️ 心态提示

> ⭐ **生命周期是 Rust 学习路上最大的坎**。第一次接触"显式标注引用生命周期"会觉得莫名其妙——为什么不让编译器自己算？答案是：**编译器希望我们告诉它"这段引用至少活多久"的承诺**。你标注的并不是真实生命周期，而是**关系**。
>
> 学完本章你应该能：
> 1. 看懂几乎所有 borrow check 报错
> 2. 写出"通用、对多种类型都能用"的库代码
> 3. 在标准库源码里认出 trait 约束
>
> **如果卡住超过 30 分钟，先看下一章，回来再啃**——生命周期常需要"绕一圈"才理解。

---

回到 [项目总览](../README.md)
