# 03 · 闭包（Closures）

> **本章目标**：理解闭包语法、捕获机制、三种 trait（`Fn` / `FnMut` / `FnOnce`），会用闭包参数化行为。

## 3.1 闭包 = 可以捕获环境的匿名函数

```rust
let add_one = |x: i32| x + 1;
println!("{}", add_one(5));    // 6
```

> 💡 闭包和函数的最大区别：闭包**能访问定义它的作用域里的变量**。

```rust
let base = 10;
let add_base = |x: i32| x + base;    // 捕获 base
println!("{}", add_base(5));         // 15
```

## 3.2 闭包的类型推断

```rust
let add = |x, y| x + y;             // 类型由调用推断
let a = add(1, 2);                  // 推断为 i32
let b = add(1.0, 2.0);              // 推断为 f64
// 同一闭包不同类型使用——会编译错（一个闭包只有一个类型）
```

## 3.3 捕获的三种方式

```rust
let s = String::from("hi");

let consume = || s.into_bytes();    // 拿所有权（move）
// consume();                      // 之后再用 s 编译错
```

```rust
let mut s = String::from("hi");
let mut push = || s.push_str(" world");   // 可变借用
push();
push();
println!("{}", s);                 // "hi world"
```

```rust
let s = String::from("hi");
let read = || println!("{s}");     // 不可变借用
read(); read();
```

> **闭包"自动选择"最少侵入的捕获方式**。

## 3.4 三种 trait：`Fn` / `FnMut` / `FnOnce`

| Trait | 含义 | 调用次数 |
|-------|------|----------|
| `FnOnce` | 拿走捕获的变量 | **只能调一次** |
| `FnMut` | 可变借用捕获的变量 | **可调多次** |
| `Fn` | 不可变借用捕获的变量 | **可调多次 + 可与其他闭包共存** |

> **包含关系**：`Fn` ⊂ `FnMut` ⊂ `FnOnce`
> （任何 `Fn` 也是 `FnMut`；任何 `FnMut` 也是 `FnOnce`）

**举例**：

```rust
fn call_once<F: FnOnce()>(f: F) { f(); }       // 接受任意闭包
fn call_mut<F: FnMut()>(f: F) { f(); f(); }    // 接受 FnMut 或 Fn
fn call<F: Fn()>(f: F) { f(); f(); }           // 只接受 Fn
```

> 大多数"普通"闭包都是 `Fn`；要改捕获的变量是 `FnMut`；要消费的是 `FnOnce`。

## 3.5 强制 move：`move` 关键字

```rust
let s = String::from("hi");
let consume = move || println!("{s}");     // 强制把 s 移进闭包
// println!("{s}");                       // ❌ s 已 move
```

**典型用途**：闭包**活得比**当前作用域久（异步、跨线程）。

```rust
use std::thread;

let v = vec![1, 2, 3];
let handle = thread::spawn(move || {
    println!("{:?}", v);    // v 移进新线程
});
handle.join().unwrap();
```

## 3.6 闭包作为参数

```rust
fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(x)
}

let result = apply(|n| n * 2, 5);   // 10
let result = apply(|n| n + 1, 5);   // 6
```

## 3.7 闭包作为返回值：`impl Fn` / `impl FnMut`

```rust
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n                    // 必须 move，否则 n 活得不够久
}

let add5 = make_adder(5);
println!("{}", add5(10));            // 15
```

> **不能**直接 `Box<dyn Fn>` 装在静态返回里（要 dyn 转换），更简单的就是 `impl Fn`。

## 3.8 `Fn` 系列 trait 与 `Iterator` 组合

```rust
let v = vec![1, 2, 3, 4, 5];
let sum: i32 = v.iter().sum();
let evens: Vec<&i32> = v.iter().filter(|&&x| x % 2 == 0).collect();
let doubled: Vec<i32> = v.iter().map(|&x| x * 2).collect();
let any_gt_3 = v.iter().any(|&x| x > 3);
```

> 迭代器方法是 **`FnMut`** 闭包——可调用多次。

## 3.9 闭包 + 泛型约束

```rust
fn call_on_each<T, F>(items: Vec<T>, mut f: F)
where
    F: FnMut(&T),
{
    for item in &items { f(item); }
}

call_on_each(vec![1, 2, 3], |n| println!("{n}"));
call_on_each(vec!["a", "b"], |s| println!("{}", s.len()));
```

## 3.10 闭包与所有权

```rust
let mut v = vec![1, 2, 3];

// FnMut：可改 v
let mut add_one = || v.push(4);
add_one();
add_one();

// 闭包借用 v 期间，外部不能用
// println!("{:?}", v);     // ❌ borrow check
add_one();                    // 调一次后 v 的借用结束
println!("{:?}", v);          // ✅
```

## 3.11 真实场景：自定义比较器

```rust
fn largest<T: PartialOrd + Copy>(v: &[T], cmp: impl Fn(T, T) -> std::cmp::Ordering) -> Option<T> {
    if v.is_empty() { return None; }
    let mut best = v[0];
    for &x in &v[1..] {
        if cmp(x, best) == std::cmp::Ordering::Greater {
            best = x;
        }
    }
    Some(best)
}

fn main() {
    let nums = [1, 5, 3, 9, 2];
    let l1 = largest(&nums, |a, b| a.cmp(&b));           // 最大
    let l2 = largest(&nums, |a, b| b.cmp(&a));           // 最小
    println!("max = {:?}, min = {:?}", l1, l2);
}
```

## 3.12 闭包与线程

```rust
use std::thread;

let data = vec![1, 2, 3];

// 单线程模拟
let handle = thread::spawn(move || {
    let sum: i32 = data.iter().sum();
    sum
});

println!("sum = {}", handle.join().unwrap());
```

> 闭包要能 `Send` 才能跨线程——大多数情况编译器自动推断。

## 3.13 闭包与 `Iterator`：自定义 collect

```rust
let v = vec![1, 2, 3];

let s: i32 = v.iter().fold(0, |acc, x| acc + x);   // 6

let pairs: Vec<(i32, i32)> = v.iter().enumerate()
    .map(|(i, x)| (*x, *x * 2))
    .collect();
```

## 3.14 闭包 trait 速查

```rust
fn consume<F: FnOnce() -> String>(f: F) { let _ = f(); }     // 任意闭包
fn mutate<F: FnMut() -> ()>(mut f: F) { f(); f(); }          // 任意可改闭包
fn read_only<F: Fn() -> ()>(f: F) { f(); f(); f(); }         // 只读闭包
```

| 调用方式 | 实现哪个 trait |
|----------|----------------|
| `f()` | `FnOnce` |
| `f()`（要改内部状态） | `FnMut` |
| `f()`（多次 + 共享） | `Fn` |
| `f()` 拿返回值 | `FnOnce` / `FnMut` / `Fn` 都行 |

## 3.15 对比其他语言

| 概念 | Rust | Python | JavaScript | C++ |
|------|------|--------|-----------|-----|
| 闭包语法 | `\|x\| x + 1` | `lambda x: x+1` | `x => x+1` | `[\&x]{return x+1;}` |
| 捕获环境 | ✅ | ✅ | ✅ | ✅ |
| 类型标注 | 可推断 | 动态 | 动态 | 模板推断 |
| 三个 trait | Fn / FnMut / FnOnce | — | — | function / mutable_function / once_function |
| move 闭包 | `move \|\| ...` | — | — | — |
| 闭包当参数 | `impl Fn` / `FnMut` | 高阶函数 | 高阶函数 | `std::function` |

## 3.16 常见坑

```rust
// ❌ 闭包借用 v 期间，外部不能再借
let mut v = vec![1, 2, 3];
let push_one = || v.push(4);
v.push(5);           // ❌ cannot borrow as mutable
push_one();

// ✅ 把外部的 push 放在闭包调用前 / 调用后
v.push(5);
let push_one = || v.push(4);
push_one();
```

---

## 🏋️ 本章小练习

**练习 3.1**：写 `fn apply_twice<F: Fn(i32) -> i32>(f: F, x: i32) -> i32`，返回 `f(f(x))`。

**练习 3.2**：写 `fn make_adder(n: i32) -> impl Fn(i32) -> i32`。

**练习 3.3**：写 `fn filter_positive(v: Vec<i32>) -> Vec<i32>`，**用闭包 + 迭代器**实现，**不要**用显式 `for` 循环。

**练习 3.4**：写一个 `Closure as 参数` 的排序函数：

```rust
fn sort_by<T, F: FnMut(&T, &T) -> std::cmp::Ordering>(v: &mut [T], cmp: F)
```

**练习 3.5**（move 关键字）：写一个多线程 worker：

```rust
let data = vec![1, 2, 3, 4, 5];
let handle = std::thread::spawn(move || data.iter().sum::<i32>());
// 注释掉 move，编译错？看看报错说啥
```

---

下一章：[04 · 迭代器深入 →](./04-iterators.md)
