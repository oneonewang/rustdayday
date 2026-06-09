# 01 · Rust 工具链

> **本章目标**：装好 `rustup` / `cargo` / `rustc`，知道它们各自干什么。

## 1.1 三个工具的关系

| 工具 | 角色 | 类比 |
|------|------|------|
| `rustup` | Rust **版本管理器**。可以装/切多个 toolchain（stable / beta / nightly）、管理组件（rustfmt、clippy）和 target（wasm32、嵌入式） | `nvm`（Node）、`pyenv`（Python） |
| `rustc` | Rust **编译器**。一般你不会直接调用它 | `gcc` / `javac` |
| `cargo` | Rust 的**构建系统 + 包管理器 + 测试运行器**。`cargo new` 建项目，`cargo build` 编译，`cargo run` 跑，`cargo test` 测 | `npm` + `webpack` + `jest` 的合体 |

> 💡 **日常只跟 `cargo` 打交道**。`rustup` 装一次就好，`rustc` 你几乎不直接敲。

## 1.2 安装

### Linux / macOS（推荐方式）

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

按提示选 `1) Proceed with installation (default)`，结束后 `source` 一下环境：

```bash
source "$HOME/.cargo/env"
```

### Windows

去 <https://rustup.rs/> 下载 `rustup-init.exe`，按向导走。如果用 WSL，建议直接在 WSL 里装 Linux 版。

### 验证安装

```bash
rustc --version   # rustc 1.95.0 (...)
cargo --version   # cargo 1.95.0 (...)
rustup --version  # rustup 1.28.x (...)
```

> 看到三个版本号就说明装好了。

## 1.3 离线 / 镜像加速（可选）

如果官方源慢，可以切到国内镜像（[rsproxy](https://rsproxy.cn/)）：

```bash
# 在 ~/.bashrc 或 ~/.zshrc 里加：
export RUSTUP_DIST_SERVER="https://rsproxy.cn"
export RUSTUP_UPDATE_ROOT="https://rsproxy.cn/rustup"
# 装好后切 cargo 镜像
mkdir -p ~/.cargo
cat >> ~/.cargo/config.toml <<'EOF'
[source.crates-io]
replace-with = 'rsproxy-sparse'

[source.rsproxy]
registry = "https://rsproxy.cn/crates.io-index"

[source.rsproxy-sparse]
registry = "sparse+https://rsproxy.cn/index/"

[registries.rsproxy]
index = "https://rsproxy.cn/crates.io-index"

[net]
git-fetch-with-cli = true
EOF
```

## 1.4 常用命令速查

```bash
# ---- 项目生命周期 ----
cargo new my-app          # 二进制项目（含 src/main.rs、.gitignore、初始化 git）
cargo new --lib my-lib    # 库项目（含 src/lib.rs）
cargo init                # 在当前目录初始化（已存在 src/ 时用）

# ---- 编译运行 ----
cargo build               # 编译（debug 模式），产物在 target/debug/
cargo build --release     # release 模式（开启优化），产物在 target/release/
cargo run                 # 编译并运行
cargo check               # 仅类型检查（比 build 快，CI 常用）

# ---- 测试与质量 ----
cargo test                # 运行测试
cargo fmt                 # 用 rustfmt 格式化
cargo clippy              # 静态检查（linter，强烈建议开启）
cargo doc --open          # 生成并打开文档

# ---- 依赖管理 ----
cargo add serde           # 添加依赖（修改 Cargo.toml）
cargo update              # 更新 Cargo.lock
cargo tree                # 打印依赖树（debug 依赖冲突时用）

# ---- 工具链管理 ----
rustup update             # 升级当前 toolchain
rustup default stable     # 把 stable 设为默认
rustup install nightly    # 安装 nightly（写 unsafe 高级特性时偶尔需要）
rustup component add rustfmt clippy   # 装组件
```

## 1.5 编辑器推荐

- **VS Code** + 扩展 `rust-analyzer`（必装）+ `Even Better TOML` + `CodeLLDB`（断点调试）
- **RustRover**（JetBrains 出品，全功能 IDE，对 Rust 一流）
- **Neovim** + `rust-analyzer` + `vim-rsi`（生产级组合）

> ⚠️ 别再用老的 RLS 了，迁移到 `rust-analyzer`。

## 1.6 一个检查清单

打开终端跑一遍，全部有结果就算通过：

```bash
rustc --version
cargo --version
rustup show
cargo new --bin hello-check
cd hello-check
cargo run     # 应输出 "Hello, world!"
cd .. && rm -rf hello-check
```

---

## 🏋️ 本章小练习

**练习 1.1**：在终端跑出 `rustc --version` 和 `cargo --version`，把输出贴到你的笔记里（顺便确认哪天哪个版本）。

**练习 1.2**：执行 `cargo new --bin playground` 进入目录，看一下生成的文件结构：

```bash
tree -L 2 playground
# 或
find playground -maxdepth 2 -not -path '*/target*' -not -path '*/.git*'
```

你应该看到：

```
playground/
├── .git/
├── .gitignore
├── Cargo.toml
└── src/
    └── main.rs
```

**练习 1.3**：编辑 `src/main.rs`，把 `Hello, world!` 改成你想要的问候语，运行 `cargo run` 看到输出。**注意**：Cargo 会在文件变化时**自动**重新编译（如果你装了 `cargo-watch`，可以 `cargo watch -x run`）。

---

## 📚 延伸阅读

- 官方安装页：<https://www.rust-lang.org/tools/install>
- `rustup` book：<https://rust-lang.github.io/rustup/>
- 镜像站：<https://rsproxy.cn/>

下一章：[02 · Hello World 与第一个 Cargo 项目 →](./02-hello-world.md)
