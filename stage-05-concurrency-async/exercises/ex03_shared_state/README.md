# Exercise 03 · 共享状态

> 难度：⭐⭐  涉及：第 3 章

## 任务

### 1) `Arc<Mutex<i32>>` 多线程计数

```rust
fn parallel_count(n: usize) -> i32 {
    // n 个线程并发对共享 i32 +1
    // 返回最终值
}
```

测试：`parallel_count(100) == 100`。

### 2) `Arc<RwLock<Vec<i32>>>` 多读

```rust
fn reader_writer() {
    // 3 个读线程各打印 5 次 vec 长度
    // 1 个写线程每 50ms 追加一个 i
    // 用 RwLock 演示"读时不阻塞读、写时阻塞读"
}
```

### 3) `OnceLock` 全局单次初始化

```rust
use std::sync::OnceLock;
static CONFIG: OnceLock<String> = OnceLock::new();

fn get_config() -> &'static String { todo!() }
```

测试：第一次 `get_config` 跑 init，第二次直接返回缓存。

### 4) 死锁演示与修复

```rust
fn deadlock_fix() {
    // 两个 Mutex 互相加锁会死锁
    // 修法：固定加锁顺序
}
```

### 5) 毒化处理

```rust
fn poisoned_recover() {
    // 一个子线程 panic 持锁，主线程 lock 拿到 PoisonError
    // 用 into_inner() 拿 inner MutexGuard
}
```

## 验收

每题至少一个测试。

## 提示

- `Mutex::lock()` 返回 `LockResult`——`Err` 是毒化
- `RwLock::read` 多个并存，`write` 互斥
- 死锁：固定顺序、按"层次"加锁

## 进阶

实现一个"线程安全的工作池"：

```rust
struct Pool { items: Mutex<Vec<Job>> }
impl Pool {
    fn submit(&self, job: Job)
    fn take(&self) -> Option<Job>     // 多个 worker 抢
}
```

完成 → [ex04_async_await](../ex04_async_await)
