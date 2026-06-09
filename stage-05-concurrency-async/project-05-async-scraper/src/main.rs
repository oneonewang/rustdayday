// Project 05 · 异步 HTTP 抓取器
// 任务：见 README.md

mod server;

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use futures::future::join_all;
use tokio::sync::Semaphore;

#[derive(Debug)]
struct FetchResult {
    url: String,
    status: Result<u16, String>,
    body_len: usize,
    elapsed: Duration,
}

async fn fetch_one(client: reqwest::Client, url: String, dur: Duration) -> FetchResult {
    let start = Instant::now();
    let result = tokio::time::timeout(dur, async {
        let resp = client.get(&url).send().await.map_err(|e| e.to_string())?;
        let status = resp.status().as_u16();
        let body = resp.text().await.map_err(|e| e.to_string())?;
        Ok::<(u16, usize), String>((status, body.len()))
    })
    .await;

    let elapsed = start.elapsed();
    match result {
        Ok(Ok((status, body_len))) => FetchResult { url, status: Ok(status), body_len, elapsed },
        Ok(Err(e))                 => FetchResult { url, status: Err(e), body_len: 0, elapsed },
        Err(_)                     => FetchResult { url, status: Err("timeout".into()), body_len: 0, elapsed },
    }
}

async fn run_demo() {
    println!("=== 演示 1：基础并发抓取（3 URLs）===");
    let mut routes = HashMap::new();
    routes.insert("/".to_string(), "<h1>Home</h1>".repeat(50));
    routes.insert("/about".to_string(), "<h1>About</h1>".repeat(20));
    routes.insert("/blog".to_string(), "<h1>Blog</h1>".repeat(100));
    let mut delays = HashMap::new();
    delays.insert("/slow".to_string(), 200);   // /slow 延迟 200ms
    let routes = Arc::new(routes);
    let delays = Arc::new(delays);

    let port = server::start(0, Arc::clone(&routes), Arc::clone(&delays)).await;
    let base = format!("http://127.0.0.1:{port}");

    let urls = vec![format!("{base}/"), format!("{base}/about"), format!("{base}/blog")];
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .user_agent("rustdayday-scraper/0.1")
        .build()
        .unwrap();

    let start = Instant::now();
    let results: Vec<_> = join_all(
        urls.iter().cloned().map(|u| {
            let c = client.clone();
            async move { fetch_one(c, u, Duration::from_secs(2)).await }
        })
    ).await;
    let total = start.elapsed();

    print_results(&results, total);

    println!("\n=== 演示 2：限流 2 并发抓 5 URLs ===");
    let urls: Vec<String> = (1..=5).map(|i| format!("{base}/p/{i}")).collect();
    // 注：这些 URL 在 server 里没有注册，会 404，但**耗时仍然模拟**
    let sem = Arc::new(Semaphore::new(2));
    let start = Instant::now();
    let results: Vec<_> = join_all(
        urls.iter().cloned().enumerate().map(|(i, u)| {
            let c = client.clone();
            let sem = Arc::clone(&sem);
            async move {
                let _permit = sem.acquire().await.unwrap();
                // 额外 sleep 200ms 模拟网络延迟——展示限流效果
                tokio::time::sleep(Duration::from_millis(200)).await;
                println!("  worker #{i} 开始抓 {u}");
                let r = fetch_one(c, u, Duration::from_secs(2)).await;
                println!("  worker #{i} 完成");
                r
            }
        })
    ).await;
    let total = start.elapsed();
    print_results(&results, total);
    println!("  （5 个任务各 200ms，限流 2 并发 → 总耗时 ≈ 5×200/2 = 500ms）");

    println!("\n=== 演示 3：超时处理 ===");
    let slow_url = format!("{base}/slow");
    let start = Instant::now();
    let r = fetch_one(client.clone(), slow_url.clone(), Duration::from_millis(100)).await;
    let total = start.elapsed();
    println!("{} 超时阈值 100ms，实际耗时 {:?}", r.url, r.elapsed);
    match r.status {
        Err(e) if e == "timeout" => println!("  ✅ 正确超时"),
        _                         => println!("  ❌ 期望超时但得到 {:?}", r.status),
    }

    println!("\n=== 演示 4：错误 URL ===");
    let bad = vec![
        format!("{base}/nope"),        // 404
        format!("{base}/"),            // OK
        "http://127.0.0.1:1/never".to_string(),  // 连不上
    ];
    let results = join_all(
        bad.iter().cloned().map(|u| {
            let c = client.clone();
            async move { fetch_one(c, u, Duration::from_secs(2)).await }
        })
    ).await;
    for r in &results {
        match &r.status {
            Ok(s) => println!("  ✓ {:<40} status={}", r.url, s),
            Err(e) => println!("  ✗ {:<40} ERR: {}", r.url, e),
        }
    }

    println!("\n=== 演示 5：所有断言 ===");
    run_assertions(&routes, &client).await;
    println!("全部断言通过 ✅");
}

fn print_results(results: &[FetchResult], total: Duration) {
    for r in results {
        match &r.status {
            Ok(s)  => println!("  ✓ {:<40} {} {:>6} bytes in {:?}", r.url, s, r.body_len, r.elapsed),
            Err(e) => println!("  ✗ {:<40} ERR: {} in {:?}", r.url, e, r.elapsed),
        }
    }
    println!("  总耗时: {:?}", total);
}

async fn run_assertions(routes: &Arc<HashMap<String, String>>, client: &reqwest::Client) {
    // 用 server::start 跑一个真 server
    let delays: Arc<HashMap<String, u64>> = Arc::new(HashMap::new());
    let port = server::start(0, Arc::clone(routes), Arc::clone(&delays)).await;
    let url = format!("http://127.0.0.1:{port}/");
    let r = fetch_one(client.clone(), url, Duration::from_secs(1)).await;
    assert_eq!(r.status, Ok(200));
    assert!(r.body_len > 0);

    // 404 测试
    let url_404 = format!("http://127.0.0.1:{port}/nope");
    let r404 = fetch_one(client.clone(), url_404, Duration::from_secs(1)).await;
    assert_eq!(r404.status, Ok(404));
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    run_demo().await;
}
