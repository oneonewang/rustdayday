# 02 · RefCell：内部可变性

> **本章目标**：理解什么是"内部可变性"，会用 `RefCell<T>` / `Cell<T>`，知道什么时候值得用、什么时候是反模式。

## 2.1 什么是内部可变性（Interior Mutability）？

Stage 1 学的借用规则：

> 不可变引用 `&T` 不能修改 `T`。

但**有些场景**你想"对外不可变，对内可变"——比如 mock 对象、共享缓存、图节点的边。

> **内部可变性** = 借用规则**推迟到运行时**检查的类型。**编译器允许** `&RefCell<T>` 调 `.borrow_mut()`，**运行时**检查是否冲突。

| 类型 | 检查时机 |
|------|----------|
| 普通 `T` | 编译期（借用检查器） |
| `RefCell<T>` | 运行时（refcount + 标志位） |
| `Mutex<T>` | 运行时（OS 锁） |
| `RwLock<T>` | 运行时（读写锁） |

## 2.2 `RefCell<T>`：单线程的运行时借用检查

```rust
use std::cell::RefCell;

let data = RefCell::new(5);

// 不可变借用
let r1 = data.borrow();
let r2 = data.borrow();
println!("{} {}", r1, r2);    // 5 5

// 可变借用
drop(r1); drop(r2);
let mut r3 = data.borrow_mut();
*r3 += 1;
drop(r3);
println!("{}", data.borrow());   // 6
```

> ⚠️ **运行时**如果违反借用规则 → **panic**：
> ```rust
> let r1 = data.borrow();
> let r2 = data.borrow_mut();   // 💥 already borrowed
> ```

## 2.3 `Cell<T>`：更轻量的内部可变性（适用于 `Copy` 类型）

```rust
use std::cell::Cell;

let c = Cell::new(5);
c.set(10);                    // 内部直接改，不需可变引用
let v = c.get();
println!("{v}");              // 10
```

> - `RefCell<T>` 借出 `&T` / `&mut T`
> - `Cell<T>` 直接 **get / set 值**（仅对 `Copy` 类型友好）

## 2.4 `Rc<RefCell<T>>` 经典模式：共享 + 可变

```rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Counter { value: RefCell<u32> }

let a = Rc::new(Counter { value: RefCell::new(0) });
let b = Rc::clone(&a);

*a.value.borrow_mut() += 1;
*b.value.borrow_mut() += 1;

println!("a.value = {}", a.value.borrow());   // 2
println!("b.value = {}", b.value.borrow());   // 2
```

> `a` 和 `b` 都"指向同一个计数器"，**任一改都会影响另一方**。

## 2.5 树 / 图节点的经典模式

```rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,     // 共享同一个父 → children 可改
    parent: Option<Rc<Node>>,             // 父不需改 → 直接 Rc
}

let leaf = Rc::new(Node {
    value: 3,
    children: RefCell::new(vec![]),
    parent: None,
});

let root = Rc::new(Node {
    value: 1,
    children: RefCell::new(vec![Rc::clone(&leaf)]),
    parent: None,
});

// 让 leaf 反向指 root
leaf.parent = Some(Rc::clone(&root));
```

> 注意 `children` 是 `RefCell<Vec>`——这样**新建** `leaf` 时**先**给空 `parent`、**再**修改，**避免循环**。

## 2.6 打破循环引用：`Weak<T>`

```rust
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Option<Weak<Node>>>,     // Weak 不增加强计数
}

let leaf = Rc::new(Node {
    children: RefCell::new(vec![]),
    parent: RefCell::new(None),
});
let parent = Rc::new(Node {
    children: RefCell::new(vec![Rc::clone(&leaf)]),
    parent: RefCell::new(None),
});
*leaf.parent.borrow_mut() = Some(Rc::downgrade(&parent));
// 父 → 强 → 子；子 → 弱 → 父
// 不循环：父 drop 时子自动 drop
```

## 2.7 什么时候用 `RefCell`？

✅ **适合**：

- 共享 + 可变（如 mock 对象、配置中心）
- 树 / 图：节点需要反向指针
- 测试桩：能改"const"对象

❌ **不适合**：

- 你能用 `&mut` 解决问题时——**不要用 `RefCell`**（编译期检查更安全）
- 跨线程（用 `Mutex`）
- 性能敏感的代码（运行时开销）

## 2.8 性能对比

| 操作 | `&mut T` | `RefCell<T>` |
|------|----------|--------------|
| 编译期检查 | ✅ | ❌（运行时） |
| 运行时开销 | 零 | 引用计数 + 标志位 |
| 越界时行为 | 编译错 | panic |

> `RefCell` 比 `&mut` 慢约 5-10 倍（小 benchmark）。能不用就不用。

## 2.9 `OnceCell` / `LazyLock`：延迟初始化

```rust
use std::sync::OnceLock;

static CONFIG: OnceLock<String> = OnceLock::new();

fn get_config() -> &'static String {
    CONFIG.get_or_init(|| "computed-once".to_string())
}
```

> `std::sync::LazyLock`（Rust 1.80+）更方便：
> ```rust
> use std::sync::LazyLock;
> static CONFIG: LazyLock<String> = LazyLock::new(|| "...".into());
> ```

## 2.10 一个反模式：滥用 `RefCell`

```rust
// ❌ 没必要
struct User { name: RefCell<String> }
let u = User { name: RefCell::new("Alice".into()) };
u.name.borrow_mut().push_str(" Smith");

// ✅ 普通 mut 就行
struct User { name: String }
let mut u = User { name: "Alice".into() };
u.name.push_str(" Smith");
```

> **`RefCell` 的存在不等于"应该用 RefCell"**。

## 2.11 对比其他语言

| 概念 | Rust | C++ | Java | JavaScript |
|------|------|-----|------|-----------|
| 内部可变性 | `RefCell` / `Cell` | `mutable` / `const_cast` | `volatile` / 反射 | getter/setter |
| 借用冲突 | 运行时 panic | UB | `ConcurrentModificationException` | — |
| 单例 | `OnceLock` | `std::call_once` | 静态初始化 | 顶层 var |
| 弱引用 | `Weak` | `weak_ptr` | `WeakReference` | `WeakRef` |

---

## 🏋️ 本章小练习

**练习 2.1**：用 `RefCell` 写一个"消息总线"：

```rust
struct MessageBus { messages: RefCell<Vec<String>> }
impl MessageBus {
    fn new() -> Self
    fn publish(&self, msg: &str)
    fn snapshot(&self) -> Vec<String>     // 不可变借用得到快照
}
```

**练习 2.2**：用 `Cell<i32>` 写一个"计数器"（只对 `Copy` 类型友好）。

**练习 2.3**：写一个树节点（值 + 多个子节点 + 一个 `Weak` parent），演示"无循环"。

**练习 2.4**（**思考**）：`Cell<T>` 和 `RefCell<T>` 哪个更快？什么时候必须用 `RefCell`？

---

下一章：[03 · 闭包 →](./03-closures.md)
