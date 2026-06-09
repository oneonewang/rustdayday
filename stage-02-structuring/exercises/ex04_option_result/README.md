# Exercise 04 · Option 与 Result

> 难度：⭐⭐  涉及：第 4 章

## 任务

### 1) 安全除法

```rust
fn safe_div(a: i32, b: i32) -> Option<i32> {
    // b == 0 → None
}
```

### 2) 字符串解析（带错误）

```rust
fn parse_age(s: &str) -> Result<u32, String> {
    // 不能解析返回 Err("...")；解析成功但 < 0 或 > 150 返回 Err("年龄不合理")
    // 其他返回 Ok(n)
}
```

### 3) `first_or` 组合子练习

```rust
fn first_or(v: &[i32], default: i32) -> i32 {
    // 用 .first().copied().unwrap_or(default)
    // 不用 match
}
```

### 4) `?` 链

```rust
fn read_int_from_file(path: &str) -> Result<i32, std::io::Error> {
    // 用 std::fs::read_to_string + trim + parse + ?
    // 错误类型不匹配时用 .map_err？
}
```

### 5) 链式组合子

```rust
fn first_word_len(s: &str) -> usize {
    // 找第一个空白字符的位置（不是 byte，找 char 位置）
    // 找不到返回 0
    // 用组合子链，不用 match
}
```

## 验收

`main` 里跑：

```rust
assert_eq!(safe_div(10, 2), Some(5));
assert_eq!(safe_div(10, 0), None);

assert_eq!(parse_age("25"), Ok(25));
assert!(parse_age("abc").is_err());
assert!(parse_age("200").is_err());

assert_eq!(first_or(&[1, 2, 3], 99), 1);
assert_eq!(first_or(&[], 99), 99);

assert_eq!(first_word_len("hello world"), 5);
assert_eq!(first_word_len("noseparator"), 12);
```

## 提示

- `Result::map_err(|e| format!("...")` 可以改错误类型
- `?` 也能用在 `Option` 上
- 找 char 位置用 `s.find(char::is_whitespace)`
- `unwrap_or` 是 `Option` 的方法，`unwrap_or_default` 走 `T::default()`

## 进阶

写一个 `try_two<T, U, E, F: FnOnce() -> Result<U, E>>(a: Result<T, E>, f: F) -> Result<(T, U), E>`：先解 `a`，再调用 `f` 拿第二个，结果打包成元组返回。

完成 → [ex05_error_handling](../ex05_error_handling)
