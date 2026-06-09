# Exercise 05 · 错误处理

> 难度：⭐⭐  涉及：第 5 章

## 任务

写一个 `read_numbers` 函数，**不用 `unwrap`**，用自定义 enum 错误类型。

```rust
#[derive(Debug)]
enum AppError {
    Io(String),                              // 路径 / 错误信息
    Parse(String, std::num::ParseIntError),  // 出错的行内容
    Format(String),                          // 格式错误
}

impl std::fmt::Display for AppError { ... }
impl std::error::Error for AppError {}
impl From<std::io::Error> for AppError { ... }
impl From<std::num::ParseIntError> for AppError { ... }

fn read_numbers(path: &str) -> Result<Vec<i32>, AppError> {
    // 1. 打开并读文件
    // 2. 每行用 trim()
    // 3. 空行 / 注释行（# 开头）跳过
    // 4. parse 到 i32；失败 → Err(Parse(...))
    // 5. 收集到 Vec
}
```

## 测试

准备一个 `numbers.txt`（**在 `src/main.rs` 里写**生成它的代码）：

```
# 测试文件
10
-5
0
42
abc           # 这一行应触发 Parse 错误
100
```

## 验收

- `read_numbers("numbers.txt")` 应返回 `Err(...)`（带行号或内容）
- 把测试文件修成全合法数字，重新跑应返回 `Ok(vec![10, -5, 0, 42, 100])`
- `main` 用 `?` 调 `read_numbers`，根据 `Result` 决定打印成功列表 / 错误

## 提示

- `?` 自动调用 `From::from`，所以实现 `From<io::Error>` 和 `From<ParseIntError>` 就能让 `?` 转换
- 想要错误里带行号？自己维护一个 `line_no` 计数器
- 写文件用 `std::fs::write`

## 进阶

用 `thiserror` crate 简化错误定义：

```toml
[dependencies]
thiserror = "1"
```

```rust
use thiserror::Error;

#[derive(Debug, Error)]
enum AppError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("parse error on line {line}: {source}")]
    Parse {
        line: usize,
        #[source]
        source: std::num::ParseIntError,
    },

    #[error("format error: {0}")]
    Format(String),
}
```

完成 → [ex06_modules](../ex06_modules)
