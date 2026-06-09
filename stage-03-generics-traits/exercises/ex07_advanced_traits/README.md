# Exercise 07 · 高级 Trait

> 难度：⭐⭐⭐  涉及：第 5 章

## 任务

### 1) 关联类型 + Iterator

```rust
struct Counter { max: u32, count: u32 }

impl Counter {
    fn new(max: u32) -> Self { Self { max, count: 0 } }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
```

测试：`Counter::new(5).sum::<u32>() == 15`。

### 2) Supertrait

```rust
trait Person { fn name(&self) -> &str; }
trait Student: Person { fn school(&self) -> &str; }

struct Alice;
impl Person for Alice { fn name(&self) -> &str { "Alice" } }
impl Student for Alice { fn school(&self) -> &str { "MIT" } }

fn greet_student<S: Student>(s: &S) {
    println!("{} 在 {} 读书", s.name(), s.school());
}
```

### 3) newtype 绕开孤儿

```rust
use std::num::Wrapping;

struct Wrap(Wrapping<u32>);

impl std::fmt::Display for Wrap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // 注意：先 deref 拿到 Wrapping<u32>
        write!(f, "Wrap({})", self.0)
    }
}
```

### 4) 自定义迭代器 + take + collect

实现 `Alternating` 迭代器，交替产出 0/1/0/1/...：

```rust
struct Alternating { curr: bool }
impl Iterator for Alternating {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        // 翻转 curr，返回对应值
    }
}
```

测试：`Alternating { curr: false }.take(6).collect::<Vec<_>>() == vec![0, 1, 0, 1, 0, 1]`。

### 5) 默认泛型参数

```rust
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Money(u64);     // cents

// i32 + Money → Money（自动 cents）
impl Add<Money> for i32 { ... }

// Money + Money → Money
impl Add for Money { ... }
```

测试：
```rust
assert_eq!(Money(100) + Money(50), Money(150));
assert_eq!(50 + Money(100), Money(150));   // i32 触发 Add<Money> for i32
```

## 验收

每题至少一个测试用例。

## 提示

- 关联类型用 `type Item = ...;` 写在 `impl` 里
- 关联类型 vs 泛型：`Iterator` 用关联类型（每种实现者只一种 Item）
- 默认泛型参数 `trait Add<Rhs = Self>`：不写时 `Rhs = Self`

## 进阶

写一个 `Summable` trait：

```rust
trait Summable<T = Self> {       // 默认 Self
    fn sum_items(items: &[T]) -> T
}

impl Summable for i32 { fn sum_items(items: &[Self]) -> Self { items.iter().sum() } }
impl Summable<f64> for f64 { ... }    // f64 也能用，Output 不同
```

完成 → [project-03-lru-cache](../project-03-lru-cache) 见！
