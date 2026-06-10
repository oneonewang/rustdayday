//! Project B · rustfind
//!
//! ripgrep 风格内容搜索 CLI。

mod args;
mod error;
mod search;
mod sink;
mod source;
mod walker;

use std::process::ExitCode;

use crate::args::Cli;
use crate::search::run_search;
use crate::sink::{CountingSink, StdoutSink};

fn main() -> ExitCode {
    let cli = match Cli::parse_args() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("参数错误: {e}");
            return ExitCode::from(2);
        }
    };

    let result = match cli.output_mode() {
        args::OutputMode::Count => {
            let sink = CountingSink::new();
            run_search(&cli, sink)
        }
        _ => {
            let sink = StdoutSink::new(cli.json);
            run_search(&cli, sink)
        }
    };

    match result {
        Ok(matched) => {
            if matched {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(1)   // grep 习惯：1 = 没找到
            }
        }
        Err(e) => {
            eprintln!("错误: {e:#}");
            ExitCode::from(1)
        }
    }
}
