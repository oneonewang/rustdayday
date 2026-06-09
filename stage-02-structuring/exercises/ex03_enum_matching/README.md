# Exercise 03 · enum 与模式匹配

> 难度：⭐⭐  涉及：第 3 章

## 任务

### 1) 定义 `IpAddr` enum

```rust
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
```

写方法 `display(&self) -> String`：
- `V4(192, 168, 1, 1)` → `"192.168.1.1"`
- `V6("::1")` → `"::1"`（直接打印字符串）

### 2) 定义 `Event` enum

```rust
enum Event {
    Click { x: i32, y: i32 },
    KeyPress(char),
    Resize { width: u32, height: u32 },
    Quit,
}
```

写 `handle(&self)`：
- `Click { x: 0, y: 0 }` → 打印 "原点点击"
- `Click { x, y }`（其他）→ 打印 "点击 (x, y)"
- `KeyPress('q')` → 打印 "退出（按 q）"
- `KeyPress(' ')` → 打印 "按了空格"
- `KeyPress(c)` → 打印 "按了 c"
- `Resize { width, height }` → 打印 "窗口缩放 widthxheight"
- `Quit` → 打印 "退出（事件）"

### 3) 写一个 `Expr` enum 的 evaluator

```rust
#[derive(Debug)]
enum Expr {
    Num(i32),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
}
```

实现 `evaluate(&self) -> i32`，例子：
- `Expr::Num(5)` → 5
- `Neg(Num(3))` → -3
- `Add(Num(1), Mul(Num(2), Num(3)))` → 7

### 4) `while let` 练习

写一个函数 `drain_print(v: &mut Vec<i32>)`，**用 `while let`** 把 `v` 一个个 pop 出来并打印。

```rust
let mut v = vec![1, 2, 3];
drain_print(&mut v);
// 打印 3, 2, 1
assert!(v.is_empty());
```

## 验收

写一个 `main` 跑所有任务，输出每一步的实际值。

## 提示

- 模式里直接写字面量：`Event::KeyPress('q') => ...`
- `Box<Expr>` 是 `Expr` 的智能指针，递归结构必须用指针包一层
- 在 `match` 里**用 `*box_expr`** 解引用（Stage 4 详讲 Box）

## 进阶

把 `Expr` 扩展支持 `Sub` 和 `Div`，并处理 `Div(_, Num(0))` 错误——返回 `Option<i32>`，除零返回 `None`。

完成 → [ex04_option_result](../ex04_option_result)
