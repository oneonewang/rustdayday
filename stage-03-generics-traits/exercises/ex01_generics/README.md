# Exercise 01 · 泛型基础

> 难度：⭐  涉及：第 1 章

## 任务

### 1) 泛型函数

```rust
fn swap<T>(pair: (T, T)) -> (T, T) {
    // 交换 tuple 的两个元素
}

fn largest<T: PartialOrd + Copy>(v: &[T]) -> Option<T> {
    // 空切片返回 None
}
```

### 2) 泛型 struct

```rust
#[derive(Debug)]
struct Pair<T> {
    first: T,
    second: T,
}

impl<T> Pair<T> {
    fn new(a: T, b: T) -> Self
    fn swap(self) -> Self
    fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Pair<U>    // 接受一个闭包
}
```

### 3) 泛型 enum

```rust
#[derive(Debug)]
enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Either<L, R> {
    fn is_left(&self) -> bool
    fn into_left(self) -> Option<L>     // 不是 Left 时返回 None
    fn map_left<L2, F: FnOnce(L) -> L2>(self, f: F) -> Either<L2, R>
}
```

## 验收

写一个 `main` 跑下面这些：

```rust
assert_eq!(swap((1, 2)), (2, 1));
assert_eq!(swap(("a", "b")), ("b", "a"));
assert_eq!(largest(&[3, 1, 4, 1, 5, 9, 2, 6]), Some(9));
assert_eq!(largest::<i32>(&[]), None);

let p = Pair::new(1, 2);
let mapped = p.map(|x| x * 10);
assert_eq!(mapped.first, 10);

let e: Either<i32, &str> = Either::Left(42);
assert!(e.is_left());
assert_eq!(e.into_left(), Some(42));
```

## 提示

- `map` 用 `FnOnce`——传入闭包可能是消耗 `self` 的
- `map_left` 把 `Either::Left(l)` 转成 `Either::Left(f(l))`，`Right` 不变
- `largest` 需要 `T: Copy` 才能从引用里拿值；用 `.copied()` 或 `*v.first()?`

## 进阶

写一个泛型 `Tree<T>`：

```rust
enum Tree<T> {
    Leaf,
    Node(T, Box<Tree<T>>, Box<Tree<T>>),
}

impl<T> Tree<T> {
    fn count(&self) -> usize                       // 节点数
    fn depth(&self) -> usize                       // 深度
}
```

完成 → [ex02_traits](../ex02_traits)
