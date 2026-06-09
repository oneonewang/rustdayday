# Exercise 06 · Send / Sync

> 难度：⭐⭐⭐  涉及：第 6 章

## 任务

### 1) Rc → Arc 修编译错

```rust
use std::rc::Rc;

#[tokio::main]
async fn main() {
    let r = Rc::new(5);
    tokio::spawn(async move {
        println!("{r}");
    });
}
```

预期：编译错。改成 `Arc` 让它能。

### 2) Send / Sync 静态检查

```rust
fn assert_send<T: Send>() {}
fn assert_sync<T: Sync>() {}

fn main() {
    assert_send::<i32>();
    assert_sync::<i32>();
    // assert_send::<std::rc::Rc<i32>>();      // ❌
    // assert_sync::<std::cell::RefCell<i32>>();  // ❌
}
```

把注释打开看错。

### 3) async future 跨 await 的 Send

```rust
use std::rc::Rc;

async fn uses_rc() {
    let r = Rc::new(5);
    println!("before: {r}");     // await 之前用完
    some_op().await;             // 跨 await 时 r 已 drop
}

async fn bad_uses_rc() {
    let r = Rc::new(5);
    some_op().await;             // 跨 await 时 r 还活着——!Send
    println!("{r}");
}
```

> `bad_uses_rc` 调用 `tokio::spawn(bad_uses_rc())` 编译失败。

### 4) 智能指针的 Send / Sync 矩阵

| 类型 | Send | Sync | 测一下 |
|------|------|------|------|
| `i32` | ✅ | ✅ | |
| `String` | ✅ | ✅ | |
| `Vec<i32>` | ✅ | ✅ | |
| `Rc<i32>` | ❌ | ❌ | |
| `Arc<i32>` | ✅ | ✅ | |
| `RefCell<i32>` | ✅ | ❌ | |
| `Mutex<i32>` | ✅ | ✅ | |
| `*const i32` | ❌ | ❌ | |

写 `fn check<T: Send + Sync>() {}` 把每个类型代入，看哪些过、哪些不过。

### 5) `unsafe impl Send`（仅学习用！）

```rust
struct MyPtr(*mut u8);
unsafe impl Send for MyPtr {}
unsafe impl Sync for MyPtr {}
```

> **永远不要在生产代码里随便写 `unsafe impl Send / Sync`**——除非你**非常**清楚自己在干什么。

## 验收

每题验证 Send / Sync 编译结果。

## 进阶

写一个 `Send + 'static` 的辅助 trait：

```rust
trait Job: Send + 'static {
    fn execute(self: Box<Self>);
}
```

实现它：让任意 `FnOnce() + Send + 'static` 都自动是 Job。

> 提示：实现 trait 需要 `impl<T: FnOnce() + Send + 'static> Job for T { ... }`——直接给函数指针实现 trait。

完成 → [ex07_async_streams](../ex07_async_streams)
