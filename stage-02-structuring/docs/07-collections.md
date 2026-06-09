# 07 · 集合：Vec / String 深入 / HashMap

> **本章目标**：熟练使用 `Vec<T>` / `String` / `HashMap<K, V>`，知道何时用哪个，掌握最常用方法 + 迭代器组合子入门。

## 7.1 `Vec<T>`：动态数组

### 创建

```rust
let v: Vec<i32> = Vec::new();              // 空
let v = vec![1, 2, 3];                     // 用宏（推断 i32）
let v = Vec::from([1, 2, 3]);              // 从数组
let mut v = Vec::with_capacity(10);        // 预分配容量
```

### 增删改

```rust
let mut v = vec![1, 2, 3];

v.push(4);                          // 追加
v.insert(1, 10);                    // 在下标 1 插入 10 → [1, 10, 2, 3, 4]
v.pop();                            // 弹出末尾 → 返回 Option(4)
v.remove(1);                        // 删下标 1 → [1, 2, 3]
v.retain(|&x| x > 1);               // 保留满足条件的
v.clear();
v.extend([10, 20, 30]);             // 拼接迭代器
v.append(&mut vec![100, 200]);      // 全部 move 过来
```

### 读

```rust
let v = vec![10, 20, 30];

let x = v[0];                       // 越界 panic
let x = v.get(0);                   // 越界返回 None  ← 推荐
let x = v.first();                  // Option<&T>
let x = v.last();                   // Option<&T>
let x = v.contains(&20);            // bool
```

### 长度

```rust
v.len();                            // 元素个数
v.is_empty();
v.capacity();                       // 已分配容量（≥ len）
```

### 迭代

```rust
let v = vec![1, 2, 3];

// 不可变引用
for x in &v { println!("{x}"); }

// 可变引用
for x in &mut v { *x *= 2; }

// 拿走所有权
for x in v { println!("{x}"); }     // v 之后不可用

// 带下标
for (i, x) in v.iter().enumerate() {
    println!("{i}: {x}");
}
```

### 排序

```rust
let mut v = vec![3, 1, 4, 1, 5, 9, 2, 6];
v.sort();                                       // 升序，i32 实现 Ord
v.sort_by(|a, b| b.cmp(a));                     // 降序
v.sort_by_key(|s| s.len());                     // 按 key 排
v.dedup();                                      // 去掉连续重复
```

> 不可变 `Vec` 排序？不行，**`sort` 需要 `&mut self`**。

### 二分查找

```rust
let mut v = vec![1, 3, 5, 7, 9];
match v.binary_search(&5) {
    Ok(i)  => println!("5 在下标 {i}"),
    Err(i) => println!("应该插入到 {i}"),
}
```

> ⚠️ **必须**先排序。

## 7.2 `String` 深入

### 创建

```rust
let s = String::new();                    // 空
let s = String::from("hello");
let s = "hello".to_string();
let s = format!("x = {x}, y = {y}");      // 不会 panic，分配新 String
```

### 增改

```rust
let mut s = String::from("foo");

s.push(' ');                  // 追加单字符
s.push_str("bar");            // 追加 &str
s += "baz";                   // 等价于 s = s + "baz"
s.insert(0, '>');             // 在下标 0 插入
s.insert_str(s.len(), "<");  // 在末尾追加
```

> `+` 运算符实际上是 `fn add(self, s: &str) -> String`——**左边 move 走，右边借用**。

### 删

```rust
let mut s = String::from("hello!");
s.pop();                              // 弹出末尾字符，返回 Option<char>，s = "hello"
s.truncate(3);                        // 截断到 3 字节，s = "hel"（注意 UTF-8）
s.remove(0);                           // 删下标 0 处的字符（按字节），s = "el"
s.clear();
```

> ⚠️ `truncate` / `remove` 都按**字节**下标，对中文可能切坏——小心。

### 查

```rust
let s = "hello world";

s.len();                          // 11（字节长度）
s.chars().count();                // 11（Unicode 标量个数）
s.is_empty();
s.contains("world");
s.starts_with("hello");
s.ends_with("world");
s.find("world");                  // Option<usize> 字节下标
```

### 切

```rust
let s = "hello world";
let hello: &str = &s[0..5];       // "hello" — 字节切片
let world = &s[6..];              // "world"

// 字符串 split
let parts: Vec<&str> = "a,b,c".split(',').collect();
let lines: Vec<&str> = "a\nb\nc".lines().collect();
let trimmed = "  hi  ".trim();    // "hi"
let trimmed_left = "  hi  ".trim_start();
```

### 替换

```rust
let s = "I like cats";
let s = s.replace("cats", "dogs");        // "I like dogs"
let s = s.replacen("a", "b", 1);          // 只替换前 1 次
```

### String vs &str

| 表达式 | 类型 |
|--------|------|
| `"literal"` | `&'static str` |
| `s.to_string()` / `String::from(s)` | `String`（拥有） |
| `&s[..]` / `&s` | `&str`（借用） |

> **函数参数**：要"只读"用 `&str`；要"拥有可改"用 `String`。
> **结构体字段**：要"拥有"用 `String`；要"借用"用 `&str` + 生命周期（Stage 3 详讲）。

## 7.3 `HashMap<K, V>`：哈希表

```rust
use std::collections::HashMap;

let mut m: HashMap<String, i32> = HashMap::new();
m.insert("alice".to_string(), 30);
m.insert("bob".to_string(), 25);
```

### 创建

```rust
let m = HashMap::from([
    ("alice", 30),
    ("bob", 25),
]);

// 收集 tuple 列表
let m: HashMap<_, _> = vec![("a", 1), ("b", 2)].into_iter().collect();
```

### 读写

```rust
let v = m.get("alice");               // Option<&V>
let v = m.get("alice").copied();      // Option<V>（如果 V: Copy）
let v = m.get("alice").unwrap_or(&0); // 默认值

m.insert("alice", 31);                // 覆盖
m.entry("alice").or_insert(0);        // 不存在才插入 0
```

> ⚠️ `HashMap` **不能**用 `m["alice"]` 这种索引——会**自动插入**。新手很容易踩坑，**总用 `.get()`**。

### 三个高频场景

#### A. 计数

```rust
let text = "hello world hello rust";
let mut counts: HashMap<&str, usize> = HashMap::new();

for word in text.split_whitespace() {
    *counts.entry(word).or_insert(0) += 1;
}
// counts = {"hello": 2, "world": 1, "rust": 1}
```

#### B. 累加（同 key 加和）

```rust
let pairs = vec![("a", 1), ("b", 2), ("a", 3)];
let mut m: HashMap<&str, i32> = HashMap::new();
for (k, v) in pairs {
    *m.entry(k).or_insert(0) += v;
}
```

#### C. 不存在时插入初值（`entry().or_insert_with(f)`）

```rust
let mut cache: HashMap<String, Vec<i32>> = HashMap::new();
let v = cache.entry("k".to_string()).or_insert_with(Vec::new);
v.push(1);
```

### 删除

```rust
m.remove("alice");       // Option<V>
m.retain(|k, v| *v > 0); // 保留满足条件的
m.clear();
```

### 迭代

```rust
for (k, v) in &m {
    println!("{k} = {v}");
}

// 只迭代 key 或 value
for k in m.keys() { ... }
for v in m.values() { ... }
for v in m.values_mut() { *v += 1; }
```

## 7.4 迭代器组合子入门（剧透 Stage 4 详讲）

`Vec` / `HashMap` 都能迭代，所以所有迭代器方法都能用。

```rust
let v = vec![1, 2, 3, 4, 5];

// map: 元素转换
let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();   // [2, 4, 6, 8, 10]

// filter: 筛选
let evens: Vec<&i32> = v.iter().filter(|x| *x % 2 == 0).collect();  // [&2, &4]

// fold: 累计
let sum: i32 = v.iter().fold(0, |acc, x| acc + x);   // 15

// sum（特化）
let total: i32 = v.iter().sum();

// any / all
let has_zero = v.iter().any(|&x| x == 0);    // false
let all_pos  = v.iter().all(|&x| x > 0);     // true

// find
let first_even = v.iter().find(|&&x| x % 2 == 0);   // Some(&2)
```

> 链式：

```rust
let text = "Hello, world! 你好, Rust!";
let words: Vec<&str> = text
    .split(|c: char| !c.is_alphanumeric())
    .filter(|s| !s.is_empty())
    .collect();
// ["Hello", "world", "你好", "Rust"]
```

## 7.5 一个真实场景：词频统计

```rust
use std::collections::HashMap;

fn word_count(text: &str) -> HashMap<String, usize> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for word in text.split_whitespace() {
        let word = word.to_lowercase();
        *counts.entry(word).or_insert(0) += 1;
    }
    counts
}

fn main() {
    let text = "The quick brown fox jumps over the lazy dog the";
    let counts = word_count(text);
    let mut sorted: Vec<_> = counts.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));   // 按频率降序
    for (word, n) in sorted {
        println!("{word:>5}  {}", "#".repeat(*n));
    }
}
```

输出：

```
  the  ###
  fox  #
 dog  #
 ...
```

## 7.6 `Vec` / `HashMap` 性能小贴士

| 场景 | 提示 |
|------|------|
| 已知大概长度 | `Vec::with_capacity(n)` 避免反复扩容 |
| 大量 `String` key | `HashMap<String, V>` 没问题；想要更快用 `&str` 借（但有生命周期） |
| 需要稳定迭代顺序 | 用 `IndexMap`（`indexmap` crate）或 `BTreeMap` |
| 需要排序的 Map | `BTreeMap<K, V>` |
| 高频小 Map | 默认 `HashMap`（基于 hashbrown / SipHash）够用 |

## 7.7 对比其他语言

| 集合 | Rust | C++ | Java | Python | Go |
|------|------|-----|------|--------|-----|
| 动态数组 | `Vec<T>` | `std::vector` | `ArrayList` | `list` | `slice` |
| 字符串 | `String` / `&str` | `std::string` | `String` | `str` | `string` |
| 哈希表 | `HashMap<K,V>` | `unordered_map` | `HashMap` | `dict` | `map` |
| 有序映射 | `BTreeMap<K,V>` | `map` | `TreeMap` | — | — |
| 索引越界 | panic | UB | `IndexOutOfBoundsException` | `IndexError` | panic |

---

## 🏋️ 本章小练习

**练习 7.1**：写 `group_by_parity(v: Vec<i32>) -> (Vec<i32>, Vec<i32>)`，返回（偶数列表，奇数列表）。

**练习 7.2**：写 `top_n(m: HashMap<String, i32>, n: usize) -> Vec<(String, i32)>`，按 value 降序取前 n 个。

**练习 7.3**：写一个 `WordIndex` 结构：

```rust
struct WordIndex { map: HashMap<String, Vec<usize>> }

impl WordIndex {
    fn from_text(text: &str) -> Self { ... }
    fn positions(&self, word: &str) -> Option<&Vec<usize>> { ... }
}
```

支持从一段文本构建索引，查询某词出现过的所有下标。

**练习 7.4**（用组合子）：写一个函数 `count_words_starting_with_vowel(text: &str) -> usize`——把文本拆词、过滤、计数。

**练习 7.5**（真实小项目）：写一个命令行 `wc` 简化版，接受文件路径作为参数，打印 `行数 词数 字节数`。
提示：`std::env::args()` 拿参数；`fs::read_to_string()` 读文件；`str::lines()` 计行；`split_whitespace()` 计词；`len()` 计字节。

---

下一章：[08 · 阶段复习与综合自测 →](./08-stage-review.md)
