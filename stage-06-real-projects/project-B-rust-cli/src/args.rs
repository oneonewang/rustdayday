//! CLI 参数定义（clap derive）

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug, Clone)]
#[command(name = "rustfind", version, about = "ripgrep-style content search")]
pub struct Cli {
    /// 要搜索的正则表达式
    pub pattern: String,

    /// 要搜索的路径（文件或目录；默认 .）
    #[arg(default_value = ".")]
    pub paths: Vec<PathBuf>,

    /// 大小写不敏感
    #[arg(short, long)]
    pub ignore_case: bool,

    /// 显示行号
    #[arg(short = 'n', long)]
    pub line_number: bool,

    /// 只列出匹配的文件名
    #[arg(short = 'l', long)]
    pub files_with_matches: bool,

    /// 每个文件显示匹配数
    #[arg(short, long)]
    pub count: bool,

    /// 包含隐藏文件
    #[arg(long)]
    pub hidden: bool,

    /// 跳过的扩展名（逗号分隔，如 "log,tmp"）
    #[arg(long, value_delimiter = ',')]
    pub skip_ext: Vec<String>,

    /// 只搜索的扩展名（逗号分隔）
    #[arg(long, value_delimiter = ',')]
    pub ext: Vec<String>,

    /// 输出 JSON 格式
    #[arg(long)]
    pub json: bool,

    /// 线程数（默认：CPU 核数）
    #[arg(long)]
    pub threads: Option<usize>,

    /// 最大文件大小（字节；默认 10MB）
    #[arg(long, default_value = "10485760")]
    pub max_file_size: u64,

    /// 模式：text / files-with-matches / count（一般由其他 flag 自动推导）
    #[arg(long, value_enum, hide = true)]
    pub mode: Option<OutputMode>,
}

#[derive(clap::ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutputMode {
    Text,
    FilesWithMatches,
    Count,
}

impl Cli {
    pub fn parse_args() -> Result<Self> {
        Ok(Cli::parse())
    }

    /// 推导实际输出模式（CLI flag → OutputMode）
    pub fn output_mode(&self) -> OutputMode {
        if let Some(m) = self.mode {
            return m;
        }
        if self.count {
            OutputMode::Count
        } else if self.files_with_matches {
            OutputMode::FilesWithMatches
        } else {
            OutputMode::Text
        }
    }
}
