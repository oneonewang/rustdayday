//! Project A · 异步 Web 后端
//!
//! 演示：axum 路由、共享状态、错误处理、tracing 日志、API 端到端自检。

mod error;
mod model;
mod routes;
mod state;

use axum::routing::get;
use axum::Router;
use axum::Server;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub use error::AppError;
pub use model::{CreateTask, Task, UpdateTask};
pub use state::AppState;

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(routes::health))
        .route("/tasks", get(routes::list_tasks).post(routes::create_task))
        .route("/tasks/:id", get(routes::get_task)
            .put(routes::update_task)
            .delete(routes::delete_task))
        .with_state(state)
}

pub async fn serve(addr: SocketAddr, state: AppState) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let app = build_router(state);
    let listener = TcpListener::bind(addr).await?;
    tracing::info!("listening on http://{addr}");
    let std_listener = listener.into_std()?;
    std_listener.set_nonblocking(false)?;
    Server::from_tcp(std_listener)?.serve(app.into_make_service()).await?;
    Ok(())
}

fn init_tracing() {
    let _ = tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("info,tower_http=info,axum=info")))
        .with(tracing_subscriber::fmt::layer())
        .try_init();
}

#[tokio::main]
async fn main() {
    init_tracing();

    // 启动模式：1) 真实服务，2) 自检
    let mode = std::env::args().nth(1).unwrap_or_else(|| "serve".into());
    match mode.as_str() {
        "serve" => {
            let state = AppState::new();
            // 塞两条示例数据
            let _ = state.add_task("学完 Stage 6 Project A".into()).await;
            let _ = state.add_task("准备 Project B".into()).await;
            serve("0.0.0.0:3000".parse().unwrap(), state).await.unwrap();
        }
        "selftest" => {
            run_self_test().await;
        }
        other => {
            eprintln!("未知模式: {other}（可选：serve / selftest）");
            std::process::exit(1);
        }
    }
}

/// 启动 server 在后台，curl 全部端点，验证响应。
async fn run_self_test() {
    init_tracing();
    let state = AppState::new();
    let port = pick_port();
    let addr: SocketAddr = format!("127.0.0.1:{port}").parse().unwrap();
    tokio::spawn(async move {
        let _ = serve(addr, state).await;
    });
    // 等 server 起来
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    let base = format!("http://127.0.0.1:{port}");

    println!("\n=== A 端到端自检 ===\n");

    println!("--- GET /health ---");
    let r: serde_json::Value = get_json(&format!("{base}/health")).await;
    println!("  -> {r}");

    println!("\n--- POST /tasks (create) ---");
    let r1: serde_json::Value = post_json(&format!("{base}/tasks"), &serde_json::json!({"title": "学 Rust"}))
        .await;
    println!("  -> {r1}");
    let id1 = r1["id"].as_str().unwrap().to_string();

    let r2: serde_json::Value = post_json(&format!("{base}/tasks"), &serde_json::json!({"title": "做 Stage 6"}))
        .await;
    let id2 = r2["id"].as_str().unwrap().to_string();
    println!("  -> {r2}");

    println!("\n--- GET /tasks (list) ---");
    let list: serde_json::Value = get_json(&format!("{base}/tasks")).await;
    println!("  -> {} tasks", list.as_array().unwrap().len());

    println!("\n--- GET /tasks/:id ---");
    let r3: serde_json::Value = get_json(&format!("{base}/tasks/{id1}")).await;
    println!("  -> {r3}");

    println!("\n--- PUT /tasks/:id (mark done) ---");
    let r4: serde_json::Value = put_json(&format!("{base}/tasks/{id1}"), &serde_json::json!({"done": true}))
        .await;
    println!("  -> {r4}");

    println!("\n--- DELETE /tasks/:id ---");
    let status = delete_request(&format!("{base}/tasks/{id2}")).await;
    println!("  -> status {status}");

    println!("\n--- GET /tasks/:id (after delete → 404) ---");
    let (status, body): (u16, serde_json::Value) = get_with_status(&format!("{base}/tasks/{id2}")).await;
    println!("  -> status {status}, body {body}");

    println!("\n--- 完整断言 ---");
    run_assertions(&base).await;
    println!("全部断言通过 ✅");
}

fn pick_port() -> u16 {
    // 让 OS 分配随机端口
    std::net::TcpListener::bind("127.0.0.1:0")
        .expect("bind 0")
        .local_addr()
        .unwrap()
        .port()
}

async fn get_json(url: &str) -> serde_json::Value {
    reqwest::get(url).await.unwrap().json().await.unwrap()
}

async fn get_with_status(url: &str) -> (u16, serde_json::Value) {
    let r = reqwest::get(url).await.unwrap();
    let status = r.status().as_u16();
    let body: serde_json::Value = r.json().await.unwrap_or(serde_json::Value::Null);
    (status, body)
}

async fn post_json(url: &str, body: &serde_json::Value) -> serde_json::Value {
    let client = reqwest::Client::new();
    client.post(url).json(body).send().await.unwrap().json().await.unwrap()
}

async fn put_json(url: &str, body: &serde_json::Value) -> serde_json::Value {
    let client = reqwest::Client::new();
    client.put(url).json(body).send().await.unwrap().json().await.unwrap()
}

async fn delete_request(url: &str) -> u16 {
    let client = reqwest::Client::new();
    client.delete(url).send().await.unwrap().status().as_u16()
}

async fn run_assertions(base: &str) {
    let r: serde_json::Value = get_json(&format!("{base}/health")).await;
    assert_eq!(r["status"], "ok");

    let r: serde_json::Value = post_json(
        &format!("{base}/tasks"),
        &serde_json::json!({"title": "assertion task"}),
    ).await;
    let id = r["id"].as_str().unwrap().to_string();
    assert!(!id.is_empty());
    assert_eq!(r["title"], "assertion task");
    assert_eq!(r["done"], false);

    // mark done
    let r: serde_json::Value = put_json(
        &format!("{base}/tasks/{id}"),
        &serde_json::json!({"done": true}),
    ).await;
    assert_eq!(r["done"], true);

    // delete
    let s = delete_request(&format!("{base}/tasks/{id}")).await;
    assert_eq!(s, 204);

    // 404
    let (s, _): (u16, _) = get_with_status(&format!("{base}/tasks/{id}")).await;
    assert_eq!(s, 404);
}
