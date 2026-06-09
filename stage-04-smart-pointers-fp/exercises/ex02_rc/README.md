# Exercise 02 · Rc

> 难度：⭐⭐  涉及：第 1 章

## 任务

### 1) 共享节点

```rust
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    children: Vec<Rc<Node>>,
}

let leaf = Rc::new(Node { value: 5, children: vec![] });
let root = Rc::new(Node {
    value: 1,
    children: vec![Rc::clone(&leaf), Rc::clone(&leaf)],   // leaf 出现两次
});
```

验证：leaf 强引用计数 = 3（变量 leaf + root 两次 children）。

### 2) 共享链表

```rust
#[derive(Debug)]
struct List<T> {
    head: Option<Rc<Node<T>>>,
}

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Option<Rc<Node<T>>>,
}

impl<T> List<T> {
    fn new() -> Self { Self { head: None } }
    fn prepend(&self, elem: T) -> List<T> {
        // 不消耗 self，共享 tail
    }
    fn head(&self) -> Option<&T>
    fn tail(&self) -> List<T>
}
```

测试：prepend 多次，验证计数。

### 3) Rc 计数追踪

写一个函数 `count_rc<T>(r: &Rc<T>) -> usize` 返回强引用计数（**就是** `Rc::strong_count` 的别名包装）。

### 4) 父子节点（**单向**，等 ex03 学 RefCell 再加反向）

```rust
#[derive(Debug)]
struct Parent {
    name: String,
    child: Rc<Child>,
}

#[derive(Debug)]
struct Child {
    name: String,
}
```

## 验收

每个任务至少一个测试。

## 提示

- `Rc::clone(&x)` 增加计数，**不**深拷贝
- 同一节点被多个所有者时**不能**改（要用 `RefCell`）

## 进阶

写一个"文件包含图"：用 `Rc<FileNode>` 表示，`FileNode` 持有 `Vec<Rc<FileNode>>` 表示 include 关系。

完成 → [ex03_refcell](../ex03_refcell)
