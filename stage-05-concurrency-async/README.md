# Stage 5：并发与异步

> 目标：会用 `std::thread`、Tokio 异步 runtime、`Send` / `Sync` 约束，写出**高性能**的并发程序。

## 📍 你将学到

- **线程**：`std::thread` 与 `spawn`、`JoinHandle`、scoped threads
- **消息传递**：`mpsc` 通道
- **共享状态**：`Arc<Mutex<T>>` / `Arc<RwLock<T>>` / `OnceLock`
- **`async` / `await`** 与 `Future` trait
- **Tokio**：runtime、spawn、join! / select!、tokio 同步原语
- **`Send` / `Sync`** 两个 marker trait——理解为什么有些东西**不能**跨线程

## 🗺️ 章节导航

| # | 文档 | 主题 | 预计时间 |
|---|------|------|----------|
| 1 | [docs/01-threads.md](./docs/01-threads.md) | `std::thread::spawn`、scoped threads | 45 min |
| 2 | [docs/02-channels.md](./docs/02-channels.md) | `mpsc` 通道 | 45 min |
| 3 | [docs/03-shared-state.md](./docs/03-shared-state.md) | `Arc<Mutex<T>>` / `Arc<RwLock<T>>` | 45 min |
| 4 | [docs/04-async-await.md](./docs/04-async-await.md) | `async` / `await`、Future、Pin | 60 min |
| 5 | **[docs/05-tokio.md](./docs/05-tokio.md)** ⭐ | **Tokio runtime、async I/O** | **90 min** |
| 6 | [docs/06-send-sync.md](./docs/06-send-sync.md) | `Send` / `Sync` 规则、约束传播 | 45 min |
| 7 | [docs/07-stage-review.md](./docs/07-stage-review.md) | 知识地图、自测 10 题 | 30 min |

## 🛠️ 练习

| 练习 | 主题 | 难度 |
|------|------|------|
| [ex01_threads](./exercises/ex01_threads) | `std::thread::spawn`、JoinHandle | ⭐ |
| [ex02_channels](./exercises/ex02_channels) | `mpsc` 单/多发送者 | ⭐ |
| [ex03_shared_state](./exercises/ex03_shared_state) | `Arc<Mutex<T>>` 多线程计数器 | ⭐⭐ |
| [ex04_async_await](./exercises/ex04_async_await) | `async` / `await` 基础 | ⭐⭐ |
| [ex05_tokio](./exercises/ex05_tokio) | Tokio runtime、spawn、join!、select! | ⭐⭐⭐ |
| [ex06_send_sync](./exercises/ex06_send_sync) | Send / Sync 约束 | ⭐⭐⭐ |
| [ex07_async_streams](./exercises/ex07_async_streams) | async Stream、并发抓取 | ⭐⭐⭐ |

## 🎯 综合项目

[`project-05-async-scraper/`](./project-05-async-scraper) — **异步 HTTP 抓取器**。

- 用 `reqwest` + Tokio 抓取多个 URL
- `join!` 并发等待
- `select!` 处理超时
- 报告每个 URL 的状态、耗时、内容长度

## 📖 推荐节奏

1. 按顺序读 7 篇文档（**第 5 章 Tokio 至少 90 分钟**）
2. 边读边敲示例
3. 完成 [`exercises/`](./exercises) 7 个练习
4. 做 [`project-05-async-scraper/`](./project-05-async-scraper)（预计 1.5–2.5 天）
5. 读 [docs/07-stage-review.md](./docs/07-stage-review.md) 自测
6. 提交 git，进入 Stage 6

## 🔗 对照官方资料

本教程对应：
- [The Rust Book](https://doc.rust-lang.org/book/) 第 16 章（无畏并发）
- [Asynchronous Programming in Rust](https://rust-lang.github.io/async-book/)（async 专书）
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)

## ⚠️ 心态提示

> 异步是 Rust 学习路上"第二次大坎"（第一次是生命周期）。三个关键概念需要理清：
>
> 1. **`Future` 是个惰性值**——不 poll 就不动。`async fn` 返回 `Future` 但**不立即跑**。
> 2. **`.await` 才会真执行**——`tokio::spawn(fut)` 把 future 交给 runtime，runtime poll 它。
> 3. **`Send` / `Sync` 是编译期守门员**——`tokio::spawn` 要求 future 是 `Send`（能跨线程）。
>
> 学完本章你应该能：
> 1. 写出"1000 个并发 HTTP 请求"的代码
> 2. 区分 `thread::spawn` 和 `tokio::spawn`
> 3. 看懂 `future cannot be sent between threads safely` 报错

---

回到 [项目总览](../README.md)
