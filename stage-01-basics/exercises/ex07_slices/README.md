# Exercise 07 · 切片练习

> 难度：⭐⭐  
> 涉及：第 8 章（slice / `&str` / `&[T]`）

## 任务

### 1) 第一个单词

```rust
fn first_word(s: &str) -> &str {
    // 返回第一个空格之前的内容；如果没有空格，返回整个 s
}
```

测试：
- `first_word("hello world")` → `"hello"`
- `first_word("hello")` → `"hello"`
- `first_word(" a")` → `""`
- `first_word("")` → `""`

### 2) 修剪首尾空格（**用切片**，不分配新内存）

```rust
fn trim_spaces(s: &str) -> &str {
    // 去掉首尾的 ASCII 空格
}
```

测试：
- `trim_spaces("  hello  ")` → `"hello"`
- `trim_spaces("no_spaces")` → `"no_spaces"`
- `trim_spaces("   ")` → `""`
- `trim_spaces("")` → `""`

### 3) 数组切片求和

```rust
fn sum(slice: &[i32]) -> i32 {
    // slice.iter().sum()
}
```

测试：
- `sum(&[1, 2, 3, 4])` → `10`
- `sum(&[])` → `0`

### 4) 数组切片的窗口（**进阶**）

```rust
fn windows_of_3(arr: &[i32]) -> Vec<&[i32]> {
    // 返回所有长度为 3 的连续子切片
    // 例如 [1,2,3,4,5] -> [[1,2,3], [2,3,4], [3,4,5]]
}
```

**不要**用外部 crate 里的 `windows()`——自己写一个 `for` 循环。

## 验收

写一个 `main`，把上面 4 个函数的所有测试用例都跑一遍，输出形如：

```
first_word("hello world") = "hello"
first_word("hello") = "hello"
first_word(" a") = ""
first_word("") = ""

trim_spaces("  hello  ") = "hello"
trim_spaces("no_spaces") = "no_spaces"
trim_spaces("   ") = ""
trim_spaces("") = ""

sum(&[1, 2, 3, 4]) = 10
sum(&[]) = 0

windows_of_3(&[1, 2, 3, 4, 5]) = [[1, 2, 3], [2, 3, 4], [3, 4, 5]]
windows_of_3(&[1, 2]) = []
```

## 提示

- 字符串字面量 `"..."` 本身就是 `&str`，可以直接当参数传
- `s.bytes().enumerate()` 可以按字节遍历
- 比较字节和 ASCII 空格：`c == b' '`
- 返回切片的子切片：直接 `&s[start..end]`

## 思考题

为什么 `first_word` 用 `&str` 作参数比 `&String` 好？

> 答：`&str` 可以接 `&String`（自动 deref）、`&"literal"`、和别的 `&str`。**一个签名，三种调用方式**。

## 进阶（可选）

- `trim_spaces` 改成泛型版本（暂时跳过，本练习要求 `&str`）
- 写 `is_palindrome(s: &str) -> bool`，判断字符串是否为回文（`"abba"`、`"上海自来水来自海上"`）

完成 4 个函数后，[project-01-guess-game](../project-01-guess-game) 见！
