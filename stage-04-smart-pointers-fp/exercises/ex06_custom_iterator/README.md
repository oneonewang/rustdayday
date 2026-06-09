# Exercise 06 · 自定义 Iterator

> 难度：⭐⭐⭐  涉及：第 4 章

## 任务

### 1) `FibIter`

```rust
struct FibIter { curr: u64, next: u64 }

impl Iterator for FibIter {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        // 产出 curr，迭代
    }
}
```

测试：`(0..).zip(FibIter { curr: 0, next: 1 }).map(|(_, f)| f).take(10).collect::<Vec<_>>() == [1, 1, 2, 3, 5, 8, 13, 21, 34, 55]`。

### 2) `StrSplit`

```rust
struct StrSplit<'a> { haystack: &'a str, delim: char }

impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        // 用 split_once
    }
}
```

测试：`"a,b,c".chars().filter(|c| *c != ' ')` 等等。验证 split 后还能再用迭代器。

### 3) `Cycle`（无限循环）

```rust
struct Cycle<T> { curr: usize, items: Vec<T> }

impl<T: Clone> Iterator for Cycle<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // curr 转到末尾时归零
    }
}
```

测试：take(7).collect()。

### 4) `Counter` 进阶

实现：

```rust
struct Counter { count: u32 }

impl Counter {
    fn new() -> Self { Self { count: 0 } }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        // 0, 1, 2, 3, ... 无限
    }
}
```

`Counter::new().zip(Counter::new().skip(1)).take(5).map(|(a, b)| a * b).sum()` == ?

### 5) 实现 `DoubleEndedIterator`

```rust
struct RevDigits { n: u32 }
impl RevDigits {
    fn new(n: u32) -> Self { Self { n } }
}

impl Iterator for RevDigits {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.n == 0 { None } else {
            let d = self.n % 10;
            self.n /= 10;
            Some(d)
        }
    }
}

impl DoubleEndedIterator for RevDigits {
    fn next_back(&mut self) -> Option<Self::Item> {
        // 反向：最高位优先
    }
}
```

## 验收

- FibIter 产出前 10 项
- StrSplit 切分 + collect
- Cycle 7 个元素的循环
- Counter 计算
- RevDigits 双向

## 提示

- 状态保存：迭代器本身 `&mut self`
- 返回 None 表示"耗尽"
- 实现 `next` 后**自动**拿到所有 70+ 适配器

## 进阶

实现一个**状态机迭代器**：

```rust
enum State { Start, SawPlus, SawNumber, End }

struct Tokenizer { input: Vec<char>, pos: usize, state: State }

impl Iterator for Tokenizer {
    type Item = String;     // 产出 token
    fn next(&mut self) -> Option<Self::Item> { ... }
}
```

完成 → [ex07_smart_pointers_advanced](../ex07_smart_pointers_advanced)
