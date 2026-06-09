# 05 · 错误处理

> **本章目标**：用 `?` 运算符优雅地传播错误，用 `panic!` 处理不可恢复情况，写一个清晰可用的错误类型。

## 5.1 两类错误

| 类别 | 例子 | 修法 |
|------|------|------|
| **可恢复** | 文件不存在、网络超时、用户输入非法 | `Result<T, E>` + `?` |
| **不可恢复** | 索引越界、除以 0、违反不变量 | `panic!` / 主动 assert |

> 💡 **经验法则**：写"给开发者看的代码"（原型、demo、测试）——panic 没问题；写"给最终用户跑的程序"——能 `Result` 就别 panic。

## 5.2 `panic!` 什么时候合适

```rust
panic!("崩了");                              // 显式 panic
let v = vec![1, 2, 3];
v[99];                                         // 💥 运行时 panic：index out of bounds
let none: Option<i32> = None;
none.unwrap();                                 // 💥 called Option::unwrap() on a None

// 还可以断言
assert_eq!(2 + 2, 4);
assert!(n > 0, "n 必须是正数，得到 {n}");
debug_assert!(condition, "...");               // release 模式下被编译器去掉
```

### `panic!` 的好处和代价

| | 优点 | 代价 |
|---|------|------|
| 栈展开 | 沿途调用所有变量析构，资源干净 | 比 abort 模式慢一点 |
| 错误信息 | 自带 backtrace、文件 + 行号 | 用户看到"程序崩了" |
| `catch_unwind` | 可以**接住** panic（异步、FFI 边界用） | 复杂且不推荐 |

> 默认 `panic = unwind`（栈展开）；`Cargo.toml` 改 `panic = "abort"` 改为直接 abort（更小更快但资源不会清理）。

## 5.3 错误是值：`Result` 是函数返回的标准方式

```rust
fn read_username_from_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = std::fs::File::open(path)?;     // ? 1
    let mut s = String::new();
    file.read_to_string(&mut s)?;                  // ? 2
    Ok(s)
}
```

`?` 运算符的语义：

> 如果 `Result` 是 `Ok(v)`，解开得到 `v`，**继续往下执行**；
> 如果是 `Err(e)`，**整个函数立刻返回 `Err(e.into())`**。

`e.into()` 的关键是**错误类型自动转换**（靠 `From` trait）。

## 5.4 `?` 与 `Option`

`?` 也能用在 `Option<T>` 上：

```rust
fn first_word(s: &str) -> Option<&str> {
    let bytes = s.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        if b == b' ' {
            return Some(&s[0..i]);
        }
    }
    None
}

fn last_word(s: &str) -> Option<&str> {
    let first = first_word(s)?;
    Some(first)   // 这里只是示例
}
```

> `Option` 上 `?` 把 `None` 当作"提前返回 `None`"。

## 5.5 把多种错误统一成自定义错误类型

项目里经常要"读文件 + 解析 JSON + 查数据库"——每种操作返回的错误类型都不一样，怎么统一？

### 入门：`Box<dyn Error>`

```rust
use std::error::Error;

fn run() -> Result<(), Box<dyn Error>> {
    let content = std::fs::read_to_string("config.toml")?;   // io::Error → Box<dyn Error>
    let n: i32 = content.trim().parse()?;                    // ParseIntError → Box<dyn Error>
    println!("n = {n}");
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("错误: {e}");
    }
}
```

> `?` 的 `e.into()` 把任何 `Error` 转成 `Box<dyn Error>`（因为 `Box<dyn Error>: From<E> for E: Error`）。

### 进阶：自定义 enum 错误

```rust
use std::fs;
use std::io;
use std::num::ParseIntError;

#[derive(Debug)]
enum AppError {
    Io(io::Error),
    Parse(ParseIntError),
    InvalidConfig(String),
}

// 为每种类型实现 Display
impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AppError::Io(e)         => write!(f, "I/O 错误: {e}"),
            AppError::Parse(e)      => write!(f, "解析错误: {e}"),
            AppError::InvalidConfig(s) => write!(f, "配置无效: {s}"),
        }
    }
}

// 标记为标准 Error
impl std::error::Error for AppError {}

// 实现 From，让 ? 能自动转换
impl From<io::Error>          for AppError { fn from(e: io::Error)        -> Self { AppError::Io(e) } }
impl From<ParseIntError>      for AppError { fn from(e: ParseIntError)    -> Self { AppError::Parse(e) } }
```

> 现在 `?` 能自动把 `io::Error` / `ParseIntError` 转成 `AppError`。

### 工业级：直接用 `thiserror` crate

自己写 `Display` + `From` 太啰嗦——`thiserror` 一行搞定：

```toml
[dependencies]
thiserror = "1"
```

```rust
use thiserror::Error;

#[derive(Debug, Error)]
enum AppError {
    #[error("I/O 错误: {0}")]
    Io(#[from] io::Error),

    #[error("解析错误: {0}")]
    Parse(#[from] ParseIntError),

    #[error("配置无效: {0}")]
    InvalidConfig(String),
}
```

> `#[error(...)]` 自动实现 `Display`，`#[from]` 自动实现 `From`。

## 5.6 `main` 返回 `Result`

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("a.txt")?;
    println!("{}", content);
    Ok(())
}
```

如果返回 `Err`，**`main` 也会优雅地打印错误并以非零退出码退出**——非常适合 CLI 工具。

## 5.7 错误处理"风格指南"

> 1. 库代码：返回具体的 `Result<T, MyError>`，让调用方决定。
> 2. 应用代码（main / CLI / Web handler）：用 `?` 把所有错误统一到**一个**应用错误类型。
> 3. **不要**用 `unwrap` / `expect` 压平错误——除了以下情况：
>    - 单元测试
>    - 注释里能证明"绝不可能失败"
>    - 你就是想让程序崩
> 4. **不要**把所有错误转成字符串丢进 `String`——保留类型信息才能 match。
> 5. **错误信息要可行动**：写"文件 a.txt 不存在"比 "open failed" 有用。

## 5.8 错误 vs 异常：思维转换

| 维度 | Rust `Result` | Java/Python 异常 |
|------|---------------|------------------|
| 性质 | 一个值 | 控制流 |
| 类型 | `Result<T, E>` 写死 | `throws Exception`（Java 编译期能查，但绕开很容易） |
| 强制处理 | ✅ 必须 match | ❌ 容易吞掉 |
| 性能 | 零成本（栈上） | 栈展开/异常表，**出错时**才付代价 |
| 表达"多类错误" | enum 变体 | 多层 catch |

> 简单的"记忆口诀"：**Rust 错误是数据，异常是控制流**。

## 5.9 错误处理模式速查

```rust
// 模式 1：传播
fn a() -> Result<T, E> { b()? }

// 模式 2：转换错误类型
fn c() -> Result<T, MyError> { b().map_err(MyError::from)? }

// 模式 3：带默认值
let v = b().unwrap_or(default);

// 模式 4：记录 + 继续
if let Err(e) = do_thing() {
    log::warn!("...{e}");
}

// 模式 5：panic 退出
let v = b().expect("必须成功");

// 模式 6：转换成 Option
let v = b().ok();
```

## 5.10 真实场景：一个完整的配置读取器

```rust
use std::fs;
use std::error::Error;
use std::num::ParseIntError;

#[derive(Debug)]
struct Config {
    name: String,
    port: u16,
}

#[derive(Debug)]
enum ConfigError {
    Io(String),
    ParseInt(ParseIntError),
    Format(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ConfigError::Io(p)        => write!(f, "无法读取配置 {}", p),
            ConfigError::ParseInt(e)  => write!(f, "数字解析失败: {e}"),
            ConfigError::Format(s)    => write!(f, "配置格式错误: {s}"),
        }
    }
}

impl std::error::Error for ConfigError {}
impl From<ParseIntError> for ConfigError { fn from(e: ParseIntError) -> Self { Self::ParseInt(e) } }

fn load_config(path: &str) -> Result<Config, ConfigError> {
    let content = fs::read_to_string(path)
        .map_err(|_| ConfigError::Io(path.to_string()))?;

    let mut name = None;
    let mut port = None;

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') { continue; }

        let (k, v) = line.split_once('=')
            .ok_or_else(|| ConfigError::Format(format!("缺少 =: {line}")))?;

        match k.trim() {
            "name" => name = Some(v.trim().to_string()),
            "port" => port = Some(v.trim().parse()?),
            other  => return Err(ConfigError::Format(format!("未知键 {other}"))),
        }
    }

    Ok(Config {
        name: name.ok_or_else(|| ConfigError::Format("缺少 name".into()))?,
        port: port.ok_or_else(|| ConfigError::Format("缺少 port".into()))?,
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    let cfg = load_config("app.conf")?;
    println!("name = {}, port = {}", cfg.name, cfg.port);
    Ok(())
}
```

> 配套示例配置 `app.conf`：
> ```
> # 应用配置
> name = my-app
> port = 8080
> ```

## 5.11 对比其他语言

| 风格 | Rust | Go | Java | Python |
|------|------|-----|------|--------|
| 标准做法 | `Result<T, E>` + `?` | 多返回值 `if err != nil` | 异常 + `throws` | 异常 + `try/except` |
| 错误是 | 值 | 值 | 控制流 | 控制流 |
| 不可忽略 | ✅ | 容易忽略 | 容易忽略 | 容易忽略 |
| 自定义错误 | enum + trait | `error` 接口 | 异常类 | 异常类 |

---

## 🏋️ 本章小练习

**练习 5.1**：写 `read_int(path: &str) -> Result<i32, AppError>`，从文件读一行数字并 parse 出来。`AppError` 用 enum 区分 I/O 和 parse 错误。

**练习 5.2**：把 `Box<dyn Error>` 改成自己的 enum 错误类型，重新实现上一题。

**练习 5.3**（用 `thiserror`）：在 `Cargo.toml` 加 `thiserror = "1"`，用 `#[derive(Error)]` 重写练习 5.2。

**练习 5.4**：`main` 返回 `Result<(), Box<dyn Error>>`，调用方捕获并打印"友好的错误信息 + 退出码 1"。

**练习 5.5**（思考题）：什么时候用 `panic!` 比 `Result` 合适？列出 3 种情况。

---

下一章：[06 · 模块与包 →](./06-modules-crates.md)
