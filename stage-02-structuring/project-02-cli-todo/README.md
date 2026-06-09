# Project 02 · CLI Todo 工具

> 难度：⭐⭐  
> 综合运用：struct / enum / 模式匹配 / Option & Result / 模块 / 文件 I/O / serde  
> 预计时间：2 – 3 小时

## 🎯 项目目标

实现一个命令行 Todo 工具：

```bash
$ todo add "学完 Stage 2"
$ todo add "做 project-02-cli-todo"
$ todo list
[ ] #1   学完 Stage 2
[ ] #2   做 project-02-cli-todo
$ todo done 1
$ todo list
[x] #1   学完 Stage 2
[ ] #2   做 project-02-cli-todo
$ todo remove 2
$ todo list
[x] #1   学完 Stage 2
```

数据存在 `todos.json`（可通过 `TODO_FILE` 环境变量换路径），跨多次运行保留。

## 🚀 怎么跑

```bash
cd project-02-cli-todo
cargo run -- add "学完 Stage 2"
cargo run -- list
cargo run -- done 1
```

> 第一次构建需要下载 `serde` / `serde_json` 及其依赖。**如果遇到网络问题**，用 `cargo build --offline`（本地缓存里有 1.0.228 / 1.0.145 等版本）。

## 📂 项目结构

```
project-02-cli-todo/
├── Cargo.toml
├── README.md          ← 你正在读
└── src/
    ├── main.rs        # 入口：解析命令、调用业务
    ├── cli.rs         # 命令行参数解析
    ├── todo.rs        # Todo 业务结构
    ├── storage.rs     # JSON 持久化
    └── error.rs       # 统一错误类型
```

## 🪜 怎么写

按"自底向上"顺序，每步都能跑：

### Step 1：定义 `Todo` 和 `TodoList`

```rust
// todo.rs
pub struct Todo { id: u32, text: String, done: bool }
pub struct TodoList { items: Vec<Todo>, next_id: u32 }
```

实现 `add` / `complete` / `remove` / `print`。

### Step 2：定义 `AppError`

```rust
// error.rs
pub enum AppError {
    NotFound(u32),
    Io(std::io::Error),
    Json(serde_json::Error),
}
```

实现 `Display` / `Error` / `From` 转换（让 `?` 能用）。

### Step 3：实现 `storage::load` / `save`

```rust
// storage.rs
pub fn load(path: &str) -> Result<TodoList, AppError>
pub fn save(path: &str, list: &TodoList) -> Result<(), AppError>
```

**注意**：文件不存在时返回**空** `TodoList::default()`，不要 panic。

### Step 4：解析 CLI

```rust
// cli.rs
pub enum Command { Add(String), List, Done(u32), Remove(u32) }
pub fn parse_args() -> Command
```

用 `std::env::args()`，不用 clap。

### Step 5：组装 `main.rs`

```rust
fn main() -> Result<(), AppError> {
    let path = std::env::var("TODO_FILE").unwrap_or_else(|_| "todos.json".to_string());
    let cmd = cli::parse_args();
    let mut list = storage::load(&path)?;
    // match cmd 调用业务
    storage::save(&path, &list)?;
    Ok(())
}
```

## 🧪 测试建议

```bash
# 添加几条
cargo run -- add "学完 Stage 2"
cargo run -- add "做 project-02"
cargo run -- add "准备 Stage 3"

# 列出
cargo run -- list

# 完成第 1 条
cargo run -- done 1

# 删除第 2 条
cargo run -- remove 2

# 再次列出
cargo run -- list

# 试错：完成不存在的 id
cargo run -- done 999   # 应报 "任务 #999 不存在"

# 试错：未知子命令
cargo run -- foo        # 应打印 usage
```

## 📋 功能矩阵

| 命令 | 行为 |
|------|------|
| `add <text>` | 追加新任务，自动分配 id |
| `list` / `ls` | 打印所有任务，`[x]` / `[ ]` 表示状态 |
| `done <id>` | 把 id 的任务标记为完成；id 不存在则报 `NotFound` |
| `remove <id>` / `rm <id>` | 删 id 任务；id 不存在则报 `NotFound` |
| `-h` / `--help` / `help` | 打印 usage |

## 🎁 扩展挑战（可选）

1. **`--json` 输出**：`list --json` 时打印 JSON 而不是表格
2. **优先级**：加 `priority: u8` 字段，列表按优先级排序
3. **截止日期**：加 `due: Option<String>`，列表显示"过期/今天/将来"
4. **搜索**：`search <keyword>` 过滤包含关键词的任务
5. **批量完成**：`done 1 3 5` 一次标多条
6. **彩色输出**：用 `\x1b[...]` ANSI 转义（可借助 `colored` / `owo-colors` crate）
7. **用 clap 替换手写 CLI**：`clap = { version = "4", features = ["derive"] }`

## ✅ 完成判定

- [ ] `cargo run -- list` 能跑通
- [ ] `add` / `list` / `done` / `remove` 都正常
- [ ] 错误（不存在 id、未知命令）有友好提示，**不 panic**
- [ ] `todos.json` 文件能在两次运行间保留数据
- [ ] 至少完成 1 个扩展挑战

完成 → 回到 [Stage 2 README](../README.md) 准备进入 Stage 3。
