// error.rs — 统一错误类型

use std::fmt;

#[derive(Debug)]
pub enum AppError {
    NotFound(u32),
    Io(std::io::Error),
    Json(serde_json::Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::NotFound(id)    => write!(f, "任务 #{id} 不存在"),
            AppError::Io(e)            => write!(f, "I/O 错误: {e}"),
            AppError::Json(e)          => write!(f, "JSON 解析错误: {e}"),
        }
    }
}

impl std::error::Error for AppError {}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self { AppError::Io(e) }
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self { AppError::Json(e) }
}
