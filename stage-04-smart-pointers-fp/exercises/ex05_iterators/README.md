# Exercise 05 · 迭代器组合子

> 难度：⭐⭐  涉及：第 4 章

## 任务

### 1) 基础链式

```rust
fn squares_of_even(v: Vec<i32>) -> Vec<i32> {
    v.into_iter().filter(|x| x % 2 == 0).map(|x| x * x).collect()
}
```

### 2) `fold` 求和

```rust
fn sum(v: &[i32]) -> i32 { v.iter().fold(0, |acc, &x| acc + x) }
fn product(v: &[i32]) -> i32 { v.iter().fold(1, |acc, &x| acc * x) }
```

### 3) zip / enumerate

```rust
fn pair_with_index(v: Vec<String>) -> Vec<(usize, String)> {
    // 保留非空项
    v.into_iter().enumerate().filter(|(_, s)| !s.is_empty()).collect()
}
```

### 4) chain 拼接

```rust
fn concat<T: Clone>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    a.into_iter().chain(b).collect()
}
```

### 5) 复杂：词频 Top K

```rust
use std::collections::HashMap;

fn top_k_words(text: &str, k: usize) -> Vec<(String, usize)> {
    // 1. split_whitespace
    // 2. map to_lowercase
    // 3. 统计 HashMap
    // 4. 转 Vec
    // 5. sort_by key 降序
    // 6. take(k)
}
```

> **不用**手写循环——全用迭代器。

### 6) peekable

```rust
fn take_while_increasing(v: &[i32]) -> Vec<i32> {
    // 取单调递增的最长前缀
    // 用 peekable：每步 peek 下一个决定要不要吃
}
```

## 验收

每题至少一个测试。

## 提示

- `enumerate()` 给 `iter` 加下标
- `chain` 不消耗 left
- 词频统计：`.fold` 或 `entry().or_insert(0)` 都可以
- `take_while` 已有标准实现，可以直接用——但 peekable 练习要自己写

## 进阶

写一个"分批处理"：

```rust
fn batches<T: Clone>(v: Vec<T>, n: usize) -> Vec<Vec<T>> {
    // 把 vec 切成每 n 个一组
    // [1,2,3,4,5,6,7] n=3 → [[1,2,3], [4,5,6], [7]]
}
```

完成 → [ex06_custom_iterator](../ex06_custom_iterator)
