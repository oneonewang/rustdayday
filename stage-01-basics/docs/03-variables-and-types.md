# 03 · 变量与数据类型

> **本章目标**：理解 Rust 的**不可变默认**、**shadowing**、**类型推断**，以及所有标量 / 复合类型。

## 3.1 不可变性是默认值（重点！）

```rust
fn main() {
    let x = 5;
    println!("x = {x}");
    x = 6;  // ❌ 编译错：cannot assign twice to immutable variable
}
```

Rust 的变量**默认不可变**（immutable）。这是和大多数语言最大的不同之一：

| 语言 | 默认 | 改成可变的方式 |
|------|------|----------------|
| Rust | 不可变 | `let mut x = 5;` |
| Go | 可变 | `const` 不可变 |
| JavaScript / Python / Java | 可变 | `const` / `final` 不可变 |

> 💡 **为什么？** 不可变 = 推理简单、并发安全、容易并行。Rust 把"安全默认"做成了语言层面的强制约定。

要可变，加 `mut`：

```rust
let mut x = 5;
x = 6;  // ✅
```

## 3.2 Shadowing（变量遮蔽）

Rust 允许用 `let` 重新声明同名变量，**新绑定会遮蔽**（shadow）旧的：

```rust
fn main() {
    let x = 5;
    let x = x + 1;          // 遮蔽，新 x = 6
    let x = x * 2;          // 遮蔽，新 x = 12
    println!("x = {x}");    // 12
}
```

跟 `mut` 的关键区别：

| 特性 | `let` shadowing | `mut` |
|------|----------------|-------|
| 类型可以变 | ✅ `let x = "5"; let x = x.len();` | ❌ 同一变量类型不能变 |
| 是否分配新内存 | 是（重新绑定） | 否（修改原内存） |
| 是否仍可变 | 不一定 | 仍可变 |

shadowing 在**转换类型**时特别有用：

```rust
let spaces = "   ";            // &str
let spaces = spaces.len();     // usize
println!("{spaces}");          // 3
```

## 3.3 标量类型（Scalar）

Rust 有 4 类标量：整数、浮点、布尔、字符。

### 整数

| 长度 | 有符号 | 无符号 |
|------|--------|--------|
| 8  | `i8`  | `u8`  |
| 16 | `i16` | `u16` |
| 32 | `i32` | `u32` |
| 64 | `i64` | `u64` |
| 128 | `i128` | `u128` |
| arch | `isize` | `usize` |

> `isize` / `usize` 的位数跟 CPU 架构走（64 位机就是 64 位）。**集合长度和索引**默认用 `usize`。

数字字面量：

```rust
let a = 98_222;        // 十进制，_ 是分隔符
let b = 0xff;          // 十六进制
let c = 0o77;          // 八进制
let d = 0b1111_0000;   // 二进制
let e: u8 = b'A';      // 字节字面量
```

> 字面量没指定类型时，**默认 `i32`**。需要别的类型要显式标注。

### 浮点

`f32`（单精度）和 `f64`（双精度），**默认 `f64`**（和大多数语言不同——Go 默认 f32）。

```rust
let x = 2.0;       // f64
let y: f32 = 3.0;  // 显式 f32
```

### 布尔

`true` / `false`，类型 `bool`，占用 1 字节。

### 字符

`char` 是 **4 字节 Unicode 标量值**（U+0000 ~ U+D7FF / U+E000 ~ U+10FFFF），用单引号：

```rust
let c = 'z';
let z: char = 'ℤ';
let heart_eyed_cat = '😻';
```

> ⚠️ **注意**：`char` 不是字节。Rust 字符串底层是 UTF-8 字节序列（不是 char 序列），索引字符串要小心——这关系到 Stage 2 字符串处理。

## 3.4 复合类型（Compound）

### 元组（Tuple）

**固定长度**、**可装不同类型**：

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);

let (x, y, z) = tup;          // 解构
println!("y = {y}");          // 6.4

let five_hundred = tup.0;     // 索引
let one = tup.2;
```

> 元组**超过 12 个元素**编译器会嫌你啰嗦，建议用 struct。

### 数组（Array）

**固定长度**、**同类型**，**分配在栈上**：

```rust
let a = [1, 2, 3, 4, 5];
let months = ["January", "February", "March"];

let a: [i32; 5] = [1, 2, 3, 4, 5];      // 类型 + 长度
let a = [3; 5];                          // [3, 3, 3, 3, 3]
```

> ⚠️ **和 `Vec<T>` 的区别**：数组长度写死，编译期确定；`Vec` 是堆上可变长的。绝大多数情况下你用 `Vec`。

越界访问会**运行时 panic**：

```rust
let a = [1, 2, 3];
let element = a[10];   // 💥 thread 'main' panicked at 'index out of bounds'
```

## 3.5 类型推断与标注

Rust 编译器很聪明，能从上下文推断类型：

```rust
let x = 5;          // 推断为 i32
let v = vec![1, 2]; // 推断为 Vec<i32>
```

但**有时候**必须显式标注。比如同一字面量被多种类型使用：

```rust
let guess: u32 = "42".parse().expect("Not a number!");
//  ^^^^^^^^ 没有 : u32 会编译失败
```

或者参数多态的边界：

```rust
fn twice<T>(x: T) -> T { ... }   // T 由调用方决定
```

## 3.6 一个对照表：Rust vs 其他语言

| 概念 | Rust | C | Go | Java | Python/JS |
|------|------|---|-----|------|-----------|
| 整数默认 | `i32` | `int` (32) | `int` (64) | `int` (32) | 不分 |
| 浮点默认 | `f64` | `double` | `float64` | `double` | `float` |
| 字符串 | `&str`（切片）和 `String`（拥有） | `char*` | `string` | `String` | `str` |
| 可变性 | 默认可变要加 `mut` | 默认可变 | 默认可变 | 默认可变 | 默认可变 |
| 数组长度 | 编译期固定 | 编译期固定 | slice 长度可变 | 定长 | 长度可变 |
| 空值 | 没有 `null`，用 `Option<T>` | `NULL` | `nil` | `null` | `None` / `null` |

---

## 🏋️ 本章小练习

**练习 3.1**：写出下面这段代码，看 Rust 报什么错：
```rust
fn main() {
    let x = 5;
    x = x + 1;
    println!("x = {x}");
}
```
把 `x` 改成 `let mut x`，看错误消失。

**练习 3.2**：用 shadowing 把一个字符串 `"42abc"` 转换成一个 `i32` 数字（用 `parse`）。提示：解析会失败，要 `unwrap()` 或 `expect()`。

```rust
fn main() {
    let s = "42abc";
    // 你的代码：把 s 通过 shadowing 转成 i32
}
```

**练习 3.3**：定义一个数组 `[1, 2, 3, 4, 5]`，用一个 for 循环（先提前用，Stage 1 第 4 章会详讲）打印每个元素的平方。

---

下一章：[04 · 控制流 →](./04-control-flow.md)
