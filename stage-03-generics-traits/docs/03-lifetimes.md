# 03 · 生命周期（Lifetimes）⭐

> **本章目标**：理解为什么需要生命周期标注，掌握三种生命周期语法（函数 / 结构体 / 方法），熟悉**省略规则**，能看懂大多数 borrow check 报错。
>
> ⭐ **本章是 Rust 学习路上最大的坎**。**第一次接触看不懂是正常的**——多看几遍，多写几个例子，会突然"打通"。

## 3.1 为什么需要生命周期？

```rust
fn longest(x: &str, y: &str) -> &str {     // ❌ 编译错
    if x.len() > y.len() { x } else { y }
}
```

报错：

```
error[E0106]: missing lifetime specifier
 --> src/main.rs:1:27
  |
1 | fn longest(x: &str, y: &str) -> &str {
  |               ----     ----     ^^^^^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but
          there is no value for it to be borrowed from.
```

**问题**：编译器看了签名，不知道返回的 `&str` 是借 `x` 还是借 `y`，**没法保证返回值的引用活得够久**。

> 💡 生命周期**不是**改变引用真实活多久——而是**告诉编译器引用之间的"先后关系"**。

## 3.2 显式标注生命周期

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

读法：声明一个**生命周期参数** `'a`，让 `x` / `y` / 返回值都"活到 `'a` 那么久"。

```rust
fn main() {
    let a = String::from("hi");
    let result;
    {
        let b = String::from("hello");
        result = longest(&a, &b);     // ❌ a 活得比 b 久——但 b 是 'a 的一部分
    }
    println!("{}", result);            // 💥 b 已经 drop
}
```

> 修复：把 `b` 提到外面。

```rust
let a = String::from("hi");
let b = String::from("hello");
let result = longest(&a, &b);   // ✅ 都能活到 result 用完
println!("{}", result);
```

**编译器的承诺**：`'a` 选的是 `x` 和 `y` **都还活着**的最短那段——所以**只要 `'a` 还在，引用的对象都还活着**。

## 3.3 函数签名中的生命周期语法

```rust
fn func<'a>(x: &'a str) -> &'a str { x }
```

| 部分 | 含义 |
|------|------|
| `<'a>` | 声明生命周期参数（可多个：`<'a, 'b>`） |
| `&'a str` | 引用，**至少**活到 `'a` 结束 |
| `-> &'a str` | 返回的引用也活到 `'a` |

> ⚠️ 注意 `'a` 是**下界**——`&'a str` 实际可能活**更久**，但保证**至少** `'a`。

## 3.4 多个生命周期参数

```rust
fn longest_with_announcement<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    // 返回值借用 x，跟 y 没关系
    println!("比较 {} 和 {}", x, y);
    x
}
```

也可以这样：

```rust
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    if x.len() > y.len() { x } else { y }    // ❌ 不能返回 &'b str 当 &'a str
}
```

## 3.5 结构体中的生命周期

```rust
struct Excerpt<'a> {
    text: &'a str,           // 引用作为字段
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let e = Excerpt { text: first_sentence };
    println!("{}", e.text);
}
```

> 💡 结构体持有引用 = 结构体的生命周期**不超过**它引用的对象。

**多个字段**：

```rust
struct Pair<'a, 'b> {
    first: &'a str,
    second: &'b str,
}
```

## 3.6 生命周期省略规则（Elision Rules）

为了**不让所有函数都写满 `<'a>`**，Rust 编译器**推断**三种模式：

### 规则 1：每个引用参数**都**有一个独立的生命周期

```rust
fn foo(x: &str, y: &str)   // 隐式 =  fn foo<'a, 'b>(x: &'a str, y: &'b str)
fn foo(x: &mut str)        // 隐式 =  fn foo<'a>(x: &'a mut str)
```

### 规则 2：如果只有一个引用参数，它的生命周期赋给所有输出引用

```rust
fn first_word(s: &str) -> &str
// 隐式 =  fn first_word<'a>(s: &'a str) -> &'a str
```

### 规则 3：如果有 `&self` 或 `&mut self`（方法），self 的生命周期赋给所有输出引用

```rust
impl<'a> Excerpt<'a> {
    fn text(&self) -> &str { ... }   // 隐式 =  fn text<'a>(&'a self) -> &'a str
}
```

**如果三个规则都不够，编译错**——必须**手动**标注。

## 3.7 `'static` 生命周期

`'static` 是**最长寿**的生命周期——活到**程序结束**。

```rust
let s: &'static str = "I am a string literal";
```

> 字符串字面量是 `'static`：被硬编码到二进制文件的只读段。

**常见使用**：线程、全局数据、错误信息。

```rust
fn announce() -> &'static str { "Ready!" }
```

> ⚠️ **不要**为了"修编译错"就把所有东西标成 `'static`——99% 的情况你其实想用 `'a` + 调整作用域。

## 3.8 一个真实场景：返回 `&'a str` 而不是 `String`

```rust
// ❌ 多一次分配
fn first_word_long(s: &String) -> String {
    s.split(' ').next().unwrap().to_string()
}

// ✅ 零分配
fn first_word<'a>(s: &'a String) -> &'a str {
    s.split(' ').next().unwrap()
}
```

> 💡 这是 Rust 性能的一大利器——通过生命周期，**借用**而不是**复制**。

## 3.9 struct + 生命周期 + 泛型

```rust
struct Ref<'a, T> {
    value: &'a T,
}

impl<'a, T> Ref<'a, T> {
    fn new(value: &'a T) -> Self { Self { value } }
    fn get(&self) -> &T { self.value }
}
```

## 3.10 完整例子：迭代器返回引用

```rust
struct StrSplit<'a> {
    haystack: &'a str,
    delim: char,
}

impl<'a> StrSplit<'a> {
    fn new(haystack: &'a str, delim: char) -> Self {
        Self { haystack, delim }
    }
}

impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;        // Item 是关联类型（下一章详讲）

    fn next(&mut self) -> Option<Self::Item> {
        self.haystack.split_once(self.delim).map(|(before, after)| {
            self.haystack = after;
            before
        })
    }
}

fn main() {
    let s = "a,b,c,d";
    for part in StrSplit::new(s, ',') {
        println!("{part}");
    }
    // 输出 a, b, c, d
}
```

> 没有 `String` 分配！全部是原 `s` 的切片。

## 3.11 常见 borrow check 报错怎么读

| 报错 | 含义 | 修法 |
|------|------|------|
| `expected named lifetime parameter` | 函数返回引用但没说借谁 | 加 `<'a>` |
| `x does not live long enough` | 引用比所有者活得久 | 延长所有者，或缩短引用使用 |
| `cannot return value referencing local variable` | 返回了局部变量的引用 | 返回 `String` / 调整逻辑 |
| `lifetime mismatch` | 两个生命周期关系不对 | 加 `where 'a: 'b` 或重排代码 |
| `borrowed value does not live long enough` | 借的对象活得不够长 | 延长对象 / 缩短借用 |

## 3.12 三个实战模式

### 模式 1：内部数据结构借用外部数据

```rust
struct Parser<'a> {
    input: &'a str,
    pos: usize,
}
```

### 模式 2：让函数返回借用输入的引用

```rust
fn first_word(s: &str) -> &str { ... }    // 生命周期省略
```

### 模式 3：trait 方法借用 self

```rust
trait Shape {
    fn describe(&self) -> &str;            // 隐式 'a: 来自 &self
}
```

## 3.13 什么时候**不需要**写生命周期？

- **所有引用都是"传入并立即用"**——省略规则能搞定
- **结构体字段是 String 而不是 `&str`**——所有权帮你解决
- **直接返回 `String`**——自己拥有，不存在借用问题

## 3.14 一个"绕过生命周期"的常见做法：clone()

```rust
fn make_greeting(name: &str) -> String {
    format!("Hello, {}!", name)            // 分配新 String，不借
}
```

> 性能上稍微差，但**代码更简单**。学习阶段不丢人。

## 3.15 对比其他语言

| 概念 | Rust | C / C++ | Java | Go |
|------|------|---------|------|-----|
| 引用生命周期 | 编译期静态检查 | 程序员负责 | GC 兜底 | GC 兜底 |
| 显式标注 | `&'a T` | — | — | — |
| 悬垂引用 | 编译期禁止 | 运行时 UB | 不会发生 | 不会发生 |
| 运行时开销 | 零 | 零 | GC 暂停 | GC 暂停 |
| 内存安全 | 编译期保证 | 程序员保证 | GC | GC |

## 3.16 学习路径建议

1. 写 3-5 个 "expected named lifetime parameter" 错误的修复
2. 写 2-3 个带 `&str` 字段的 struct
3. 读完标准库 `str` / `Iterator` 的源码片段
4. 如果学完本章还卡——**去做练习 7.x（生命周期）**，**别跳过**

---

## 🏋️ 本章小练习

**练习 3.1**：修对：

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}
```

**练习 3.2**：写 `struct Excerpt<'a> { text: &'a str }`，实现 `new` 和 `level(&self) -> u8`（句子长度的对数级别，1-5 之外返回 0）。

**练习 3.3**：写 `fn first_three(s: &str) -> &str`（返回头 3 个字符；不到 3 个就全返）。

**练习 3.4**（重写 std 方法）：用生命周期重写 `str::split_once`：

```rust
fn my_split_once(s: &str, delim: char) -> Option<(&str, &str)>
```

**练习 3.5**（难点）：写一个 `Longest<'a>` struct：

```rust
struct Longest<'a> { s: &'a str }

impl<'a> Longest<'a> {
    fn new(s: &'a str) -> Self
    fn show(&self) -> &str          // 借 self
}
```

**练习 3.6**（必做）：把"struct + 生命周期 + 泛型"组合起来：

```rust
struct Holder<'a, T> {
    name: &'a str,
    value: &'a T,
}
```

实现 `new` / `get_name` / `get_value`。

**练习 3.7**（绕开法）：写一个**不**用生命周期的版本——所有字段都是 `String`。对比两个版本感受"借用 vs 拥有"的区别。

---

下一章：[04 · Trait Object（动态分发）→](./04-trait-objects.md)
