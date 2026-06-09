# 04 · Option 与 Result ⭐

> **本章目标**：掌握 Rust 取代 `null` 和异常的两个核心类型：`Option<T>` 和 `Result<T, E>`。这是写"安全 Rust"的基础。

## 4.1 为什么 Rust 没有 `null`？

`null`（或 `nil` / `None` / `undefined`）是著名的 **"十亿美元错误"**（Tony Hoare 原话）——它让"值可能不存在"成为隐式的、运行时才暴露的陷阱。

Rust 的解决：**用类型系统强制你处理"没有"的情况**。

```rust
let x: i32 = null;       // ❌ Rust 中根本不存在 null
let x: Option<i32> = None;  // ✅ 想表达"可能没有"？用 Option
```

## 4.2 `Option<T>`：值可能不存在

```rust
enum Option<T> {
    None,        // 没有
    Some(T),     // 有，值是 T
}
```

> 它是标准库 `enum`——`Option` / `Some` / `None` 不需要 `use`，**直接用**。

```rust
fn find(haystack: &[i32], needle: i32) -> Option<usize> {
    for (i, &x) in haystack.iter().enumerate() {
        if x == needle {
            return Some(i);
        }
    }
    None
}

fn main() {
    let xs = [1, 2, 3, 5, 8];
    match find(&xs, 3) {
        Some(i) => println!("找到了，下标 {i}"),
        None    => println!("没找到"),
    }
}
```

> ⚠️ `Option<T>` **比 `T` 多一种状态**——`match` 必须两个分支都处理。

## 4.3 `Option` 的常用组合子

`Option` 有大量方法——下面这些最常用：

| 方法 | 行为 | 签名 |
|------|------|------|
| `unwrap()` | `Some` 拿值，`None` panic | `T` |
| `expect(msg)` | `unwrap` 但 panic 信息可定制 | `T` |
| `unwrap_or(default)` | `None` 时给默认值 | `T` |
| `unwrap_or_else(\|\| ...)` | `None` 时调用闭包 | `T` |
| `map(\|x\| ...)` | 转换内部值 | `Option<U>` |
| `and_then(\|x\| ...)` | 链式（返回 `Option` 的函数） | `Option<U>` |
| `filter(\|x\| ...)` | 满足条件留 `Some`，否则 `None` | `Option<T>` |
| `or(other)` | 自己 `None` 时用 `other` | `Option<T>` |
| `ok_or(err)` | `Option` → `Result` | `Result<T, E>` |
| `is_some()` / `is_none()` | 判断 | `bool` |

```rust
let x: Option<i32> = Some(5);
let y: Option<i32> = None;

let a = x.unwrap();                  // 5
let b = y.unwrap_or(0);              // 0
let c = x.map(|n| n * 2);           // Some(10)
let d = y.map(|n| n * 2);           // None
let e = x.and_then(|n| if n > 0 { Some(n) } else { None });  // Some(5)
```

### 链式示例

```rust
let result = "  42  "
    .trim()                              // "42"
    .parse::<i32>()                      // Ok(42)  ← String parse 返回 Result
    .ok()                                // Some(42)
    .map(|n| n * 2)                      // Some(84)
    .unwrap_or(0);                       // 84
```

## 4.4 `Result<T, E>`：可能出错的运算

```rust
enum Result<T, E> {
    Ok(T),       // 成功，值是 T
    Err(E),      // 失败，错误是 E
}
```

```rust
use std::fs::File;
use std::io::Error;

fn open(path: &str) -> Result<File, Error> {
    File::open(path)
}
```

## 4.5 `Result` 常用方法

| 方法 | 行为 | 签名 |
|------|------|------|
| `unwrap()` / `expect(msg)` | 成功拿值，失败 panic | `T` |
| `unwrap_or(default)` | 失败给默认值 | `T` |
| `map(\|x\| ...)` | 改成功值 | `Result<U, E>` |
| `map_err(\|e\| ...)` | 改错误值 | `Result<T, F>` |
| `and_then(\|x\| ...)` | 链式（返回 `Result`） | `Result<U, E>` |
| `or_else(\|e\| ...)` | 失败时 fallback | `Result<T, F>` |
| `is_ok()` / `is_err()` | 判断 | `bool` |
| `ok()` | `Result` → `Option`（丢错误） | `Option<T>` |
| `err()` | `Result` → `Option`（丢值） | `Option<E>` |

```rust
let r: Result<i32, &str> = Ok(5);

let a = r.unwrap();                       // 5
let b = r.map(|n| n * 2);                // Ok(10)
let c = r.map_err(|e| format!("err: {e}"));
let d: Result<i32, &str> = Err("oops");
let e = d.unwrap_or(0);                  // 0
```

## 4.6 处理 `Result` 的几种风格

### A. `match`

```rust
match read_file("a.txt") {
    Ok(content)  => println!("{}", content),
    Err(e)       => println!("读取失败: {e}"),
}
```

### B. `if let`（只关心 Ok）

```rust
if let Ok(content) = read_file("a.txt") {
    println!("{}", content);
}
```

### C. `unwrap` / `expect`（开发期方便）

```rust
let content = read_file("a.txt").expect("a.txt 必须存在");
```

> ⚠️ **`unwrap` / `expect` 不应出现在产品代码里**——panic 会让进程崩溃。但**原型**、**测试**、**确定不会失败**的地方用一下没毛病。

### D. `?` 运算符（最常用，见下章）

## 4.7 `Option` 和 `Result` 互相转换

```rust
// Option → Result
let o: Option<i32> = Some(5);
let r: Result<i32, &str> = o.ok_or("没有值");

// Result → Option（丢弃错误信息）
let r: Result<i32, &str> = Ok(5);
let o: Option<i32> = r.ok();
```

## 4.8 `?` 运算符（先睹为快，下章细讲）

```rust
fn read_username(path: &str) -> Result<String, std::io::Error> {
    let mut s = String::new();
    std::fs::File::open(path)?.read_to_string(&mut s)?;
    Ok(s)
}
```

> `?` 是"成功就拆开拿值，失败就提前 `return Err(...)`"的语法糖。

## 4.9 一个"找用户"完整例子

```rust
#[derive(Debug)]
struct User {
    id: u32,
    name: String,
}

#[derive(Debug)]
enum FindError {
    NotFound,
    DatabaseDown,
}

fn find_user(id: u32) -> Result<User, FindError> {
    if id == 0 { return Err(FindError::NotFound); }
    if id > 1000 { return Err(FindError::DatabaseDown); }
    Ok(User { id, name: format!("user-{id}") })
}

fn greet(id: u32) -> String {
    match find_user(id) {
        Ok(u)  => format!("你好，{}！", u.name),
        Err(FindError::NotFound)    => "用户不存在".into(),
        Err(FindError::DatabaseDown) => "数据库连不上，稍后再试".into(),
    }
}

fn main() {
    println!("{}", greet(5));         // 你好，user-5！
    println!("{}", greet(0));         // 用户不存在
    println!("{}", greet(9999));      // 数据库连不上，稍后再试
}
```

> 用 enum 表达"具体几类错误"是 Rust 的惯用法。下一章 [05-error-handling.md](./05-error-handling.md) 会讲 `?` + `From` + `thiserror` 把这模式工程化。

## 4.10 自定义错误 vs `Box<dyn Error>`

**入门**：用 `Box<dyn std::error::Error>`——能装任何实现了 `Error` 的类型：

```rust
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("config.toml")?;
    let n: i32 = content.trim().parse()?;
    println!("n = {n}");
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("错误: {e}");
        std::process::exit(1);
    }
}
```

**进阶**：用 `thiserror` crate 写自定义错误（项目里会用）。

## 4.11 对比其他语言

| 概念 | Rust | C / Go | Java | Python |
|------|------|--------|------|--------|
| 表达"无" | `Option<T>` | `NULL` / `nil` / 哨兵 | `null`（任何引用） | `None` / `null` |
| 表达"可能错" | `Result<T, E>` | `error` 多返回值 / `if err != nil` | `try/catch` 异常 | `try/except` 异常 |
| 类型系统强制 | ✅ 编译期 | ❌ | ❌ | ❌ |
| 错误是值 | ✅ | ✅ | ❌（控制流） | ❌（控制流） |
| 不可忽略 | ✅（必须 match） | ❌ | ❌ | ❌ |

> 💡 **关键思想**：在 Java/Python 里你可以 `try { ... } catch (...) { /* 忘了 */ }` 把异常吞掉。在 Rust 里你**必须**用 `match` 处理 `Result`——`Err` 不会被偷偷丢掉。

## 4.12 一个对照练习

**Python 版本**：

```python
def get_age(user):
    if user is None:
        return None           # 可能返回 None
    return user.get("age")
```

调用方可以这么写：

```python
print(get_age(None) + 1)       # 💥 TypeError: unsupported operand
```

**Rust 版本**：

```rust
fn get_age(user: Option<&HashMap<String, i32>>) -> Option<i32> {
    user?.get("age").copied()
}

fn main() {
    let n = get_age(None);
    println!("{:?}", n);       // None，不会崩
    let n = get_age(None).unwrap_or(0);
    println!("{}", n);         // 0
    let n = get_age(None).unwrap();
    //                  ^^^^^^^ 💥 编译期就能看到 risk，但运行时 panic
}
```

> "panic 是另一种选择"——但**只在你写 `unwrap` 时才会**。**默认必须**显式处理 `None`。

---

## 🏋️ 本章小练习

**练习 4.1**：用 `Option` 写一个 `safe_div(a: i32, b: i32) -> Option<i32>`，除数为 0 返回 `None`，否则 `Some(a / b)`。

**练习 4.2**：写一个 `Vec<i32>::first` 的手动实现 `my_first(v: &[i32]) -> Option<&i32>`。

**练习 4.3**：用组合子实现：

```rust
let config: Option<&str> = Some("  42  ");
let n: Option<i32> = config
    .map(|s| s.trim())            // Option<&str>
    .and_then(|s| s.parse().ok()) // Option<i32>
    .map(|n| n * 2);              // Some(84)
assert_eq!(n, Some(84));
```

**练习 4.4**：`?` 运算符练习。写一个函数读 `Cargo.toml` 第一行：

```rust
use std::fs;
use std::io;

fn read_first_line(path: &str) -> io::Result<String> {
    let content = fs::read_to_string(path)?;   // 先这样，详看下章
    Ok(content.lines().next().unwrap_or("").to_string())
}
```

调用并处理 `Result`。**完整 `?` 教学在下一章**。

---

下一章：[05 · 错误处理 →](./05-error-handling.md)
