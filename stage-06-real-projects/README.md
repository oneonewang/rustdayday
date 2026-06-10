# Stage 6：实战项目

> 目标：把 Stage 1–5 全部能力**串起来**——做 4 个端到端的真实项目，每个都是一个完整的 Rust 应用程序。

## 📦 4 个项目

| 代号 | 方向 | 技术栈 | 阶段 |
|------|------|--------|------|
| **A** | 异步 Web 后端 | axum + tokio + serde + tracing | 本阶段实现 |
| **B** | 高性能 CLI 工具 | clap + rayon + 自定义 trait | 下一阶段 |
| **C** | WebAssembly 前端 | yew / seed + wasm-bindgen | 之后 |
| **D** | 系统编程 | 文件 I/O / FFI / unsafe | 最后 |

> 每个项目都**独立可运行**——独立 cargo 项目、独立的 Cargo.toml、独立的依赖、独立的 README。

## 🗂️ 目录结构

```
stage-06-real-projects/
├── README.md                       # 本文件（总览）
├── project-A-async-web/            # 异步 Web 后端
│   ├── README.md
│   ├── Cargo.toml
│   └── src/
├── project-B-rust-cli/             # 高性能 CLI
│   ├── README.md
│   └── ...
├── project-C-wasm-frontend/        # WebAssembly
│   ├── README.md
│   └── ...
└── project-D-systems/              # 系统编程
    ├── README.md
    └── ...
```

## 🎯 整体目标

完成 Stage 6 之后，你应该能：

1. 独立设计一个**异步 Web 后端**（axum + tokio + 数据库）
2. 写一个**生产级 CLI 工具**（参数解析 + 错误处理 + 并行 + 进度条）
3. 把 Rust 编译到 **WebAssembly** 在浏览器跑
4. 用 **unsafe** + FFI 调 C 库
5. 把所有概念融会贯通——**写自己想写的 Rust 项目**

## 📖 推荐节奏

每个项目 2-4 天：
- A 先做（async + tokio 复习）
- B 接着（系统级工程）
- C 接着（新领域扩展）
- D 收尾（unsafe 深入）

---

## A · 异步 Web 后端（axum）

> **本阶段正在做**——见 [project-A-async-web/](./project-A-async-web/)

技术栈：axum 0.6 + tokio 1 + serde + tracing + uuid  
目标：CRUD 任务管理 API，演示 axum 路由、共享状态、错误处理、tracing 日志、自检测试。

---

## B · 高性能 CLI 工具（计划中）

技术栈：clap 4 + rayon + 自定义 trait + anyhow  
目标：类似 ripgrep 的"文件内容搜索" CLI，支持 glob、正则、并行、彩色输出、进度条。

---

## C · WebAssembly 前端（计划中）

技术栈：Yew / Seed + wasm-bindgen + web-sys  
目标：一个跑在浏览器的 TODO 应用，编译成 wasm 部署。

---

## D · 系统编程（计划中）

技术栈：unsafe + FFI + 文件 I/O + mmap  
目标：一个工具：分析 ELF 文件结构、读取进程内存、调 C 标准库。

---

回到 [项目总览](../README.md)
