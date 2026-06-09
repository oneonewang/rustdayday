# Project 05 · 异步 HTTP 抓取器

> 难度：⭐⭐⭐  
> 综合运用：Tokio runtime、`async` / `await`、`tokio::time::timeout`、`tokio::sync::Semaphore`、自定义 HTTP server、错误处理  
> 预计时间：2 – 3 小时

## 🎯 项目目标

实现一个**真异步**的 HTTP 抓取器，演示：

1. **并发抓取**（`join!` / `join_all`）
2. **限流**（`Semaphore`）
3. **超时**（`tokio::time::timeout`）
4. **错误处理**（超时 / 连不上 / 404）

## 📂 项目结构

```
project-05-async-scraper/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs        # 演示 + 抓取器逻辑
    └── server.rs      # 极简 HTTP server（用于本地 demo）
```

## 🚀 怎么跑

```bash
cd project-05-async-scraper
cargo run
```

**完全离线**——`server.rs` 启动一个本地 HTTP server，模拟多个 URL，scrape 自己的 server。

## 📚 涉及的核心概念

| 概念 | 出处 |
|------|------|
| `#[tokio::main]` | 第 5 章 |
| `async fn` + `.await` | 第 4 章 |
| `join_all` / `join!` | 第 5 章 |
| `tokio::time::timeout` | 第 5 章 |
| `tokio::sync::Semaphore` | 第 5 章 |
| `Send + 'static` 约束 | 第 6 章 |
| `reqwest` 异步 HTTP 客户端 | 第 5 章 |

## 🪜 怎么写

### Step 1：HTTP 抓取函数

```rust
async fn fetch_one(client: reqwest::Client, url: String, dur: Duration) -> FetchResult {
    let res = tokio::time::timeout(dur, async {
        let resp = client.get(&url).send().await?;
        let status = resp.status().as_u16();
        let body = resp.text().await?;
        Ok::<_, reqwest::Error>((status, body.len()))
    }).await;

    match res {
        Ok(Ok((s, l)))  => FetchResult { url, status: Ok(s), body_len: l, elapsed: ... },
        Ok(Err(e))      => FetchResult { url, status: Err(e.to_string()), ... },
        Err(_)          => FetchResult { url, status: Err("timeout".into()), ... },
    }
}
```

### Step 2：并发抓取

```rust
let results: Vec<_> = futures::future::join_all(
    urls.iter().map(|u| {
        let c = client.clone();
        async move { fetch_one(c, u.clone(), Duration::from_secs(2)).await }
    })
).await;
```

### Step 3：限流

```rust
let sem = Arc::new(Semaphore::new(2));    // 最多 2 并发

for url in urls {
    let sem = Arc::clone(&sem);
    let permit = sem.acquire().await.unwrap();   // 等名额
    // ... fetch ...
    drop(permit);                                 // 释放
}
```

## 🧪 演示

`main.rs` 跑 5 个演示：

1. **基础并发**：3 个 URL 并发抓，验证总耗时 ≈ 最慢的（不是相加）
2. **限流**：5 个任务，限流 2 并发，验证总耗时 ≈ 5 × 200ms / 2 = 500ms
3. **超时**：timeout(100ms) 抓一个 200ms 任务
4. **错误处理**：404 / 连不上 都返回 Err
5. **断言**：fetch 状态码 = 200、body 长度 > 0

## 🏃 运行结果示例

```
=== 演示 1：基础并发抓取（3 URLs）===
  ✓ http://127.0.0.1:54321/         200    4500 bytes in 12.3ms
  ✓ http://127.0.0.1:54321/about    200    1800 bytes in 8.7ms
  ✓ http://127.0.0.1:54321/blog     200    9000 bytes in 11.2ms
  总耗时: 13.4ms           ← 并发！
  （串行会是 12+8+11 = 31ms）

=== 演示 2：限流 2 并发抓 5 URLs ===
  worker #0 开始抓 http://127.0.0.1:54321/p/1
  worker #1 开始抓 http://127.0.0.1:54321/p/2
  worker #0 完成
  worker #1 完成
  worker #2 开始抓 http://127.0.0.1:54321/p/3
  ...
  总耗时: 502.1ms          ← 5 × 200ms / 2 = 500ms
```

## 🎁 扩展挑战

1. **真实 URL 抓取**：把 server 换成真实 URL 列表
2. **重试**：用 `retry` 包装 `fetch_one`，失败重试 N 次
3. **速率限制**：用 token bucket，每秒最多 N 个请求
4. **流式输出**：用 `tokio::sync::mpsc` 把每个完成的结果**流**给消费者
5. **持久化**：抓到的结果存到文件（serde + JSON）
6. **并发 + 共享 client**：`reqwest::Client` 内部用 `Arc`，clone 共享
7. **优雅关闭**：监听 `Ctrl-C`，完成已开始的请求再退出

## ⚠️ 简化说明

- HTTP server 极简（不支持 keep-alive、不支持 chunked）—— 真实场景用 axum / hyper
- 不处理 chunked / streaming 响应
- 错误信息简化

## ✅ 完成判定

- [ ] `cargo run` 跑通，所有演示输出合理
- [ ] 演示 1 总耗时 < 50ms（并发）
- [ ] 演示 2 总耗时 ≈ 500ms（限流）
- [ ] 演示 3 验证超时
- [ ] 演示 4 错误处理正确
- [ ] 至少完成 1 个扩展挑战

完成 → 回到 [Stage 5 README](../README.md) 准备进入 Stage 6。
