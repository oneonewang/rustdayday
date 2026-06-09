# 04 · 控制流

> **本章目标**：掌握 `if` 表达式和三种循环（`loop` / `while` / `for`），理解 Rust 没有 `?:` 三元、没有传统 C 风格 for。

## 4.1 `if` 表达式

```rust
fn main() {
    let n = 7;

    if n < 5 {
        println!("less than 5");
    } else if n == 5 {
        println!("exactly 5");
    } else {
        println!("greater than 5");
    }
}
```

**重要特性**：

1. 条件**不加括号**（Go 同款）。`if n < 5 {}` 而不是 `if (n < 5) {}`。
2. 条件必须是 **`bool`**，**不会**自动把整数当布尔（C / Python / JS 的坑）。`if 1 { ... }` 直接编译错。

### `if` 也是表达式（能返回值）

```rust
fn main() {
    let n = 7;
    let s = if n > 0 { "positive" } else { "non-positive" };
    println!("{s}");
}
```

> 💡 **没有三元 `?:`**。`if` 表达式就是它。好处：所有分支必须返回**同类型**——编译器会强制一致性。

```rust
let s = if n > 0 { "positive" } else { 1 };
// ❌ expected `&str`, found integer
```

## 4.2 `loop`：无限循环

```rust
let mut i = 0;
loop {
    println!("i = {i}");
    i += 1;
    if i == 3 { break; }
}
```

`loop` 可以**返回值**，常配合 `break val`：

```rust
fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;     // 直接 break 一个值
        }
    };
    println!("result = {result}");  // 20
}
```

### 循环标签（嵌套时指定 break 哪层）

```rust
fn main() {
    'outer: for i in 0..3 {
        'inner: for j in 0..3 {
            if i == 1 && j == 1 { break 'outer; }
            println!("({i},{j})");
        }
    }
}
```

> 标签语法：单引号开头 `'name:`。开始看着像生命周期（第 3 章会大量出现），但**循环标签**用在 `'label:` 上，**生命周期**用在 `'a` 上——位置不同。

## 4.3 `while`：条件循环

```rust
let mut n = 3;
while n != 0 {
    println!("{n}");
    n -= 1;
}
println!("LIFTOFF!");
```

> ⚠️ **没有 `do-while`**。如果一定要"先执行后判断"，用 `loop { ... if !cond { break; } }`。

## 4.4 `for`：遍历迭代器

这是 Rust 里**最常用**的循环，因为 Rust 集合都能迭代：

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    for elem in a {
        println!("{elem}");
    }
}
```

### `Range` 类型

```rust
// 0..3  -> 0, 1, 2 （不包含 3）
// 0..=3 -> 0, 1, 2, 3 （包含 3）
for n in 1..=5 {
    println!("{n}");  // 1 2 3 4 5
}

// 反向
for n in (1..=5).rev() {
    println!("{n}");  // 5 4 3 2 1
}
```

### 不需要索引（但有需要时用 `enumerate`）

```rust
let v = vec!["a", "b", "c"];

for (i, s) in v.iter().enumerate() {
    println!("{i}: {s}");
}
```

> ❌ **不要写 C 风格 `for (let i = 0; i < a.len(); i++)`**。Rust 鼓励直接遍历元素，避开索引就能避开一堆越界 bug。

## 4.5 一个"找第一个偶数"的例子

```rust
fn main() {
    let nums = [1, 3, 5, 7, 8, 9, 10];
    let first_even = nums.iter().find(|&&x| x % 2 == 0);
    match first_even {
        Some(&n) => println!("第一个偶数是 {n}"),
        None => println!("没有偶数"),
    }
}
```

> 看到 `match` / `Some` / `None` 不用慌——Stage 2 详讲。**本阶段只要能跑通，知道 `loop` / `while` / `for` 怎么用即可。**

## 4.6 对比其他语言

| 构造 | Rust | C / Java | Python | Go |
|------|------|----------|--------|-----|
| `if` 加括号 | ❌ | ✅ | ❌ | ❌ |
| 整数当布尔 | ❌ | ✅（C） | ✅ | ❌ |
| 三元 | ❌（用 `if` 表达式代替） | ✅ | ❌（用 `if`/`else` 表达式代替） | ❌ |
| 无限循环 | `loop {}` | `for(;;)` / `while(1)` | `while True:` | `for {}` |
| 范围循环 | `for i in 0..n` | `for(int i=0;i<n;i++)` | `for i in range(n):` | `for i := 0; i < n; i++` |
| 带标签 break | ✅ | ❌（C） | ❌ | ❌（C 风格 goto 替代） |

---

## 🏋️ 本章小练习

**练习 4.1**：打印 1 到 100，**3 的倍数打印 "Fizz"，5 的倍数打印 "Buzz"，15 的倍数打印 "FizzBuzz"，否则打印数字**。FizzBuzz 经典题。

**练习 4.2**：用 `loop` + `break` 写一个"猜数循环"：从 1 加到第一个大于 1000 的偶数，返回这个数和走了多少步。

**练习 4.3**：写一个 9x9 乘法表。提示：

```rust
for i in 1..=9 {
    for j in 1..=i {
        // 你的代码
    }
    println!();
}
```

---

下一章：[05 · 函数 →](./05-functions.md)
