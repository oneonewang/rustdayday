# 03 · Enum 与模式匹配 ⭐

> **本章目标**：理解 Rust 的 `enum` 远强于大多数语言（可以携带数据），用 `match` 穷尽所有分支，用 `if let` 简化单分支。

## 3.1 最简单的 enum

```rust
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let d = Direction::Up;
}
```

> 这就是熟悉的"枚举"，和其他语言差不多。

## 3.2 ⭐ enum 可以携带数据（关键特性）

```rust
enum Shape {
    Circle(f64),                        // 半径
    Rectangle(f64, f64),                // 宽 高
    Triangle { base: f64, height: f64 },// 命名字段
}
```

每个**变体（variant）**可以携带不同类型、不同数量的数据——这才是 Rust enum 的精髓。

> 💡 这种类型在**类型论**里叫"和类型 / sum type / tagged union"：一个 `Shape` 值**要么**是 `Circle`，**要么**是 `Rectangle`，**要么**是 `Triangle`，**三选一**。

```rust
fn area(s: &Shape) -> f64 {
    match s {
        Shape::Circle(r)            => std::f64::consts::PI * r * r,
        Shape::Rectangle(w, h)      => w * h,
        Shape::Triangle { base, height } => 0.5 * base * height,
    }
}
```

## 3.3 enum 上的方法

```rust
impl Shape {
    fn describe(&self) -> String {
        match self {
            Shape::Circle(r)         => format!("圆，半径 {}", r),
            Shape::Rectangle(w, h)   => format!("矩形 {}x{}", w, h),
            Shape::Triangle { base, height } => format!("三角形 底{} 高{}", base, height),
        }
    }
}
```

## 3.4 ⭐ `match`：穷尽的分支

```rust
fn process(d: Direction) {
    match d {
        Direction::Up    => println!("↑"),
        Direction::Down  => println!("↓"),
        Direction::Left  => println!("←"),
        Direction::Right => println!("→"),
    }
}
```

**穷尽性检查**：编译器强制你列出**所有**变体——漏一个就编译错。

```rust
match d {
    Direction::Up   => println!("↑"),
    Direction::Down => println!("↓"),
    // 漏了 Left 和 Right → 编译错
}
```

> 💡 这就是为什么 Rust 不用 `default` 分支也能保证安全——编译器看漏没漏。

### 穷尽性的好处

假设你后来给 `Direction` 加了 `Diagonal`：

```rust
enum Direction { Up, Down, Left, Right, Diagonal }
```

所有**之前**用 `match` 匹配 `Direction` 的地方，**全部编译失败**——提示你"这个枚举你还没穷尽处理"。

> 这样的好处是：**加新变体不会静默通过测试**。Java 的 `switch` 加一个 `enum` 值，可能完全没人发现某些分支没处理。

## 3.5 `match` 也是表达式

```rust
let label = match d {
    Direction::Up    => "上",
    Direction::Down  => "下",
    Direction::Left  => "左",
    Direction::Right => "右",
};
```

> 所有分支必须返回**相同类型**。

## 3.6 match 里的"绑定"

```rust
match shape {
    Shape::Circle(r)        => println!("半径 {}", r),     // 绑到 r
    Shape::Rectangle(w, h)  => println!("{}x{}", w, h),    // w, h
    Shape::Triangle { base, height } => ...                  // 同名字段简写
}
```

## 3.7 模式可以"过滤"：`match` 守卫

```rust
fn classify(n: i32) -> &'static str {
    match n {
        0          => "零",
        1..=9      => "一位数",
        10..=99    => "两位数",
        n if n < 0 => "负数",
        _          => "其他",
    }
}
```

> `n if n < 0 => ...` 是**匹配守卫**——在模式后面加一个 `if` 条件。

## 3.8 `if let`：单分支简化

当你只关心一个变体、其他都不在乎：

```rust
// 不用 if let
match config {
    Some(max) => println!("最大值 = {max}"),
    _         => (),                      // 占位，烦
}

// 用 if let
if let Some(max) = config {
    println!("最大值 = {max}");
}
```

### `if let ... else`

```rust
if let Some(max) = config {
    println!("最大值 = {max}");
} else {
    println!("没有配置");
}
```

> 等价于 `match` 加上 `_ => ...`——但更简洁。

## 3.9 `while let`：循环直到不匹配

```rust
let mut stack = vec![1, 2, 3];

while let Some(top) = stack.pop() {
    println!("弹出 {top}");
}
// 弹出 3
// 弹出 2
// 弹出 1
```

## 3.10 `let ... else`：必须匹配，否则走 else 分支

Rust 1.65+ 引入：

```rust
fn describe_shape(s: &Shape) -> &'static str {
    let Shape::Circle(r) = s else {        // 必须是 Circle
        return "不是圆";
    };
    if r > &10.0 { "大圆" } else { "小圆" }
}
```

> 比 `match` 简洁，比 `if let` 强制——更适合"不匹配就 return / break / continue"的场景。

## 3.11 `matches!` 宏

```rust
fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U')
}

fn process(d: Direction) {
    if matches!(d, Direction::Up | Direction::Down) {
        println!("垂直方向");
    }
}
```

> `matches!` 返回 `bool`——把模式当作谓词用。

## 3.12 嵌套模式

```rust
enum Event {
    Click { x: i32, y: i32 },
    KeyPress(char),
    Resize { width: u32, height: u32 },
    Quit,
}

fn handle(e: &Event) {
    match e {
        Event::Click { x: 0, y: 0 } => println!("原点点击"),
        Event::Click { x, y }       => println!("点击 ({x}, {y})"),
        Event::KeyPress('q') | Event::Quit => println!("退出"),
        Event::KeyPress(c)          => println!("按键 {c}"),
        Event::Resize { width, height } => println!("缩放 {width}x{height}"),
    }
}
```

> - 模式可以嵌套字段
> - 字面量直接做模式
> - `|` 表示"或"
> - 顺序很重要，**先匹配的具体放前面**

## 3.13 `@` 绑定：匹配的同时把整个值绑住

```rust
match n {
    n @ 1..=9 => println!("一位数 {n}"),
    n @ 10..=99 => println!("两位数 {n}"),
    _ => println!("其他"),
}
```

## 3.14 一个完整的"代数值"小例子

```rust
#[derive(Debug)]
enum Expr {
    Num(i32),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

fn evaluate(e: &Expr) -> i32 {
    match e {
        Expr::Num(n)             => *n,
        Expr::Add(l, r)          => evaluate(l) + evaluate(r),
        Expr::Mul(l, r)          => evaluate(l) * evaluate(r),
    }
}

fn main() {
    // 1 + 2 * 3 = 7
    let e = Expr::Add(
        Box::new(Expr::Num(1)),
        Box::new(Expr::Mul(
            Box::new(Expr::Num(2)),
            Box::new(Expr::Num(3)),
        )),
    );
    println!("evaluate = {}", evaluate(&e));   // 7
}
```

> 这个例子用了 `Box<T>`——`Box<Expr>` 是堆上指针。**递归 enum 必须用指针**，否则编译器算不出大小。Stage 4 详讲 `Box`，这里先"知其然"。

## 3.15 对比其他语言

| 概念 | Rust | C | Java | TypeScript | Swift |
|------|------|---|------|------------|-------|
| 简单枚举 | ✅ | enum | enum | enum | enum |
| **变体携带数据** | ✅ | ❌ | ❌（要 sealed class 模拟） | ❌（要 union type） | ✅ associated value |
| 穷尽匹配 | ✅ 编译期 | ❌ | ❌ | 全局 `never` | ✅ |
| 模式匹配 | ✅ | ❌ | pattern matching（Java 21+） | ❌ | ✅ |

> 💡 Rust 的 enum + match 直接对应 ML / Haskell / OCaml / F# / Scala 里的"代数数据类型"。理解了这个，你就能表达"分类 + 每类不同信息"的所有问题。

---

## 🏋️ 本章小练习

**练习 3.1**：定义 `enum IpAddr { V4(u8, u8, u8, u8), V6(String) }`，写 `display` 方法打印 `"192.168.1.1"` 或 `"::1"` 形式（V6 简化直接打印字符串）。

**练习 3.2**：用 enum 表达 JSON 值：

```rust
enum Json {
    Null,
    Bool(bool),
    Number(f64),
    String_(String),
    Array(Vec<Json>),
    Object(std::collections::HashMap<String, Json>),
}
```

写 `fn to_string(j: &Json) -> String` 把任意 `Json` 序列化成 JSON 字符串。**这是后续很多项目的基础**。

**练习 3.3**：定义 `enum BinaryTree { Empty, Node(i32, Box<BinaryTree>, Box<BinaryTree>) }`，写 `sum` / `height` / `contains` 三个方法。

**练习 3.4**：`while let` 练习。写一个函数，用 `Vec::pop` 把一个 `Vec<i32>` 倒序打印出来。

---

下一章：[04 · Option 与 Result ⭐ →](./04-option-result.md)
