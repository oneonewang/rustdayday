//! Source trait：从"某处"读出字节流
//!
//! 设计为 trait 让"文件 / stdin / 网络 / 压缩文件"都能统一接入。

use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::PathBuf;

/// 任何能产生按行读取的字节流
pub trait Source {
    /// 拿到一个 BufRead（一次读完）
    fn into_bufread(self) -> anyhow::Result<Box<dyn BufRead + Send>>;
}

pub struct FileSource {
    pub path: std::path::PathBuf,
}

impl FileSource {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }
}

impl Source for FileSource {
    fn into_bufread(self) -> anyhow::Result<Box<dyn BufRead + Send>> {
        let f = File::open(&self.path)?;
        Ok(Box::new(BufReader::new(f)))
    }
}
