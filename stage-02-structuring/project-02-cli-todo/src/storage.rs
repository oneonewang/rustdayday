// storage.rs — 读写 JSON 文件

use std::fs;
use std::path::Path;

use crate::error::AppError;
use crate::todo::TodoList;

pub fn load(path: &str) -> Result<TodoList, AppError> {
    if !Path::new(path).exists() {
        return Ok(TodoList::default());
    }
    let content = fs::read_to_string(path)?;
    if content.trim().is_empty() {
        return Ok(TodoList::default());
    }
    let list: TodoList = serde_json::from_str(&content)?;
    Ok(list)
}

pub fn save(path: &str, list: &TodoList) -> Result<(), AppError> {
    let content = serde_json::to_string_pretty(list)?;
    fs::write(path, content)?;
    Ok(())
}
