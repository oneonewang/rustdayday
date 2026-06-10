# Rust 系统学习项目 · rustdayday

> 从零到中高级的 Rust 学习教程，分阶段产出，每阶段独立闭环（文档 → 练习 → 项目）。

## 📚 学习路径

| 阶段 | 主题 | 状态 | 入口 |
|------|------|------|------|
| **Stage 1** | Rust 基础语法与工具链 | ✅ 已完成 | [stage-01-basics/](./stage-01-basics/) |
| **Stage 2** | 结构化代码与错误处理 | ✅ 已完成 | [stage-02-structuring/](./stage-02-structuring/) |
| **Stage 3** | 泛型、Trait 与生命周期 | ✅ 已完成 | [stage-03-generics-traits/](./stage-03-generics-traits/) |
| **Stage 4** | 智能指针、闭包、迭代器 | ✅ 已完成 | [stage-04-smart-pointers-fp/](./stage-04-smart-pointers-fp/) |
| **Stage 5** | 并发与异步 | ✅ 已完成 | [stage-05-concurrency-async/](./stage-05-concurrency-async/) |
| **Stage 6A** | 异步 Web 后端 | ✅ 已完成 | [stage-06-real-projects/project-A-async-web/](./stage-06-real-projects/project-A-async-web/) |
| **Stage 6B** | 高性能 CLI 工具 | ✅ 已完成 | [stage-06-real-projects/project-B-rust-cli/](./stage-06-real-projects/project-B-rust-cli/) |
| Stage 6C–D | WASM / 系统编程 | 📅 待开始 | — |

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

## 🛠️ Zed IDE 调试（已配好）

仓库里已配好两个 Zed 配置文件：

- **`.zed/debug.json`**：列出全部 16 个小程序的调试入口。在 Zed 里按 `F4`（或菜单 Run → Start Debugger），从下拉里选一个就能直接断点调试。
- **`.zed/tasks.json`**：6 个常用 cargo 任务（run / check / test / build --release / fmt / clippy）。光标停在某个项目的 `src/main.rs` 上按对应快捷键，就会在**那个项目目录**里跑 cargo。

### 第一次使用需要做的事

1. **装调试器**：
   - macOS：`brew install lldb`
   - Ubuntu / Debian：`sudo apt install lldb`
2. **装 rust-analyzer 扩展**：Zed 设置里搜 `rust-analyzer` 启用。
3. **打开项目根**：在 Zed 里"Open Folder"打开 `rustdayday/` 整个目录。
4. **第一次跑某个程序**前，可能要等 Zed 在该子目录自动建好 `target/`。如果构建慢，终端里先 `cd` 进那个子目录跑一次 `cargo build` 预热。

### 常见坑

| 报错 | 原因 | 修法 |
|------|------|------|
| `error: unexpected argument '' found` | Zed 把空字符串传给 cargo | 删掉 `~/.config/zed/settings.json` 里 Rust 相关的 `args` 字段，只留 `.zed/debug.json` |
| 找不到 `target/debug/xxx` | 该子项目还没 build 过 | 在子目录 `cargo build` 一次 |
| 调试器 attach 失败 | 没装 lldb | 见上面"装调试器" |
| rust-analyzer 不工作 | 没装 rust 工具链 | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` |

> ⚠️ **特别注意**：每个小程序都是**独立 cargo 项目**，都有自己单独的 `target/` 目录。**根目录没有 `Cargo.toml`，不要在根目录跑 `cargo build`**——会找不到 `package` 段。

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
- [x] **Stage 3**：
  - [x] 6 篇教程文档（泛型 / Trait / 生命周期 / Trait Object / 高级 Trait / 阶段复习）
  - [x] 7 个练习项目
  - [x] 1 个综合项目（泛型 LRU 缓存）
  - [x] 编译验证：7/7 练习可编译
  - [x] LRU 缓存项目完整运行（6 个演示 + 完整断言通过）
  - [x] Zed 调试配置扩展到 25 个 entry
- [x] **Stage 4**：
  - [x] 6 篇教程文档（Box/Rc / RefCell / 闭包 / 迭代器 / 智能指针深入 / 阶段复习）
  - [x] 7 个练习项目
  - [x] 1 个综合项目（手写 JSON 解析器）
  - [x] 编译验证：7/7 练习可编译
  - [x] JSON 解析器项目完整运行（5 个演示 + 完整断言通过；支持 null/bool/数字/字符串/数组/对象/嵌套）
  - [x] Zed 调试配置扩展到 33 个 entry
- [x] **Stage 5**：
  - [x] 7 篇教程文档（线程/通道/共享状态/async/Tokio/Send-Sync/阶段复习）
  - [x] 7 个练习项目（含 tokio 依赖）
  - [x] 1 个综合项目（异步 HTTP 抓取器：手写 server + reqwest 客户端 + 限流/超时/错误处理）
  - [x] 编译验证：7/7 练习可编译
  - [x] 异步抓取器项目完整运行（5 演示 + 完整断言通过；演示 1 并发 3ms / 演示 2 限流 ~600ms / 演示 3 超时 100ms / 演示 4 错误处理 / 演示 5 断言）
  - [x] Zed 调试配置扩展到 41 个 entry
