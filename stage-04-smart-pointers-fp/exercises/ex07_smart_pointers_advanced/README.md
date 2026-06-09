# Exercise 07 · 智能指针深入

> 难度：⭐⭐⭐  涉及：第 5 章

## 任务

### 1) `Arc<Mutex<i32>>` 跨线程计数

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn parallel_increment(n: usize) -> i32 {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..n {
        let counter = Arc::clone(&counter);
        let h = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(h);
    }

    for h in handles { h.join().unwrap(); }

    *counter.lock().unwrap()
}
```

测试：`parallel_increment(10) == 10`、`parallel_increment(100) == 100`。

### 2) `Arc<RwLock<T>>` 多读单写

```rust
use std::sync::{Arc, RwLock};

struct Config {
    map: Arc<RwLock<std::collections::HashMap<String, String>>>,
}

impl Config {
    fn new() -> Self { ... }
    fn get(&self, key: &str) -> Option<String>
    fn set(&self, key: String, value: String)
}
```

测试：多线程读 + 单线程写。

### 3) `Cow<str>` 裁剪

```rust
use std::borrow::Cow;

fn trim_if_long(s: &str, max_len: usize) -> Cow<str> {
    if s.len() <= max_len {
        Cow::Borrowed(s)             // 不分配
    } else {
        Cow::Owned(s[..max_len].to_string())
    }
}
```

测试两种情况：短串 0 分配、长串 1 分配。

### 4) `Weak` 父子节点

```rust
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Option<Weak<Node>>>,
    children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    fn new(v: i32) -> Rc<Self> { ... }
    fn child_of(self: &Rc<Self>, v: i32) -> Rc<Node> { ... }
    fn parent_value(&self) -> Option<i32> {
        self.parent.borrow().as_ref().and_then(|w| w.upgrade().map(|p| p.value))
    }
}
```

### 5) `OnceLock` 延迟初始化

```rust
use std::sync::OnceLock;

static GREETING: OnceLock<String> = OnceLock::new();

fn get_greeting() -> &'static String {
    GREETING.get_or_init(|| "Hello, world!".to_string())
}
```

测试：多次调都返回同一引用。

## 验收

每题至少一个测试。

## 进阶

写一个"读者-写者"工作流：
- 多个 `read` 线程并发读
- 一个 `write` 线程偶发写
- 写时不能有读

用 `Arc<RwLock<Vec<i32>>>` 实现，验证不变量。

完成 → [project-04-text-parser](../project-04-text-parser) 见！
