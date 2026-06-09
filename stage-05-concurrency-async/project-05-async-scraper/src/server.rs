// 极简 HTTP server（仅用于本地 demo）
// 给定 (path, body) 对；未知 path 返回 404
// 给定 (path, delay_ms) 可选地延迟响应

use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::Notify;

pub type Routes = Arc<HashMap<String, String>>;
pub type Delays = Arc<HashMap<String, u64>>;

pub async fn start(port: u16, routes: Routes, delays: Delays) -> u16 {
    let listener = TcpListener::bind(("127.0.0.1", port))
        .await
        .expect("failed to bind");
    let port = listener.local_addr().unwrap().port();

    tokio::spawn(async move {
        loop {
            let (mut socket, _) = match listener.accept().await {
                Ok(s) => s,
                Err(_) => continue,
            };
            let routes = Arc::clone(&routes);
            let delays = Arc::clone(&delays);
            tokio::spawn(async move {
                let mut buf = vec![0u8; 4096];
                let n = match socket.read(&mut buf).await {
                    Ok(n) if n > 0 => n,
                    _ => return,
                };
                let req = String::from_utf8_lossy(&buf[..n]);
                let path = req
                    .lines()
                    .next()
                    .and_then(|l| l.split_whitespace().nth(1))
                    .unwrap_or("/");

                // 模拟延迟
                if let Some(&ms) = delays.get(path) {
                    tokio::time::sleep(std::time::Duration::from_millis(ms)).await;
                }

                let (status_line, body) = match routes.get(path) {
                    Some(b) => ("HTTP/1.1 200 OK", b.clone()),
                    None    => ("HTTP/1.1 404 Not Found", "404 not found".to_string()),
                };

                let resp = format!(
                    "{}\r\nContent-Length: {}\r\nContent-Type: text/html; charset=utf-8\r\nConnection: close\r\n\r\n{}",
                    status_line, body.len(), body
                );
                let _ = socket.write_all(resp.as_bytes()).await;
            });
        }
    });

    // 防止 dead-code 警告（如果 Notify 不导入）
    let _ = Notify::const_new;

    port
}
