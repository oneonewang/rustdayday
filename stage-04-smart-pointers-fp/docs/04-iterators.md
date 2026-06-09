# 04 · 迭代器深入（Iterators）

> **本章目标**：深入理解 `Iterator` trait，写出"惰性 + 链式 + 零分配"的数据处理流水线，实现自己的迭代器。

## 4.1 `Iterator` trait

```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;

    // 默认实现 70+ 方法...
}
```

> 实现 `next` 一个方法，**自动**获得 `map` / `filter` / `fold` / `sum` 等几十个适配器。

## 4.2 三种迭代方式

```rust
let v = vec![1, 2, 3];

v.iter()              // 产生 &T
v.iter_mut()          // 产生 &mut T
v.into_iter()         // 产生 T（消耗 v）
```

> ⚠️ 三种方式**都**实现 `Iterator`，但 `Item` 类型不同。

## 4.3 惰性求值（Lazy Evaluation）

> **重要概念**：迭代器适配器（`map` / `filter` 等）**不立即执行**——只在"消费"时才跑。

```rust
let v = vec![1, 2, 3, 4, 5];
let iter = v.iter().map(|x| { println!("mapping {x}"); x * 2 });
// 上面这行什么都不会打印！

let sum: i32 = iter.sum();   // 这里才开始执行
// 输出：mapping 1, mapping 2, mapping 3, mapping 4, mapping 5
// sum = 30
```

> 💡 这就是为什么链式可以"零分配"——中间没有 Vec 临时存储。

## 4.4 消费器（Consumer）

把迭代器"耗尽"得到结果：

| 方法 | 用途 | 返回 |
|------|------|------|
| `collect::<Vec<_>>()` | 收集到 Vec | `Vec<T>` |
| `sum::<i32>()` | 求和 | `i32` |
| `product::<i32>()` | 累乘 | `T` |
| `count()` | 计数 | `usize` |
| `fold(init, \|acc, x\| ...)` | 累计 | `T` |
| `for_each(\|x\| ...)` | 遍历副作用 | `()` |
| `min()` / `max()` | 找最大最小 | `Option<T>` |
| `any(\|x\| ...)` / `all(\|x\| ...)` | 谓词 | `bool` |
| `find(\|x\| ...)` | 找第一个 | `Option<T>` |
| `position(\|x\| ...)` | 找下标 | `Option<usize>` |
| `nth(n)` | 第 n 个 | `Option<T>` |
| `last()` | 最后一个 | `Option<T>` |

## 4.5 适配器（Adapter）

产生新迭代器：

| 方法 | 用途 |
|------|------|
| `map(\|x\| ...)` | 元素转换 |
| `filter(\|x\| ...)` | 过滤 |
| `filter_map(\|x\| ...)` | 过滤 + 映射（一步） |
| `take(n)` | 取前 n |
| `skip(n)` | 跳过前 n |
| `step_by(n)` | 步进 |
| `chain(other)` | 拼接 |
| `zip(other)` | 配对 |
| `enumerate()` | 加下标 |
| `rev()` | 反转（需 `DoubleEndedIterator`） |
| `cycle()` | 无限循环（需 `Clone`） |
| `inspect(\|x\| ...)` | 调试用，不改值 |
| `flat_map(\|x\| iter)` | 拍平 |
| `flatten()` | 拍平一层 |
| `cloned()` | `&T` → `T`（`T: Clone`） |
| `copied()` | `&T` → `T`（`T: Copy`） |
| `peekable()` | 加 `peek()` 能力 |
| `fuse()` | 迭代失败后永远返回 None |

## 4.6 完整例子

```rust
let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

let result: i32 = v.iter()
    .filter(|&&x| x % 2 == 0)        // 留偶数
    .map(|&x| x * x)                  // 平方
    .sum();                            // 求和
// 2^2 + 4^2 + 6^2 + 8^2 + 10^2 = 4+16+36+64+100 = 220
```

## 4.7 性能 vs 手写循环

```rust
// 显式循环
let mut sum = 0;
for x in &v {
    if x % 2 == 0 { sum += x * x; }
}

// 迭代器链
let sum: i32 = v.iter()
    .filter(|&&x| x % 2 == 0)
    .map(|&x| x * x)
    .sum();
```

> 两版**生成的机器码几乎一样**（LLVM 把迭代器内联）。**迭代器 = 高级语法 + 零开销**。

## 4.8 自定义 Iterator

```rust
struct Counter { count: u32 }

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let sum: u32 = Counter { count: 0 }.sum();   // 1+2+3+4+5 = 15
    println!("sum = {sum}");

    let v: Vec<u32> = Counter { count: 0 }.collect();
    assert_eq!(v, vec![1, 2, 3, 4, 5]);
}
```

> 实现 `next` 后，**所有**适配器 / 消费器**自动**可用。

## 4.9 实现其他相关 trait

```rust
struct Counter { count: u32 }

// 1. 默认精确大小
impl ExactSizeIterator for Counter {
    fn len(&self) -> usize { 5 - self.count as usize }
}

// 2. 反向迭代
impl DoubleEndedIterator for Counter {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            let v = 5 - self.count;
            self.count += 1;
            Some(v)
        } else {
            None
        }
    }
}
```

## 4.10 一个"零分配切片"迭代器

```rust
struct StrSplit<'a> {
    haystack: &'a str,
    delim: char,
}

impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.haystack.split_once(self.delim).map(|(before, after)| {
            self.haystack = after;
            before
        })
    }
}

fn main() {
    let s = "a,b,c,d";
    let parts: Vec<&str> = StrSplit { haystack: s, delim: ',' }.collect();
    println!("{:?}", parts);   // ["a", "b", "c", "d"]
}
```

> 全程**零分配**——每个 part 都是原 `s` 的切片。

## 4.11 状态机迭代器

```rust
enum State {
    Start,
    Middle,
    End,
}

struct StateMachine { state: State }

impl Iterator for StateMachine {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            State::Start  => { self.state = State::Middle; Some("start") }
            State::Middle => { self.state = State::End;    Some("middle") }
            State::End    => None,
        }
    }
}
```

## 4.12 真实场景：自定义 Range 步进

```rust
struct StepRange { curr: i32, end: i32, step: i32 }

impl Iterator for StepRange {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr < self.end {
            let v = self.curr;
            self.curr += self.step;
            Some(v)
        } else {
            None
        }
    }
}

fn main() {
    let s: Vec<i32> = StepRange { curr: 0, end: 10, step: 3 }.collect();
    // [0, 3, 6, 9]
}
```

## 4.13 高级：`fuse` / `peekable` / `chunks`

```rust
let v = vec![1, 2, 3, 4, 5];

// peekable：能"偷看下一个"
let mut iter = v.iter().peekable();
while let Some(&x) = iter.peek() {
    if *x > 3 { break; }
    println!("{}", iter.next().unwrap());
}

// chunks：按 N 个一组
for chunk in v.chunks(2) {
    println!("{:?}", chunk);   // [1, 2], [3, 4], [5]
}

// windows：按 N 个滑动
for w in v.windows(3) {
    println!("{:?}", w);      // [1,2,3], [2,3,4], [3,4,5]
}
```

## 4.14 迭代器"消失"的真相

迭代器适配器**不存数据**——它们**包装**上一个迭代器。`v.iter().map(...).filter(...)` 是 3 层薄包装，没分配。

```rust
// 适配器其实长这样（简化）
struct Map<I, F> { iter: I, f: F }
impl<I, F> Iterator for Map<I, F>
where
    I: Iterator,
    F: FnMut(I::Item) -> U,
{
    type Item = U;
    fn next(&mut self) -> Option<U> {
        self.iter.next().map(&mut self.f)   // 每次从 iter 拿一个
    }
}
```

## 4.15 对比其他语言

| 概念 | Rust | C++ | Java | Python |
|------|------|-----|------|--------|
| 迭代器 trait | `Iterator` | `iterator` | `Iterator` | `__iter__` / `__next__` |
| 惰性 | ✅ | ✅（视图） | ✅（Stream） | ✅（generator） |
| 零开销 | ✅ | ✅ | ❌（对象开销） | ❌ |
| 适配器 | `map` / `filter` | `transform` / `filter` | `map` / `filter` | `map` / `filter` |
| 消费 | `sum` / `collect` | `std::accumulate` | `reduce` | `sum` / `list` |

## 4.16 一个实战：文件行迭代

```rust
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn process_lines(path: &str) -> io::Result<usize> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let count = reader
        .lines()                                       // Lines<BufReader<File>>
        .filter_map(Result::ok)                        // 跳过读错误
        .filter(|line| !line.trim().is_empty())        // 跳空行
        .map(|line| line.to_lowercase())               // 标准化
        .count();

    Ok(count)
}
```

---

## 🏋️ 本章小练习

**练习 4.1**：写 `Counter` 迭代器（产出 1, 2, 3, ..., n），用 `take(5).sum()` 验证。

**练习 4.2**：链式练习：把 1..=100 里 3 的倍数平方求和（用 `filter` / `map` / `sum`）。

**练习 4.3**：实现 `MyFilter<I, P>`，**手工**写一个 filter 适配器（不调 `filter`），验证跟 `.filter()` 行为一致。

**练习 4.4**：实现一个**反向** Range：

```rust
struct RevRange { curr: i32, end: i32 }
impl Iterator for RevRange { ... }
impl DoubleEndedIterator for RevRange { ... }   // 顺便实现
```

**练习 4.5**：实现 `StrSplit` 迭代器（同本节示例），加 `DoubleEndedIterator` 支持。

**练习 4.6**（真实）：写一个"读 CSV 第一列"的函数，用迭代器组合子。

---

下一章：[05 · 智能指针深入 →](./05-smart-pointers-advanced.md)
