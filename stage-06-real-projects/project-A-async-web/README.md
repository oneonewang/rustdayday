# Project A · 异步 Web 后端

> 难度：⭐⭐⭐  
> 综合运用：Stage 5 全部内容 + axum / tokio / serde / tracing  
> 预计时间：3 – 4 小时

## 🎯 项目目标

实现一个**生产级风格**的异步 Web 后端：

- `axum` 路由 + 共享状态
- `tokio::sync::RwLock` 进程内存储（生产换 sqlx）
- `tracing` 结构化日志
- 统一 `AppError` → HTTP 状态码
- 端到端自检（用 `reqwest` 自己 curl 自己）

## 📂 项目结构

```
project-A-async-web/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs       # 入口 + 路由注册 + 自检
    ├── error.rs      # AppError + IntoResponse
    ├── model.rs      # Task / CreateTask / UpdateTask
    ├── state.rs      # AppState（共享 CRUD 存储）
    └── routes.rs     # 7 个 handler
```

## 🚀 怎么跑

```bash
cd project-A-async-web

# 真实服务
cargo run                # 监听 0.0.0.0:3000
cargo run -- serve

# 端到端自检（启动 server、自己 curl、断言）
cargo run -- selftest
```

## 🌐 API 端点

| 方法 | 路径 | 行为 | 成功码 |
|------|------|------|--------|
| GET | `/health` | 健康检查 | 200 |
| GET | `/tasks` | 列表 | 200 |
| POST | `/tasks` | 创建（body `{title}`） | 201 |
| GET | `/tasks/:id` | 读单条 | 200 / 404 |
| PUT | `/tasks/:id` | 更新（body `{title?, done?}`） | 200 / 404 |
| DELETE | `/tasks/:id` | 删除 | 204 / 404 |

## 📚 涉及的核心概念

| 概念 | 出处 |
|------|------|
| `#[tokio::main]` | Stage 5 |
| `axum::Router` + 路由 | 本项目 |
| `State` 提取器 | 本项目 |
| `Path` / `Json` 提取器 | 本项目 |
| `IntoResponse` trait | Stage 2 trait 章节 |
| `Arc<RwLock<HashMap>>` | Stage 5 |
| `tracing` + `EnvFilter` | Stage 5 |
| 端到端自检 | 综合 |

## 🏃 自检运行结果示例

```
=== A 端到端自检 ===

--- GET /health ---
  -> {"status":"ok"}

--- POST /tasks (create) ---
  -> {"id":"a1b2-...","title":"学 Rust","done":false,"created_at":1717958400}

--- GET /tasks (list) ---
  -> 2 tasks

--- GET /tasks/:id ---
  -> {...}

--- PUT /tasks/:id (mark done) ---
  -> {"id":"a1b2-...","done":true,...}

--- DELETE /tasks/:id ---
  -> status 204

--- GET /tasks/:id (after delete → 404) ---
  -> status 404, body {"error":"task ... not found"}

--- 完整断言 ---
全部断言通过 ✅
```

## 🎁 扩展挑战

1. **改用 sqlx + SQLite**：用 `sqlx` + `tokio::sync::Mutex<SqlitePool>` 替换 in-memory HashMap
2. **加用户认证**：用 `axum-extra` + JWT 中间件
3. **加 CORS**：用 `tower-http::cors`
4. **分页**：`?limit=20&offset=0` 参数 + 响应头 `X-Total-Count`
5. **搜索**：`GET /tasks?q=rust` 用 `iter().filter().collect()`
6. **OpenAPI 文档**：用 `utoipa` 自动生成
7. **WebSocket**：`/ws/tasks` 推送变更（用 `axum::extract::ws`）
8. **Docker 化**：写 `Dockerfile` 多阶段构建

## ✅ 完成判定

- [ ] `cargo run -- serve` 跑通，可手动 curl
- [ ] `cargo run -- selftest` 跑通
- [ ] 6 个端点 + 错误处理全工作
- [ ] 至少完成 1 个扩展挑战

完成 → 回到 [Stage 6 README](../README.md)，告诉我开始 Project B。
