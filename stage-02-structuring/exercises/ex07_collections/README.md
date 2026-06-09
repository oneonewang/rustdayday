# Exercise 07 · 集合

> 难度：⭐⭐  涉及：第 7 章

## 任务

### 1) Vec 基础

```rust
fn filter_positive(v: Vec<i32>) -> Vec<i32>
fn sort_desc(v: &mut Vec<i32>)
fn dedup_sorted(v: &mut Vec<i32>)     // 输入已排序
```

### 2) String 操作

```rust
fn reverse_words(s: &str) -> String
// "hello world" -> "world hello"

fn is_palindrome(s: &str) -> bool
// 忽略大小写、忽略非字母数字字符
// "A man, a plan, a canal: Panama" -> true
```

### 3) HashMap 统计

```rust
fn char_count(s: &str) -> HashMap<char, usize>
// "hello" -> {'h':1, 'e':1, 'l':2, 'o':1}

fn top_k_words(text: &str, k: usize) -> Vec<(String, usize)>
// 词频前 k 名，按词频降序
```

### 4) 综合：简化版 `wc`

写一个 `wc` 函数，统计字符串里的：
- 行数
- 词数（按 `split_whitespace`）
- 字节数

```rust
fn wc(text: &str) -> (usize, usize, usize)  // (lines, words, bytes)
```

### 5) 写一个 `WordIndex`（数据结构练习）

```rust
struct WordIndex { map: HashMap<String, Vec<usize>> }

impl WordIndex {
    fn from_text(text: &str) -> Self
    fn positions(&self, word: &str) -> Option<&Vec<usize>>
    fn unique_words(&self) -> usize
}
```

测试：
```rust
let text = "the quick brown fox jumps over the lazy dog the";
let idx = WordIndex::from_text(text);
assert_eq!(idx.positions("the"), Some(&vec![0, 6, 9]));  // 下标
assert_eq!(idx.unique_words(), 8);
```

## 验收

- 写一个 `main` 跑所有测试，输出每个测试的实际值

## 提示

- `chars()` 拿 Unicode 标量
- `is_alphanumeric()` 来自 `char` 类型
- `HashMap` 迭代时**借用** key/value：`for (k, v) in &map`
- 找 Top K：`into_iter().collect::<Vec<_>>()` + `sort_by` + `truncate`

## 进阶

写一个 `Grouping` 函数：用 `entry().or_insert_with(Vec::new)` 把 list 按奇偶分到两组。

完成 → [project-02-cli-todo](../project-02-cli-todo) 见！
