# Exercise 03 · 温度转换

> 难度：⭐  
> 涉及：第 3、5 章（浮点 / 函数返回值 / 表达式）

## 任务

实现两个温度转换函数：

```rust
fn celsius_to_fahrenheit(c: f64) -> f64    // F = C * 9.0 / 5.0 + 32.0
fn fahrenheit_to_celsius(f: f64) -> f64    // C = (F - 32.0) * 5.0 / 9.0
```

## 测试用例

| 输入 | 函数 | 期望输出 |
|------|------|----------|
| `0.0` | `celsius_to_fahrenheit` | `32.0` |
| `100.0` | `celsius_to_fahrenheit` | `212.0` |
| `32.0` | `fahrenheit_to_celsius` | `0.0` |
| `-40.0` | `fahrenheit_to_celsius` | `-40.0`（华氏/摄氏在 -40 相同） |

## 验收

写一个 `main` 把上面 4 个用例都跑一遍，输出形如：

```
0.0°C = 32.0°F
100.0°C = 212.0°F
32.0°F = 0.0°C
-40.0°F = -40.0°C
```

## 提示

- 注意 `9 / 5` 在 Rust 里是**整数除法**得 `1`，要用 `9.0 / 5.0`
- 函数体最后一行**没有分号**，因为它是表达式
- `println!("{c}°C = {f}°F", c = 0.0, f = celsius_to_fahrenheit(0.0));` 是命名参数写法

## 进阶（可选）

写一个 `format_temp(c: f64) -> String`，返回 `"0.0°C = 32.0°F"` 这样的字符串，`main` 里直接打印 `format_temp(x)`。

完成后进 [ex04_fizzbuzz](../ex04_fizzbuzz)
