# 05 · Tokio：实战异步运行时 ⭐

> **本章目标**：会用 Tokio 写一个真实可运行的并发程序，掌握 `tokio::spawn` / `join!` / `select!` / `mpsc` / `time`。

## 5.1 Tokio 是什么？

> **Tokio** = Rust 异步生态的事实标准。一个 runtime + 异步 I/O + 同步原语 + 工具。

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

> `features = ["full"]` 打开所有子模块——开发期方便；release 最好按需开（`rt-multi-thread`, `macros`, `time`, `sync`, `net` 等）。

## 5.2 `#[tokio::main]`：runtime 入口

```rust
#[tokio::main]
async fn main() {
    println!("hello async");
}
```

> 等价于：

```rust
fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        println!("hello async");
    });
}
```

**参数**：

```rust
#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() { ... }

#[tokio::main(flavor = "current_thread")]      // 单线程
async fn main() { ... }
```

> `current_thread`：单线程、调度器简单、debug 友好
> `multi_thread`：默认，多核并行

## 5.3 `spawn`：后台任务

```rust
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let h1 = tokio::spawn(async {
        sleep(Duration::from_millis(100)).await;
        println!("task 1 done");
        1
    });

    let h2 = tokio::spawn(async {
        sleep(Duration::from_millis(50)).await;
        println!("task 2 done");
        2
    });

    let v1 = h1.await.unwrap();
    let v2 = h2.await.unwrap();
    println!("v1={v1}, v2={v2}");
}
```

> 任务**并发**跑（不阻塞），但**不保证顺序**。

## 5.4 `join!`：同一任务里并发

```rust
use tokio::time::{sleep, Duration};

async fn task(s: u64) -> u64 {
    sleep(Duration::from_millis(s)).await;
    s
}

#[tokio::main]
async fn main() {
    let start = std::time::Instant::now();
    let (a, b, c) = tokio::join!(
        task(100),
        task(100),
        task(100),
    );
    println!("a={a}, b={b}, c={c}, 耗时 {:?}", start.elapsed());
}
```

> 总耗时 ≈ 100ms（不是 300ms）—— `join!` 在**同一 task**里并发 await。

## 5.5 `try_join!`：有一个失败就取消其他

```rust
use tokio::try_join;

async fn good() -> Result<i32, &'static str> { Ok(1) }
async fn bad() -> Result<i32, &'static str> { Err("oops") }

#[tokio::main]
async fn main() {
    match try_join!(good(), bad(), good()) {
        Ok((a, b, c)) => println!("{a} {b} {c}"),
        Err(e)         => println!("失败: {e}"),
    }
}
```

> `bad()` 一旦失败，其他 task 立刻被取消（drop）。

## 5.6 `select!`：race + 超时

```rust
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let result = tokio::select! {
        v = async { 42 } => v,
        _ = sleep(Duration::from_millis(100)) => -1,
    };
    println!("result = {result}");   // 42（async 立即完成）
}
```

> `select!` 监听多个 future，**先完成的赢**；其他被取消。
>
> 超时是 `select!` 的最常见用法。

## 5.7 `tokio::time` 套件

| 函数 | 用途 |
|------|------|
| `sleep(dur).await` | 等一段时间 |
| `interval(dur)` | 周期性 tick |
| `timeout(dur, fut).await` | 给 future 加超时 |
| `Instant::now()` / `elapsed()` | 测时 |

```rust
use tokio::time::{timeout, Duration};

#[tokio::main]
async fn main() {
    match timeout(Duration::from_millis(200), slow_op()).await {
        Ok(v)  => println!("ok: {v}"),
        Err(_) => println!("超时"),
    }
}

async fn slow_op() -> u32 {
    tokio::time::sleep(Duration::from_secs(1)).await;
    42
}
```

## 5.8 `tokio::sync::mpsc`：多发多收的通道

```rust
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(100);   // 缓冲 100

    for i in 0..5 {
        let tx = tx.clone();
        tokio::spawn(async move {
            tx.send(format!("msg {i}")).await.unwrap();
        });
    }
    drop(tx);   // 关闭

    while let Some(msg) = rx.recv().await {
        println!("{msg}");
    }
}
```

> 比 std 的 `mpsc` 强大：**多个 receiver**（`broadcast` / `watch`），且不会阻塞。

## 5.9 其他 Tokio 同步原语

| 类型 | 用途 |
|------|------|
| `tokio::sync::Mutex` | async-aware 互斥（**可重入**） |
| `tokio::sync::RwLock` | async-aware 读写锁 |
| `tokio::sync::Notify` | 一次性通知（one-shot） |
| `tokio::sync::Semaphore` | 信号量 |
| `tokio::sync::oneshot` | 单发单收 |
| `tokio::sync::broadcast` | 多收 |
| `tokio::sync::watch` | 配置中心（最后值） |

**与 std 同步原语对比**：

| | `std::sync::Mutex` | `tokio::sync::Mutex` |
|---|--------------------|--------------------|
| 加锁 | 同步阻塞 | async `.lock().await` |
| 持锁等 await | ❌ 死锁 | ✅ |
| 跨 await 安全 | ❌ | ✅ |
| 性能 | 更快 | 稍慢 |

> **规则**：async 代码里要用 `tokio::sync::*`——**不要**混用 std 锁。

## 5.10 `tokio::task::spawn` 的限制

```rust
let v = vec![1, 2, 3];
tokio::spawn(async {
    println!("{:?}", v);    // ❌ v 没有 move
});

// ✅
let v = vec![1, 2, 3];
tokio::spawn(async move {
    println!("{:?}", v);    // ✅
});
```

> `tokio::spawn` 要求 future 是 **`Send + 'static`**。

## 5.11 `tokio::spawn` vs `tokio::task::spawn_local`

```rust
let local = tokio::task::LocalSet::new();
local.run_until(async {
    tokio::task::spawn_local(async { /* 不要求 Send */ }).unwrap();
}).await;
```

> `spawn_local` 不要求 `Send`——但**必须**在 `LocalSet` 里跑。

## 5.12 一个 HTTP 并发抓取示例

```rust
use std::time::Duration;

#[tokio::main]
async fn main() {
    let urls = vec![
        "https://example.com",
        "https://example.org",
        "https://example.net",
    ];

    let start = std::time::Instant::now();
    let client = reqwest::Client::new();

    let tasks: Vec<_> = urls.into_iter().map(|url| {
        let client = client.clone();
        tokio::spawn(async move {
            let resp = client.get(url).send().await?;
            let body = resp.text().await?;
            Ok::<_, reqwest::Error>((url, body.len()))
        })
    }).collect();

    for t in tasks {
        match t.await.unwrap() {
            Ok((url, len)) => println!("{url}: {len} 字节"),
            Err(e)         => println!("{url}: ERR {e}"),
        }
    }
    println!("总耗时 {:?}", start.elapsed());
}
```

> 3 个 URL **并发**抓，耗时 ≈ 最慢那个（不是相加）。

## 5.13 `tokio::spawn` 任务的错误处理

```rust
let h = tokio::spawn(async { 42 / 0 });    // panic
let r = h.await;
assert!(r.is_err());                        // JoinError
```

> 子任务 panic → `JoinError`——不会让整个 runtime 死。

## 5.14 `tokio::main` vs `block_on`

```rust
fn main() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        println!("hi from manual runtime");
    });
}
```

> `#[tokio::main]` 是糖——手动建 runtime 更灵活（比如多 runtime、嵌入其他框架）。

## 5.15 测试 async 代码

```rust
#[tokio::test]
async fn my_async_test() {
    let v = async_op().await;
    assert_eq!(v, 42);
}
```

> `#[tokio::test]` = `#[test]` + 自动包 `tokio::test::block_on`。
>
> 默认 `current_thread` runtime——**测试要并发时**加 `flavor = "multi_thread"`。

## 5.16 性能：Tokio vs 裸 std 线程

| 维度 | std 线程 | Tokio |
|------|----------|-------|
| 启动开销 | 8KB 栈 | 几百字节 |
| 10 万并发 | ❌ OOM | ✅ 几百 MB |
| CPU 密集 | ✅ | ❌ |
| I/O 密集 | ❌（线程闲着） | ✅ |

## 5.17 一个"大并发"案例

```rust
use tokio::sync::Semaphore;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let sem = Arc::new(Semaphore::new(10));   // 最多同时 10 个
    let mut tasks = vec![];

    for i in 0..1000 {
        let sem = Arc::clone(&sem);
        tasks.push(tokio::spawn(async move {
            let _permit = sem.acquire().await.unwrap();
            // do work
            sleep(std::time::Duration::from_millis(100)).await;
            i
        }));
    }

    for t in tasks { t.await.unwrap(); }
}
```

> 1000 个任务，但**最多 10 个并发**——Semaphore 做限流。

## 5.18 对比其他语言

| 概念 | Rust Tokio | Go | Node.js |
|------|------------|-----|---------|
| Runtime | 显式 | 隐式（runtime） | V8 内置 |
| 协程 | task | goroutine | Promise |
| Channel | `tokio::sync::mpsc` | `chan` | EventEmitter |
| Mutex | `tokio::sync::Mutex` | `sync.Mutex` | — |
| 超时 | `select!` | `select` | `Promise.race` |
| 性能 | 极优 | 极优 | 单线程 |

---

## 🏋️ 本章小练习

**练习 5.1**：`tokio::spawn` 3 个任务并发跑（`sleep(100ms)`），`join!` 等它们，验证总耗时 ≈ 100ms。

**练习 5.2**：`tokio::time::timeout` 给一个 sleep 1s 的 future 加 100ms 超时，处理超时返回 `None`。

**练习 5.3**：`tokio::sync::mpsc`：5 个 producer 各发 10 条，1 个 consumer 收。

**练习 5.4**：`select!` race 两个 sleep，验证先到者胜。

**练习 5.5**（真实）：模拟"100 个 URL 抓取，限流 10 并发"——用 Semaphore。

**练习 5.6**（思考）：为什么 `tokio::sync::Mutex` 可重入，`std::sync::Mutex` 不可？

> 答：std 的 lock 是"持锁"语义，重入会 deadlock；tokio 的 lock 是"当前任务持锁"——同一任务重入 = 同一身份，不阻塞。

---

下一章：[06 · Send / Sync →](./06-send-sync.md)
