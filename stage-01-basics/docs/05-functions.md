# 05 · 函数

> **本章目标**：会写函数，理解 Rust 的"语句 vs 表达式"——这是和很多语言不一样的地方。

## 5.1 基本语法

```rust
fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    let r = add(1, 2);
    println!("r = {r}");
}
```

- 关键字 `fn`
- 参数必须**显式标注类型**（`x: i32, y: i32`）——这和变量不同（变量可推断）
- 返回值用 `-> Type`
- 函数体最后一行**没有分号**——因为它是**表达式**

> ⚠️ 注意：Rust 用 **snake_case** 命名函数和变量（`my_function`），**PascalCase** 命名类型和 trait（`MyStruct`、`MyTrait`）。编译器对 `non_snake_case` 会警告。

## 5.2 语句（Statement） vs 表达式（Expression）

这是 Rust 非常重要的区分：

| 概念 | 含义 | 是否有值 | 例子 |
|------|------|----------|------|
| **语句** | 执行动作 | ❌ | `let x = 5;` |
| **表达式** | 求值产生一个值 | ✅ | `x + 1`、`if cond { a } else { b }`、`{ ... }`（块） |

**关键点**：Rust 中**几乎一切都是表达式**——`if`、`match`、块 `{}`、`loop`（配合 `break value`）都能返回值。

```rust
let y = {
    let x = 3;
    x + 1        // 注意：没有分号
};

println!("y = {y}");  // 4
```

> 块表达式的值 = 最后一个**没有分号**的表达式。如果加了 `;`，就变成了"语句"，整个块返回 `()`。

```rust
let y = {
    let x = 3;
    x + 1;       // ❌ 加了分号，块变成 ()
};
//  ^^^^ expected `()`, found integer
```

> 💡 **这解释了为什么函数返回值不能加分号**。`x + y` 写成 `x + y;` 函数就返回 `()`，与 `-> i32` 不匹配。

## 5.3 提前返回

```rust
fn abs(x: i32) -> i32 {
    if x < 0 { return -x; }   // 显式 return
    x                          // 隐式返回（最后表达式）
}
```

`return` 可以提前结束函数，返回 `()` 类型的函数可以省 `return` 和 `;`：

```rust
fn say_hello(name: &str) {
    println!("Hello, {name}");
}
```

## 5.4 没有默认参数 / 没有可变参数

```rust
fn greet(name: &str) { ... }
greet("world");
greet();                  // ❌
greet("a", "b");          // ❌
```

想要"可选"或"多变"参数？请用：
- `Option<T>` 模拟可选参数
- trait `From` 把多种类型转为函数参数类型
- 或 `&[T]` 接收变长列表

这些 Stage 2-3 会涉及。

## 5.5 多个返回值：用元组

```rust
fn min_max(xs: &[i32]) -> (i32, i32) {
    let mut min = i32::MAX;
    let mut max = i32::MIN;
    for &x in xs {
        if x < min { min = x; }
        if x > max { max = x; }
    }
    (min, max)
}

fn main() {
    let xs = [3, 1, 4, 1, 5, 9, 2, 6];
    let (lo, hi) = min_max(&xs);
    println!("min={lo}, max={hi}");
}
```

## 5.6 嵌套 vs 模块化

```rust
// 内联（不推荐大项目里这么用）
fn outer() {
    fn inner() { println!("inner"); }
    inner();
}
```

实际项目里通过 **模块** 拆文件（Stage 2 详讲）。函数可以写在**任何**作用域里。

## 5.7 注释

```rust
// 行注释，注释到行尾

/*
   块注释
   可以跨行
*/

/// 文档注释，**会**出现在 `cargo doc` 生成的文档里
/// Markdown 格式
/// 
/// # Example
/// ```
/// let s = my_crate::add(1, 2);
/// assert_eq!(s, 3);
/// ```
fn add(a: i32, b: i32) -> i32 { a + b }

//! 库级文档注释，写在 lib.rs 顶部
```

## 5.8 提前预告：发散函数

```rust
fn never_returns() -> ! {
    panic!("I give up");
}
```

`!` 是 **never 类型**，表示函数永不返回（panic 或无限循环）。Stage 2 还会再见。

## 5.9 对比其他语言

| 特性 | Rust | Go | JavaScript | Python |
|------|------|-----|-----------|--------|
| 函数参数类型 | 必填 | 必填 | 不需要 | 不需要 |
| 多返回值 | 元组 | 多返回值 | 数组/对象 | 元组 |
| 函数是一等公民 | ✅ | ✅ | ✅ | ✅ |
| 默认参数 | ❌（用 `Option`） | ❌ | ✅ | ✅ |
| 闭包 | ✅ | ✅（func） | ✅（箭头） | ✅（lambda） |
| 命名返回值 | ❌ | ✅（可命名返回值） | ❌ | ❌ |

---

## 🏋️ 本章小练习

**练习 5.1**：写 `celsius_to_fahrenheit(c: f64) -> f64` 和 `fahrenheit_to_celsius(f: f64) -> f64`，公式 `F = C * 9/5 + 32`。

**练习 5.2**：写 `factorial(n: u64) -> u64`，**分别用循环和递归实现两份**。注意：递归版本会撞上栈溢出，你可以加一行 `if n > 20 { panic!("n too big for recursion") }`。

**练习 5.3**：写 `divmod(a: i32, b: i32) -> (i32, i32)`，返回商和余数。**注意 b == 0 的情况**——本阶段可以 `panic!`，Stage 2 会教你用 `Result`。

**练习 5.4（坑）**：下面这段代码有什么问题？修一下：
```rust
fn plus_one(x: i32) -> i32 {
    x + 1;
}
```

---

下一章（重头戏）：[06 · 所有权 →](./06-ownership.md)
