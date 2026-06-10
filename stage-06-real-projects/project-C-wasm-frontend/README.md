# Project C · Rust → WebAssembly 前端

> 难度：⭐⭐⭐  
> 综合运用：wasm-bindgen 0.2、wasm32-unknown-unknown target、--target web 自动 JS glue  
> 预计时间：2 – 3 小时

## 🎯 项目目标

把 Rust 代码**编译成 WebAssembly**，在浏览器里跑——展示 Rust 编译到 wasm、与 JS 互操作。

```
Rust 源码 ──cargo build --target wasm32──> .wasm
                                │
                                └──wasm-bindgen──> .js (glue)
                                                  │
                                                  ▼
                                      浏览器 <script type="module">
```

## 📂 项目结构

```
project-C-wasm-frontend/
├── Cargo.toml          # lib + cdylib
├── README.md
├── src/
│   └── lib.rs          # 用 #[wasm_bindgen] 暴露函数
├── www/
│   └── index.html      # 加载并使用 .wasm
└── tests/
    └── smoke.rs        # 用 wasmtime 离线跑 .wasm 验证导出
```

## 🚀 怎么跑

```bash
cd project-C-wasm-frontend

# 1) 编译到 wasm
cargo build --release --target wasm32-unknown-unknown

# 2) 生成 JS glue（wasm-bindgen 0.2+ 内置，不再需要 wasm-bindgen-cli）
#    这个 .js 会引用 init() 异步函数和 6 个 export
mkdir -p www
cp target/wasm32-unknown-unknown/release/wasm_frontend.wasm www/
# 注：wasm-bindgen 0.2 配合 --target=web 会在 build 时输出 .js，但我们用 --release 默认不输出
# 推荐用下面这个命令同时生成 .js + .wasm：
#   cargo build --release --target wasm32-unknown-unknown
#   然后用 wasm-bindgen CLI（如果装了）跑：
#     wasm-bindgen --target web --out-dir www target/.../wasm_frontend.wasm
#
# 简化版：用 cargo + wasm-bindgen 自带的 build.rs（见 Cargo.toml 注释）
```

> 因为环境无网络、没有 wasm-bindgen-cli，本项目演示**两种方式**：
> 1. `cargo build` 直接产生 .wasm（包含所有导出）
> 2. `cargo test` 跑 `tests/smoke.rs`，用 `wasmtime` 离线**调用**这些导出并断言

## 🌐 浏览器侧（需要 wasm-bindgen-cli 才能跑）

打开 `www/index.html`——会加载 .wasm 并把 6 个 Rust 函数挂到 JS：
- `factorial(n)` 阶乘
- `fibonacci(n)` 斐波那契
- `is_prime(n)` 质数判断
- `reverse(s)` 字符串反转
- `sum_array([..])` 数组求和
- `version()` 版本号

## 🧪 自检（离线，用 wasmtime）

```bash
cargo test --test smoke
```

会：
1. 编译 wasm
2. 用 `wasmtime` 加载 .wasm
3. 调 `factorial(10)`、`fibonacci(20)` 等
4. 断言结果

## 📚 核心概念

| 概念 | 用途 |
|------|------|
| `wasm-bindgen` 0.2+ | Rust → JS 的桥 |
| `#[wasm_bindgen]` | 标注要导出到 JS 的函数 |
| `--target wasm32-unknown-unknown` | 编译目标 |
| `--target web` (CLI) | 生成 `init()` 风格的 JS glue |
| `cdylib` crate type | 编译成动态库（wasm 模块） |
| `wasmtime` | 离线 wasm runtime，能调导出函数 |

## 🏃 自检运行示例

```
running 1 test
test tests::smoke_run ... ok

test result: ok. 1 passed; 0 failed
```

## 🎁 扩展挑战

1. **真跑浏览器**：装 `wasm-pack` + `wasm-bindgen-cli` → 完整跑 `www/index.html`
2. **加 web-sys DOM 操作**：从 Rust 端 `document.createElement`
3. **互调 JS → Rust**：在 Rust 调 JS 函数（用 `js_sys::Function`）
4. **共享内存**：用 `SharedArrayBuffer` + `Uint8Array` 在 JS/Rust 间共享大数组
5. **WASI**：切到 `wasm32-wasi` target，Rust 当独立 CLI 程序
6. **GoLive**：把 `www/` 部署到 GitHub Pages

## ⚠️ 离线构建说明

- 本项目**不依赖** yew / seed / web-sys / gloo 等大 crate——只用 `wasm-bindgen`
- `wasm-bindgen` 0.2.120 缓存了
- `wasmtime` 36 用于离线运行自检

## ✅ 完成判定

- [ ] `cargo build --release --target wasm32-unknown-unknown` 跑通
- [ ] `cargo test --test smoke` 全过
- [ ] 6 个导出函数被 wasmtime 验证

完成 → 回到 [Stage 6 README](../README.md) 告诉我开始 Project D。
