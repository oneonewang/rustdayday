# 08 · Stage 2 复习与综合自测

> **本章目标**：把 Stage 2 串成一张图，做 10 道综合题。**通过再进 Stage 3。**

## 8.1 知识地图

```
Stage 2 概念图
════════════════════════════════════════════════════════════════

   数据描述                行为                  错误处理
   ───────                ────                  ───────
   struct (具名/Tuple/Unit)  impl 块              panic! / assert
   方法  &self / &mut self   关联函数 ::new        Result<T, E>
   关联函数 self             多个 impl              ? 运算符
   字段初始化简写 ..                          From<E> 转换
                                                     │
   分类与状态              错误是值                  thiserror
   ──────────              ────────                  ─────
   enum 简单                不用 try/catch           #[derive(Error)]
   enum 携带数据  ◀── 强！   match 必须穷尽           #[from]
   match 穷尽                if let 单分支             #[error("...")]
   if let / while let        let ... else
   matches! 宏                组合子
   @ 绑定 / 守卫              map / and_then
   嵌套模式                  ok_or / ok
                            Box<dyn Error>
                                  │
                                  ▼
                       ┌──────────────────────┐
                       │  代码组织（模块）      │
                       │  • crate / package    │
                       │  • mod / pub / use    │
                       │  • 拆文件 mod foo;    │
                       │  • pub use 重导出    │
                       │  • crate:: 绝对路径   │
                       └──────────────────────┘
                                  │
                                  ▼
                       ┌──────────────────────┐
                       │  常用集合             │
                       │  • Vec<T>             │
                       │  • String + &str     │
                       │  • HashMap<K,V>      │
                       │  • 迭代器组合子入门   │
                       └──────────────────────┘
```

## 8.2 一句话回顾

- **struct** = 给元组字段起名字；**方法 = 第一个参数是 `self` 的函数**。
- **enum** = 分类 + 每类带数据；**`match` 强制穷尽**。
- **`Option<T>`** 替 `null`；**`Result<T, E>`** 替异常；**`?` 替 try/catch**。
- **模块** = `mod` 声明 + `pub` 控可见性 + `use` 引路径 + 文件拆分。
- **集合**：读用 `.get()`（返 `Option`），不用 `[i]`（越界 panic）；迭代器是"链式数据处理管道"。

## 8.3 综合自测（10 题）

每题先想、再敲。**必须能编译并产生预期结果**。

### 题 1：颜色 enum
定义 `enum Color { Rgb(u8, u8, u8), Hsl(u16, u8, u8) }`，写一个 `to_string` 方法：
- `Color::Rgb(255, 0, 0)` → `"rgb(255, 0, 0)"`
- `Color::Hsl(0, 100, 50)` → `"hsl(0, 100, 50)"`

### 题 2：几何体计算
写一个 `enum Shape { Circle(f64), Rect(f64, f64) }`，实现：
- `area(&self) -> f64`
- `perimeter(&self) -> f64`

### 题 3：Vec 统计
写一个 `stats(v: &[i32]) -> Option<(i32, i32, f64)>`，返回 `(min, max, average)`，空切片返回 `None`。

### 题 4：分桶
写一个 `bucketize(v: Vec<i32>, n: usize) -> Vec<Vec<i32>>`，把元素按"下标 mod n"分到 n 个桶里。

### 题 5：HashMap 反转
写一个 `invert(m: HashMap<K, V>) -> HashMap<V, Vec<K>>`（K、V 都要实现 `Hash + Eq + Clone`），把 value 当 key，相同 value 的所有 key 收集到一个 Vec。

### 题 6：链式 StringBuilder（Stage 2.4 练习题）
```rust
struct StringBuilder { ... }
impl StringBuilder {
    fn new() -> Self
    fn append(self, s: &str) -> Self     // 拿所有权
    fn build(self) -> String
}
```

### 题 7：选项 enum
定义：
```rust
enum Cmd { Add(i32), Sub(i32), Reset, Print }
```
写一个 `apply(state: &mut i32, cmd: Cmd)`，根据命令修改 state 或打印：
- `Add(n)`：`state += n`
- `Sub(n)`：`state -= n`
- `Reset`：`state = 0`
- `Print`：打印 state

### 题 8：找最长的方法名（链式 + 闭包）
写 `longest_method_name(v: &[&str]) -> Option<&str>`，返回最长字符串（`None` 空切片）。**用 `iter().max_by_key(...)`**。

### 题 9：错误传播
写 `read_numbers(path: &str) -> Result<Vec<i32>, AppError>`，从文件读数字（每行一个），用自定义 `AppError` enum 区分 `Io` 和 `Parse` 错误。**不要**用 `unwrap`。

### 题 10：模块化
把下面这段代码拆成两个文件 `lib.rs` 和 `geometry.rs`：
```rust
struct Circle { radius: f64 }
impl Circle { fn area(&self) -> f64 { std::f64::consts::PI * self.radius * self.radius } }
struct Rect { w: f64, h: f64 }
impl Rect { fn area(&self) -> f64 { self.w * self.h } }
```
要求：
- `lib.rs` 暴露 `pub mod geometry;` 和 `pub use geometry::{Circle, Rect};`
- 调用方可以 `use mycrate::{Circle, Rect};`

## 8.4 答案要点

| 题 | 关键点 |
|----|--------|
| 1 | `match self` 两个分支 |
| 2 | `match self` 求值 |
| 3 | 空判断 + `iter().min/max` + 求和除以 len |
| 4 | `v.into_iter().enumerate()` + `mod n` |
| 5 | 注意 `entry().or_insert_with(Vec::new).push(k.clone())` |
| 6 | 每个方法拿所有权返回 `Self` |
| 7 | 模式匹配每种变体 |
| 8 | `v.iter().max_by_key(\|s\| s.len()).copied()` |
| 9 | `#[derive(thiserror::Error)]` + `#[from]` + `?` |
| 10 | `mod.rs` 与子文件的对应 |

## 8.5 自测通过标准

- 10 题中至少 **8 题** 在 **45 分钟内** 一次写对。
- 所有 [`exercises/`](./../exercises) 和 [`project-02-cli-todo/`](./../project-02-cli-todo) 能 `cargo run` 通过。

## 8.6 阶段回顾清单

> 在进入 Stage 3 之前，自问自答：

- [ ] struct 三种形态会写
- [ ] `impl` 块的写法、`self` 的三种形态
- [ ] enum 携带数据 + `match` 穷尽
- [ ] `if let` / `while let` / `let else` 区别和适用场景
- [ ] `Option<T>` / `Result<T, E>` 都会用
- [ ] `?` 运算符、错误传播
- [ ] 会写自定义错误（用 `thiserror` 或手写）
- [ ] 模块：声明 / `pub` / `use` / 拆文件
- [ ] `Vec` / `String` / `HashMap` 常用方法熟
- [ ] 迭代器链式：`map` / `filter` / `collect` / `sum`

如果某条不确定，回去重读对应章节。

## 8.7 推荐复习间隔

> - 写完 1 天后做 8.3 自测
> - 进入 Stage 3 之前再做一遍
> - Stage 4 之前最后一遍（Rust 基础就稳了）

---

🎉 Stage 2 完！准备好后告诉我开始 Stage 3（泛型、Trait、生命周期——Rust 抽象能力的核心）。
