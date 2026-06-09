// Exercise 04 · async / await
// 任务：见 README.md

use std::time::Duration;
use tokio::time::sleep;

async fn add(a: i32, b: i32) -> i32 { a + b }

async fn slow(name: &str, ms: u64) -> String {
    sleep(Duration::from_millis(ms)).await;
    format!("{name} done")
}

async fn slow_op() -> u32 {
    sleep(Duration::from_secs(1)).await;
    42
}

async fn with_timeout() -> Option<u32> { todo!() }

async fn first_done() -> u32 { todo!() }

async fn fetch_id(id: u32) -> Result<String, String> { todo!() }

#[tokio::main]
async fn main() {
    // TODO
}
