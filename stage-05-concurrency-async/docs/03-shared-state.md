# 03 · 共享状态（Shared State）

> **本章目标**：会用 `Arc<Mutex<T>>` / `Arc<RwLock<T>>` / `OnceLock` 在多线程间共享数据，理解"锁"和"毒化"。

## 3.1 共享 vs 消息传递

两种并发模型：

| 模型 | 思路 | 工具 |
|------|------|------|
| 消息传递 | "不要共享内存，传递消息" | `mpsc::channel` |
| 共享状态 | "共享内存，但用锁保护" | `Arc<Mutex<T>>` |

> 都对，**根据场景选**。Rust 两者都做得很好。

## 3.2 `Mutex<T>`：互斥锁

```rust
use std::sync::Mutex;

let m = Mutex::new(5);
{
    let mut num = m.lock().unwrap();
    *num = 10;
}   // 锁在这里释放（guard drop）

println!("{}", m.lock().unwrap());   // 10
```

> `lock()` 返回 `LockResult<MutexGuard<T>>`——**`MutexGuard`** 离开作用域自动释放锁。

> ⚠️ **忘 drop guard** → 死锁！

```rust
let mut g1 = m.lock().unwrap();
let mut g2 = m.lock().unwrap();   // 💥 deadlock：g1 还没 drop
```

## 3.3 `Arc<Mutex<T>>` 跨线程共享

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let counter = Arc::clone(&counter);
    let h = thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
    });
    handles.push(h);
}

for h in handles { h.join().unwrap(); }

println!("count = {}", counter.lock().unwrap());   // 10
```

> `Arc` 共享所有权 + `Mutex` 互斥访问 = **跨线程共享 + 可变**的标准模式。

## 3.4 `Mutex` 毒化（Poisoning）

```rust
let m = Arc::new(Mutex::new(0));
let m2 = Arc::clone(&m);

let h = thread::spawn(move || {
    let mut g = m2.lock().unwrap();
    *g = 1;
    panic!("boom");          // 持锁线程 panic，锁中毒
});
h.join().unwrap_err();

let r = m.lock();
assert!(r.is_err());        // 拿到 Err(PoisonError)
```

> **毒化 = 持锁线程 panic → 锁进入"毒"状态，其他 lock 拿到 Err**。
>
> 修法：用 `.lock().unwrap_or_else(|e| e.into_inner())` 拿 inner，或者干脆 `.unwrap()` 接受 panic 时的状态。

## 3.5 `RwLock<T>`：多读单写

```rust
use std::sync::RwLock;

let lock = RwLock::new(5);

// 多个读
{
    let r1 = lock.read().unwrap();
    let r2 = lock.read().unwrap();
    println!("{} {}", *r1, *r2);
}   // 全部 drop 后，写者才能拿

{
    let mut w = lock.write().unwrap();
    *w = 10;
}
```

> **读多写少**时 `RwLock` 比 `Mutex` 快很多。
>
> **写多**时 `RwLock` 比 `Mutex` 慢（更复杂的内部状态机）——直接用 `Mutex`。

## 3.6 `Mutex<T>` vs `RwLock<T>`

| 维度 | Mutex | RwLock |
|------|-------|--------|
| 读写互斥 | 读写互斥 | 读读不互斥 |
| 读多写少 | 一般 | 优秀 |
| 写多 | 优秀 | 一般（开销大） |
| 毒化 | ✅ | ✅ |
| 死锁 | 简单（一个 guard） | 复杂（多个 read guard） |

## 3.7 `OnceLock<T>` / `LazyLock<T>`：一次性初始化

```rust
use std::sync::OnceLock;

static CONFIG: OnceLock<String> = OnceLock::new();

fn get_config() -> &'static String {
    CONFIG.get_or_init(|| {
        println!("首次初始化（只跑一次）");
        "computed value".to_string()
    })
}

fn main() {
    println!("{}", get_config());
    println!("{}", get_config());
    println!("{}", get_config");
}
```

> 输出"computed value" 3 次，但 "首次初始化（只跑一次）" 只打印 1 次。

**`LazyLock`**（Rust 1.80+）更直观：

```rust
use std::sync::LazyLock;

static CONFIG: LazyLock<String> = LazyLock::new(|| "computed value".to_string());

fn main() {
    println!("{}", *CONFIG);
}
```

## 3.8 用 `Arc<Mutex<Receiver>>` 做"多 worker"

```rust
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

let (tx, rx) = mpsc::channel();
let rx = Arc::new(Mutex::new(rx));

let mut handles = vec![];
for id in 0..3 {
    let rx = Arc::clone(&rx);
    let h = thread::spawn(move || loop {
        let msg = rx.lock().unwrap().recv().unwrap();
        match msg {
            WorkMsg::Job(n) => println!("worker {id} 处理 {n}"),
            WorkMsg::Quit => break,
        }
    });
    handles.push(h);
}

for i in 0..10 {
    tx.send(WorkMsg::Job(i)).unwrap();
}
for _ in 0..3 {
    tx.send(WorkMsg::Quit).unwrap();
}
for h in handles { h.join().unwrap(); }

enum WorkMsg { Job(u32), Quit }
```

> ⚠️ 锁竞争很严重——`tokio::mpsc` 是更好的选择。

## 3.9 死锁模式

**模式 1：持锁等锁**

```rust
let a = Mutex::new(1);
let b = Mutex::new(2);

// 线程 1
let _a = a.lock().unwrap();
let _b = b.lock().unwrap();   // 线程 2 持 b，在等 a

// 线程 2
let _b = b.lock().unwrap();
let _a = a.lock().unwrap();   // 死锁
```

**修法**：固定加锁顺序（永远先 a 再 b）。

**模式 2：`Arc<Mutex<()>>` 当信号量**

```rust
let lock = Arc::new(Mutex::new(()));
let _g = lock.lock().unwrap();
let _g2 = lock.lock().unwrap();   // 💥 同一线程持锁，第二次等
```

> ⚠️ `std::sync::Mutex` **不可重入**——同一线程第二次 lock 会 deadlock。`tokio::sync::Mutex` 默认可重入。

## 3.10 一个反模式：粒度太粗

```rust
// ❌ 一个大锁保护一切
let state = Arc::new(Mutex::new(AppState::new()));

// 每次操作都锁整个 state
state.lock().unwrap().users.insert(...);
state.lock().unwrap().orders.push(...);

// ✅ 拆开
let users = Arc::new(Mutex::new(vec![]));
let orders = Arc::new(Mutex::new(vec![]));
```

> **锁粒度越细，争用越少**——但代码越复杂。要平衡。

## 3.11 性能

| 锁 | 大约成本 |
|----|----------|
| `Mutex::lock`（无争用） | 几十纳秒 |
| `Mutex::lock`（高争用） | 几微秒 + OS 调度 |
| `RwLock::read`（无争用） | 几纳秒 |
| `RwLock::write` | 同 Mutex |

> **没有争用时**锁开销可以忽略——比 GC 暂停小几个数量级。

## 3.12 真实场景：连接池

```rust
use std::sync::{Arc, Mutex};

struct Pool<T> {
    items: Mutex<Vec<T>>,
}

impl<T> Pool<T> {
    fn new() -> Self { Self { items: Mutex::new(vec![]) } }
    fn acquire(&self) -> Option<T> {
        self.items.lock().unwrap().pop()
    }
    fn release(&self, item: T) {
        self.items.lock().unwrap().push(item);
    }
}
```

## 3.13 决策树

```
要共享？
├── 否 → 消息传递
└── 是
    ├── 只初始化一次？ → OnceLock / LazyLock
    ├── 读多写少？ → Arc<RwLock<T>>
    └── 通用 → Arc<Mutex<T>>
```

## 3.14 对比其他语言

| 概念 | Rust | Go | Java |
|------|------|-----|------|
| 互斥 | `Mutex` | `sync.Mutex` | `synchronized` |
| 读写锁 | `RwLock` | `sync.RWMutex` | `ReadWriteLock` |
| Once | `OnceLock` | `sync.Once` | 静态初始化 |
| 死锁时行为 | poison | panic | 异常 |

---

## 🏋️ 本章小练习

**练习 3.1**：`Arc<Mutex<i32>>` 多线程计数：10 个线程各 `+1`，最终 = 10。

**练习 3.2**：`Arc<RwLock<Vec<i32>>>`：3 个读线程各打印 5 次；1 个写线程每 100ms 追加一个随机数。

**练习 3.3**：`OnceLock<i32>` 存"全局配置"，第一个读到时初始化。

**练习 3.4**：演示死锁：两个 `Mutex<i32>` 互相加锁，验证固定顺序能修。

**练习 3.5**（真实场景）：实现一个"任务队列"——多 producer 入队，多 worker 出队处理。

> 提示：worker 用 `Arc<Mutex<Receiver<Task>>>` 共享 receiver；或用 `crossbeam_channel`（更强大）。

---

下一章：[04 · async / await 基础 →](./04-async-await.md)
