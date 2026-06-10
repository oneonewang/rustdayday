//! 文件系统遍历：walkdir + 过滤

use std::path::{Path, PathBuf};

use walkdir::WalkDir;

use crate::args::Cli;

/// 把 Cli 里的 paths 展开成"实际要扫的文件列表"
pub fn collect_files(cli: &Cli) -> Vec<PathBuf> {
    let mut out = vec![];
    for p in &cli.paths {
        if p.is_file() {
            out.push(p.clone());
        } else if p.is_dir() {
            let walker = WalkDir::new(p)
                .follow_links(false)
                .into_iter()
                .filter_entry(|e| filter_entry(e, cli));
            for entry in walker.flatten() {
                if entry.file_type().is_file() {
                    out.push(entry.path().to_path_buf());
                }
            }
        }
    }
    out.sort();
    out.dedup();
    out
}

fn filter_entry(entry: &walkdir::DirEntry, cli: &Cli) -> bool {
    // 隐藏文件
    if !cli.hidden {
        if let Some(name) = entry.file_name().to_str() {
            if name.starts_with('.') {
                return false;
            }
        }
    }

    // 目录总是 pass（除非要 hide）
    if entry.file_type().is_dir() {
        return true;
    }

    // 扩展名过滤
    if let Some(ext) = entry.path().extension().and_then(|s| s.to_str()) {
        let ext_lower = ext.to_lowercase();
        if !cli.skip_ext.is_empty() && cli.skip_ext.iter().any(|e| e.to_lowercase() == ext_lower) {
            return false;
        }
        if !cli.ext.is_empty() && !cli.ext.iter().any(|e| e.to_lowercase() == ext_lower) {
            return false;
        }
    } else if !cli.ext.is_empty() {
        // 指定了 --ext 但文件没扩展名——跳过
        return false;
    }

    true
}

/// 跳过过大的文件
pub fn is_file_too_large(path: &Path, max: u64) -> bool {
    match std::fs::metadata(path) {
        Ok(m) => m.len() > max,
        Err(_) => false,
    }
}
