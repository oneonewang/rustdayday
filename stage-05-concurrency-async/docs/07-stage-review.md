# 07 · Stage 5 复习与综合自测

> **本章目标**：把 Stage 5 串成图，做 10 道综合题。**通过再进 Stage 6。**

## 7.1 知识地图

```
Stage 5 概念图
════════════════════════════════════════════════════════════════

   线程                  消息传递               共享状态
   ────                  ──────                ───────
   std::thread::spawn    mpsc::channel         Arc<Mutex<T>>
   JoinHandle            sync_channel          Arc<RwLock<T>>
   move 闭包             Sender::clone         OnceLock
   thread::scope         Receiver as Iter      Mutex 毒化
   Rayon（池）            close 语义             死锁与避免

       │                     │                       │
       └──────────┬──────────┴───────────────────────┘
                  │
                  ▼
   ⭐ async / await / Future / Tokio
   ─────────────────────────────────
   async fn  = 工厂，调用不跑
   .await    = 真执行，可挂起
   Future    = 状态机，poll / Pending / Ready
   tokio::spawn = 后台任务（要 Send + 'static）
   tokio::join! / select! = 并发 / race
   tokio::sync::* = 异步锁（可 await）
   tokio::time::* = 异步时间

   Send / Sync 守门员
   ─────────────────
   Send = 类型可跨线程 move
   Sync = &T 可跨线程共享
   编译器自动检查 + 必要时 unsafe impl
```

## 7.2 一句话回顾

- **线程** = 操作系统的调度单位；裸线程适合少量后台任务。
- **消息传递**（`mpsc`）= "不要共享内存"；简单、可推理。
- **共享状态**（`Arc<Mutex<T>>`）= 共享内存 + 锁；灵活但要小心死锁。
- **async / await** = 编译期生成状态机；runtime 调度，**几万并发不耗线程**。
- **Tokio** = Rust 异步生态的事实标准。
- **`Send` / `Sync`** = 编译期守门员，**自动**判断类型能不能跨线程。

## 7.3 综合自测（10 题）

### 题 1：多线程计数

用 `std::thread::spawn` + `Arc<Mutex<i32>>`，10 个线程并发对共享计数器 `+1`，最终 = 10。

### 题 2：mpsc 通道

主线程开 3 个 worker，每个 worker 发 10 条 `i32` 给主线程；主线程打印总和。

### 题 3：scoped threads

用 `thread::scope` 写一个并行排序：`v.sort_by(...)` 在两个线程里分别处理半段。

### 题 4：tokio::join! 并发

写 3 个 `async fn` 各 `sleep(100ms).await` 返回不同值，用 `tokio::join!` 并发，验证总耗时 ≈ 100ms。

### 题 5：tokio::time::timeout

写一个 `async fn with_retry` 包裹任意 future，超时 200ms 返回 `Err`。

### 题 6：tokio::mpsc 多发多收

5 个 producer 各发 20 条，1 个 consumer 收，验证收到 100 条且顺序不保证。

### 题 7：select! race

```rust
async fn fast() -> u32 { 100 }
async fn slow() -> u32 { /* sleep 200ms */ 200 }

tokio::select! {
    v = fast() => println!("fast 胜: {v}"),
    v = slow() => println!("slow 胜: {v}"),
}
```

### 题 8：Send 编译错

定义一个含 `Rc` 的 future，**故意**让它不能 `tokio::spawn`——然后改成 `Arc` 让它能。

### 题 9：异步限流

用 `tokio::sync::Semaphore` 实现"1000 个任务，但最多 10 并发"。

### 题 10：综合：并发抓取 + 报告

写一个 `async fn scrape(url: &str) -> (String, usize)`，模拟 100ms 抓取返回 `(url, 200)`。在 main 里用 `join!` 并发跑 5 个，验证总耗时 ≈ 100ms。

## 7.4 答案要点

| 题 | 关键点 |
|----|--------|
| 1 | `Arc::clone` + `lock().unwrap() += 1` |
| 2 | `tx.clone()` 给 worker，主线程收 |
| 3 | `thread::scope(\|s\| s.spawn(...))` |
| 4 | `tokio::join!` 三并发，**不开新线程** |
| 5 | `timeout(dur, fut).await` |
| 6 | `mpsc::channel(buf)` + 多个 `tx.clone` |
| 7 | `select!` 选先到的 |
| 8 | `Rc` → `Arc` |
| 9 | `Semaphore::new(10)` + `acquire().await` |
| 10 | `join!` 三个 async fn |

## 7.5 通过标准

- 10 题中 **8 题** 在 60 分钟内一次写对
- 所有 [`exercises/`](./../exercises) 和 [`project-05-async-scraper/`](./../project-05-async-scraper) 能 `cargo build` 通过

## 7.6 阶段回顾清单

> 进入 Stage 6 之前自问：

- [ ] 裸 `thread::spawn` 和 `tokio::spawn` 何时用
- [ ] `move` 闭包、scoped threads
- [ ] mpsc 通道 close 语义
- [ ] `Arc<Mutex<T>>` / `Arc<RwLock<T>>` / `OnceLock`
- [ ] `async fn` 返回 future 但**不**立即跑
- [ ] `.await` 才真执行
- [ ] `tokio::join!` / `try_join!` / `select!` 区别
- [ ] `tokio::sync::Mutex` vs `std::sync::Mutex`
- [ ] `Send` / `Sync` 含义
- [ ] 知道为什么 `Rc` 不能 `tokio::spawn`
- [ ] 看懂 "future cannot be sent between threads safely" 报错

## 7.7 推荐复习间隔

> - 写完 1 天后做 7.3 自测
> - 进入 Stage 6 之前再做一遍
> - 1 个月后再做（async / Tokio 是 Rust 后期最常用工具）

---

🎉 Stage 5 完！准备好后告诉我开始 Stage 6（实战项目——选 2-3 个完整端到端项目）。
