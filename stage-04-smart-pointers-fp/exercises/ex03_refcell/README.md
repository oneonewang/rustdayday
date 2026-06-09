# Exercise 03 · RefCell

> 难度：⭐⭐  涉及：第 2 章

## 任务

### 1) LazyCache

```rust
use std::cell::RefCell;

struct LazyCache<T> {
    value: RefCell<Option<T>>,
    init: Box<dyn Fn() -> T>,
}

impl<T> LazyCache<T> {
    fn new(init: impl Fn() -> T + 'static) -> Self
    fn get_or_init(&self) -> &T     // 第一次算，之后返回缓存
    fn reset(&self)                 // 清空，强制下次再算
}
```

测试：用一个带计数器的 init 函数验证只跑一次。

### 2) Cell<i32> 计数器

```rust
use std::cell::Cell;

struct Counter { value: Cell<u32> }
impl Counter {
    fn new() -> Self { Self { value: Cell::new(0) } }
    fn increment(&self) { self.value.set(self.value.get() + 1); }
    fn get(&self) -> u32 { self.value.get() }
}
```

### 3) 树 + Rc<RefCell<Node>>

```rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct TreeNode {
    value: i32,
    children: RefCell<Vec<Rc<TreeNode>>>,
}

impl TreeNode {
    fn new(v: i32) -> Rc<Self>
    fn add_child(self: &Rc<Self>, v: i32) -> Rc<Self>
    fn count(&self) -> usize
    fn depth(&self) -> usize
}
```

### 4) Weak 打破循环

```rust
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Parent {
    children: RefCell<Vec<Rc<Child>>>,
}

#[derive(Debug)]
struct Child {
    parent: RefCell<Option<Weak<Parent>>>,
}
```

测试：parent drop 后，child.parent.upgrade() 返回 None。

## 验收

- LazyCache：第二次调 `get_or_init` 不调 init
- Cell 计数器：`increment()` 后 `get()` 返回新值
- 树：add_child 后 children 列表含新节点
- Weak：parent drop 后 child 能感知

## 进阶

写一个"观察者模式"：Subject 持有 `Rc<RefCell<Vec<Box<dyn Observer>>>>`，Observer 有 `on_notify(&self, msg: &str)`。

完成 → [ex04_closures](../ex04_closures)
