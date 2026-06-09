# Exercise 05 · 运算符重载与标准 trait

> 难度：⭐⭐  涉及：第 5 章

## 任务

### 1) `+` 重载

```rust
use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point { x: i32, y: i32 }

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}
```

测试：`p1 + p2 == p3`。

### 2) `+=` 重载

```rust
use std::ops::AddAssign;
impl AddAssign for Point { ... }
```

测试：`p1 += p2;`。

### 3) `*` 重载：标量 × 向量

```rust
use std::ops::Mul;

impl Mul<i32> for Point {
    type Output = Point;
    fn mul(self, scalar: i32) -> Point {
        Point { x: self.x * scalar, y: self.y * scalar }
    }
}
```

测试：`p * 3`。

### 4) `Display` + `FromStr`

```rust
struct Money { cents: u64 }   // 内部存"分"避免浮点

impl std::fmt::Display for Money {
    // 12345 cents → "123.45"
}

impl std::str::FromStr for Money {
    type Err = String;
    // "123.45" → Money { cents: 12345 }
    // 错误格式返回 Err
}
```

测试：
```rust
let m: Money = "100.00".parse().unwrap();
assert_eq!(m.to_string(), "100.00");

let m = Money { cents: 5 };
assert_eq!(m.to_string(), "0.05");
```

### 5) `From` 自动转 `Into`

```rust
impl From<i32> for Money { ... }      // i32 当作 cents
```

测试：
```rust
let m: Money = 100.into();
assert_eq!(m.cents, 100);
```

## 验收

每个 impl 至少一个测试。

## 提示

- `Display` 输出 `cents / 100.cents % 100`
- `FromStr` 拆分 `.` 两边，再解析
- `From` 实现了之后 `Into` 自动给

## 进阶

实现 `PartialOrd` / `Ord` / `PartialEq` / `Eq` / `Hash` for `Money`，使它能进 `BTreeMap<Money, X>`。

> `derive(PartialEq, Eq)` 对所有字段比较就够用——但 `cents: u64` 已经是 Eq，所以可 derive。

完成 → [ex06_trait_objects](../ex06_trait_objects)
