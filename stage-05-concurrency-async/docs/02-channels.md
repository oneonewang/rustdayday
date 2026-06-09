# 02 · 通道（Channels）

> **本章目标**：会用 `mpsc` 通道做"线程间消息传递"，理解 `Sender` / `Receiver` 的克隆与关闭语义。

## 2.1 什么是通道？

> **通道（channel）** = 线程间传递数据的"管道"。一个端发送、一个端接收。

**MPSC** = Multiple Producer, Single Consumer（多生产者单消费者）。`std::sync::mpsc`。

> "多生产者单消费者"——多个线程**发**，一个线程**收**。

## 2.2 第一个通道

```rust
use std::sync::mpsc;
use std::thread;

let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    let msg = String::from("hi");
    tx.send(msg).unwrap();
    // println!("{msg}");   // ❌ msg 已 move
});

let received = rx.recv().unwrap();
println!("收到: {received}");   // 收到: hi
```

> `send` 拿所有权，**接收方**才能拿——天然数据流。

## 2.3 `recv` vs `try_recv`

| 方法 | 行为 |
|------|------|
| `rx.recv()` | 阻塞等消息 |
| `rx.try_recv()` | 不阻塞，没消息返回 `Err(TryRecvError::Empty)` |

```rust
loop {
    match rx.try_recv() {
        Ok(msg)  => println!("got: {msg}"),
        Err(mpsc::TryRecvError::Empty)    => { /* 等会儿 */ }
        Err(mpsc::TryRecvError::Disconnected) => break,
    }
}
```

## 2.4 把 `Receiver` 当迭代器

```rust
for received in rx {
    println!("{received}");
    if received == "quit" { break; }
}
```

> `Receiver` 实现 `Iterator`——`None` 时**自动等**。**没有"空"的概念**——要么有消息、要么 sender 都 drop 了。

## 2.5 多生产者：克隆 `Sender`

```rust
let (tx, rx) = mpsc::channel();
let mut handles = vec![];

for i in 0..5 {
    let tx_clone = tx.clone();         // 克隆 sender
    let h = thread::spawn(move || {
        tx_clone.send(format!("消息 {i}")).unwrap();
    });
    handles.push(h);
}

drop(tx);                              // 主线程的 tx 不再发，drop

for h in handles { h.join().unwrap(); }

for msg in rx {
    println!("{msg}");
}
```

> 收到 5 条消息，**顺序不确定**（线程调度）。

## 2.6 通道关闭语义

- **所有 `Sender` 都 drop 后**，`Receiver` 收到 `Err(RecvError::Disconnected)`（或者 `for` 循环结束）
- `Receiver` 早 drop → `Sender::send` 返回 `Err(SendError)`

```rust
let (tx, rx) = mpsc::channel();
drop(rx);
let r = tx.send(1);
assert!(r.is_err());     // 没有接收方了
```

## 2.7 同步通道：`sync_channel`

`mpsc::channel` 是**异步**的——`send` 不会阻塞。`sync_channel(bound)` 会**等**接收方：

```rust
let (tx, rx) = mpsc::sync_channel(2);   // 缓冲 2

tx.send(1).unwrap();
tx.send(2).unwrap();
// tx.send(3) 会阻塞——直到接收方 recv
```

> 适合"背压"：发送方不能无限堆数据。

## 2.8 一个完整例子：工作池

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    let mut workers = vec![];

    for id in 0..3 {
        // 每个 worker 各自 rx.clone
        // 但 mpsc 只允许一个 rx...
        // 解决：见共享状态章节
    }
    // ...
}
```

> ⚠️ `mpsc` 是**单消费者**——多个 worker 不能各 `clone(rx)`。要多个 worker 收，得用 `Arc<Mutex<Receiver<T>>>` 或换 tokio 的 `mpsc`。

## 2.9 消息类型：常用模式

```rust
enum WorkMsg {
    Job(u32),
    Quit,
}

// worker 收到 Quit 就退出
for msg in rx {
    match msg {
        WorkMsg::Job(n)  => process(n),
        WorkMsg::Quit    => break,
    }
}
```

## 2.10 性能

| 通道 | 异步 (`mpsc::channel`) | 同步 (`sync_channel`) |
|------|------------------------|----------------------|
| send 阻塞 | 不阻塞（除非无 receiver） | 缓冲满阻塞 |
| 缓冲 | 无限 | 有限 |
| 适用 | 大多数 | 背压场景 |
| 性能 | 一次原子 + 锁 | 一次原子 + 条件变量 |

> 单次 `send`/`recv` 在 **几百纳秒**量级——比 OS 锁还快。

## 2.11 真实场景：日志聚合器

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    // 3 个 worker 都发日志
    for i in 0..3 {
        let tx = tx.clone();
        thread::spawn(move || {
            for j in 0..5 {
                tx.send(format!("[worker {i}] message {j}")).unwrap();
                thread::sleep(std::time::Duration::from_millis(10));
            }
        });
    }
    drop(tx);

    // 一个收集者
    for msg in rx {
        println!("[aggregator] {msg}");
    }
}
```

## 2.12 对比其他语言

| 概念 | Rust `mpsc` | Go `chan` | Java `BlockingQueue` |
|------|-------------|-----------|----------------------|
| 多发 | ✅ clone `Sender` | ✅ | ✅ |
| 单收 | ✅ | ✅ | ✅ |
| 多收 | ❌（mpsc）/ 用共享 | ✅ | ❌ |
| 同步/异步 | 两个 | 一个 | 一个 |
| close 语义 | sender drop | `close(ch)` | 队列满 |
| 缓冲 | sync_channel(n) | `make(chan, n)` | capacity |

> 💡 Go 通道更通用（多收），Rust 把"多收"留给了 `Arc<Mutex<Receiver>>` 或 `tokio::sync::mpsc`。

---

## 🏋️ 本章小练习

**练习 2.1**：主线程生成 1..=10，每秒发一个；子线程收到就打印。

**练习 2.2**：3 个 worker 线程各发 10 条 "hello from {id}"；主线程收，按收到顺序打印。

**练习 2.3**：`sync_channel(3)` 演示：发送 5 条 + 接收 1 条 + 发送 1 条 + 接收 1 条 + ...，看哪一步阻塞。

**练习 2.4**（真实场景）：用通道实现"主线程接收，3 个 worker 处理数字平方并返回"——**注意** mpsc 是单收，要每个 worker 自己的 receiver。

> 提示：用 `Arc<Mutex<Receiver<T>>>` 给 worker 共享 receiver；或用 tokio 的 mpsc。

---

下一章：[03 · 共享状态（Shared State）→](./03-shared-state.md)
