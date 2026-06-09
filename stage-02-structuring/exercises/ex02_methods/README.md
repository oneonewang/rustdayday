# Exercise 02 · 方法

> 难度：⭐⭐  涉及：第 2 章

## 任务

为 `Rectangle` 写完整方法集：

```rust
#[derive(Debug, Clone, Copy)]
struct Rectangle { width: f64, height: f64 }

impl Rectangle {
    fn new(w: f64, h: f64) -> Self
    fn square(size: f64) -> Self
    fn area(&self) -> f64
    fn perimeter(&self) -> f64
    fn is_square(&self) -> bool
    fn can_contain(&self, other: &Rectangle) -> bool       // self 装得下 other？
    fn scale(&mut self, factor: f64)                        // 原地放大
    fn shrink_to_fit(self, target: &Rectangle) -> Self      // 拿所有权，缩小到不超过 target
}
```

## 测试用例

```rust
let r = Rectangle::new(4.0, 6.0);
assert_eq!(r.area(), 24.0);
assert_eq!(r.perimeter(), 20.0);
assert!(!r.is_square());

let s = Rectangle::square(5.0);
assert!(s.is_square());
assert!(r.can_contain(&Rectangle::new(2.0, 3.0)));
assert!(!r.can_contain(&Rectangle::new(10.0, 1.0)));

let mut r2 = r;
r2.scale(2.0);
assert_eq!(r2.area(), 96.0);

let r3 = r2.shrink_to_fit(&Rectangle::new(10.0, 10.0));
// r3.width 和 height 都不超过 10
```

## 验收

在 `main` 里跑完所有断言（或打印对比），输出对每个 `assert_eq!` 的实际值。

## 提示

- `scale` 改自己，必须 `&mut self`
- `shrink_to_fit` 拿所有权 + 借用另一个，签名是 `fn shrink_to_fit(self, target: &Rectangle) -> Self`
- 想"不改 self、只读字段"，**用 `&self`**

## 进阶

为 `Rectangle` 实现 `std::fmt::Display`：

```rust
impl std::fmt::Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Rectangle({}x{})", self.width, self.height)
    }
}
```

这样 `println!("{}", r)` 能直接用，不需要 `{:?}`。

完成 → [ex03_enum_matching](../ex03_enum_matching)
