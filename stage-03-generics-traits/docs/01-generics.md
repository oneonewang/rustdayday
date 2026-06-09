# 01 · 泛型（Generics）

> **本章目标**：会写"参数化类型 / 函数"的代码，理解泛型 vs trait 的区别。

## 1.1 为什么需要泛型？

```rust
// 没有泛型，每种类型都写一遍
fn largest_i32(v: &[i32]) -> i32 { ... }
fn largest_f64(v: &[f64]) -> f64 { ... }
fn largest_char(v: &[char]) -> char { ... }
```

用泛型：

```rust
fn largest<T: PartialOrd + Copy>(v: &[T]) -> T {
    let mut largest = v[0];
    for &item in v {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

> 💡 **泛型 vs trait**：
> - **泛型** `<T>`：编译时**单态化**（monomorphization）——每种具体类型生成一份代码
> - **trait object** `dyn Trait`：**运行时**动态分发——一份代码处理多种类型（下一章详讲）

## 1.2 泛型函数

```rust
fn identity<T>(x: T) -> T { x }

let a = identity(5);          // a: i32
let b = identity("hi");       // b: &str
let c: f64 = identity(3.14);  // 显式标注
```

**多个泛型参数**：

```rust
fn pair<T, U>(a: T, b: U) -> (T, U) { (a, b) }
```

## 1.3 泛型 struct

```rust
struct Point<T> {
    x: T,
    y: T,
}

let p1 = Point { x: 1, y: 2 };            // Point<i32>
let p2 = Point { x: 1.0, y: 2.0 };        // Point<f64>
```

**多个泛型参数**：

```rust
struct Pair<T, U> {
    first: T,
    second: U,
}

let p = Pair { first: 1, second: "hello" };   // Pair<i32, &str>
```

**泛型方法**：在 `impl` 后面写 `<T>`：

```rust
impl<T> Point<T> {
    fn x(&self) -> &T { &self.x }
}

// 只为特定类型实现
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}
```

**方法自己的泛型**（与结构体泛型无关）：

```rust
impl<T> Point<T> {
    fn mix<U>(self, other: Point<U>) -> Pair<T, U> {
        Pair { first: self.x, second: other.x }
    }
}
```

## 1.4 泛型 enum

Stage 2 见过的 `Option<T>` / `Result<T, E>` 就是泛型 enum：

```rust
enum Option<T> { None, Some(T) }
enum Result<T, E> { Ok(T), Err(E) }
```

自己写一个：

```rust
#[derive(Debug)]
enum Either<L, R> {
    Left(L),
    Right(R),
}

let a: Either<i32, &str> = Either::Left(42);
let b: Either<i32, &str> = Either::Right("error");
```

## 1.5 单态化（Monomorphization）

> 泛型**不是**运行时的"动态类型"。编译器**会**为每个用到的具体类型生成一份专门的代码。

```rust
let p1 = Point { x: 1, y: 2 };      // 生成 Point<i32>
let p2 = Point { x: 1.0, y: 2.0 };  // 生成 Point<f64>
```

**好处**：运行时**零开销**（和手写两份代码完全一样）。  
**代价**：二进制变大（每种类型一份代码）。

> 💡 跟 Java / C# 的"类型擦除" / "装箱"完全相反——Rust 选了"代码膨胀"换"零运行时开销"。

## 1.6 泛型的性能

| 维度 | Rust 泛型 | Java 泛型（C# 类似） | C++ 模板 |
|------|-----------|---------------------|----------|
| 编译产物 | 单态化（多份代码） | 类型擦除（一份代码 + 装箱） | 单态化（多份代码） |
| 运行时开销 | 零 | 装箱拆箱开销 | 零 |
| 二进制大小 | 较大 | 较小 | 较大 |
| 编译时间 | 较长（每种类型一次） | 较短 | 很长 |
| 错误信息 | 友好（指明具体类型） | 难读（`Object`） | 难读（十几屏） |

## 1.7 什么时候用泛型？

- **同一个逻辑对多种类型都成立**（如 `largest` 对 `i32` / `f64` / `String` 都能用）
- **数据结构**：`Vec<T>` / `HashMap<K, V>` / `Result<T, E>` / 自定义容器
- **避免代码重复**

什么时候**不**用？

- 只有一种类型——直接写具体类型
- 想要"运行时不固定类型"——用 trait object（下一章）

## 1.8 完整例子：泛型 Stack

```rust
#[derive(Debug)]
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self { Self { items: Vec::new() } }
    fn push(&mut self, item: T) { self.items.push(item); }
    fn pop(&mut self) -> Option<T> { self.items.pop() }
    fn peek(&self) -> Option<&T> { self.items.last() }
    fn len(&self) -> usize { self.items.len() }
    fn is_empty(&self) -> bool { self.items.is_empty() }
}

fn main() {
    let mut s: Stack<i32> = Stack::new();
    s.push(1);
    s.push(2);
    s.push(3);
    println!("{:?}", s);       // Stack { items: [1, 2, 3] }
    println!("top = {:?}", s.peek());   // Some(3)
    println!("pop = {:?}", s.pop());    // Some(3)
}
```

## 1.9 对比其他语言

| 概念 | Rust | C++ | Java | TypeScript | Go 1.18+ |
|------|------|-----|------|------------|----------|
| 泛型 | `<T>` | `template <typename T>` | `<T>`（擦除） | `<T>`（擦除） | `[T any]` |
| 约束 | `T: Trait` | `requires` 概念（20 才有） | `<T extends X>` | `extends` | 无（用 interface） |
| 单态化 | ✅ | ✅ | ❌ | ❌ | ✅ |
| 类型擦除 | ❌ | ❌ | ✅ | ✅ | ❌ |

## 1.10 一个常见坑

```rust
fn largest<T>(v: &[T]) -> T {
    let mut largest = v[0];        // ❌ T 不一定 Copy
    for &item in v { ... }
    largest
}
```

要"能拷贝、比较大"必须加约束：

```rust
fn largest<T: PartialOrd + Copy>(v: &[T]) -> T { ... }
```

> 下一章 [02-traits.md](./02-traits.md) 详细讲 `T: Trait` 怎么写。

---

## 🏋️ 本章小练习

**练习 1.1**：写一个 `swap<T>(pair: (T, T)) -> (T, T)`，把元组两个元素交换。

**练习 1.2**：写一个 `Vec<T>` 的 wrapper struct `MyVec<T>`，实现 `new` / `push` / `len` / `last` / `is_empty`。再为 `MyVec<i32>` 加一个特化方法 `sum`（仅 i32 能用）。

**练习 1.3**：写一个泛型 `Pair<T>`，实现：

```rust
impl<T: PartialOrd> Pair<T> {
    fn cmp_display(&self) -> &T       // 返回较大的那个的引用
}
```

**练习 1.4**（思考）：为什么 `Option<T>` 必须是泛型？写一个**非泛型**的 `MyOption`，然后**用**它——感受一下泛型带来的便利。

---

下一章：[02 · Trait →](./02-traits.md)
