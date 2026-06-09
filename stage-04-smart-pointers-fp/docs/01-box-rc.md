# 01 · Box 与 Rc：堆分配与共享所有权

> **本章目标**：理解 `Box<T>` 何时用、`Rc<T>` 何时用，掌握递归类型、DST、trait object 这三个常见场景。

## 1.1 为什么需要"智能指针"？

Stage 1–3 我们几乎所有数据都**在栈上**——`let x = 5;`、`Vec<T>` 内部用堆、`String` 内部用堆但栈上有"句柄"。

> **智能指针** = 实现了 `Deref`（有时还有 `Drop`）的类型。**它们本身是栈上的，但"指向"堆上的数据**。

最常用的智能指针：

| 类型 | 作用 | 共享？ | 可变？ | 线程安全？ |
|------|------|--------|--------|-----------|
| `Box<T>` | 唯一所有者，堆分配 | ❌ | ✅ | ✅ |
| `Rc<T>` | 共享所有者，引用计数 | ✅（单线程） | ❌ | ❌ |
| `Arc<T>` | 共享所有者，原子计数 | ✅ | ❌ | ✅ |
| `RefCell<T>` | 内部可变性 | ❌ | ✅ | ❌ |
| `Mutex<T>` | 互斥锁 | ❌ | ✅ | ✅ |

> 💡 **口诀**：
> - 唯一 + 堆 → `Box`
> - 多读 + 单线程 → `Rc`
> - 多读 + 多线程 → `Arc`
> - 多改 + 单线程 → `RefCell`（包在 `Rc` 里）
> - 多改 + 多线程 → `Mutex`（包在 `Arc` 里）

## 1.2 `Box<T>`：唯一所有者的堆分配

### 基础

```rust
let b = Box::new(5);          // 5 在堆上，b 是栈上的指针
println!("b = {b}");          // b = 5（实现了 Deref）
```

> Box 离开作用域，**自动**释放堆内存（实现 `Drop`）。

### 用途 1：递归类型

Stage 2 见过 `List` 那种递归 enum：

```rust
enum List {
    Cons(i32, Box<List>),       // ⭐ 关键：Box 抹平递归
    Nil,
}

use List::{Cons, Nil};

let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
```

**为什么需要 Box**：

```
不用 Box:
  List { Cons(i32, List) }   →  List 里又有 List → 无限嵌套 → 编译器算不出大小

用 Box:
  List { Cons(i32, Box<List>) }   →  Box 固定大小（指针），List 整体大小 = i32 + 指针
```

> 所有递归类型**必须**用指针包一层（Box / Rc / Arc）。

### 用途 2：动态大小类型（DST）

```rust
let s: Box<str> = Box::new("hello"[..]);
let a: Box<[i32]> = Box::new([1, 2, 3]);
```

> `str` / `[T]` 本身**不能**直接存在——编译器不知道多大。用 Box 装一下就 OK。

### 用途 3：Trait object

```rust
trait Draw { fn draw(&self); }
struct Button { label: String }
impl Draw for Button { fn draw(&self) { println!("[{}]", self.label); } }

let widget: Box<dyn Draw> = Box::new(Button { label: "OK".into() });
widget.draw();
```

> `Box<dyn Draw>` 是 Stage 3 学过的"异质集合"基础：`Vec<Box<dyn Draw>>`。

## 1.3 `Deref` 与 `*`

`Box<T>` 实现 `Deref<Target = T>`——`b` 当 `T` 用**自动解引用**。

```rust
let b = Box::new(String::from("hi"));
b.len()              // 自动 *(**b).len()  →  String::len
b.chars()            // 同上
```

**手动解引用**用 `*`：

```rust
let b = Box::new(5);
assert_eq!(*b, 5);   // 解出 i32
```

> 💡 **解引用强制转换（deref coercion）**：函数参数 `&str` 时，传 `&String` 自动转；传 `&Box<String>` 也自动转。

## 1.4 自定义 Box 行为：Deref + Drop

```rust
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> Self { MyBox(x) }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T { &self.0 }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) { println!("dropping MyBox!"); }
}

fn main() {
    let b = MyBox::new(String::from("hi"));
    println!("{}", b.len());    // 自动 deref
}   // 离开作用域，触发 drop
```

## 1.5 `Rc<T>`：单线程共享所有权

```rust
use std::rc::Rc;

let a = Rc::new(5);
let b = Rc::clone(&a);        // 引用计数 +1
let c = Rc::clone(&a);
println!("count = {}", Rc::strong_count(&a));   // 3
drop(b);
println!("count = {}", Rc::strong_count(&a));   // 2
```

> `Rc::clone` **不**深拷贝，只增加引用计数（cheap）。

### 经典场景：树 / 图（节点多 parent）

```rust
use std::rc::Rc;

#[derive(Debug)]
enum Node {
    Leaf(i32),
    Branch(i32, Rc<Node>, Rc<Node>),
}

let leaf = Rc::new(Node::Leaf(5));
let root = Node::Branch(1, Rc::clone(&leaf), Rc::clone(&leaf));
// leaf 被两个 parent 共享
```

> ⚠️ `Rc` **不可变**——多个所有者都只能读。要改得用 `Rc<RefCell<T>>`（下一章）。

## 1.6 什么时候用哪个？

| 场景 | 选 |
|------|-----|
| 我要唯一的堆数据 | `Box<T>` |
| 我需要**一个**容器装多种类型 | `Box<dyn Trait>` |
| 树 / 图节点多 parent | `Rc<Node>` |
| 同上，但运行时还要改 | `Rc<RefCell<Node>>` |
| 同上，跨线程 | `Arc<Node>` / `Arc<Mutex<Node>>` |

## 1.7 性能开销

| 类型 | 内存 | 解引用开销 |
|------|------|-----------|
| `Box<T>` | 一个指针（栈）+ T（堆） | 一次 |
| `Rc<T>` | 一个指针（栈）+ 强/弱引用计数（堆） + T（堆） | 一次（外加 clone 时原子 +1） |
| 普通引用 `&T` | 一个指针（栈） | 一次 |

> Box 和 `&T` 几乎一样快；`Rc` 因为要维护计数有少量额外开销。

## 1.8 `Rc` 的循环引用问题

```rust
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    next: Option<Rc<RefCell<Node>>>,
}

let a = Rc::new(RefCell::new(Node { next: None }));
let b = Rc::new(RefCell::new(Node { next: Some(Rc::clone(&a)) }));

a.borrow_mut().next = Some(Rc::clone(&b));
// a → b → a 循环，永远不会释放 💥
```

修法：用 `Weak<T>` 打破循环（下一章详讲）。

## 1.9 对比其他语言

| 概念 | Rust | C++ | Java | Python |
|------|------|-----|------|--------|
| 唯一堆所有者 | `Box<T>` | `unique_ptr` | — | — |
| 共享所有者 | `Rc<T>` | `shared_ptr` | GC 引用 | GC 引用 |
| 内部可变性 | `RefCell<T>` | `mutable` / const_cast | — | — |
| 弱引用 | `Weak<T>` | `weak_ptr` | `WeakReference` | `weakref` |
| 引用计数 | `Rc::strong_count` | `use_count` | — | `sys.getrefcount` |

## 1.10 一个真实例子：链表 + Rc

```rust
use std::rc::Rc;

#[derive(Debug)]
struct List<T> {
    head: Option<Rc<Node<T>>>,
}

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Option<Rc<Node<T>>>,
}

impl<T> List<T> {
    fn new() -> Self { Self { head: None } }

    fn prepend(&self, elem: T) -> List<T> {
        List { head: Some(Rc::new(Node { elem, next: self.head.clone() })) }
    }

    fn head(&self) -> Option<&T> { self.head.as_ref().map(|n| &n.elem) }

    fn tail(&self) -> List<T> {
        List { head: self.head.as_ref().and_then(|n| n.next.clone()) }
    }
}

fn main() {
    let list = List::new().prepend(1).prepend(2).prepend(3);
    println!("head = {:?}", list.head());    // Some(3)
    let tail = list.tail();
    println!("tail head = {:?}", tail.head());   // Some(2)
}
```

> 注意 `prepend` 拿 `&self`、返回新 `List`——共享链表，**零拷贝**。

---

## 🏋️ 本章小练习

**练习 1.1**：定义递归 enum `BinaryTree { Empty, Node(i32, Box<BinaryTree>, Box<BinaryTree>) }`，实现 `sum` / `contains` / `height`。

**练习 1.2**：用 `Rc` 实现一个简单的图：

```rust
struct Graph { nodes: Vec<Rc<Node>> }
struct Node { id: u32, neighbors: Vec<Rc<Node>> }
```

**练习 1.3**：写一个"零分配"的链表（`prepend` 拿 `&self` 返回新 list，用 `Rc` 共享）。

**练习 1.4**：自定义 `Box` 简化版（`MyBox<T>` + `Deref` + `Drop`），打印 drop 顺序观察 RAII。

---

下一章：[02 · RefCell：内部可变性 →](./02-refcell.md)
