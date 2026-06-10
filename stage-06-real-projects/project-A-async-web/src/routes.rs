//! HTTP 路由处理函数

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;
use uuid::Uuid;

use crate::error::AppError;
use crate::model::{CreateTask, Task, UpdateTask};
use crate::state::AppState;

pub async fn health() -> impl IntoResponse {
    Json(json!({ "status": "ok" }))
}

pub async fn list_tasks(State(state): State<AppState>) -> impl IntoResponse {
    let v = state.list_tasks().await;
    Json(v)
}

pub async fn get_task(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Task>, AppError> {
    let t = state.get_task(id).await?;
    Ok(Json(t))
}

pub async fn create_task(
    State(state): State<AppState>,
    Json(body): Json<CreateTask>,
) -> Result<(StatusCode, Json<Task>), AppError> {
    let t = state.create_task(body).await?;
    Ok((StatusCode::CREATED, Json(t)))
}

pub async fn update_task(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateTask>,
) -> Result<Json<Task>, AppError> {
    let t = state.update_task(id, body).await?;
    Ok(Json(t))
}

pub async fn delete_task(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    state.delete_task(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
