# 04 · Trait Object：动态分发

> **本章目标**：会用 `dyn Trait` 在运行时处理多种类型，理解静态分发 vs 动态分发的取舍。

## 4.1 静态分发 vs 动态分发

| | 静态分发（泛型 `<T: Trait>`） | 动态分发（`dyn Trait`） |
|---|-------------------------------|------------------------|
| 编译产物 | 每种类型一份代码 | 一份代码 + 虚表 |
| 运行时 | 零开销 | 多一次间接调用 |
| 二进制大小 | 较大 | 较小 |
| 可以异质集合？ | ❌ | ✅ |
| 代码内联 | 容易 | 难 |

> 💡 **经验法则**：
> - 泛型：**类型固定**，行为一致 → 静态分发（快）
> - dyn：**类型动态变化**，需要"按 trait 看待" → 动态分发（灵活）

## 4.2 定义一个 trait

```rust
trait Draw {
    fn draw(&self);
}
```

## 4.3 用 trait object

```rust
struct Circle { radius: f64 }
impl Draw for Circle { fn draw(&self) { println!("○ 半径 {}", self.radius); } }

struct Square { side: f64 }
impl Draw for Square { fn draw(&self) { println!("□ 边长 {}", self.side); } }
```

### 用 `Box<dyn Trait>` 装单个

```rust
let shapes: Vec<Box<dyn Draw>> = vec![
    Box::new(Circle { radius: 1.0 }),
    Box::new(Square { side: 2.0 }),
];

for shape in &shapes {       // 注意 &Box<dyn Draw>
    shape.draw();             // 自动解引用再调用
}
```

### 用 `&dyn Trait` 借用

```rust
let c = Circle { radius: 1.0 };
let s: &dyn Draw = &c;        // 不需要 Box
s.draw();
```

> **Box vs &dyn**：Box 拥有，&dyn 借用。

## 4.4 完整例子：GUI 组件列表

```rust
trait Draw { fn draw(&self); }
trait Click { fn on_click(&self); }

// 一个组件可以同时实现多个 trait
trait Widget: Draw + Click { }

// 对象安全的 trait 才能 dyn
// (没有泛型方法、没有 Self 用法)

struct Button { label: String }
impl Draw for Button { fn draw(&self) { println!("[{}]", self.label); } }
impl Click for Button { fn on_click(&self) { println!("{} clicked", self.label); } }

fn render_all(widgets: &[Box<dyn Draw>]) {
    for w in widgets { w.draw(); }
}

fn main() {
    let widgets: Vec<Box<dyn Draw>> = vec![
        Box::new(Button { label: "确定".to_string() }),
    ];
    render_all(&widgets);
}
```

## 4.5 对象安全（Object Safety）

不是所有 trait 都能做 `dyn Trait`。规则：

> **trait 中所有方法必须满足：**
> 1. 返回类型**不能**是 `Self`
> 2. 不能有泛型参数
> 3. 用 `where Self: Sized` 排除的方法不算

```rust
// ❌ 不是对象安全
trait Bad {
    fn clone_self(&self) -> Self;          // 返回 Self
    fn convert<T>(&self, x: T) -> T;       // 泛型方法
}

// ✅ 是对象安全
trait Good {
    fn describe(&self) -> &str;
    fn draw(&self);
}
```

编译器会**明确**告诉你"trait 不能 dyn"。

> 💡 习惯做法：把"对象安全的方法"放主 trait，把"非对象安全的（如 `clone()` 返回 `Self`）"放辅助 trait。

## 4.6 静态分发 vs 动态分发：完整对比

```rust
// 静态
fn render_static<T: Draw>(items: &[T]) {
    for it in items { it.draw(); }
}

// 动态
fn render_dynamic(items: &[Box<dyn Draw>]) {
    for it in items { it.draw(); }
}
```

调用：

```rust
let circles = vec![Circle { radius: 1.0 }, Circle { radius: 2.0 }];
render_static(&circles);          // 生成 render_static_for_Circle
```

```rust
let mixed: Vec<Box<dyn Draw>> = vec![
    Box::new(Circle { radius: 1.0 }),
    Box::new(Square { side: 2.0 }),
];
render_dynamic(&mixed);            // 一份代码处理所有类型
```

> **关键差别**：`render_static` 的入参**必须全是同一种类型**；`render_dynamic` 可以混合。

## 4.7 `Box<dyn Trait>` vs `&dyn Trait` vs `Rc<dyn Trait>`

| 写法 | 用途 |
|------|------|
| `Box<dyn Trait>` | 拥有，**可装进 Vec、跨函数传递、存到 struct** |
| `&dyn Trait` | 借用（不能装进 Vec、不能存为字段） |
| `Rc<dyn Trait>` | 单线程共享所有权（多用于树 / 图） |
| `Arc<dyn Trait>` | 多线程共享所有权 |

```rust
// Box：最常用
let screen: Vec<Box<dyn Draw>> = vec![...];

// Rc：单线程共享
use std::rc::Rc;
let node = Rc::new(Button { ... }) as Rc<dyn Draw>;

// Arc：多线程共享
use std::sync::Arc;
let shared = Arc::new(Button { ... }) as Arc<dyn Draw>;
```

## 4.8 在 struct 里存 trait object

```rust
struct Screen {
    widgets: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn new() -> Self { Self { widgets: vec![] } }
    fn add(&mut self, w: Box<dyn Draw>) { self.widgets.push(w); }
    fn render(&self) { for w in &self.widgets { w.draw(); } }
}
```

## 4.9 关联类型 + dyn：什么时候不行

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

> `Iterator` 用了关联类型 `Item`——`Self::Item` 是**类型**，dyn 时**没法**确定。所以 `Iterator` 不能直接 `dyn Iterator`。

**解法**：用 `trait IteratorOf<T> { fn next(&mut self) -> Option<T>; }`，把关联类型变成 trait 参数。

## 4.10 完整的"策略模式"

```rust
trait PricingStrategy {
    fn calculate(&self, base: f64) -> f64;
}

struct NoDiscount;
impl PricingStrategy for NoDiscount {
    fn calculate(&self, base: f64) -> f64 { base }
}

struct Percentage(f64);     // 折扣百分比
impl PricingStrategy for Percentage {
    fn calculate(&self, base: f64) -> f64 { base * (1.0 - self.0 / 100.0) }
}

struct FixedOff(f64);
impl PricingStrategy for FixedOff {
    fn calculate(&self, base: f64) -> f64 { base - self.0 }
}

fn checkout(strategy: &dyn PricingStrategy, base: f64) -> f64 {
    strategy.calculate(base)
}

fn main() {
    let s1 = Percentage(20.0);
    let s2 = FixedOff(30.0);
    let s3 = NoDiscount;
    println!("{}", checkout(&s1, 100.0));   // 80
    println!("{}", checkout(&s2, 100.0));   // 70
    println!("{}", checkout(&s3, 100.0));   // 100
}
```

## 4.11 性能影响

动态分发每次调用**多一次间接跳转**（虚表查找）：

```
静态: call  draw_concrete    ; 直接跳到具体实现
动态: mov   rax, [vtable]    ; 先取虚表指针
       call  [rax + offset]   ; 再虚表里调具体实现
```

通常这个开销在 1-5 纳秒级，**绝大多数应用感知不到**。

## 4.12 用 dyn 还是 enum？

**经验法则**：

| 情况 | 选择 |
|------|------|
| 类型集合**已知且固定** | enum + match（Stage 2） |
| 类型集合**运行时扩展**（插件系统） | dyn Trait |
| 想要零开销 | 泛型 + trait bound |
| 想存到 `Vec` / `HashMap` | `Vec<Box<dyn Trait>>` |
| 类型集合**不固定、行为差异大** | dyn Trait |

## 4.13 对比其他语言

| 概念 | Rust `dyn Trait` | Java interface | C++ 虚函数 | Go interface |
|------|------------------|----------------|-----------|--------------|
| 调度方式 | 虚表（vtable） | 虚方法表 | 虚函数表 | iface table |
| 运行时开销 | 间接调用 | 间接调用 | 间接调用 | 间接调用 |
| 必须显式 | 是（`dyn` 关键字） | 否 | 是（virtual） | 否（隐式） |
| 类型擦除 | 否（保留类型信息） | 是 | 否 | 是 |

---

## 🏋️ 本章小练习

**练习 4.1**：写一个 `Drawer` struct，里面存 `Vec<Box<dyn Draw>>`，实现 `add` / `render_all`。

**练习 4.2**：改写 Stage 2 的 `Shape` enum——但这次**用 dyn Trait**：

```rust
fn render(shapes: &[Box<dyn Shape>])  // Shape 是 trait
```

**练习 4.3**：`Rc<dyn Trait>` 练习——写一个简单的"单链表"节点，每个节点可以是 `int` 或 `string`：

```rust
enum Node { Val(Box<dyn ToString>), Next(Rc<Node>), End }
```

**练习 4.4**：观察"非对象安全"的 trait 编译错：

```rust
trait Bad { fn clone_self(&self) -> Self; }
fn render(_: &dyn Bad) {}    // ❌
```

**练习 4.5**（真实场景）：写一个 `Logger`：

```rust
trait Logger { fn log(&self, msg: &str); }

struct StdoutLogger;
struct FileLogger { path: String }

fn run_with_logger(logger: &dyn Logger) { ... }
```

---

下一章：[05 · 高级 Trait →](./05-advanced-traits.md)
