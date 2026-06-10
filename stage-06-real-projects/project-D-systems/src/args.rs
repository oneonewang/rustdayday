//! CLI 参数

use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "readelf-lite", version, about = "极简 file/elf/hexdump 工具")]
pub struct Cli {
    /// 要分析的文件
    pub file: PathBuf,

    /// 模式（默认自动检测）
    #[arg(long, value_enum, default_value_t = Mode::Auto)]
    pub mode: Mode,

    /// 总是 hexdump（即使识别为 ELF）
    #[arg(long)]
    pub hex: bool,

    /// hexdump / 字符串提取的字节数限制
    #[arg(long, default_value_t = 256)]
    pub limit: usize,

    /// 字符串最小长度
    #[arg(long, default_value_t = 4)]
    pub min_str: usize,
}

#[derive(clap::ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Auto,
    Elf,
    Hex,
    Strings,
}
