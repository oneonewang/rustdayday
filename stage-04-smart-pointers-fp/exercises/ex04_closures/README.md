# Exercise 04 · 闭包

> 难度：⭐⭐  涉及：第 3 章

## 任务

### 1) 基础闭包

```rust
fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 { f(x) }
fn apply_mut<F: FnMut(i32) -> i32>(mut f: F, x: i32) -> i32 { f(x); f(x) }
fn apply_once<F: FnOnce(i32) -> i32>(f: F, x: i32) -> i32 { f(x) }
```

测试三种 trait 各传一个闭包。

### 2) 闭包捕获

```rust
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}
```

测试：add5(10) == 15。

### 3) 闭包当排序键

```rust
fn sort_by<T, F: FnMut(&T, &T) -> std::cmp::Ordering>(v: &mut [T], mut cmp: F) {
    // 自己实现简单排序（用 sort_by_cached_key 或手写 bubble）
    v.sort_by(cmp);
}
```

测试：用一个 `&str` 列表按长度排。

### 4) `filter_positive` 用迭代器 + 闭包

```rust
fn filter_positive(v: Vec<i32>) -> Vec<i32> {
    v.into_iter().filter(|&x| x > 0).collect()
}
```

### 5) 闭包捕获方式识别

为以下每个闭包说明它捕获了什么（borrow / mut borrow / move）：

```rust
let s = "hi".to_string();
let a = || println!("{s}");             // ?
let mut s = "hi".to_string();
let b = || s.push('!');                  // ?
let s = "hi".to_string();
let c = move || println!("{s}");         // ?
```

## 验收

- 每个 `apply_*` 各一个测试
- `make_adder` 多组测试
- `filter_positive` 测试
- 闭包捕获的"答案"用注释写出来

## 提示

- `apply_mut` 调两次 `f(x)`，所以要 `FnMut`
- 闭包按"最小侵入"选择捕获方式

## 进阶

写一个 `Cache` struct，包装 `HashMap<K, V>` + `RefCell`，支持闭包驱动的"读时计算"：

```rust
struct Cache<K, V> { map: RefCell<HashMap<K, V>> }
impl<K: std::hash::Hash + Eq + Clone, V: Clone> Cache<K, V> {
    fn get_or_insert_with(&self, key: K, f: impl FnOnce() -> V) -> V
}
```

完成 → [ex05_iterators](../ex05_iterators)
