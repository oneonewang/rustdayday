# Exercise 02 · 通道

> 难度：⭐  涉及：第 2 章

## 任务

### 1) 单发送单接收

```rust
fn one_message() -> String {
    // 起一个线程发 "hello"，主线程接收
}
```

### 2) 多生产者

```rust
fn multi_producer(n: usize) -> Vec<i32> {
    // 3 个 worker 各发 n 条 1..=n；主线程收，返回总和
    // 验证总和 = 3 * (1..=n).sum()
}
```

### 3) 用 Receiver 当迭代器

```rust
fn collect_until_quit() -> Vec<String> {
    // 用 for msg in rx 循环，直到收到 "quit"
}
```

### 4) sync_channel 背压

```rust
fn sync_producer_consumer() {
    // sync_channel(2) 演示：发送 5 条，每发 1 条前先让 consumer 收
    // 验证缓冲只有 2，发送方第 3 条会阻塞
}
```

### 5) close 语义

```rust
fn close_demo() {
    // drop(rx)，再 tx.send 验证 Err
    // drop(tx)，再 rx.recv 验证 Err
}
```

## 验收

每个任务至少一个测试。

## 提示

- `tx.clone()` 增加生产者
- `drop(tx)` 后 `rx.recv()` 返回 `Err(RecvError::Disconnected)`
- `sync_channel(bound)` 第 bound+1 次 send 阻塞

## 进阶

写一个简单的"消息总线"：

```rust
struct MessageBus { tx: mpsc::Sender<String> }
impl MessageBus {
    fn new() -> (Self, mpsc::Receiver<String>) { ... }
    fn publish(&self, msg: &str) { ... }
}
```

完成 → [ex03_shared_state](../ex03_shared_state)
