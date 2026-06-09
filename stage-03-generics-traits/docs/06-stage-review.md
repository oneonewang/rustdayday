# 06 · Stage 3 复习与综合自测

> **本章目标**：把 Stage 3 串成图，做 10 道综合题。**通过再进 Stage 4。**

## 6.1 知识地图

```
Stage 3 概念图
════════════════════════════════════════════════════════════════

   泛型                Trait              生命周期 ⭐
   ────                ─────              ───────
   fn f<T>(...)        trait T { ... }    fn f<'a>(x: &'a T) -> &'a T
   struct S<T>         impl T for X       struct S<'a> { x: &'a T }
   enum E<T>           trait bound        'a, 'b, 'static
   impl<T>             T: Bound           省略规则
   单态化              默认方法           关系（不是真实寿命）
   零运行时开销        关联类型
                       where 子句
                            │
                            ▼
                  ┌─────────────────────┐
                  │  Trait Object        │
                  │  • dyn Trait         │
                  │  • Box<dyn Trait>   │
                  │  • 对象安全          │
                  │  • 动态分发          │
                  └─────────────────────┘
                            │
                            ▼
                  ┌─────────────────────┐
                  │  高级 Trait          │
                  │  • 关联类型          │
                  │  • 运算符重载        │
                  │  • supertrait        │
                  │  • newtype 模式      │
                  │  • 完全限定语法      │
                  │  • 孤儿规则          │
                  └─────────────────────┘
```

## 6.2 一句话回顾

- **泛型** = "对任意类型都能用"，编译期单态化，**零运行时开销**。
- **Trait** = 行为契约；`T: Bound` 约束泛型；**默认实现 + 关联类型**让它比 interface 更强。
- **生命周期** = 标注"引用之间谁活得更久"；**省略规则**让多数代码不用写。
- **`dyn Trait`** = 运行时多态；`Box<dyn Trait>` 用于存进 `Vec` / struct。
- **newtype 模式** = 绕开孤儿规则的"标准武器"。

## 6.3 综合自测（10 题）

### 题 1：泛型函数

```rust
fn pick_first<T>(v: &[T]) -> Option<&T> { ... }
```

空切片返回 `None`。要求 `T: Copy` 才能用——**修对**。

### 题 2：泛型 struct

写 `Stack<T>` 实现 `push` / `pop` / `peek` / `len`，再加一个**仅**为 `T: Default` 的方法 `clear_and_push_default(&mut self)`。

### 题 3：trait + 实现

```rust
trait AsJson {
    fn to_json(&self) -> String;
}

impl AsJson for i32 { ... }
impl AsJson for String { ... }
impl AsJson for bool { ... }
```

写 `Vec<Box<dyn AsJson>>` 容器，演示"不同类型都当 JSON 看"。

### 题 4：trait bound 条件方法

```rust
struct Wrapper<T> { value: T }
impl<T> Wrapper<T> {
    fn new(v: T) -> Self
    fn into_inner(self) -> T
}
impl<T: std::fmt::Display> Wrapper<T> {
    fn print(&self)
}
```

写调用方测试：`Wrapper::new(42).print()` 能编，`Wrapper::new(vec![1, 2]).print()` 编不过。

### 题 5：生命周期

修对：

```rust
fn longest_word(s: &str) -> &str {
    s.split_whitespace().max_by_key(|w| w.len()).unwrap()
}
```

并写 `fn longest_word_in(s1: &str, s2: &str) -> &str`（在两个字符串中找最长单词）。

### 题 6：结构体 + 生命周期

```rust
struct Row<'a> {
    id: u32,
    name: &'a str,
}

impl<'a> Row<'a> {
    fn new(id: u32, name: &'a str) -> Self
    fn rename(&mut self, new_name: &'a str)   // 改名
}
```

写 `let mut r = Row::new(1, "alice"); r.rename("bob");`。

### 题 7：dyn Trait

写一个 `Command` trait（`fn execute(&self)`），为 `AddCommand` / `SubCommand` / `PrintCommand` 实现，然后写一个 `Runner` struct 装 `Vec<Box<dyn Command>>` 并 `run_all`。

### 题 8：运算符重载

定义 `Meters(f64)` 和 `Seconds(f64)`，实现 `Meters / Seconds = Mps(f64)`（自定义 `Mps` 类型）。用 `std::ops::Div`。

### 题 9：newtype 绕开孤儿

为 `std::num::Wrapping<u32>` 实现 `std::fmt::Display`——`Wrapping` 是标准库的，不能直接 `impl Display for Wrapping<u32>`，**必须** newtype。

### 题 10：综合

写 `struct List<T> { head: Option<Box<Node<T>>> }`（单链表），Node 持有 `T` 和 `Option<Box<Node<T>>>`。实现 `push` / `pop` / `len` / `iter(&self) -> Iter<T>`（用 `&Node<T>`）。

## 6.4 答案要点

| 题 | 关键点 |
|----|--------|
| 1 | `T: Copy` 才能用——`v.first().copied()` 拿值，或 `*v.first()?` |
| 2 | 两个 `impl` 块；第二个用 `T: Default` |
| 3 | 各种类型 `to_json` 返回 String；`Box<dyn AsJson>` 装不同类型 |
| 4 | rustc 自动报错；Vec 没有 Display |
| 5 | 省略规则搞定：`fn longest_word(s: &str) -> &str` |
| 6 | 改 name 后老的引用会失效——小心使用 |
| 7 | `Box<dyn Command>` 必须对象安全 |
| 8 | `impl Div<Seconds> for Meters { type Output = Mps; ... }` |
| 9 | `struct MyWrap(Wrapping<u32>); impl Display for MyWrap { ... }` |
| 10 | 递归类型必须 Box；iter 单独写 `Iter<'a, T>` 持有 `&'a Node<T>` |

## 6.5 通过标准

- 10 题中 **8 题** 在 60 分钟内一次写对
- 所有 [`exercises/`](./../exercises) 和 [`project-03-lru-cache/`](./../project-03-lru-cache) 能 `cargo build` 通过

## 6.6 阶段回顾清单

> 进入 Stage 4 之前自问：

- [ ] 泛型函数 / struct / enum 都会写
- [ ] 知道单态化是"零开销"的关键
- [ ] trait 定义 + impl + 默认方法
- [ ] trait bound 和 where 子句
- [ ] 关联类型 vs 泛型参数的区别
- [ ] 生命周期语法和省略规则
- [ ] 结构体里持有引用要加生命周期
- [ ] `'static` 是什么、什么时候用
- [ ] `dyn Trait` 和 `Box<dyn Trait>` 的用法
- [ ] 运算符重载通过实现 std trait
- [ ] supertrait 和 newtype 模式
- [ ] 孤儿规则和绕开法

某条不确定就回去重读。

## 6.7 推荐复习间隔

> - 写完 1 天后做 6.3 自测
> - 进入 Stage 4 之前再做一遍
> - 1 个月后再做（生命周期、Trait 这些概念需要反复过）

---

🎉 Stage 3 完！准备好后告诉我开始 Stage 4（智能指针 / 闭包 / 迭代器——Rust 函数式风格与所有权延伸）。
