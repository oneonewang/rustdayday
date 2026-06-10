//! Project D · readelf-lite
//!
//! 演示：unsafe Rust + libc FFI + memmap2 + 手解二进制格式。

mod args;
mod elf;
mod error;
mod hexdump;
mod mmap_io;

use std::process::ExitCode;

use clap::Parser;

use crate::args::Cli;

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(&cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("错误: {e:#}");
            ExitCode::from(1)
        }
    }
}

fn run(cli: &Cli) -> anyhow::Result<()> {
    // mmap 文件
    let mapped = mmap_io::MappedFile::open(&cli.file)?;
    let bytes = mapped.as_slice();

    println!("=== readelf-lite: {} ===", cli.file.display());
    println!("大小: {} bytes\n", bytes.len());

    // 检测格式
    if let Some(kind) = detect_kind(bytes) {
        println!("检测到: {:?}\n", kind);
    } else {
        println!("检测到: 未知格式");
    }

    match cli.mode {
        args::Mode::Auto => {
            if let Some(kind) = detect_kind(bytes) {
                match kind {
                    FileKind::Elf => print_elf_info(bytes)?,
                    FileKind::Text => {
                        println!("(看起来像纯文本文件)");
                        if cli.hex {
                            hexdump::print_hex(bytes, cli.limit);
                        }
                    }
                }
            } else {
                hexdump::print_hex(bytes, cli.limit);
            }
        }
        args::Mode::Elf => print_elf_info(bytes)?,
        args::Mode::Hex => hexdump::print_hex(bytes, cli.limit),
        args::Mode::Strings => {
            // 提取可打印字符串（用 libc::memchr 找 \0 边界）
            print_strings(bytes, cli.min_str);
        }
    }

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileKind {
    Elf,
    Text,
}

pub fn detect_kind(bytes: &[u8]) -> Option<FileKind> {
    // ELF magic: 0x7F 'E' 'L' 'F'
    if bytes.len() >= 4 && bytes[..4] == [0x7f, b'E', b'L', b'F'] {
        return Some(FileKind::Elf);
    }
    // 纯文本启发：80% 以上是可打印 ASCII / UTF-8
    if !bytes.is_empty() {
        let printable = bytes
            .iter()
            .take(4096)
            .filter(|&&b| (0x20..=0x7e).contains(&b) || b == b'\n' || b == b'\t' || b == b'\r')
            .count();
        if printable * 100 / bytes.len().min(4096) > 80 {
            return Some(FileKind::Text);
        }
    }
    None
}

fn print_elf_info(bytes: &[u8]) -> anyhow::Result<()> {
    let info = elf::parse(bytes)?;
    elf::print_info(&info, bytes);
    Ok(())
}

fn print_strings(bytes: &[u8], min_len: usize) {
    println!("--- 可打印字符串 (min={} bytes) ---\n", min_len);
    let mut start: Option<usize> = None;
    let mut current = 0usize;
    for (i, &b) in bytes.iter().enumerate() {
        if (0x20..=0x7e).contains(&b) {
            if start.is_none() {
                start = Some(i);
            }
            current = i;
        } else {
            flush_str(bytes, &mut start, &mut current, i, min_len);
        }
    }
    flush_str(bytes, &mut start, &mut current, bytes.len(), min_len);
}

fn flush_str(bytes: &[u8], start: &mut Option<usize>, current: &mut usize, i: usize, min_len: usize) {
    if let Some(s) = start.take() {
        let len = *current - s + 1;
        if len >= min_len {
            if let Ok(text) = std::str::from_utf8(&bytes[s..=*current]) {
                println!("{:08x}  {}", s, text);
            }
        }
    }
}
