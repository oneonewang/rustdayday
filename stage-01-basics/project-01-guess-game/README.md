# Project 01 · 猜数字游戏

> 难度：⭐⭐  
> 综合运用：循环、match、标准输入、外部 crate、错误处理  
> 预计时间：1.5 – 2 小时（包含读文档、敲代码、跑通）

## 🎯 项目目标

写一个命令行猜数字游戏：

1. 程序"心里"想一个 1–100 之间的整数
2. 玩家从键盘输入猜测
3. 程序告诉玩家"大了" / "小了" / "猜中了"
4. 一直循环到猜中为止
5. 显示总尝试次数
6. 支持输入 `quit` / `exit` 退出

## 🚀 怎么跑

```bash
cd project-01-guess-game
cargo run
```

## 📚 你需要知道的概念

| 概念 | 出处 | 提示 |
|------|------|------|
| 外部 crate 依赖 | 第 2 章 | 在 `Cargo.toml` 加 `rand = "0.8"` |
| 随机数 | crate 文档 | `rand::thread_rng().gen_range(1..=100)` |
| 标准输入 | std 文档 | `io::stdin().read_line(&mut buf)` |
| 字符串解析 | 第 3 章 | `s.trim().parse::<u32>()` |
| `match` 表达式 | 第 4 章 | `match guess.cmp(&secret) { Less => ..., Greater => ..., Equal => ... }` |
| 循环 | 第 4 章 | `loop { ... break; }` |
| 错误处理 | 入门即可 | 解析失败用 `match`，输入失败 `expect` |

## 🪜 分步建议

按这个顺序写，每步都能跑：

### Step 1：Hello

`main` 只打印一行欢迎语。`cargo run` 验证能跑。

### Step 2：硬编码的 secret

```rust
let secret = 42;
println!("秘密数字是 {secret}（先写死）");
```

### Step 3：读一次输入并打印

```rust
let mut buf = String::new();
io::stdin().read_line(&mut buf).expect("...");
let guess: u32 = buf.trim().parse().expect("请输入数字");
println!("你输入的是 {guess}");
```

### Step 4：加循环 + 比较

```rust
loop {
    // 读输入
    // match guess.cmp(&secret) { ... }
    // 猜中 break
}
```

### Step 5：加 rand

在 `Cargo.toml` 加依赖，`use rand::Rng;`，`gen_range` 替代硬编码。

### Step 6：解析错误优雅处理

用 `match guess.parse() { Ok(n) => n, Err(_) => { println!("...重新输入"); continue; } }` 替代 `expect`。

### Step 7：完成最终版

加退出命令、尝试次数、最终提示。

## 🧪 测试建议

跑一遍下面这个流程，确认每个分支都对：

```
=== 猜数字游戏 ===
我已经想好了一个 1 到 100 之间的整数。
输入你的猜测，我会告诉你是大了、小了还是猜中了。
输入 'quit' 退出游戏。

请输入你的猜测：
abc
'abc' 不是合法数字，请重新输入。

请输入你的猜测：
50
你猜的是：50
太大了，再小一点！

请输入你的猜测：
25
你猜的是：25
太小了，再大一点！

请输入你的猜测：
quit
下次再玩！秘密数字是 X。
```

## 🎁 扩展挑战（可选）

1. **限制最大尝试次数**（比如 7 次），超过就 Game Over
2. **难度选择**：开始时问玩家"简单 (1-50) / 普通 (1-100) / 困难 (1-1000)"
3. **成绩历史**：用 `Vec<(u32, u32)>` 存 (局数, 尝试次数)，结束游戏时打印
4. **倒计时**：用 `std::time::Instant` 记录耗时
5. **可重玩**：猜中后问"再玩一局？(y/n)"

## 📂 项目结构

```
project-01-guess-game/
├── Cargo.toml
├── README.md   ← 你正在读
└── src/
    └── main.rs ← 完整参考实现
```

## 🔗 参考资料

- 官方 Book 第 2 章：<https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html>
- `rand` crate 文档：<https://docs.rs/rand/>
- `std::io` 文档：<https://doc.rust-lang.org/std/io/index.html>

## ✅ 完成判定

- [ ] `cargo run` 能跑通
- [ ] 程序能读输入、解析、比较
- [ ] 解析错误不崩溃
- [ ] 输入 `quit` 能退出
- [ ] 至少完成 1 个扩展挑战（可选）

完成 → 回到 [Stage 1 README](../README.md) 准备进入 Stage 2。
