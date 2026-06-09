# 07 · 借用（Borrowing）⭐

> **本章目标**：用 `&` / `&mut` 在**不转移所有权**的前提下让函数访问数据；理解借用检查器的"排他可变"规则。

## 7.1 引用 = 借用

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);   // &s1 创建一个引用（borrow）
    println!("'{s1}' has {len} letters");  // s1 仍然有效！
}

fn calculate_length(s: &String) -> usize {  // s 是 &String，是引用
    s.len()
}   // s 离开作用域，但它**没有**所有权，什么都不会 drop
```

> 💡 **把引用理解为"指针"**：它保存了值的地址。但比 C 指针安全——Rust 在编译期保证引用总是指向有效值。

```
    s1                          s（参数）
  ┌──────────┐              ┌──────────┐
  │ ptr ─────┼─────────────►│          │
  │ len: 5   │              └──────────┘
  │ cap: 5   │                ▲
  └──────────┘                │
                              └── &s1：指向 s1 的栈上 ptr
```

## 7.2 不可变引用 `&T`

```rust
let s = String::from("hello");
let r1 = &s;
let r2 = &s;
println!("{r1} {r2}");  // ✅ 可以同时有多个不可变引用
```

> 不可变引用只能"读"，不影响值，所以可以同时存在任意多个。

## 7.3 可变引用 `&mut T`

```rust
let mut s = String::from("hello");
let r = &mut s;
r.push_str(", world");
println!("{r}");
```

要点：
- 被借用的变量本身必须是 `mut`。
- 同一时刻**只能有一个** `&mut T`。
- 存在 `&mut T` 时，**不能再有 `&T`**（防止"读着读着别人改了"）。

```rust
let mut s = String::from("hello");
let r1 = &s;        // OK
let r2 = &s;        // OK
let r3 = &mut s;    // ❌ cannot borrow `s` as mutable because it is also borrowed as immutable
```

> 这就是借用检查器在保护你：**多人读可以，读+写不行；多人写更不行。**

## 7.4 借用的"作用域"与 NLL

Rust 2018 之后引入了 **NLL（Non-Lexical Lifetimes）**：引用的"存活期"只看**最后一次使用**的位置，不是花括号。

```rust
let mut s = String::from("hello");

let r1 = &s;
let r2 = &s;
println!("{r1} {r2}");   // r1, r2 最后一次使用
// 从这里开始，r1 r2 已经"死"了

let r3 = &mut s;         // ✅ OK，因为 r1 r2 已经不再使用
r3.push_str("!");
println!("{r3}");
```

> 早期 Rust（2015 edition）会报错。现在的代码只要遵循"最后一次使用"原则就行。

## 7.5 悬垂引用（Dangling Reference）

```rust
fn dangle() -> &String {           // ❌ 编译错
    let s = String::from("hi");
    &s                              // 返回 s 的引用
}   // s 在这里被 drop，引用指向已释放的内存
```

> 在 Rust 中，**编译器直接拒绝**这种代码。这是 Rust 比 C / C++ 强的关键点之一（C 要到运行时才崩）。

正确做法：把所有权返回去。

```rust
fn no_dangle() -> String {
    let s = String::from("hi");
    s     // 把所有权移给调用方
}
```

## 7.6 借用规则总结

> **当你不确定时，回到这三条：**
>
> 1. 同一时刻，**只能有一个 `&mut T`**，或者**任意多个 `&T`**，**不能同时**。
> 2. 引用必须总是**指向有效值**。
> 3. 引用本身不能比它的所有者活得更久。

## 7.7 把练习 6.2 改"漂亮"

练习 6.2 你大概是这样：

```rust
fn word_count(s: String) -> usize { s.len() }   // s 被 move 进函数，外面 s 失效
```

改用借用后：

```rust
fn word_count(s: &String) -> usize { s.len() }   // 只借不拿
```

调用：

```rust
let s = String::from("book");
let n = word_count(&s);            // &s 借给函数
println!("'{s}' has {n} letters"); // s 仍然有效
```

> 💡 **更推荐**直接用 `&str`（字符串切片，详见 [08-slices.md](./08-slices.md)），适用面更广：
> ```rust
> fn word_count(s: &str) -> usize { s.len() }
> ```

## 7.8 一个真实场景：修改变量

```rust
fn append_world(s: &mut String) {
    s.push_str(", world");
}

fn main() {
    let mut s = String::from("hello");
    append_world(&mut s);
    println!("{s}");   // hello, world
}
```

## 7.9 实战模式：迭代时修改

```rust
fn main() {
    let mut v = vec![1, 2, 3];
    for x in &mut v {
        *x += 10;
    }
    println!("{:?}", v);   // [11, 12, 13]
}
```

> `*x` 是**解引用**（类似 C 的 `*`）。`&mut v` 让迭代器给出 `&mut i32`，`*x += 10` 实际修改底层值。

## 7.10 借用检查器报错怎么读

最常见的几个：

| 报错 | 含义 | 修法 |
|------|------|------|
| `borrow of moved value: x` | x 之前被移走了 | 改用 `&x` / `x.clone()` |
| `cannot borrow x as mutable, as it is not declared mutable` | 想改 x 但没标 `mut` | 加 `let mut x` |
| `cannot borrow x as mutable because also borrowed as immutable` | 已经有 `&x` 还在 | 把不可变引用的作用域挪开，或重新组织代码 |
| `x does not live long enough` | 引用比所有者活得久 | 让所有者活得久，或把数据移出来 |
| `cannot return reference to local variable` | 悬垂引用 | 返回所有权或把数据移到外面 |

> 🛟 **心态**：编译器报错时，**先看建议（note: ...）**，Rust 编译器非常善于"我猜你想做的是 X，要不要这样？"。

## 7.11 对比其他语言

| 概念 | Rust | C / C++ | Java / Python / JS |
|------|------|---------|-------------------|
| 显式借用 | `&` / `&mut` | `&` / `*&`（不强制） | 不存在（隐式引用） |
| 别名 + 可变 | **编译期禁止** | 自由发挥（UB） | 自由发挥 |
| 悬垂引用 | **编译期禁止** | 自由发挥（UB） | GC 保证不会 |
| 性能开销 | 零 | 零 | 引用计数有开销，GC 有暂停 |

## 7.12 一段代码里看完整规则

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;       // r1: &String
    let r2 = &s;       // r2: &String（不可变可以并存）
    println!("{r1} {r2}");
    // r1 r2 死了

    let r3 = &mut s;   // ✅ 现在没人借用，可以可变借用
    r3.push_str("!!!");
    println!("{r3}");
}  // r3 死，s 死（drop 一次）
```

---

## 🏋️ 本章小练习

**练习 7.1**：修好下面所有编译错（**不要删除任何代码**）：

```rust
fn main() {
    let s = String::from("hello");
    change(&s);
}

fn change(s: &String) {
    s.push_str(", world");   // 想追加
}
```

**练习 7.2**：先看下面这段，**预测**哪些行会编译错：

```rust
let mut s = String::from("hi");
let a = &s;
let b = &mut s;
println!("{a} {b}");
```

**练习 7.3**：修好：

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &c) in bytes.iter().enumerate() {
        if c == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
```
——调用方传 `s = "Hello world"`，期望拿到 `"Hello"`。**注意函数签名里的 `&String`**，看 [下一章 08-slices.md](./08-slices.md) 如何用 `&str` 让它更通用。

**练习 7.4（悬垂引用练习）**：先看，再自己写一个被编译器拒绝的悬垂引用。

```rust
fn longest(x: &str, y: &str) -> str {  // 缺 lifetime，下面会讲
    if x.len() > y.len() { x } else { y }
}
```

---

下一章：[08 · 切片（Slice）→](./08-slices.md)
