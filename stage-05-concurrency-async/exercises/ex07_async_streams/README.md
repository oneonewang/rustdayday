# Exercise 07 · async 流与并发抓取

> 难度：⭐⭐⭐  涉及：第 4 + 5 章

## 任务

### 1) 实现一个简单 async Stream

```rust
use futures::stream::{self, StreamExt};

#[tokio::main]
async fn main() {
    let mut s = stream::iter(1..=5);
    while let Some(v) = s.next().await {
        println!("{v}");
    }
}
```

### 2) 用 tokio::time::interval 当流

```rust
use tokio::time::{interval, Duration};
use futures::stream::StreamExt;

#[tokio::main]
async fn main() {
    let mut tick = interval(Duration::from_millis(100));
    for _ in 0..3 {
        tick.next().await;
        println!("tick");
    }
}
```

### 3) 并发抓取模拟

```rust
async fn fake_fetch(url: &str) -> (String, usize) {
    sleep(Duration::from_millis(100)).await;
    (url.to_string(), url.len() * 10)
}

#[tokio::main]
async fn main() {
    let urls = vec!["a.com", "b.com", "c.com", "d.com", "e.com"];
    let start = std::time::Instant::now();
    
    let results: Vec<_> = futures::future::join_all(
        urls.iter().map(|&u| fake_fetch(u))
    ).await;
    
    println!("{:?}", results);
    println!("耗时 {:?}", start.elapsed());
    // 验证 ≈ 100ms（不是 500ms）
}
```

### 4) tokio::mpsc 当流

```rust
use tokio::sync::mpsc;
use futures::stream::StreamExt;

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel(10);
    tokio::spawn(async move {
        for i in 0..5 {
            tx.send(i).await.unwrap();
        }
    });
    
    let mut s = rx;  // Receiver 实现 Stream
    while let Some(v) = s.next().await {
        println!("{v}");
    }
}
```

### 5) 自定义 Stream

```rust
use futures::stream::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};

struct CounterStream { count: u32, max: u32 }

impl Stream for CounterStream {
    type Item = u32;
    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Option<Self::Item>> {
        // 计数到 max 返回 None
    }
}
```

### 6) 限流 10 worker 抢 100 任务

```rust
use tokio::sync::Semaphore;

#[tokio::main]
async fn main() {
    // 100 个任务发到 mpsc
    // 10 worker 用 Semaphore(10) 限流抢
    // 验证 100 任务都能完成
}
```

## 验收

每题验证输出。

## 提示

- `futures::stream::StreamExt::next().await` 当 `Stream`
- `tokio::sync::mpsc::Receiver` 实现了 `Stream`
- 自定义 Stream 要 `Pin<&mut Self>`

## 进阶

写一个 `buffer_unordered(n)`：每 n 个 future 并发跑，返回结果流。

> 提示：futures crate 有 `buffer_unordered`，先看用法，再自己实现简化版。

完成 → [project-05-async-scraper](../project-05-async-scraper) 见！
