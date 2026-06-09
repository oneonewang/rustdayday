# Exercise 01 · Box

> 难度：⭐  涉及：第 1 章

## 任务

### 1) 自定义 Box 行为

```rust
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> Self { MyBox(x) }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T { &self.0 }
}
```

验证：传 `&MyBox<String>` 给 `&str` 参数（deref coercion）。

### 2) 递归 enum

```rust
enum IntList {
    Cons(i32, Box<IntList>),
    Nil,
}

impl IntList {
    fn new() -> Self { IntList::Nil }
    fn prepend(self, elem: i32) -> Self { IntList::Cons(elem, Box::new(self)) }
    fn sum(&self) -> i32 { ... }
    fn len(&self) -> usize { ... }
    fn to_vec(&self) -> Vec<i32> { ... }
}
```

测试：
```rust
let list = IntList::new().prepend(3).prepend(2).prepend(1);
assert_eq!(list.sum(), 6);
assert_eq!(list.len(), 3);
assert_eq!(list.to_vec(), vec![1, 2, 3]);
```

### 3) Trait object 容器

```rust
trait Animal { fn name(&self) -> &str; }

struct Dog;
struct Cat;

impl Animal for Dog { fn name(&self) -> &str { "Dog" } }
impl Animal for Cat { fn name(&self) -> &str { "Cat" } }

fn make_zoo() -> Vec<Box<dyn Animal>> {
    vec![Box::new(Dog), Box::new(Cat)]
}
```

### 4) 二叉树

```rust
enum BinaryTree {
    Empty,
    Node(i32, Box<BinaryTree>, Box<BinaryTree>),
}

impl BinaryTree {
    fn leaf(v: i32) -> Self { BinaryTree::Node(v, Box::new(BinaryTree::Empty), Box::new(BinaryTree::Empty)) }
    fn node(v: i32, l: BinaryTree, r: BinaryTree) -> Self { BinaryTree::Node(v, Box::new(l), Box::new(r)) }
    fn sum(&self) -> i32
    fn height(&self) -> usize
    fn contains(&self, v: i32) -> bool
}
```

## 验收

每个任务至少一个测试。

## 提示

- 递归 enum **必须** `Box` 包装
- `prepend` 拿所有权（self 之后不用）→ 不用 &mut
- `to_vec` 先 sum 再算，**或者**先 `&[]` 然后 push

## 进阶

写一个用 Box 包装的 `SExpression` enum（`Atom(String)` / `List(Vec<SExpression>)`），解析 lisp 风格表达式。

完成 → [ex02_rc](../ex02_rc)
