# 02 · 方法与关联函数

> **本章目标**：会写 `impl` 块，理解 `self` 的三种形态（`&self` / `&mut self` / `self`），区分"方法"与"关联函数"。

## 2.1 方法 = 第一个参数是 `self` 的函数

```rust
#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn main() {
    let r = Rectangle { width: 30.0, height: 50.0 };
    println!("面积 = {}", r.area());     // 自动取 &r
}
```

> 💡 `r.area()` 看着像"无参方法"，但**自动**变成 `Rectangle::area(&r)`。

## 2.2 `self` 的三种形态

| 形态 | 含义 | 何时用 |
|------|------|--------|
| `&self` | 不可变借用 | 只读方法（90% 情况） |
| `&mut self` | 可变借用 | 要修改字段 |
| `self` | 拿走所有权 | 消费型转换（少见） |

```rust
impl Rectangle {
    fn area(&self) -> f64 {                              // 1. 只读
        self.width * self.height
    }

    fn scale(&mut self, factor: f64) {                   // 2. 原地缩放
        self.width *= factor;
        self.height *= factor;
    }

    fn into_square(self) -> Rectangle {                  // 3. 消费 self，返回新
        let s = self.width.max(self.height);
        Rectangle { width: s, height: s }
    }
}

fn main() {
    let r = Rectangle { width: 4.0, height: 3.0 };
    println!("面积 = {}", r.area());         // 12.0

    let mut r2 = Rectangle { width: 4.0, height: 3.0 };
    r2.scale(2.0);                           // 8.0 x 6.0
    println!("r2 = {:?}", r2);

    let r3 = r2.into_square();              // r2 被消费
    println!("r3 = {:?}", r3);
    // println!("{:?}", r2);                // ❌ r2 已经被 move
}
```

## 2.3 关联函数（"静态方法"）

**没有 `self` 参数**的 `impl` 块里的函数。常用于**构造函数**：

```rust
impl Rectangle {
    fn new(width: f64, height: f64) -> Rectangle {       // 构造
        Rectangle { width, height }
    }

    fn square(size: f64) -> Rectangle {                  // 命名构造
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let r = Rectangle::new(10.0, 20.0);                   // :: 调用
    let s = Rectangle::square(15.0);
}
```

> **约定俗成**：构造函数叫 `new()`，其他命名构造用描述性名字（`square()` / `from_str()` / `parse()`）。

## 2.4 多个 `impl` 块

允许在**同一个 crate 里为同一个 struct 写多个 `impl` 块**——常用于分离"通用方法"和"特定 trait 实现"：

```rust
impl Rectangle {
    fn area(&self) -> f64 { self.width * self.height }
}

// 可以分开
impl Rectangle {
    fn perimeter(&self) -> f64 { 2.0 * (self.width + self.height) }
}
```

> 这只是组织代码的便利，**和单个 `impl` 块没有功能差异**。

## 2.5 方法名和字段名冲突？

```rust
struct Rectangle { width: f64, height: f64 }

impl Rectangle {
    fn width(&self) -> f64 { self.width }     // ❌ 字段名和方法名同名，编译错
}
```

**编译器会指出**哪个是字段、哪个是方法。

## 2.6 自动引用 / 解引用（method resolution）

```rust
let r = Rectangle { width: 4.0, height: 3.0 };
r.area();      // 调用时，编译器自动看 area 的签名是 &self / &mut self / self
               // 然后自动 &r / &mut r / r
```

| 签名 | 编译器自动 | 适用场景 |
|------|------------|----------|
| `fn f(&self)` | `(&r).f()` | 默认 |
| `fn f(&mut self)` | `(&mut r).f()` | r 本身要 mut |
| `fn f(self)` | `(r).f()` | r 之后用不到 |

> 如果你写的 `f` 取 `&mut self`，调用方 `r` 必须是 `mut r`。

## 2.7 `Self` 大写 = 当前类型

```rust
impl Rectangle {
    fn new(w: f64, h: f64) -> Self {       // Self == Rectangle
        Self { width: w, height: h }        // == Rectangle { ... }
    }
}
```

> 在 `impl` 块里 `Self` 永远指代"被实现的那个类型"。

## 2.8 一个完整例子：链表节点

```rust
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),   // 一个节点 = 值 + 指向下一个
    Nil,
}

use List::{Cons, Nil};

impl List {
    fn new() -> List { Nil }

    fn prepend(self, elem: i32) -> List {   // 拿所有权，构造新链表
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> usize {                // 不可变借用，递归求长度
        match self {
            Cons(_, tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        match self {
            Cons(head, tail) => format!("{}, {}", head, tail.stringify()),
            Nil => format!("Nil"),
        }
    }
}

fn main() {
    let list = List::new()
        .prepend(1)
        .prepend(2)
        .prepend(3);
    println!("{}", list.stringify());  // 3, 2, 1, Nil
    println!("长度 = {}", list.len()); // 3
}
```

> 看到 `Box<List>` 不用慌——这是**递归类型**必须用指针包一层（下一章 enum 详解；Box Stage 4 详解）。**这例子的关键**：enum 上也能写方法。

## 2.9 对比其他语言

| 概念 | Rust | C++ | Java | Python |
|------|------|-----|------|--------|
| 方法定义位置 | `impl` 块（可分开） | class 内部 | class 内部 | class 内部 |
| `self` 形态 | `&self` / `&mut self` / `self`（编译器提示） | 引用 / 指针 / 智能指针 | 全是引用 | 隐式 self |
| 静态方法 | 关联函数 | `static` 成员 | `static` 方法 | `@staticmethod` |
| 方法解析 | 自动 `&` / `&mut` | 手动 | 手动 | 手动 |

---

## 🏋️ 本章小练习

**练习 2.1**：为 `Rectangle` 写四个方法：

```rust
fn area(&self) -> f64                       // 面积
fn perimeter(&self) -> f64                  // 周长
fn is_square(&self) -> bool                 // 是否正方形
fn double(&mut self)                        // 宽高都 ×2（原地）
```

**练习 2.2**：定义 `Circle { radius: f64 }`，写 `area` / `circumference` / `new` / `from_diameter`（关联函数：接收 `diameter`）。

**练习 2.3**：写一个 `Vec` 替代品（仅学习目的，不要真用）：

```rust
struct IntVec { data: Vec<i32> }

impl IntVec {
    fn new() -> Self
    fn push(&mut self, v: i32)
    fn pop(&mut self) -> Option<i32>         // 空时返回 None
    fn len(&self) -> usize
    fn sum(&self) -> i32
}
```

**练习 2.4**：链式调用练习

```rust
let s = StringBuilder::new()
    .append("Hello")
    .append(", ")
    .append("world")
    .append("!")
    .build();
assert_eq!(s, "Hello, world!");
```

> 提示：每个 `append` 拿所有权、返回新 builder。`build` 返回最终 `String`。

---

下一章：[03 · Enum 与模式匹配 ⭐ →](./03-enums-and-pattern-matching.md)
