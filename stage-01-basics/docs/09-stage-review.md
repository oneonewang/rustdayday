# 09 · Stage 1 复习与综合自测

> **本章目标**：把 Stage 1 串起来，画出"知识地图"，做几道综合题自测。**通过自测再进 Stage 2。**

## 9.1 知识地图

```
Stage 1 概念图
═══════════════════════════════════════════════════════

 工具链                语法基础             类型系统
 ──────                ────────             ───────
 rustup                let / let mut        i32, f64, bool, char
 cargo                 shadowing            tuple, [T; N]
 cargo new / run       if (无括号)          usize, isize
 cargo check / test    loop/while/for       字符串字面量 = &str
 rustfmt / clippy      fn (snake_case)      String vs &str
                       表达式 vs 语句
                            │
                            ▼
  ┌─────────────────────────────────────────────────┐
  │         所有权（Ownership）—— Rust 灵魂            │
  │  • 每个值一个 owner                                │
  │  • 一次一个 owner                                  │
  │  • 离开作用域 → drop                               │
  └─────────────────────────────────────────────────┘
                            │
              ┌─────────────┴────────────┐
              ▼                          ▼
        Copy 类型                   Move 语义
   (i32, bool, char, ...)        (String, Vec, Box, ...)
       赋值是位拷贝                旧变量失效，避免双重释放
                                          │
                                          ▼
                                ┌─────────────────────┐
                                │  借用（Borrowing）   │
                                │  • &T  不可变可多    │
                                │  • &mut T 排他      │
                                │  • NLL 作用域         │
                                │  • 不允许悬垂         │
                                └─────────────────────┘
                                          │
                                          ▼
                                ┌─────────────────────┐
                                │   切片（Slice）      │
                                │  • &str / &String   │
                                │  • &[T] / &Vec<T>   │
                                │  • 函数参数用 &str   │
                                └─────────────────────┘
```

## 9.2 一句话回顾

- **`let x = ...`** 默认不可变。
- **`String`** 在堆上；**`&str`** 是借用。
- **赋值 = Move**（对堆数据），**赋值 = Copy**（对栈小数据）。
- **`&` / `&mut`** 是借；**不能同时多人写**。
- **函数参数**：要"读"用 `&` / `&str` / `&[T]`；要"拥有"才传 `String` / `Vec`。

## 9.3 综合自测题（10 题）

每题先想、再敲。**编译不过也算错**——必须能编译并产出预期结果。

### 题 1：FizzBuzz
打印 1..=100：
- 3 的倍数打印 `Fizz`
- 5 的倍数打印 `Buzz`
- 15 的倍数打印 `FizzBuzz`
- 其他打印数字

### 题 2：温度转换函数
```rust
fn celsius_to_fahrenheit(c: f64) -> f64 { /* F = C * 9/5 + 32 */ }
fn fahrenheit_to_celsius(f: f64) -> f64 { /* ... */ }
```
- `celsius_to_fahrenheit(0.0)` 应得 `32.0`
- `celsius_to_fahrenheit(100.0)` 应得 `212.0`

### 题 3：斐波那契
```rust
fn fib(n: u32) -> u64 {
    // 0, 1, 1, 2, 3, 5, 8, 13, 21, ...
}
```
- `fib(0) == 0`
- `fib(1) == 1`
- `fib(10) == 55`
- **只用循环，不要递归**（避免大 n 栈溢出）

### 题 4：第一个单词
```rust
fn first_word(s: &str) -> &str { /* ... */ }
```
- `first_word("hello world")` 应得 `"hello"`
- `first_word("hello")` 应得 `"hello"`
- `first_word(" a")` 应得 `""`

### 题 5：字符串反转（用 slice）
```rust
fn reverse(s: &str) -> String { /* 不用 reverse 函数 */ }
```
- `reverse("abc")` 应得 `"cba"`
- `reverse("你好rust")` 应得 `"tsur好你"`（验证 UTF-8 处理）

### 题 6：所有权转移
看代码，**说出输出**或**指出编译错**：

```rust
fn main() {
    let s = String::from("hi");
    let s2 = s;
    println!("{s}");
    println!("{s2}");
}
```

### 题 7：借用修复
修对：

```rust
fn main() {
    let mut s = String::from("hello");
    let r = &s;
    s.push_str(" world");
    println!("{r}");
}
```

### 题 8：找最大值
```rust
fn largest(slice: &[i32]) -> Option<i32> { /* 没元素返回 None */ }
```
**注意**：本阶段**没**学 `Option`，先用一个特殊值（比如 `i32::MIN` 当哨兵）凑合。Stage 2 再回来用 `Option` 重写。

### 题 9：Vec 求平均
```rust
fn average(v: &[f64]) -> f64 {
    // v 为空时返回 0.0
}
```

### 题 10：所有权进函数又出来
写一个函数，把传入的字符串前后各加一个 `"*"` 字符，返回新 `String`：
```rust
fn add_stars(s: &str) -> String {
    // 期望 add_stars("hi") == "*hi*"
}
```

## 9.4 答案要点（先做完再对照）

| 题 | 关键点 |
|----|--------|
| 1 | `match n % 15 / n % 3 / n % 5` 优先判断 15 |
| 2 | 直接公式；浮点用 `f64` |
| 3 | `let (mut a, mut b) = (0, 1); for _ in 0..n { let c = a+b; a=b; b=c; }` |
| 4 | `bytes.iter().enumerate().find(...)` |
| 5 | 先 `chars().rev().collect()`，再 `String::from` |
| 6 | 编译错：`s` 已经被 `s2` 拿走 |
| 7 | 把 `let r = &s;` 改成 **在 `push_str` 之后** 再借 |
| 8 | `if slice.is_empty() { return i32::MIN; } *slice.iter().max().unwrap()` |
| 9 | `if v.is_empty() { 0.0 } else { v.iter().sum::<f64>() / v.len() as f64 }` |
| 10 | `format!("*{s}*")` 一行解决 |

完整参考代码见 [`exercises/`](./../exercises) 各目录。

## 9.5 如果你卡住了

按这个顺序求助：

1. **重新读相关文档章节**——大多数问题在某一节里都明说了。
2. **看 `cargo build` 报错**——Rust 编译器是"教学型"的，会告诉你哪个文件、哪一行、违反了哪条规则、可能想做什么。
3. **跑 `cargo clippy`**——会比 `cargo build` 多给一些风格 / 习惯建议。
4. **Google 报错信息**——这些报错被无数人搜过。
5. **问 AI / 社区**：[users.rust-lang.org](https://users.rust-lang.org/)、[r/rust](https://www.reddit.com/r/rust/)、[Rust Discord](https://discord.gg/rust-lang)。

## 9.6 自测通过标准

- 10 题中至少 **8 题** 在 **30 分钟内** 一次写对（不需要试错）。
- 所有 [`exercises/`](./../exercises) 和 [`project-01-guess-game/`](./../project-01-guess-game) 能 `cargo run` 出预期结果。

如果你做到了，恭喜——**Rust 最有特点的内容你已经学完了**。Stage 2 开始进入"工程化"：struct、enum、模块、错误处理、集合。

## 9.7 Stage 1 推荐复习间隔

> 隔 1 天不看，ownership 就会开始模糊。建议：
> - 写完代码 1 天后做 9.3 自测
> - 2 周后再做一遍（进入 Stage 2 之前）
> - 1 个月后做最后一遍

---

🎉 Stage 1 完！准备好后告诉我开始 Stage 2。
