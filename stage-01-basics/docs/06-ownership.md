# 06 · 所有权（Ownership）⭐

> **本章目标**：彻底理解 Rust 的"所有权"概念。**这是 Rust 和其他语言最大的差异点**——也是 Rust 能在没有 GC 的情况下保证内存安全的核心机制。

## 6.1 栈（Stack）vs 堆（Heap）

| | 栈 | 堆 |
|---|----|----|
| 速度 | 极快（指针移动） | 慢（要分配、找空位） |
| 数据 | 固定大小、生命周期清晰 | 任意大小、生命周期不定 |
| 分配 | 进函数时自动 / 出函数时自动 | 显式 `malloc` / Rust 中是 `Box::new`、`String::from` 等 |
| 释放 | 随作用域 | 需要某种机制回收（GC / RC / 手动 / 编译期） |

> Rust 的核心思想：**谁负责释放堆内存？** 答案是"拥有这块内存的变量"。**变量离开作用域，内存就立刻被释放。** 这就避免了 GC 的运行时开销，也避免了 C 的手动错误。

## 6.2 所有权三规则（必须背下来）

1. **每个值都有一个所有者（owner）变量。**
2. **一次只能有一个所有者。**
3. **所有者离开作用域，值就被丢弃（drop）。**

## 6.3 变量的作用域

```rust
fn main() {
    {                      // s 在这里无效（未声明）
        let s = "hello";   // s 从这里开始有效
        // 使用 s
    }                      // 作用域结束，s 不再有效
}
```

## 6.4 `String`：第一个堆上分配的类型

前面用过字符串字面量 `"hello"`，类型是 `&str`（**不可变、编译期知道、硬编码在二进制中**）。要可变 / 要在运行时构造，用 `String`：

```rust
let mut s = String::from("hello");
s.push_str(", world!");
println!("{s}");    // hello, world!
```

`String` 怎么释放？`String` 类型由三部分组成：
- 指针（指向堆上的字节）
- 长度（用了多少字节）
- 容量（申请了多少字节）

这三部分**存放在栈上**（固定大小），实际内容**存放在堆上**。

```rust
{
    let s = String::from("hello"); // 申请堆内存
    // 使用 s
}  // 作用域结束：自动调用 drop，释放堆内存
```

> 💡 在 C++ 中这叫 **RAII**（Resource Acquisition Is Initialization）。Rust 把这个模式作为语言基石。

## 6.5 Move 语义（重点！）

```rust
let s1 = String::from("hello");
let s2 = s1;            // ❓ 发生了什么？
println!("{s1}");        // ❌ 编译错：borrow of moved value: `s1`
```

**不是**浅拷贝，**也不是**深拷贝——是 **move（移动）**。

> 解释：把 `s1` 赋值给 `s2` 时，**栈上的三件套**（指针/长度/容量）被复制了一份，但**堆上的字节没有复制**。为了避免**双重释放**（drop 两次），Rust 让 `s1` 立即**失效**。这就是"移动"——所有权的转移。

```
       s1 (栈)              s2 (栈)
  ┌──────────────┐    ┌──────────────┐
  │ ptr ─────────┼─┐  │ ptr ─────────┼─┐
  │ len: 5       │ │  │ len: 5       │ │
  │ cap: 5       │ │  │ cap: 5       │ │
  └──────────────┘ │  └──────────────┘│
                   ▼                   ▼
       ┌──────────────────────┐
       │  h  e  l  l  o       │  ← 同一块堆内存
       └──────────────────────┘
```

> 浅拷贝（C 风格）会导致 `s1` 和 `s2` 都指向同一块内存，离开作用域时**双重释放** 💥。Rust 通过**让 `s1` 失效**避免了这个坑。

### 隐式 move 的位置

```rust
let s = String::from("hi");
takes_ownership(s);              // s 的所有权被移进函数
println!("{s}");                  // ❌ s 已经没有值了

fn takes_ownership(s: String) {   // s 进函数时成为新所有者
    println!("{s}");
}                                 // s 离开作用域，drop
```

返回值也会转移所有权：

```rust
fn gives_ownership() -> String {  // 把所有权转给调用方
    String::from("hi")
}

let s = gives_ownership();        // s 拿到所有权
```

## 6.6 Copy 类型（与 Move 相反）

不是所有类型都走 move。**任何实现了 `Copy` trait 的类型，在赋值时是真正的位拷贝，旧变量仍然可用。**

```rust
let x = 5;
let y = x;            // 复制（不是 move）
println!("x = {x}, y = {y}");   // ✅ 都有效
```

哪些类型是 `Copy`？—— **所有"旧变量重新使用毫无意义"的类型**：
- 整数（`i32`、`u8` ...）
- 浮点
- 布尔
- 字符 `char`
- 元组（**前提是里面所有元素都是 `Copy`**）—— `(i32, f64)` 是，`(i32, String)` 不是
- 数组（同上）

> 💡 规律：**分配在栈上的小东西**基本都是 `Copy`；**持有堆内存的**（`String`、`Vec<T>`、`Box<T>`）都不是 `Copy`。

**一旦类型实现了 `Drop`（要释放资源），就不能再 `Copy`**。`String` 有 `Drop`，所以不是 `Copy`。

## 6.7 Clone：显式深拷贝

如果你确实想**复制堆上的数据**：

```rust
let s1 = String::from("hello");
let s2 = s1.clone();           // 深拷贝：堆上的字节也复制了
println!("s1 = {s1}, s2 = {s2}");   // ✅ 两者都有效
```

> ⚠️ `clone()` 可能很贵（要分配新内存 + 复制数据），别无脑用。**大多数时候 move 才是对的**。

## 6.8 Copy 与 Move 的设计哲学

| 类型分类 | 默认行为 | 因为 |
|----------|----------|------|
| 整数、bool、char、小元组等栈数据 | `Copy`（位拷贝） | 复制几乎零成本，且"复制"语义清晰 |
| `String`、`Vec<T>`、`Box<T>` 等持有堆内存 | `Move`（转移所有权） | 复制要分配堆内存，昂贵；move 让"原变量失效"避免双重释放 |

> **当你不确定**：编译器会告诉你。"borrow of moved value" / "use of moved value" 是最常见的入门报错。

## 6.9 函数传值和返回值的本质

```rust
fn main() {
    let s = String::from("hello");  // s 进 main 作用域

    takes_ownership(s);             // s 移进函数；s 失效

    let x = 5;                      // x 进 main 作用域
    makes_copy(x);                  // x 是 i32，Copy，x 仍可用
} // x 离开作用域（什么都不做）; s 已经移走了

fn takes_ownership(s: String) {     // s 进函数
    println!("{s}");
} // s 离开作用域，drop

fn makes_copy(n: i32) {             // n 进函数
    println!("{n}");
} // n 离开作用域（什么都不做）
```

## 6.10 一个"所有权进进出出"的真实例子

```rust
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)   // 还得把 s 还回去，不然调用方就拿不到了
}

fn main() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);   // s1 移进；s2 是还回来的
    println!("'{s2}' 的长度是 {len}");
}
```

> 很丑对吧？所有权进了函数，函数还得还出来。**这就是"借用"要解决的问题** —— 见下一章 [07-borrowing.md](./07-borrowing.md)。

## 6.11 对比其他语言

| 内存管理 | Rust | C | C++ | Java/Go/Python/JS |
|----------|------|---|-----|-------------------|
| 方式 | 编译期 ownership | 手动 `malloc`/`free` | RAII / 智能指针 | GC |
| 双重释放 | 编译期阻止 | 运行时崩溃 | 智能指针 + 警惕 | 不会发生 |
| Use after free | 编译期阻止 | 运行时崩溃 | 智能指针 + 警惕 | 不会发生 |
| 内存泄漏 | 仍可能（`Rc` 循环引用）| 容易 | 智能指针循环引用 | GC 不会泄漏 native 内存 |
| 运行时开销 | 几乎为零 | 几乎为零 | 几乎为零 | GC 暂停 |

## 6.12 速查决策树

"我要把一个值传给另一个变量/函数/容器，应该……"

```
这个值是 Copy 类型吗？（i32、bool、f64、char、这些组成的元组/数组）
├── 是 → 直接 =，旧变量仍可用
└── 否 → 默认为 Move
         ├── 还想用旧值？用 .clone() 深拷贝
         └── 不想给所有权？用 & 借引用（下一章）
```

---

## 🏋️ 本章小练习

**练习 6.1**：判断以下每行代码哪些能编译、哪些不能，**先想再敲**：

```rust
let x = 5;
let y = x;
println!("{x} {y}");                  // ?

let s1 = String::from("hi");
let s2 = s1;
println!("{s1} {s2}");                // ?

let s3 = s2.clone();
println!("{s2} {s3}");                // ?

let a = (1, String::from("a"));
let b = a;                            // ?
println!("{:?}", a);                  // ?

fn consume(_s: String) {}
let s = String::from("x");
consume(s);
consume(s);                            // ?
```

**练习 6.2**：修好下面这段代码（**不能改函数签名**）：

```rust
fn main() {
    let s = String::from("book");
    let n = word_count(s);
    println!("'{s}' has {n} letters");
}

fn word_count(s: String) -> usize {
    s.len()
}
```

**练习 6.3**：把练习 6.2 改造成"返回 `(String, usize)`" 的版本——体会为什么这种写法很烦，**为下一章铺垫**。

---

下一章：[07 · 借用（Borrowing）⭐ →](./07-borrowing.md)
