# Exercise 04 · 生命周期

> 难度：⭐⭐⭐  涉及：第 3 章

## 任务

### 1) 修对

```rust
fn longest(x: &str, y: &str) -> &str {        // ❌
    if x.len() > y.len() { x } else { y }
}
```

加 `<'a>` 显式标注。

### 2) 自己写 `first_word`

```rust
fn first_word(s: &str) -> &str {
    // 第一个空格之前的部分
}
```

**不要**返回 `String`——必须是 `&str`。

### 3) struct 持引用

```rust
struct Row<'a> {
    id: u32,
    name: &'a str,
}

impl<'a> Row<'a> {
    fn new(id: u32, name: &'a str) -> Self
    fn name(&self) -> &str               // 借 self
}
```

测试：
```rust
let s = String::from("alice");
let r = Row::new(1, &s);
assert_eq!(r.name(), "alice");
```

### 4) 多生命周期

```rust
fn mix<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    x
}
```

`y` 没用——只是演示多生命周期参数语法。

### 5) 静态生命周期

```rust
fn app_name() -> &'static str { "rustdayday" }
```

测试：调用方拿到 `&'static str` 可以放任何地方。

### 6) 难一点

写一个 `LongestWith<'a, 'b>` struct：

```rust
struct LongestWith<'a, 'b> {
    s1: &'a str,
    s2: &'b str,
}

impl<'a, 'b> LongestWith<'a, 'b> {
    fn new(s1: &'a str, s2: &'b str) -> Self
    fn longest(&self) -> &str     // 借 self，返回较长那个的引用
    fn first(&self) -> &str       // 返回 s1
}
```

> `longest()` 的返回类型：**借 self**（不是 `'a` 也不是 `'b`），因为 `self` 的生命周期就是 `'a + 'b` 的交集。

## 验收

每个函数/方法至少一个测试。**重点是** `longest()` 和 `first()` 必须**能编译**，且测试逻辑对。

## 提示

- `first()` 的签名：`fn first(&self) -> &str`——省略规则 + self 的生命周期，编译器能推断
- `longest()` 也类似，但**要注意**它返回的是 s1 还是 s2 的引用——编译器会根据 match 分支自动选

## 进阶

写一个"返回较长单词"的函数（复用 `first_word`）：

```rust
fn longest_word(s1: &str, s2: &str) -> &str      // 在两个串里找最长单词
```

> 跟 first_word 组合：用 first_word 在每个串里找第一个单词，比较长度，返回较长的那个的引用。

完成 → [ex05_operator_overload](../ex05_operator_overload)
