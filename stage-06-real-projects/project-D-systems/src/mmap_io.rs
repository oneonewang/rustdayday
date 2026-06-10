//! mmap 包装：把文件映射到内存，省去 read/seek

use std::fs::File;
use std::path::Path;

use anyhow::Result;
use memmap2::Mmap;

pub struct MappedFile {
    _file: File,
    mmap: Mmap,
}

impl MappedFile {
    pub fn open(path: &Path) -> Result<Self> {
        let file = File::open(path)?;
        let mmap = unsafe { Mmap::map(&file) }?;
        Ok(Self { _file: file, mmap })
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.mmap
    }
}
