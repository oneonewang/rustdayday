# Exercise 05 · 所有权小测验（修编译错）

> 难度：⭐⭐  
> 涉及：第 6 章（所有权）

## 任务

下面的 `main` 有 **6 处**编译错。**先想、再改、最后看答案**。

```rust
fn main() {
    let s = String::from("hello");
    let s2 = s;
    println!("{s}");

    let n = 5;
    let m = n;
    println!("n = {n}, m = {m}");

    let s3 = String::from("world");
    takes(s3);
    println!("after: {s3}");

    let mut s4 = String::from("a");
    s4.push_str("b");
    println!("{s4}");

    let s5 = String::from("c");
    let s6 = s5.clone();
    println!("{s5} {s6}");

    let s7 = String::from("d");
    let s8 = s7;
    let s9 = s8;
    println!("{s8} {s9}");
}

fn takes(s: String) {
    println!("took: {s}");
}
```

## 思考流程

每一处先回答：
1. 报什么错？（copy / move / borrow / mut）
2. 怎么修（`clone` / `&` / 加 `mut` / 重新组织代码）？

## 验收

```bash
cargo run
```

最终能编译并输出大致如下（顺序可能略不同）：

```
hello
n = 5, m = 5
took: world
ab
c world
d d
```

## 提示

- "borrow of moved value" → 旧变量已被转移
- 整数是 `Copy` 类型，赋值不消耗原变量
- 进函数默认 move；要么 `clone`、要么传 `&s`（但本练习目的是体会 move 错误）

## 自我核对

| # | 错误 | 修法 |
|---|------|------|
| 1 | `borrow of moved value: s` | `clone` / 或直接用 `s2` |
| 2 | 这行没问题 | — |
| 3 | `borrow of moved value: s3` | `&s3` 传引用，或 `clone` |
| 4 | `cannot mutate` | 已经是 `mut`，问题在别处？仔细看 |
| 5 | 这行没问题 | — |
| 6 | `borrow of moved value: s8` | 改成用 `s9` 打印，或 `clone` |

完成且能 `cargo run` 通过后进 [ex06_borrow_checker](../ex06_borrow_checker)
