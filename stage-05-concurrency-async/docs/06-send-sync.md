# 06 · Send / Sync

> **本章目标**：理解两个 marker trait `Send` / `Sync`，知道为什么有些东西**不能**跨线程，能看懂 "future cannot be sent between threads safely"。

## 6.1 `Send` / `Sync` 是什么？

| Trait | 含义 |
|-------|------|
| `Send` | 类型的**值**可以**移动**到另一个线程 |
| `Sync` | 类型的**`&T` 引用**可以**共享**到另一个线程 |

> - `T: Send` 意味着可以把 `T` 跨线程传（move）
> - `T: Sync` 意味着 `&T` 可以同时在多个线程用
> - `T: Sync` 当且仅当 `&T: Send`（定义）

它们都是 **marker trait**——没有方法。

## 6.2 哪些类型 Send / Sync？

| 类型 | Send | Sync | 为什么 |
|------|------|------|--------|
| `i32` / `bool` / `f64` | ✅ | ✅ | 全是 Copy + 原子读写 |
| `String` | ✅ | ✅ | 内部 ptr 走 `Arc` 等机制 |
| `Vec<T>` (T: Send) | ✅ | ✅ | 堆指针可跨线程 |
| `Mutex<T>` (T: Send) | ✅ | ✅ | 锁内保护 |
| `Rc<T>` | ❌ | ❌ | 引用计数非原子 |
| `Arc<T>` (T: Send + Sync) | ✅ | ✅ | 原子引用计数 |
| `RefCell<T>` | ✅ | ❌ | 借用检查非线程安全 |
| `*const T` / `*mut T` | ❌ | ❌ | 裸指针 |
| `Cell<T>` | ✅ | ❌ | 同 RefCell |
| `&T` | ✅ | T: Sync | 借用的可共享性由被借对象决定 |

> 几乎所有**默认类型都是 `Send + Sync`**——例外是带"线程不安全"机制的：`Rc` / `RefCell` / 裸指针 / `Cell`。

## 6.3 自动派生

```rust
// 自动：
struct MyStruct { name: String, count: i32 }
// → Send + Sync（所有字段都是）

// 自动：
struct HoldsRc { rc: Rc<i32> }
// → !Send, !Sync（Rc 不是 Send）

// 自动：
struct HoldsMutex { m: Mutex<i32> }
// → Send + Sync（Mutex 内部就是线程安全的）
```

> 编译器**自动**根据字段推导——你不用写 `impl Send for ...`。

## 6.4 自己实现 Send / Sync

```rust
// ❌ 大多数情况不要这样做
struct MyType(*const u8);
unsafe impl Send for MyType {}      // 我保证是线程安全的
unsafe impl Sync for MyType {}      // 我保证 Sync
```

> ⚠️ **`unsafe impl Send / Sync` 必须自证安全**——编译器信你。如果错了，**UB 灾难**（数据竞争）。

## 6.5 `tokio::spawn` 要求 `Send + 'static`

```rust
let rc = std::rc::Rc::new(5);
tokio::spawn(async move {
    println!("{rc}");      // ❌ Rc is not Send
});
```

报错：
```
error: future cannot be sent between threads safely
   |
   |     Rc<i32> cannot be sent between threads safely
```

**修法**：

```rust
let arc = std::sync::Arc::new(5);
tokio::spawn(async move {
    println!("{arc}");     // ✅ Arc is Send
});
```

## 6.6 `Send` 和 `'static` 一起要什么？

```rust
fn require_send_static<T: Send + 'static>(_: T) {}

#[tokio::main]
async fn main() {
    let s = String::from("hi");
    require_send_static(s);          // ✅ String is Send + 'static（own）
}
```

> `'static` 不要求"永生"——只要求**不借用外部短命数据**。
>
> - `String` = own + 'static ✅
> - `&'a str` 借用 = 不到 'static ❌
> - `&'static str` 字面量 ✅

## 6.7 async 里的 Send 怎么推断？

```rust
async fn maybe_send() {           // 编译器看：返回值 impl Future 是否 Send
    let s = String::from("hi");   // s 是 Send
    some_op(&s).await;            // await 之间 s 被借用——编译器要看 some_op 的 future 是否 Send
    println!("{s}");
}
```

> 跨 `.await` 的所有**变量**都得是 `Send`——编译器**自动**检查。

```rust
async fn not_send() {
    let rc = std::rc::Rc::new(5);     // rc: !Send
    some_op().await;                   // await 把 rc "持有"在 future 里
    println!("{rc}");                  // 之后还要用 rc——所以 future 整体是 !Send
}
```

## 6.8 常见错误与修法

| 错误信息 | 原因 | 修法 |
|----------|------|------|
| `Rc<T> cannot be sent between threads` | 用 `Rc` 跨线程 | 换 `Arc` |
| `RefCell<T> cannot be sent between threads` | async 里用 `RefCell` | 用 `tokio::sync::Mutex` 或 `Mutex` |
| `*const T cannot be sent` | 裸指针 | 用 `Arc<T>` 或 `Box<T>` |
| `future cannot be sent between threads safely` | async future 含 `!Send` | 重构：把 `!Send` 数据**不跨 await** |

## 6.9 关键模式：把 `!Send` 数据**不跨 await**

```rust
async fn not_send_but_ok() {
    let rc = std::rc::Rc::new(5);
    println!("{rc}");              // rc 在 await 之前用完
    
    some_op().await;                // 跨 await 时 rc 已 drop
}
```

> 编译器**追踪**每个变量在 await 时的存活性——只在 await 时还活着的变量需要 `Send`。

## 6.10 `Send + !Sync` 模式

```rust
struct MyCell {
    inner: RefCell<i32>,
}
// MyCell: Send + !Sync（因为 &MyCell 不能跨线程）
```

> 这种类型**可以**跨线程 move（**一次**一个），但**不能**同时在多个线程共享 `&MyCell`。
>
> 例：`tokio::sync::Mutex`（tokio 的 mutex）就是 `Send + !Sync`。

## 6.11 `!Send + !Sync` 模式

```rust
struct OnlyOneThread {
    thread: std::thread::Thread,
}
// 持有 std::thread::Thread → !Send + !Sync
```

> 这种类型**必须**留在创建它的线程。

## 6.12 实战：`tokio::spawn` + `Send`

```rust
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Default)]
struct AppState { count: Mutex<i32> }

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState::default());    // Arc<...>: Send + Sync

    for _ in 0..10 {
        let state = Arc::clone(&state);
        tokio::spawn(async move {                 // ✅ Send
            let mut c = state.count.lock().await;
            *c += 1;
        });
    }
}
```

> `Arc<Mutex<i32>>` 是 `Send + Sync`——可以被 `spawn`。

## 6.13 一个 `Send` 编译错完整调试

```rust
use std::rc::Rc;

#[tokio::main]
async fn main() {
    let rc = Rc::new(5);
    tokio::spawn(async move {
        println!("{rc}");
    });
}
```

报错：
```
error[E0277]: `Rc<i32>` cannot be sent between threads safely
```

修法：
```rust
use std::sync::Arc;
let arc = Arc::new(5);
tokio::spawn(async move {
    println!("{arc}");
});
```

## 6.14 `Sync` vs `Send` 决策树

```
要跨线程用吗？
├── 否 → 任何类型都行
└── 是
    ├── 跨线程 move（一次性）
    │   └── T: Send
    │       ├── Rc → 改 Arc
    │       ├── &T → 改 owned T
    │       └── 裸指针 → 改 Arc/Box + Send
    └── 跨线程共享 &T
        └── T: Sync
            ├── Rc → 改 Arc
            ├── RefCell → 改 Mutex
            └── 裸指针 → 重写
```

## 6.15 `unsafe impl Send` 的责任

```rust
struct MyPtr(*mut u8);
unsafe impl Send for MyPtr {}
unsafe impl Sync for MyPtr {}
```

> 你向编译器**保证**：
> 1. 多个线程同时读这个指针**不会**数据竞争
> 2. 通过指针改数据**不会**破坏其他线程的不变量
>
> 如果错——**UB**，可能编译过但运行时炸。

## 6.16 真实场景：自引用 struct 怎么破

```rust
struct SelfRef {
    data: String,
    slice: Option<*const str>,     // 指向 data
}
// 编译器自动推导：!Send, !Sync（裸指针）
```

**修法**：

```rust
struct SafeSelfRef {
    data: String,
    // 不存指针，用 offset 替代——或者拆成两个独立的数据 + 锁
}
```

> 大部分自引用 struct 要重写。Tokio 的 `Pin` 也能解决一部分。

## 6.17 对比其他语言

| 概念 | Rust | C++ | Go | Java |
|------|------|-----|-----|------|
| 自动线程安全检查 | ✅ Send / Sync | ❌ UB 自负 | ❌ race detector | ❌ |
| 共享可变检查 | ✅ 编译期 | ❌ | race 检测 | ❌ |
| 误用后果 | 编译错 | 运行时崩 | 运行时崩 | 运行时崩 |
| `Rc` 等价 | `Rc<T>` | `shared_ptr`（原子） | GC | GC |
| `RefCell` 等价 | `RefCell<T>` | `mutable` + 不安全 | — | — |

---

## 🏋️ 本章小练习

**练习 6.1**：定义 `use std::rc::Rc; let r = Rc::new(5);`，放到 `tokio::spawn` 里看报什么错，改成 `Arc` 验证通过。

**练习 6.2**：演示 `Cell<i32>` 是 Send 但 `!Sync`：

```rust
fn is_sync<T: Sync>() {}
is_sync::<Cell<i32>>();   // ❌ 编译错
is_sync::<i32>();          // ✅
```

**练习 6.3**：写一个 `unsafe impl Send`（**仅作为学习，**不要**在真实代码里随便用**）：

```rust
struct MyPtr(*mut u8);
unsafe impl Send for MyPtr {}
unsafe impl Sync for MyPtr {}
```

**练习 6.4**（思考）：为什么 `tokio::sync::Mutex<T>` 是 `Send`？它的锁不是 await 友好的吗？

> 答：锁**本身**用 `unsafe` 内部实现的"等待队列"——`T: Send` 时 `tokio::Mutex<T>: Send`。

**练习 6.5**（真实）：把 Stage 4 的 `Arc<Mutex<i32>>` 多线程计数改造成 tokio 异步版本。

---

下一章：[07 · 阶段复习 →](./07-stage-review.md)
