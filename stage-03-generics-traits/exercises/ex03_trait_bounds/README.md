# Exercise 03 · trait bound 与 where

> 难度：⭐⭐  涉及：第 2 章

## 任务

### 1) 多个 trait bound

写一个泛型 `T: Display + Clone` 的 `print_twice<T>(x: T)` 函数。

### 2) where 子句

把下面这种"行内 bounds 很长"的代码改写成 `where` 形式：

```rust
fn complex<T, U>(t: &T, u: &U) -> String
where
    T: std::fmt::Display + std::fmt::Debug + Clone,
    U: std::fmt::Debug + Clone,
{
    // 你的实现
}
```

### 3) 条件式 impl

```rust
struct Wrapper<T> { value: T }

impl<T> Wrapper<T> {
    fn new(value: T) -> Self { Self { value } }
}

impl<T: std::fmt::Display> Wrapper<T> {
    fn print(&self) {
        println!("value = {}", self.value);
    }
}

impl<T: Clone> Wrapper<T> {
    fn duplicate(&self) -> Self {
        Self { value: self.value.clone() }
    }
}
```

写调用方测试：
- `Wrapper::new(42).print()` 应工作
- `Wrapper::new(vec![1, 2, 3])` 不能调 `.print()`（Vec 没有 Display）

### 4) 函数返回 `impl Trait`

```rust
fn make_iter() -> impl Iterator<Item = i32> {
    1..=5
}
```

测试：`.collect::<Vec<_>>()` 得 `[1, 2, 3, 4, 5]`。

### 5) 函数接 `impl Trait`（简化签名）

```rust
fn summarize(s: &impl std::fmt::Display) {
    // 等价于 fn summarize<T: Display>(s: &T)
}
```

## 验收

每个函数写测试；最后一个"条件式 impl"验证：i32 调 `print` 能编，Vec 调 `print` 编不过。

## 进阶

写一个 `pick<T: PartialOrd + Copy>(v: &[T], cmp: impl Fn(T, T) -> std::cmp::Ordering) -> Option<T>`，根据自定义比较器挑出"最好"的元素。

> `Fn(T, T) -> Ordering` 比 `PartialOrd` 更灵活——比如可以按"距离 0 远近"比 `i32`。

完成 → [ex04_lifetimes](../ex04_lifetimes)
