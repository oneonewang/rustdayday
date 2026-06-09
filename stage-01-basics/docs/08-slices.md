# 08 · 切片（Slice）

> **本章目标**：理解 `&str` 和 `&[T]`——一种**借用 + 范围** 的类型——以及为什么几乎所有"只读字符串参数"都该用 `&str`。

## 8.1 字符串切片 `&str`

```rust
let s = String::from("hello world");
let hello: &str = &s[0..5];     // "hello"
let world: &str = &s[6..11];    // "world"
let whole: &str = &s[..];        // "hello world"（整个）
```

语法 `&s[start..end]` 借用 `s` 的一段范围：

```
索引:   0 1 2 3 4 5 6 7 8 9 10
字符:   h e l l o   w o r l d
切片:         └────┘  └───────┘
             &s[0..5] &s[6..11]
```

要点：
- 切片是**胖指针**（fat pointer）：存了**地址**和**长度**。`&str = { ptr, len }`。
- 切片是**借用**，**不拥有**数据。`s` 释放时切片失效。
- 切片**不能修改**（`&str` 是不可变借用 `String`）。
- Range 写法：`a..b`（不含 b）、`a..=b`（含 b）、`..`（从头）、`a..`（到尾）。

## 8.2 字符串字面量的类型

```rust
let s: &str = "Hello, world!";
```

> 字面量的类型是 **`&'static str`**——一个静态生命周期的字符串切片。`"..."` 会被硬编码到二进制文件的只读段。

## 8.3 改造 7.3 的 `first_word`

把签名从 `&String` 改成 `&str`：

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        if b == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn main() {
    let s = String::from("hello world");
    println!("{}", first_word(&s));          // 用 &String → 触发 deref
    println!("{}", first_word("hi there"));  // 用 &str 直接
    println!("{}", first_word(&s[..]));     // 用 &str
}
```

> `&String` 自动转成 `&str` 的事叫 **deref coercion**，后面 Stage 3 会讲。现在记住：**函数接 `&str` 比 `&String` 灵活**。

## 8.4 其他切片：数组切片 `&[T]`

```rust
let a = [1, 2, 3, 4, 5];
let slice: &[i32] = &a[1..3];     // 借用 [2, 3]
println!("{:?}", slice);           // [2, 3]
assert_eq!(slice.len(), 2);
```

`&[i32]` 同样是个胖指针：指向数组第 i 个元素 + 长度。

```rust
fn sum(slice: &[i32]) -> i32 {
    slice.iter().sum()
}

fn main() {
    let v = vec![1, 2, 3];
    let a = [10, 20, 30];
    println!("{}", sum(&v));    // ✅ Vec 也能当 slice 用（deref coercion）
    println!("{}", sum(&a));    // ✅ 数组也能
    println!("{}", sum(&v[1..]));// 切片
}
```

> **规律**：函数接"一段同类型元素"，用 `&[T]` 比 `&Vec<T>` 灵活得多。

## 8.5 一个"切片借用整个容器"的细节

```rust
let mut v = vec![1, 2, 3, 4, 5];
let s = &v[1..3];      // 借用 v 的一部分
v.push(6);             // ❌ cannot borrow `v` as mutable because it is also borrowed as immutable
println!("{:?}", s);
```

为什么？看下 `Vec` 的内存：v.push 可能让 v 重新分配内存、复制元素、释放原内存——`s` 指向的旧位置就**悬垂**了。所以 Rust 禁止。

修法：先把 `s` 用完。

```rust
let mut v = vec![1, 2, 3, 4, 5];
let s = &v[1..3];
println!("{:?}", s);   // s 最后一次使用
v.push(6);             // ✅ OK
```

## 8.6 不能用索引取字符

```rust
let s = String::from("你好");
let h = &s[0];   // ❌ `String` cannot be indexed by `{integer}`
```

> Rust 字符串是 UTF-8 字节序列，索引 `s[0]` 拿到的是**第一个字节**，但"第一个字符"可能是多个字节（中文 3 字节）。Rust 直接禁了"按索引取字符"这个不安全操作。

要按字符遍历：

```rust
for c in "你好".chars() {
    println!("{c}");  // 你  好
}

for b in "你好".bytes() {
    println!("{b}");  // 228 189 160  (UTF-8 字节)
}
```

## 8.7 `str` vs `String` 一图流

```
字符串字面量:  "hello"          类型: &str（指向只读数据）
                ↓ String::from / to_string
             String            类型: String（拥有堆内存、可增长）
                ↓ &s[..]
             &str              类型: &str（借用 String）
```

| 类型 | 拥有 | 可变 | 存哪 | 何时用 |
|------|------|------|------|--------|
| `&'static str` | 否 | 否 | 二进制只读段 | 字面量 |
| `String` | ✅ | ✅ | 堆 | 拥有的、运行时构造的 |
| `&str` | 否 | 否 | 借来 | 函数参数、临时使用 |

> 💡 **默认**：函数签名要"只读字符串"时，**写 `&str`**。要"拥有并能改"时，**用 `String`**。

## 8.8 速查：Range 切片写法

```rust
let s = String::from("hello world");

&s[0..5]    // "hello"
&s[6..11]   // "world"
&s[..5]     // 等价于上面
&s[6..]     // 等价于上面
&s[..]      // 整个
&s[0..=4]   // "hello"（含 4）
&s[..=4]    // 等价
```

## 8.9 字符串的常见操作

```rust
let mut s = String::from("foo");
s.push(' ');              // 追加单字符
s.push_str("bar");        // 追加 &str
let combined = format!("{}-{}", s, 42);  // 不抢所有权
let s2 = s + "!";         // ❗ s 已被消耗，s2 是新 String
                            // 实际是 s + &str，返回新 String
s += "baz";               // 复合赋值，s = s + "baz"，s 还活着（重绑）
```

> `+` 运算符背后：`fn add(self, other: &str) -> String`——**左操作数 move 走**。这和 `i32 + i32`（Copy）不同，是 string 特有的。

## 8.10 对比其他语言

| 概念 | Rust | Go | Java | Python |
|------|------|-----|------|--------|
| 字符串类型 | `String` + `&str` | 单一 `string` | 单一 `String` | 单一 `str` |
| 内部编码 | UTF-8 强制 | UTF-8 | UTF-16 | 看实现 |
| 索引取字符 | ❌（要 `chars()`） | ❌（要先 `[]rune(s)`） | `s.charAt(0)` | ✅ `s[0]` |
| 切片 | `&s[1..3]` | `s[1:3]` | `s.substring(1,3)` | `s[1:3]` |
| 字符串不可变 | `&str` 不可变 | 否 | 否 | 否 |

---

## 🏋️ 本章小练习

**练习 8.1**：写 `first_word(s: &str) -> &str`，返回第一个空格之前的内容。如果没空格，返回整个字符串。

**练习 8.2**：写 `trim_spaces(s: &str) -> &str`，把首尾的 ASCII 空格去掉。**用切片实现，不分配新内存**。

**练习 8.3**：写 `middle(s: &str) -> &str`，返回奇数长度字符串最中间的那个字符的切片。例如 `middle("abcd")` 是 `""`，`middle("abcde")` 是 `"c"`。

**练习 8.4**：思考下面这段为什么能编过：
```rust
fn takes_str(s: &str) { println!("{s}"); }
let owned = String::from("hi");
takes_str(&owned);          // &String 自动转 &str
```
提示：暂时接受这个事实（**deref coercion**），Stage 3 我们会拆开讲。

---

下一章：[09 · 阶段复习与综合自测 →](./09-stage-review.md)
