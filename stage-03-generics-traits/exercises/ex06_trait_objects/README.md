# Exercise 06 · Trait Object

> 难度：⭐⭐  涉及：第 4 章

## 任务

### 1) 命令模式

```rust
trait Command {
    fn name(&self) -> &str;
    fn execute(&self);
}

struct AddCommand { delta: i32 }
impl Command for AddCommand {
    fn name(&self) -> &str { "add" }
    fn execute(&self) { println!("+{}", self.delta); }
}

struct PrintCommand { prefix: String }
impl Command for PrintCommand { ... }

struct Runner { commands: Vec<Box<dyn Command>> }

impl Runner {
    fn new() -> Self
    fn add(&mut self, cmd: Box<dyn Command>)
    fn run_all(&self)         // 顺序执行
}
```

### 2) 异质形状集合

```rust
trait Shape { fn area(&self) -> f64; }

struct Circle { radius: f64 }
struct Square { side: f64 }

impl Shape for Circle { ... }
impl Shape for Square { ... }

fn total_area(shapes: &[Box<dyn Shape>]) -> f64 {
    // 求所有形状面积之和
}
```

### 3) Rc<dyn Trait>：共享只读 trait object

```rust
use std::rc::Rc;

let shared: Rc<dyn Command> = Rc::new(AddCommand { delta: 1 });
let a = shared.clone();
let b = shared.clone();
// 三个 Rc 都指向同一个 Command
```

### 4) 验证"非对象安全"的 trait 不能 dyn

故意写一个返回 `Self` 的方法：

```rust
trait BadClone {
    fn clone_self(&self) -> Self;
}

fn use_bad(_: &dyn BadClone) {}     // ❌ expected an `object-safe` trait
```

## 验收

- `Runner` 能装 3 种命令，顺序执行
- `total_area(&[Circle, Square, Circle])` 算总和
- "非对象安全"的报错自己触发一次，看懂错误信息

## 提示

- `Box::new(Circle { ... }) as Box<dyn Shape>` 是显式转换；通常可以省略 `as`，因为上下文推断
- `Rc<dyn Trait>` 用法：定义时 `let x: Rc<dyn Command> = Rc::new(...)`

## 进阶

写一个 `trait Event { fn process(&self); }`，实现 `ClickEvent` / `KeyEvent` / `ResizeEvent`，然后写一个**事件队列**：

```rust
struct EventQueue { events: Vec<Box<dyn Event>> }
impl EventQueue {
    fn push(&mut self, e: Box<dyn Event>)
    fn drain(self) -> Vec<String>      // 执行所有事件，返回 "click at (x, y)" 等字符串
}
```

完成 → [ex07_advanced_traits](../ex07_advanced_traits)
