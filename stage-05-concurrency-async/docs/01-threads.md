# 01 · 线程（Threads）

> **本章目标**：会用 `std::thread::spawn` 起线程、收 `JoinHandle`、跨线程传值，理解为什么大多数情况下要"tokio 而不是裸线程"。

## 1.1 什么是线程？

**线程** = 操作系统调度的最小单位。同一进程内多线程**共享内存**（堆、代码段），但有独立栈。

> 进程：隔离的地址空间（一个进程崩了不影响别的）
> 线程：共享地址空间（一个线程崩了，全进程都死）

## 1.2 第一个线程

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("spawned thread: {i}");
            thread::sleep(Duration::from_millis(100));
        }
    });

    for i in 1..=3 {
        println!("main thread: {i}");
        thread::sleep(Duration::from_millis(200));
    }

    handle.join().unwrap();   // 等子线程结束
}
```

> 父子线程**几乎同时**跑——`join()` 是阻塞的，等子线程结束才往下走。

## 1.3 `move` 闭包：把数据移进新线程

```rust
let v = vec![1, 2, 3];
let handle = thread::spawn(move || {
    println!("v = {:?}", v);
});
handle.join().unwrap();
```

> **必须** `move`——新线程可能活得比当前作用域久，借用会悬垂。

## 1.4 跨线程返回值

```rust
let handle = thread::spawn(|| {
    let sum: i32 = (1..=100).sum();
    sum
});

let result = handle.join().unwrap();
println!("sum = {result}");   // 5050
```

> `JoinHandle::join()` 返回 `Result<T, _>`——子线程 panic 会得到 `Err`。

## 1.5 多个线程

```rust
let mut handles = vec![];
for i in 0..10 {
    let h = thread::spawn(move || {
        println!("线程 {i} 开始");
        thread::sleep(Duration::from_millis(50));
        println!("线程 {i} 结束");
    });
    handles.push(h);
}

for h in handles {
    h.join().unwrap();
}
```

## 1.6 线程池？自己写还是用 crate？

裸 `thread::spawn` 的问题：

- 每来一个任务就起一个线程 → 上百个任务 = 几百个线程
- 线程切换的开销不小
- 难以复用

> **生产代码几乎都用线程池**：Rayon（CPU bound）/ Tokio（I/O bound）/ 自定义 pool。

```rust
use rayon::prelude::*;

let sum: i32 = (0..1_000_000).into_par_iter().sum();
// 自动把迭代器分配到多个线程
```

## 1.7 `thread::Builder`：自定义线程属性

```rust
let handle = thread::Builder::new()
    .name("worker-1".to_string())
    .stack_size(4 * 1024 * 1024)         // 4MB
    .spawn(|| {
        println!("我是 {}", thread::current().name().unwrap());
    })
    .unwrap();
handle.join().unwrap();
```

## 1.8 闭包不是 `Send` 怎么办？

```rust
let rc = std::rc::Rc::new(5);
let h = thread::spawn(move || {
    println!("{rc}");      // ❌ Rc 不能 Send
});
```

> 编译错：`Rc<i32> cannot be sent between threads safely`
>
> 修法：换成 `Arc`。

## 1.9 scoped threads（Rust 1.63+）

普通 `spawn` 必须 `move`——闭包无法借用外部数据。**Scoped threads** 解决了这个问题：

```rust
use std::thread;

let mut list = vec![1, 2, 3];

// 普通 spawn：必须 move，list 之后不可用
// let h = thread::spawn(move || println!("{:?}", list));

// scoped：可以借！
thread::scope(|s| {
    s.spawn(|| println!("借 list = {:?}", list));
    s.spawn(|| println!("借 list 长度 = {}", list.len()));
});

// 这里 list 还活着（所有 scoped 线程在 scope 结束前 join 完）
println!("scope 后 list = {:?}", list);
```

> `thread::scope` 保证：**scope 块结束前所有线程 join 完**——所以借用安全。

## 1.10 线程间共享数据：两种模式

| 模式 | 思路 | 工具 |
|------|------|------|
| **消息传递** | "不要共享内存，传递消息" | `mpsc::channel` |
| **共享状态** | "共享内存，但用锁保护" | `Arc<Mutex<T>>` |

> 两种都能解决问题，**Go 偏爱前者、Rust 两者都用**。

## 1.11 性能：什么时候用线程

- **CPU bound**（计算密集）：**用 rayon**，把工作分到几个核心（= CPU 核数）
- **I/O bound**（网络 / 磁盘）：**用 tokio** 异步，能撑几万并发
- **偶尔**一个后台任务：裸 `thread::spawn` 也行

| | 裸线程 | rayon | tokio |
|---|--------|-------|-------|
| 适合 | 后台任务 | CPU 密集 | I/O 密集 |
| 启动开销 | 高 | 中 | 极低 |
| 并发数 | 几十~几百 | CPU 核数 | 几万+ |
| 通信 | `Arc<Mutex>` / `mpsc` | `fold` / `reduce` | `mpsc` / `oneshot` |

## 1.12 常见坑

```rust
// ❌ 闭包借用外部数据，且没等线程结束就用回了
let v = vec![1, 2, 3];
let h = thread::spawn(|| println!("{:?}", v));
v.push(4);          // ❌ 借出后改，编译错
h.join().unwrap();
```

```rust
// ❌ 子线程 panic
let h = thread::spawn(|| panic!("boom"));
let r = h.join();
assert!(r.is_err());     // r: Err(Box<Any>)
```

## 1.13 对比其他语言

| 概念 | Rust | C++ | Java | Go |
|------|------|-----|------|-----|
| 起线程 | `thread::spawn` | `std::thread` | `new Thread().start()` | `go func()` |
| 共享 | `Arc<Mutex<T>>` | `shared_ptr<mutex<T>>` | `synchronized` | `sync.Mutex` |
| 消息 | `mpsc::channel` | — | `BlockingQueue` | `chan` |
| 池 | rayon / tokio | TBB | `ExecutorService` | runtime 自带 |
| panic | `Result::Err` | 异常 | 异常 | panic |
| scoped | `thread::scope` | C++26 | ❌ | ❌ |

---

## 🏋️ 本章小练习

**练习 1.1**：用线程并行计算 `(1..=1_000_000).sum()`，分到 4 个线程，验证结果 = 500000500000。

**练习 1.2**：写一个"计数器"线程：每 100ms 打印 `count = N`，主线程 1 秒后 `handle.join()` 让它退出。

**练习 1.3**：用 `thread::scope` 并行打印 `vec!["a", "b", "c", "d"]` 每个元素——子线程**借用** vec。

**练习 1.4**（思考）：为什么 `thread::spawn` 必须 `move`，而 `thread::scope` 不需要？

> 答：scope 保证 join 在 scope 块结束前——借用有效；普通 spawn 没有这个保证。

---

下一章：[02 · 通道（Channels）→](./02-channels.md)
