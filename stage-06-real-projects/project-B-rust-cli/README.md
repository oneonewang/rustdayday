# Project B · rustfind（高性能 CLI 内容搜索）

> 难度：⭐⭐⭐  
> 综合运用：clap / rayon / 自定义 trait / anyhow / regex / walkdir  
> 预计时间：3 – 4 小时

## 🎯 项目目标

实现一个**ripgrep 风格**的内容搜索 CLI：

```bash
rustfind "TODO" src/
rustfind -i "error" /var/log/
rustfind -l "fn main" project-*/src
rustfind --json "Box<" .
```

支持：正则、隐藏文件控制、扩展名过滤、并行搜索、JSON 输出、文件计数模式。

## 📂 项目结构

```
project-B-rust-cli/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs       # clap 解析 + 入口
    ├── args.rs       # CLI struct（clap derive）
    ├── error.rs      # anyhow 集成
    ├── source.rs     # Source trait + FileSource
    ├── sink.rs       # Sink trait + StdoutSink + CountingSink
    ├── walker.rs     # walkdir 集成
    └── search.rs     # 核心搜索 + rayon 并行
```

## 🌐 CLI 界面

```
Usage: rustfind [OPTIONS] <PATTERN> [PATHS]...

Arguments:
  <PATTERN>    要搜索的正则
  [PATHS]...   路径（默认 .）

Options:
  -i, --ignore-case              大小写不敏感
  -n, --line-number              显示行号（默认就有）
  -l, --files-with-matches        只列文件名
  -c, --count                    计数
      --hidden                   包含隐藏文件
      --skip-ext <EXT>           跳过的扩展名
      --ext <EXT>                只搜的扩展名
      --json                     JSON 输出
      --threads <N>              线程数
      --max-file-size <BYTES>    最大文件（默认 10MB）
```

## 🚀 怎么跑

```bash
cd project-B-rust-cli
cargo run -- "TODO" src/                 # 在 src 里找 TODO
cargo run -- -i "ERROR" /var/log/         # 不区分大小写
cargo run -- -l "fn main" .               # 只列文件
cargo run -- -c "Box" .                  # 计数
cargo run -- --json "Vec<u8>" .          # JSON
```

## 📚 核心抽象：Source 与 Sink

| Trait | 方法 | 用途 |
|-------|------|------|
| `Source` | `into_bufread() -> Box<dyn BufRead + Send>` | 从某处读字节流 |
| `Sink` | `consume(Match)` / `finalize()` | 消费匹配结果 |

> 这两个 trait 让搜索核心**不依赖**具体文件 / 输出——以后能轻松加 `StdinSource` / `NetworkSource` / `JsonlSink`。

## 🏃 运行示例

```
$ rustfind "TODO" stage-05-concurrency-async/docs/
stage-05-concurrency-async/docs/04-async-await.md:108:// TODO 跨 await 时的 Send 怎么推断？

$ rustfind -c "fn" stage-04-smart-pointers-fp/exercises/ex05_operator_overload/src/main.rs
.../ex05_operator_overload/src/main.rs:8
```

## 🎁 扩展挑战

1. **彩色输出**：路径高亮、匹配片段高亮（用 `owo-colors` 或 `colored`）
2. **进度条**：用 `indicatif` 显示大目录扫描进度
3. **`.gitignore` 支持**：用 `ignore` crate
4. **二进制文件检测**：跳过含 NUL 字节 > N% 的文件
5. **上下文显示**：`-C 3` 显示匹配上下 3 行
6. **性能基准**：用 `criterion` 对比单线程 vs rayon
7. **HTTP 来源**：`Source` 实现 `HttpSource`，能直接搜远端文件

## ✅ 完成判定

- [ ] `cargo build` 跑通
- [ ] 5 个核心 flag 都工作（`-i` / `-l` / `-c` / `--json` / `--hidden`）
- [ ] 错误路径（坏正则 / 不可读文件）不 panic
- [ ] 至少完成 2 个扩展挑战

完成 → 回到 [Stage 6 README](../README.md) 告诉我开始 Project C。
