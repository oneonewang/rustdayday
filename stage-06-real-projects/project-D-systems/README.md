# Project D · readelf-lite（系统编程）

> 难度：⭐⭐⭐⭐  
> 综合运用：unsafe Rust + libc FFI + memmap2 + 手解二进制格式 + 字节序 + 内存布局  
> 预计时间：3 – 4 小时

## 🎯 项目目标

实现一个迷你 `readelf` + `hexdump` 工具：

```bash
readelf-lite target/release/rustfind           # ELF 解析
readelf-lite Cargo.toml --hex                 # 文本当 binary 看
readelf-lite Cargo.lock --hex --limit 128     # 前 128 字节
readelf-lite README.md --strings              # 提取可打印字符串
readelf-lite target/release/rustfind --elf    # 强制 ELF 模式
```

## 📂 项目结构

```
project-D-systems/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs       # 入口 + 文件类型检测
    ├── args.rs       # clap 参数
    ├── error.rs      # anyhow
    ├── mmap_io.rs    # memmap2 包装（unsafe）
    ├── elf.rs        # 手解 ELF header（unsafe + libc::memcmp）
    └── hexdump.rs    # 经典 hexdump
```

## 📚 核心系统编程概念

| 概念 | 用法 |
|------|------|
| `unsafe` | mmap 映射、memcmp、裸指针读 |
| `libc::memcmp` | 演示 extern "C" FFI |
| `memmap2` | mmap 文件到虚拟内存（零拷贝读） |
| 手解二进制 | 字节序 (`from_le_bytes`)、结构布局 |
| `memchr`-类操作 | 字符串提取（手动实现） |

## 🏃 运行示例

```
$ readelf-lite target/release/rustfind
=== readelf-lite: target/release/rustfind ===
大小: 4827392 bytes

检测到: Elf

--- ELF Header ---
Class:         Elf64
Data:          Little
Version:       1
OS/ABI:        UnixSystemV (ver 0)
Type:          2 (ET_EXEC (executable))
Machine:       X86_64
Entry point:   0x1c5d0
PH offset:     64 (12 entries × 56 bytes)
SH offset:     4820224 (34 entries × 64 bytes)
Flags:         0
SH strndx:     33

--- 文件头前 16 字节 ---
7f 45 4c 46 02 01 01 00  00 00 00 00 00 00 00 00

[这是一个可执行文件（ET_EXEC）]

--- Section Headers (前 5 个) ---
  [0]  name_idx=0x0  type=0x0
  [1]  name_idx=0x1b  type=0x1
  [2]  name_idx=0x21  type=0x2
  ...
```

```
$ readelf-lite Cargo.toml --hex --limit 64
=== readelf-lite: Cargo.toml ===
大小: 187 bytes

检测到: 文本文件
--- hex dump (前 64 / 187 字节) ---
00000000  5b 70 61 63 6b 61 67 65  5d 0a 6e 61 6d 65 20 3d  |[package].name = |
00000010  20 22 72 65 61 64 65 6c  66 2d 6c 69 74 65 22 0a  | "readelf-lite".|
00000020  76 65 72 73 69 6f 6e 20  3d 20 22 30 2e 31 2e 30  |version = "0.1.0|
...
```

## 🎁 扩展挑战

1. **section name 解析**：从 `e_shstrndx` 拿字符串表，解析每个 section 的名字（`.text` / `.data` 等）
2. **program header 解析**：解析 segment 类型（LOAD / DYNAMIC / INTERP）
3. **PE 解析**：再写一个手解 PE 头
4. **写入 binary**：用 `pwrite` 把一个 ELF 的某 section 抠出来
5. **动态符号表**：解析 `.dynsym` 输出 imported functions
6. **`objdump` 风格反汇编**：用 `capstone` 库（如果缓存了）

## ⚠️ 关于 unsafe

本项目**故意**大量使用 unsafe——目的就是**学习** unsafe 的正确使用。每个 unsafe 块都有注释解释**为什么**是安全的。

| unsafe 块 | 安全性 |
|-----------|--------|
| `Mmap::map(&file)` | 库 API 本身 safe，被 memmap2 标记为 unsafe 是历史原因 |
| `libc::memcmp` | 传指针 + 长度都正确 |
| `bytes.as_ptr() as *const _` | 指针指向的是有效内存 |

## ✅ 完成判定

- [ ] `cargo build` 跑通
- [ ] 对自己（`./target/debug/readelf-lite ./target/debug/readelf-lite`）能正确解析
- [ ] 对文本文件能 hexdump
- [ ] `--strings` 能提取可打印字符串
- [ ] 至少完成 1 个扩展挑战（**section 名字解析**推荐）

完成 → 回到 [Stage 6 README](../README.md) 全部 4 个项目就完成。
