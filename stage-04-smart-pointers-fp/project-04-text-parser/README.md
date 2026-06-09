# Project 04 · JSON 解析器

> 难度：⭐⭐⭐  
> 综合运用：自定义 `Parser` 结构、状态机式分发、闭包 + 迭代器、`Box` 不需要（直接用 enum 即可）、`Result` 错误处理  
> 预计时间：2 – 3 小时

## 🎯 项目目标

实现一个**手写** JSON 解析器，支持：

- `null` / `true` / `false`
- 数字（正负、小数、指数）
- 字符串
- 数组（嵌套）
- 对象（嵌套）
- 完整错误信息（"expected X, got Y"）

不支持（**留作扩展**）：转义字符 \n \t \"

## 📂 项目结构

```
project-04-text-parser/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs       # 演示 + 完整断言
    └── json.rs       # 解析器实现
```

## 🚀 怎么跑

```bash
cd project-04-text-parser
cargo run
```

## 📚 涉及的核心概念

| 概念 | 出处 | 用法 |
|------|------|------|
| 状态机 | 第 4 章 | `parse_value` 按首字符 dispatch |
| 字符级迭代 | 第 4 章 | `peek` / `bump` 不分配 |
| 闭包分发 | 第 3 章 | （可选）把状态机改成 `match` 闭包表 |
| `enum` 携带数据 | Stage 2 | `Json` 6 个变体 |
| 模式匹配穷尽 | Stage 2 | 解析分支 |
| `Result` 错误处理 | Stage 2 | `Result<Json, String>` |
| 字符串切片 | Stage 1 | 引用原输入，零分配 |

## 🪜 怎么写

### Step 1：定义 `Json` enum

```rust
pub enum Json {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),                // 递归
    Object(Vec<(String, Json)>),
}
```

### Step 2：定义 `Parser` 结构

```rust
pub struct Parser<'a> {
    input: &'a str,                  // 借原 input
    pos: usize,
}
```

辅助方法：`peek` / `bump` / `expect(c)` / `consume(lit)` / `skip_whitespace`。

### Step 3：dispatch 函数

```rust
fn parse_value(&mut self) -> Result<Json, String> {
    let c = self.peek()?;
    match c {
        'n' => self.parse_null(),
        't' | 'f' => self.parse_bool(),
        '"' => self.parse_string(),
        '[' => self.parse_array(),
        '{' => self.parse_object(),
        '-' | '0'..='9' => self.parse_number(),
        _ => Err(format!("unexpected '{c}'")),
    }
}
```

### Step 4：每种类型的 parse_*

每个都是 ~10-20 行，模式都很像。

### Step 5：Display

让 `println!("{j}")` 能打印出可读 JSON。

## 🧪 测试

`main.rs` 里跑下面这些：

1. 基本值：null / true / false / 42 / -3.14 / "hi"
2. 空数组 / 数组 / 嵌套数组
3. 对象 / 嵌套对象
4. 错误：6 种坏输入
5. Display 格式正确

## 🏃 运行结果示例

```
=== 演示 1：基本值 ===
null                       => null
true                       => true
42                         => 42
-3.14                      => -3.14
"hello, rust"              => "hello, rust"

=== 演示 3：嵌套对象 ===
{
  "name": "alice",
  "age": 30,
  ...
}

=== 演示 4：错误处理 ===
nul           => ERR: expected literal 'null'
[1, 2,        => ERR: unexpected end of input
{"a": }       => ERR: expected literal 'null'  （错——想优化"key 必须是 string"的报错）
tru           => ERR: expected 'true' or 'false'
```

## 🎁 扩展挑战

1. **转义字符**：支持 `\n` / `\t` / `\"` / `\\` / `\uXXXX`
2. **`Value` 访问助手**：`as_str()` / `as_number()` / `as_array()` 等
3. **Pretty print**：缩进格式化输出
4. **JSON Pointer**：实现 RFC 6901 风格的 `["a", "b", 0]` 路径查询
5. **Schema 验证**：写一个 schema 类型，validate JSON 是不是符合
6. **流式解析**：用自定义 `Iterator<Item = Json>` 一次返回多个 JSON 值
7. **完整 serde 兼容**：支持 NaN / Infinity 等扩展

## ⚠️ 简化说明

- 字符串里的 `\"` `\\` 会被**原样保留**（不展开）
- 数字精度按 f64（实际 JSON 用任意精度）
- 错误信息是字符串，不带行列号

## ✅ 完成判定

- [ ] `cargo run` 跑通
- [ ] 所有演示输出合理
- [ ] 6 个错误用例都返回 `Err`
- [ ] Display 输出跟标准 JSON 一致
- [ ] 至少完成 1 个扩展挑战（**`as_str()` 助手** 推荐）

完成 → 回到 [Stage 4 README](../README.md) 准备进入 Stage 5。
