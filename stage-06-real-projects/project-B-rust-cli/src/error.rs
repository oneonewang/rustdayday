//! 错误类型——anyhow::Result + 自定义严重性

use anyhow::Result;

pub type AppResult<T> = Result<T>;
