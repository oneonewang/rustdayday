# Exercise 05 · Tokio 实战

> 难度：⭐⭐⭐  涉及：第 5 章

## 任务

### 1) tokio::spawn 后台任务

```rust
#[tokio::main]
async fn main() {
    let h = tokio::spawn(async {
        sleep(Duration::from_millis(100)).await;
        42
    });
    println!("后台任务 = {}", h.await.unwrap());
}
```

### 2) tokio::sync::mpsc 多 producer

```rust
#[tokio::main]
async fn main() {
    let (tx, mut rx) = tokio::sync::mpsc::channel(100);
    for i in 0..5 {
        let tx = tx.clone();
        tokio::spawn(async move {
            tx.send(i).await.unwrap();
        });
    }
    drop(tx);
    while let Some(v) = rx.recv().await {
        println!("{v}");
    }
}
```

### 3) tokio::time::interval 周期任务

```rust
#[tokio::main]
async fn main() {
    let mut interval = tokio::time::interval(Duration::from_millis(100));
    for _ in 0..3 {
        interval.tick().await;
        println!("tick at {:?}", std::time::Instant::now());
    }
}
```

### 4) Semaphore 限流

```rust
use tokio::sync::Semaphore;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let sem = Arc::new(Semaphore::new(3));
    let mut handles = vec![];
    for i in 0..10 {
        let sem = Arc::clone(&sem);
        handles.push(tokio::spawn(async move {
            let _permit = sem.acquire().await.unwrap();
            sleep(Duration::from_millis(50)).await;
            println!("done {i}");
        }));
    }
    for h in handles { h.await.unwrap(); }
}
```

### 5) select! 模式

```rust
#[tokio::main]
async fn main() {
    tokio::select! {
        _ = sleep(Duration::from_millis(100)) => println!("超时"),
        v = async { 42 } => println!("得到 {v}"),
    }
}
```

### 6) 完整 demo：10 worker 处理 100 任务

```rust
async fn process(n: u32) -> u32 { n * n }

#[tokio::main]
async fn main() {
    // 通道发任务，10 worker 抢
    // 验证总和 = (1..=100).map(|n| n*n).sum()
}
```

## 验收

每题至少一个测试。

## 提示

- `tokio::spawn` 要求 future `Send + 'static`
- `tokio::sync::Semaphore::new(n)` 限流 n 个并发
- `interval` 自动 tick

## 进阶

写一个"速率限制"——每 100ms 最多 5 个请求通过：

> 提示：滑动窗口或 token bucket（简单版用 `tokio::time::interval` + `Semaphore::new(5)` 模拟）。

完成 → [ex06_send_sync](../ex06_send_sync)
