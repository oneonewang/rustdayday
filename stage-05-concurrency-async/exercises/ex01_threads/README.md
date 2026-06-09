# Exercise 01 · 线程基础

> 难度：⭐  涉及：第 1 章

## 任务

### 1) 并行求和

```rust
fn parallel_sum(start: u64, end: u64, threads: usize) -> u64 {
    // 分到 threads 个线程并行求和 [start, end)
    // join 全部，汇总
}
```

测试：`parallel_sum(1, 1_000_001, 4) == 500000500000`。

### 2) 跨线程返回值

```rust
fn thread_square(n: u64) -> u64 {
    // 起一个线程算 n * n，返回
}
```

### 3) scoped threads

```rust
fn double_each(v: &mut Vec<i32>) {
    // 用 thread::scope 并行（？不能直接并行 mut 借用，先用 split）
    // 改：v.chunks_mut(...).for_each(chunk 异步处理)
    // 简化为：分两段各 *2
}
```

### 4) 自定义线程

```rust
fn named_worker() {
    // 用 thread::Builder 命名为 "worker-1"，stack 4MB
}
```

## 验收

每个任务至少一个测试。

## 提示

- 普通 `thread::spawn` 必须 `move` 闭包
- 多个线程共享数据用 `Arc<Mutex<T>>`（见 ex03）
- `JoinHandle::join()` 返回 `Result<T, _>`

## 进阶

写一个"分治求和"：1..=1_000_000，递归分到叶子是单数字，返回求和。多线程版。

完成 → [ex02_channels](../ex02_channels)
