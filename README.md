# Rust 系统学习项目 · rustdayday

> 从零到中高级的 Rust 学习教程，分阶段产出，每阶段独立闭环（文档 → 练习 → 项目）。

## 📚 学习路径

| 阶段 | 主题 | 状态 | 入口 |
|------|------|------|------|
| **Stage 1** | Rust 基础语法与工具链 | ✅ 已完成 | [stage-01-basics/](./stage-01-basics/) |
| **Stage 2** | 结构化代码与错误处理 | ✅ 已完成 | [stage-02-structuring/](./stage-02-structuring/) |
| Stage 3 | 泛型、Trait 与生命周期 | 📅 待开始 | — |
| Stage 4 | 智能指针、闭包、迭代器 | 📅 待开始 | — |
| Stage 5 | 并发与异步 | 📅 待开始 | — |
| Stage 6 | 实战项目 | 📅 待开始 | — |

完整规划见 [LEARNING_PLAN.md](./LEARNING_PLAN.md)。

## 🚀 快速开始

```bash
# 1. 确认 Rust 已装好
cargo --version

# 2. 进入 Stage 1
cd stage-01-basics
cat README.md
```

## 🎯 学习方法

每阶段按这个节奏循环：

```
读文档 → 边读边敲示例 → 完成 exercises/ → 做 project/ → 复盘 → 下一阶段
```

**进入下一阶段的标准**：当阶段的所有 exercises 和 project 能 `cargo run` 通过。

## 🔗 配套资源

- [The Rust Book（官方教程）](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings（小练习）](https://github.com/rust-lang/rustlings)
- [标准库 API 文档](https://doc.rust-lang.org/std/)

## 📝 当前进度

- [x] **Stage 1**：
  - [x] 9 篇教程文档
  - [x] 7 个练习项目
  - [x] 1 个综合项目（猜数字游戏）
  - [x] 编译验证：6/7 练习可编译（ex05 故意不通过，让你修）
  - [x] 项目可运行（已通过冒烟测试）
- [x] **Stage 2**：
  - [x] 8 篇教程文档（含阶段复习）
  - [x] 7 个练习项目
  - [x] 1 个综合项目（CLI Todo 工具，支持 add/list/done/remove + JSON 持久化）
  - [x] 编译验证：7/7 练习可编译
  - [x] 项目可运行（已通过完整冒烟测试：增删查改、错误处理、JSON 持久化）
