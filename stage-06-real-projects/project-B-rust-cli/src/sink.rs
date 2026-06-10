//! Sink trait：消费"匹配项"的输出目标
//!
//! 让"标准输出 / 计数 / 收集到 Vec"等都能用同一个搜索核心。

use std::io::{self, Write};
use std::sync::Mutex;

use crate::search::Match;

/// 一个匹配消费者
pub trait Sink: Send {
    /// 消费一个匹配
    fn consume(&mut self, m: Match) -> anyhow::Result<()>;

    /// 全部跑完的回调（聚合 / 排序等）—— 默认空操作
    fn finalize(self) -> anyhow::Result<()>
    where
        Self: Sized,
    {
        Ok(())
    }
}

/// 写到 stdout
pub struct StdoutSink {
    json: bool,
}

impl StdoutSink {
    pub fn new(json: bool) -> Self { Self { json } }
}

impl Sink for StdoutSink {
    fn consume(&mut self, m: Match) -> anyhow::Result<()> {
        let stdout = io::stdout();
        let mut h = stdout.lock();
        if self.json {
            let s = serde_json::to_string(&m)?;
            writeln!(h, "{s}")?;
        } else {
            writeln!(h, "{}:{}:{}", m.path, m.line, m.text)?;
        }
        Ok(())
    }

    fn finalize(self) -> anyhow::Result<()> {
        Ok(())
    }
}

/// 只数总数（每个文件一行）
pub struct CountingSink {
    /// (path, count)
    counts: Mutex<Vec<(String, usize)>>,
}

impl CountingSink {
    pub fn new() -> Self { Self { counts: Mutex::new(vec![]) } }

    pub fn into_counts(self) -> Vec<(String, usize)> {
        self.counts.into_inner().unwrap()
    }
}

impl Sink for CountingSink {
    fn consume(&mut self, m: Match) -> anyhow::Result<()> {
        let mut counts = self.counts.lock().unwrap();
        if let Some((_, c)) = counts.iter_mut().find(|(p, _)| p == &m.path) {
            *c += 1;
        } else {
            counts.push((m.path, 1));
        }
        Ok(())
    }

    fn finalize(self) -> anyhow::Result<()> {
        for (path, n) in self.into_counts() {
            println!("{path}:{n}");
        }
        Ok(())
    }
}
