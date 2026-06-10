//! AppState：进程内共享的 CRUD 存储
//!
//! 真实生产应用换 sqlx / diesel / sea-orm；这里用 in-memory HashMap 演示模式。

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::error::AppError;
use crate::model::{CreateTask, Task, UpdateTask};

#[derive(Clone)]
pub struct AppState {
    tasks: Arc<RwLock<HashMap<Uuid, Task>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn add_task(&self, title: String) -> Task {
        let t = Task::new(title);
        self.tasks.write().await.insert(t.id, t.clone());
        t
    }

    pub async fn list_tasks(&self) -> Vec<Task> {
        let mut v: Vec<Task> = self.tasks.read().await.values().cloned().collect();
        v.sort_by_key(|t| t.created_at);
        v
    }

    pub async fn get_task(&self, id: Uuid) -> Result<Task, AppError> {
        self.tasks.read().await.get(&id).cloned()
            .ok_or(AppError::NotFound(id))
    }

    pub async fn create_task(&self, body: CreateTask) -> Result<Task, AppError> {
        body.validate().map_err(AppError::BadRequest)?;
        let t = Task::new(body.title);
        self.tasks.write().await.insert(t.id, t.clone());
        Ok(t)
    }

    pub async fn update_task(&self, id: Uuid, body: UpdateTask) -> Result<Task, AppError> {
        let mut map = self.tasks.write().await;
        let t = map.get_mut(&id).ok_or(AppError::NotFound(id))?;
        if let Some(title) = body.title {
            if title.trim().is_empty() {
                return Err(AppError::BadRequest("title 不能为空".into()));
            }
            t.title = title;
        }
        if let Some(done) = body.done {
            t.done = done;
        }
        Ok(t.clone())
    }

    pub async fn delete_task(&self, id: Uuid) -> Result<(), AppError> {
        let removed = self.tasks.write().await.remove(&id);
        if removed.is_some() { Ok(()) } else { Err(AppError::NotFound(id)) }
    }
}

impl Default for AppState {
    fn default() -> Self { Self::new() }
}
