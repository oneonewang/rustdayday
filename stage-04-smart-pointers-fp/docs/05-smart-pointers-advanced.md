# 05 · 智能指针深入：Arc / Mutex / Weak / Cow

> **本章目标**：会用 `Arc<Mutex<T>>` 处理跨线程共享，掌握 `Weak` 打破循环，学会 `Cow` 避免不必要分配。

## 5.1 速查表

| 类型 | 作用 | 线程安全？ | 性能 |
|------|------|-----------|------|
| `Box<T>` | 唯一所有者 | ✅ | 零开销 |
| `Rc<T>` | 共享 + 不可变 | ❌ | 引用计数 |
| `Arc<T>` | 共享 + 不可变 | ✅ | 原子引用计数（较慢） |
| `RefCell<T>` | 唯一 + 内部可变 | ❌ | 运行时检查 |
| `Mutex<T>` | 唯一 + 内部可变 | ✅ | OS 锁 |
| `RwLock<T>` | 多读单写 | ✅ | 读多写少更优 |
| `OnceLock<T>` | 一次性初始化 | ✅ | 一次成本 |
| `Weak<T>` | 弱引用 | ✅ | 不计数 |

## 5.2 `Arc<T>`：跨线程共享只读

```rust
use std::sync::Arc;
use std::thread;

let data = Arc::new(vec![1, 2, 3]);
let mut handles = vec![];

for i in 0..3 {
    let data = Arc::clone(&data);
    let h = thread::spawn(move || {
        println!("线程 {i}: {:?}", data);
    });
    handles.push(h);
}

for h in handles { h.join().unwrap(); }
```

> `Arc::clone` 增加**原子**引用计数，**比 `Rc::clone` 慢**，但能跨线程。

## 5.3 `Mutex<T>`：跨线程互斥

```rust
use std::sync::Mutex;
use std::thread;

let counter = Mutex::new(0);
let mut handles = vec![];

for _ in 0..10 {
    let h = thread::spawn(|| {
        let mut num = counter.lock().unwrap();
        *num += 1;
    });
    handles.push(h);
}
for h in handles { h.join().unwrap(); }

println!("counter = {}", counter.lock().unwrap());   // 10
```

> ⚠️ `lock()` 拿 `Result`——**中毒**时（持锁线程 panic）返回 `Err`。

## 5.4 ⭐ 经典组合：`Arc<Mutex<T>>`

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

println!("counter = {}", *counter.lock().unwrap());   // 10
```

> `Arc` 共享 + `Mutex` 互斥 = 跨线程的"多 owner 可改"。

## 5.5 `RwLock<T>`：多读单写

```rust
use std::sync::RwLock;
use std::thread;

let data = RwLock::new(vec![1, 2, 3]);
let mut handles = vec![];

// 多个读线程
for i in 0..3 {
    let h = thread::spawn(move || {
        let v = data.read().unwrap();
        println!("reader {i}: {:?}", *v);
    });
    handles.push(h);
}

// 一个写线程
let h = thread::spawn(|| {
    let mut v = data.write().unwrap();
    v.push(4);
});
handles.push(h);

for h in handles { h.join().unwrap(); }
```

> **性能**：`RwLock` 在**读远多于写**时优于 `Mutex`。

## 5.6 `Weak<T>`：打破循环引用

```rust
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Option<Weak<Node>>>,        // Weak 打破循环
    children: RefCell<Vec<Rc<Node>>>,
}

let leaf = Rc::new(Node {
    value: 3,
    parent: RefCell::new(None),
    children: RefCell::new(vec![]),
});

let branch = Rc::new(Node {
    value: 5,
    parent: RefCell::new(None),
    children: RefCell::new(vec![Rc::clone(&leaf)]),
});

*leaf.parent.borrow_mut() = Some(Rc::downgrade(&branch));
// branch 强 → leaf；leaf 弱 → branch；不循环
```

**`Weak` 用法**：

```rust
fn show_parent(node: &Node) {
    let parent = node.parent.borrow();
    if let Some(weak) = parent.as_ref() {
        match weak.upgrade() {
            Some(p) => println!("parent = {}", p.value),
            None    => println!("parent 已 drop"),
        }
    }
}
```

## 5.7 `Cow<T>`：写时复制

> `Cow` = Clone-on-Write。**可能借用、可能拥有**——只在真要改时才复制。

```rust
use std::borrow::Cow;

fn maybe_trim(s: &str) -> Cow<str> {
    if s.trim().len() == s.len() {
        Cow::Borrowed(s)          // 不需改 → 借用
    } else {
        Cow::Owned(s.trim().to_string())  // 要改 → 复制
    }
}

let s1 = "hello";
let s2 = "  hello  ";

println!("{}", maybe_trim(s1));   // 借用，零分配
println!("{}", maybe_trim(s2));   // 复制
```

> **典型场景**：函数可能要 / 不要修改输入时——用 `Cow` 让调用方决定。

## 5.8 `Pin<T>`：固定栈/堆上的值

```rust
use std::pin::Pin;
use std::marker::PhantomPinned;

struct SelfRef {
    ptr: *const Self,
    _marker: PhantomPinned,
}

let s = SelfRef { ptr: std::ptr::null(), _marker: PhantomPinned };
let pinned: Pin<Box<SelfRef>> = Box::pin(s);
// pinned 不能 move，确保 ptr 指向的地址不会变
```

> `Pin` 主要是**异步** + **自引用结构**（详见 Stage 5）。

## 5.9 智能指针选择决策树

```
要共享吗？
├── 否 → Box<T>
└── 是
    ├── 要改吗？
    │   ├── 否
    │   │   ├── 跨线程？ → Arc<T>
    │   │   └── 单线程？ → Rc<T>
    │   └── 是
    │       ├── 跨线程？ → Arc<Mutex<T>> / Arc<RwLock<T>>
    │       └── 单线程？ → Rc<RefCell<T>>
    └── 父子关系、可能循环？→ 用 Weak 打破
```

## 5.10 性能：原子 vs 锁

| 操作 | 成本 |
|------|------|
| `Arc::clone` | 原子 +1（CPU 几个指令） |
| `Mutex::lock` | 一次系统调用（**几微秒**） |
| `RwLock::read` | 原子（**几纳秒**） |
| `RwLock::write` | 同 `Mutex` |

> ⚠️ **锁很贵**——能不用就不用。`Arc<Mutex<T>>` 是"必须有共享 + 可变"时才用。

## 5.11 一个反模式：过度加锁

```rust
// ❌ 锁持有太久
let v = mutex.lock().unwrap();
let result = expensive_computation(&v);   // 持锁下算 5 秒
drop(mutex);

// ✅ 取出数据后再算
let v = mutex.lock().unwrap().clone();   // 锁只持有一瞬间
let result = expensive_computation(&v);
```

## 5.12 真实场景：多 worker 计数

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn parallel_count(items: Vec<i32>) -> i32 {
    let sum = Arc::new(Mutex::new(0i64));
    let chunk_size = items.len() / 4 + 1;
    let mut handles = vec![];

    for chunk in items.chunks(chunk_size) {
        let sum = Arc::clone(&sum);
        let h = thread::spawn(move || {
            let local: i64 = chunk.iter().map(|&x| x as i64).sum();
            let mut s = sum.lock().unwrap();
            *s += local;
        });
        handles.push(h);
    }

    for h in handles { h.join().unwrap(); }

    let result = *sum.lock().unwrap();
    result as i32
}

fn main() {
    let v: Vec<i32> = (1..=1_000_000).collect();
    println!("sum = {}", parallel_count(v));
}
```

## 5.13 完整对照表

| 想做的 | 类型 | 例子 |
|--------|------|------|
| 唯一堆 | `Box<T>` | 递归 enum |
| 单线程共享 | `Rc<T>` | 不可变树 |
| 单线程共享 + 可改 | `Rc<RefCell<T>>` | 缓存 |
| 跨线程共享 | `Arc<T>` | 配置 |
| 跨线程共享 + 可改 | `Arc<Mutex<T>>` | 计数器 |
| 读多写少跨线程 | `Arc<RwLock<T>>` | 路由表 |
| 一次性延迟初始化 | `OnceLock<T>` / `LazyLock<T>` | 全局配置 |
| 打破循环 | `Weak<T>` | 父子节点 |
| 可能借用可能拥有 | `Cow<'a, T>` | 字符串裁剪 |
| 自引用结构 | `Pin<P<T>>` | 异步 future |

## 5.14 对比其他语言

| 概念 | Rust | C++ | Java | Go |
|------|------|-----|------|-----|
| 共享只读 | `Arc<T>` | `shared_ptr<const T>` | `final` 字段 | 普通变量 |
| 共享可变 | `Arc<Mutex<T>>` | `shared_ptr<T>` + mutex | `synchronized` | `sync.Mutex` |
| 内部可变 | `RefCell<T>` | `mutable` | — | — |
| 弱引用 | `Weak<T>` | `weak_ptr` | `WeakReference` | — |
| 读多写少 | `RwLock<T>` | `shared_mutex` | `ReadWriteLock` | `sync.RWMutex` |
| Once init | `OnceLock<T>` | `std::call_once` | 静态初始化 | `sync.Once` |

---

## 🏋️ 本章小练习

**练习 5.1**：`Arc<Mutex<i32>>` 跨线程计数（10 个线程，每个 +1，最终 = 10）。

**练习 5.2**：用 `Cow<str>` 写一个 `trim_if(s: &str, cond: fn(char) -> bool) -> Cow<str>`——只在确实需要裁剪时分配新 String。

**练习 5.3**：用 `Weak` 写一个"父子节点"结构（无循环）。

**练习 5.4**：用 `Arc<RwLock<T>>` 写一个简单的"配置中心"——多个读者并发读，偶发更新。

**练习 5.5**（真实）：把 Stage 1 的猜数字游戏改成"多线程版"：一个线程读输入，一个线程更新状态。

---

下一章：[06 · 阶段复习 →](./06-stage-review.md)
