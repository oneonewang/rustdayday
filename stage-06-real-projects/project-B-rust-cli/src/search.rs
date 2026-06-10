//! 核心搜索逻辑

use std::io::BufRead;
use std::sync::Arc;

use rayon::prelude::*;
use regex::Regex;

use crate::args::{Cli, OutputMode};
use crate::error::AppResult;
use crate::sink::Sink;
use crate::source::{FileSource, Source};
use crate::walker::{collect_files, is_file_too_large};

#[derive(Debug, Clone, serde::Serialize)]
pub struct Match {
    pub path: String,
    pub line: usize,
    pub text: String,
}

pub fn run_search<S: Sink + 'static>(cli: &Cli, mut sink: S) -> AppResult<bool> {
    // 编译 regex
    let pattern = if cli.ignore_case {
        format!("(?i){}", cli.pattern)
    } else {
        cli.pattern.clone()
    };
    let re = Arc::new(
        Regex::new(&pattern)
            .map_err(|e| anyhow::anyhow!("无效的正则 '{}': {e}", cli.pattern))?,
    );

    // 线程池
    if let Some(n) = cli.threads {
        rayon::ThreadPoolBuilder::new()
            .num_threads(n)
            .build_global()
            .ok();
    }

    // 收集文件
    let files = collect_files(cli);
    if files.is_empty() {
        eprintln!("rustfind: 没有匹配的文件");
        return Ok(false);
    }

    let max = cli.max_file_size;
    let mode = cli.output_mode();
    let re_for_thread = Arc::clone(&re);

    // 并行搜索
    let matches: Vec<Match> = files
        .par_iter()
        .filter_map(|path| {
            if is_file_too_large(path, max) {
                eprintln!("rustfind: 跳过（>{} bytes） {}", max, path.display());
                return None;
            }
            let source = FileSource::new(path.clone());
            let mut reader = match source.into_bufread() {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("rustfind: {}: {:#}", path.display(), e);
                    return None;
                }
            };
            let mut buf = String::new();
            let mut local_matches = vec![];
            let mut line_no = 0usize;
            loop {
                buf.clear();
                if reader.read_line(&mut buf).ok()? == 0 { break; }
                line_no += 1;
                if re_for_thread.is_match(&buf) {
                    local_matches.push(Match {
                        path: path.display().to_string(),
                        line: line_no,
                        text: buf.trim_end().to_string(),
                    });
                }
            }
            if local_matches.is_empty() {
                None
            } else {
                Some(local_matches)
            }
        })
        .flatten()
        .collect();

    // 模式分发
    let matched = !matches.is_empty();
    match mode {
        OutputMode::Text => {
            for m in matches {
                sink.consume(m)?;
            }
        }
        OutputMode::FilesWithMatches => {
            let mut seen = std::collections::HashSet::new();
            for m in matches {
                if seen.insert(m.path.clone()) {
                    println!("{}", m.path);
                }
            }
        }
        OutputMode::Count => {
            let mut counts: std::collections::BTreeMap<String, usize> = Default::default();
            for m in &matches {
                *counts.entry(m.path.clone()).or_insert(0) += 1;
            }
            for (p, n) in counts {
                sink.consume(Match { path: p, line: 0, text: n.to_string() })?;
            }
        }
    }
    sink.finalize()?;

    Ok(matched)
}
