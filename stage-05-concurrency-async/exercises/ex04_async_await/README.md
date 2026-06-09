# Exercise 04 · async / await 基础

> 难度：⭐⭐  涉及：第 4 章

## 任务

### 1) 第一个 async 函数

```rust
async fn add(a: i32, b: i32) -> i32 { a + b }

#[tokio::main]
async fn main() {
    let v = add(2, 3).await;
    println!("v = {v}");  // 5
}
```

### 2) tokio::join! 并发

```rust
use std::time::Duration;
use tokio::time::sleep;

async fn slow(name: &str, ms: u64) -> String {
    sleep(Duration::from_millis(ms)).await;
    format!("{name} done")
}

#[tokio::main]
async fn main() {
    let start = std::time::Instant::now();
    let (a, b, c) = tokio::join!(
        slow("A", 100),
        slow("B", 100),
        slow("C", 100),
    );
    println!("{a}, {b}, {c}, 耗时 {:?}", start.elapsed());
    // 应该 ≈ 100ms（不是 300ms）
}
```

### 3) tokio::time::timeout

```rust
async fn slow_op() -> u32 {
    sleep(Duration::from_secs(1)).await;
    42
}

async fn with_timeout() -> Option<u32> {
    // 200ms 超时，超时返回 None
}
```

### 4) select! race

```rust
async fn first_done() -> u32 {
    tokio::select! {
        v = async { 100 } => v,
        _ = sleep(Duration::from_millis(50)) => 0,
    }
}
```

### 5) 错误传播

```rust
async fn fetch_id(id: u32) -> Result<String, String> {
    if id == 0 { return Err("id 0 不合法".to_string()); }
    Ok(format!("user-{id}"))
}

#[tokio::main]
async fn main() {
    let r = fetch_id(0).await;
    println!("{r:?}");  // Err(...)
}
```

## 验收

每题至少一个测试。

## 提示

- `#[tokio::main]` 默认是 multi-thread runtime
- `tokio::join!` 是**同一任务内**并发
- `tokio::time::timeout(dur, fut).await` 返回 `Result<T, Elapsed>`

## 进阶

写一个 `retry`：

```rust
async fn retry<F, Fut, T, E>(f: F, n: usize) -> Result<T, E>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
{
    // 调 f() 失败最多重试 n 次
}
```

完成 → [ex05_tokio](../ex05_tokio)
