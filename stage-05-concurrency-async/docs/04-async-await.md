# 04 · async / await 基础

> **本章目标**：理解 `Future` trait、`async` / `await` 语法、状态机模型、为什么需要 runtime（executor）。

## 4.1 同步 vs 异步

```rust
// 同步：阻塞当前线程等
fn read_file_sync(path: &str) -> String {
    std::fs::read_to_string(path).unwrap()    // 等磁盘
}

// 异步：让出线程等
async fn read_file_async(path: &str) -> String {
    // ...
}
```

> 同步 = 烧 CPU 等待，线程被占着
> 异步 = 让出线程，等就绪再回来——单线程撑大量并发

## 4.2 `async fn` 是什么？

```rust
async fn fetch_url(url: &str) -> String {
    // ... 异步操作
}
```

> **`async fn` 自动返回一个实现了 `Future` 的类型**——它**不会**立即跑。

```rust
let fut = fetch_url("https://example.com");   // 这行什么也不做！
let result = fut.await;                       // 这里才开始执行
```

> 💡 **关键概念**：`async` 是个"工厂"——调用只是**创建 future**。要 `.await` 才执行。

## 4.3 `Future` trait

```rust
trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready(T),       // 算完了
    Pending,        // 还没完，请 wake 我
}
```

> **Future 是个状态机**——`poll` 看完了就 `Ready`，没完就 `Pending` + 注册 waker。

## 4.4 `.await`：让出当前线程

```rust
async fn process() {
    let a = fetch_a().await;       // 挂起，等 a 完
    let b = fetch_b(a).await;      // 挂起，等 b 完
    println!("done: {b}");
}
```

> 每次 `.await` 是个**可能挂起点**。整个 `async fn` 编译出来是个**状态机**。

## 4.5 状态机示例

```rust
async fn example() -> i32 {
    let x = task_a().await;     // 挂起点 1
    let y = task_b(x).await;    // 挂起点 2
    x + y
}
```

编译后近似于：

```rust
enum ExampleState {
    Start,
    AwaitingA { fut: TaskAFuture },
    AwaitingB { fut: TaskBFuture, x: i32 },
    Done,
}
```

## 4.6 谁来 poll？Executor

> **Executor（执行器）** = 反复 poll future 直到 Ready 的"调度员"。

```rust
fn block_on<F: Future>(fut: F) -> F::Output {
    // 简单版：不停 poll
    let mut fut = Box::pin(fut);
    let waker = /* ... */;
    let mut cx = Context::from_waker(&waker);

    loop {
        match fut.as_mut().poll(&mut cx) {
            Poll::Ready(v) => return v,
            Poll::Pending   => std::thread::park(),     // 等 waker 唤醒
        }
    }
}
```

**生产用 executor**：
- `tokio::runtime::Runtime`（最流行）
- `async-std`
- `smol`
- `futures::executor::block_on`（玩具）

## 4.7 第一个 async 程序

```rust
use futures::executor::block_on;

async fn hello() -> i32 {
    println!("hello");
    42
}

fn main() {
    let v = block_on(hello());
    println!("result = {v}");
}
```

> 用 `futures` crate 当 executor。**Tokio 的版本**：

```rust
#[tokio::main]
async fn main() {
    println!("hello");
}
```

## 4.8 并发：join! 和 try_join!

```rust
use tokio;

async fn task_a() -> i32 { 1 }
async fn task_b() -> i32 { 2 }

#[tokio::main]
async fn main() {
    let (a, b) = tokio::join!(task_a(), task_b());
    println!("a={a}, b={b}");   // 1 2（并发跑）
}
```

> `tokio::join!` 在**同一任务**里并发等——不开新线程。

## 4.9 `select!`：race

```rust
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    tokio::select! {
        _ = sleep(Duration::from_millis(100)) => println!("超时"),
        v = async { 42 } => println!("得到 {v}"),
    }
}
```

> `select!` 等待**多个** future，**最先**完成的赢；其他被取消。

## 4.10 spawn：开新任务

```rust
#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        println!("后台任务");
        42
    });

    let v = handle.await.unwrap();
    println!("结果 = {v}");
}
```

> `tokio::spawn` 把 future 交给 runtime，**独立调度**——可能在不同线程跑。
>
> **要求**：future 必须是 `Send + 'static`。

## 4.11 `Send + 'static` 为什么？

```rust
fn require_send<T: Send + 'static>(_: T) {}

#[tokio::main]
async fn main() {
    let s = String::from("hi");
    tokio::spawn(async move {           // move 拿走 s
        println!("{s}");
    });
    require_send(s);                     // s 已被 move，OK
}
```

> - `Send` = 能跨线程移动
> - `'static` = 不借用外部数据（或者只借用 'static）
>
> **借用外部数据要 `'static`**——否则 runtime 不知道借用活得够久。

## 4.12 自引用结构问题

```rust
async fn bad() {
    let v = vec![1, 2, 3];
    let r = &v[0];          // 借 v
    some_async_op().await;  // 挂起期间 v 可能在 await 后变了位置？
    println!("{r}");
}
```

> ⚠️ 实际不直接编译错（编译器会拒绝）——需要 `Pin` 或重写。**详见 async-book 第 4 章**。

## 4.13 错误处理：`?` 在 async 里的用法

```rust
async fn fetch() -> Result<String, reqwest::Error> {
    let body = reqwest::get("https://example.com").await?
        .text().await?;
    Ok(body)
}
```

> `?` 在 `async fn` 里和同步一样——`Result` 是值，await 是操作。

## 4.14 取消（cancellation）

```rust
tokio::select! {
    _ = slow_op() => {}
    _ = tokio::time::sleep(Duration::from_millis(100)) => {
        // slow_op 被取消（drop），其他分支胜出
    }
}
```

> ⚠️ 被取消的 future 在 drop 时**不会**继续跑——可能留资源没释放。要 `Drop` 实现里做清理。

## 4.15 一个完整 async 函数

```rust
use std::time::Duration;

async fn slow_op() -> u32 {
    tokio::time::sleep(Duration::from_millis(500)).await;
    42
}

async fn with_timeout() -> u32 {
    tokio::time::timeout(Duration::from_millis(200), slow_op())
        .await
        .unwrap_or(0)         // 超时返回 0
}

#[tokio::main]
async fn main() {
    let start = std::time::Instant::now();
    let v = with_timeout().await;
    println!("v={v}, 耗时 {:?}", start.elapsed());
}
```

## 4.16 异步 vs 多线程

| 维度 | 多线程 | async |
|------|--------|-------|
| 启动成本 | 几百 KB 栈 | 几乎 0 |
| 并发数 | 几十~几百 | 几万+ |
| CPU 密集 | ✅（用 rayon） | ❌（一个 task 占线程） |
| I/O 密集 | ❌（线程闲着） | ✅ |
| 阻塞操作 | OK | ❌（必须用 async 版本） |

> **核心**：async 适合"很多任务都在等 I/O"——HTTP / 数据库 / 文件 / 定时器。

## 4.17 一个反模式：阻塞 runtime

```rust
// ❌ 阻塞当前 runtime worker 线程！
async fn bad() {
    std::thread::sleep(Duration::from_secs(1));   // 💥 runtime worker 被占 1 秒
}

// ✅ 用 tokio 的 sleep
async fn good() {
    tokio::time::sleep(Duration::from_secs(1)).await;
}
```

> 在 async 代码里**不要**调 std 的阻塞函数——会卡死整个 worker。

## 4.18 对比其他语言

| 概念 | Rust | JavaScript | Python | Go |
|------|------|-----------|--------|-----|
| 语法 | `async fn` / `.await` | `async` / `await` | `async def` / `await` | `go func()` |
| Future 类型 | `impl Future` | `Promise` | `Coroutine` / `Future` | goroutine |
| 调度器 | runtime（Tokio 等） | V8 event loop | asyncio | runtime |
| 取消 | drop future | Promise 不支持 | task.cancel() | runtime 没法直接 |
| 阻塞兼容 | ❌（不能混 std 阻塞） | ✅ | ✅ | ✅ |

---

## 🏋️ 本章小练习

**练习 4.1**：写 `async fn add(a: i32, b: i32) -> i32 { a + b }`，用 `block_on` 跑。

**练习 4.2**：用 `tokio::join!` 并发跑 3 个 `async fn sleep_print(n: u32)`——`n` 秒后打印 `n`——验证总耗时 ≈ 最长的那个（不是 3 个相加）。

**练习 4.3**：用 `select!` 实现"先到的赢"：

```rust
let v = select! {
    v = async { 100 } => v,
    v = async { 200 } => v,
};
```

**练习 4.4**：写一个 `async fn retry<F, T, E>(f: F, n: usize) -> Result<T, E>`，调用 f 失败重试 n 次。

**练习 4.5**（思考）：`async fn` 默认返回的 `impl Future` **不是** `Send`——为什么？怎么让它是？

> 答：因为 future 借用了 `&self` 等非 Send 引用。要 `Send`，future 内不借用外部 + 所有跨 await 的变量都是 `Send`。

---

下一章：[05 · Tokio ⭐ →](./05-tokio.md)
