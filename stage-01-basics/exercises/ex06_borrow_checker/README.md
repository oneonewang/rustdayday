# Exercise 06 · 借用检查器小测验

> 难度：⭐⭐  
> 涉及：第 7 章（借用 / `&mut` / NLL）

## 任务

下面每段代码**单独**放进 `main` 跑，每段都会编译错。**先预测、再改**：

### 段 1：可变借用的排他性

```rust
let mut s = String::from("hi");
let r1 = &s;
let r2 = &mut s;
println!("{r1} {r2}");
```

### 段 2：非 mut 不能改

```rust
let s = String::from("hi");
change(&s);

fn change(s: &String) {
    s.push_str("!");
}
```

### 段 3：可变引用存在时不能再有不可变

```rust
let mut s = String::from("hi");
let r1 = &mut s;
let r2 = &s;
println!("{r1} {r2}");
```

### 段 4：悬垂引用

```rust
fn dangle() -> &String {
    let s = String::from("d");
    &s
}
```

### 段 5：可变借用贯穿

```rust
let mut v = vec![1, 2, 3];
for x in &v {
    v.push(*x + 10);
}
```

## 怎么修

每一段的目标都是"**消除编译错**"。修好后思考：

- 为什么 Rust 这么严格？
- 不严格会出什么问题（C/C++ 里叫什么）？

## 验收

把 5 段代码**全部**修好，每段都能 `cargo run`。可以分 5 次跑（注释掉其他段），也可以一次性全放进 main 里。

## 关键规则速记

> 1. 同一时刻，**只能有一个** `&mut T`，或者**任意多个** `&T`，**不能并存**。
> 2. 引用不能比所有者活得久。
> 3. 可变借用期间，原变量不能再被借用（包括不可变借用）。

完成后进 [ex07_slices](../ex07_slices)
