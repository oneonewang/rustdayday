//! Task 模型 + 请求/响应 DTO

use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub done: bool,
    pub created_at: u64,    // unix seconds
}

impl Task {
    pub fn new(title: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            done: false,
            created_at: now_secs(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateTask {
    pub title: String,
}

impl CreateTask {
    pub fn validate(&self) -> Result<(), String> {
        if self.title.trim().is_empty() {
            return Err("title 不能为空".into());
        }
        if self.title.len() > 200 {
            return Err("title 太长（>200 字符）".into());
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateTask {
    pub title: Option<String>,
    pub done: Option<bool>,
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}
