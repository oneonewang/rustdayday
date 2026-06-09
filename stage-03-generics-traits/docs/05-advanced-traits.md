# 05 · 高级 Trait

> **本章目标**：会用关联类型、运算符重载、supertrait、newtype 模式，理解"孤儿规则"。

## 5.1 关联类型（Associated Types）

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

> `Self::Item` 是**这个 trait 实现者**指定的——**每个类型只能有一种 `Item`**。

```rust
struct Counter { count: u32 }

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
```

### 关联类型 vs 泛型

| | 关联类型 | 泛型参数 |
|---|---------|----------|
| 实现 trait 时 | **每种类型只能选一次** Item | 可以 `impl<T>` 让 T 任意 |
| 用法 | `Iterator<Item = i32>` | `IntoIterator<i32>` |
| 适用 | "**一种**实现" | "**多种**实现" |

```rust
// 关联类型：Counter 只能是某一种 Item
impl Iterator for Counter { type Item = u32; ... }

// 泛型：String 既是 IntoIterator<Item=char>，又是 IntoIterator<Item=&str>...
impl IntoIterator for String { type Item = char; ... }   // 不允许多次
```

> 关联类型更"硬"，所以**优先用关联类型**——除非确实需要"同一类型多次实现"。

## 5.2 默认泛型参数

```rust
trait Add<Rhs = Self> {                  // 默认是 Self
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
}
```

```rust
impl Add for Point { ... }              // 隐式 Rhs = Self
impl Add<(i32, i32)> for Point { ... }  // Point + (i32, i32)
```

## 5.3 运算符重载

`+` / `-` / `*` / `/` / `==` / `<` / `>` / `Display` 全部是 trait：

```rust
use std::ops::Add;

#[derive(Debug, Clone, Copy)]
struct Point { x: i32, y: i32 }

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    println!("{:?}", p1 + p2);   // Point { x: 4, y: 6 }
}
```

可重载的运算符（节选）：

| 运算符 | Trait |
|--------|-------|
| `+` | `std::ops::Add` |
| `-` | `std::ops::Sub` |
| `*` | `std::ops::Mul` |
| `/` | `std::ops::Div` |
| `%` | `std::ops::Rem` |
| `==` / `!=` | `PartialEq` |
| `<` / `>` | `PartialOrd` |
| `<<` / `>>` | `Shl` / `Shr` |
| `&` / `|` / `^` | `BitAnd` / `BitOr` / `BitXor` |
| `[]` | `Index` / `IndexMut` |
| `*x`（解引用） | `Deref` / `DerefMut` |

## 5.4 Supertrait（超 trait）

```rust
trait Person {
    fn name(&self) -> &str;
}

trait Student: Person {                // Student 必须也是 Person
    fn university(&self) -> &str;
}

struct Alice;
impl Person for Alice { fn name(&self) -> &str { "Alice" } }
impl Student for Alice { fn university(&self) -> &str { "MIT" } }
```

> 任何 `Student` 必然也是 `Person`——可以放心用 `s.name()`。

**多个 supertrait**：

```rust
trait Graduate: Person + Researcher { ... }
```

## 5.5 newtype 模式：绕过孤儿规则

**孤儿规则**：你只能为**你的**类型 / **你的** trait 实现 trait。**不能**给"别人的类型实现别人的 trait"。

```rust
impl Display for Vec<i32> { ... }      // ❌ Vec 不是我的，Display 也不是我的
```

**解法**：包一层 newtype。

```rust
struct MyVec(Vec<i32>);

impl std::fmt::Display for MyVec {     // ✅ MyVec 是我的
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}]", self.0.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", "))
    }
}
```

> 💡 **newtype 模式** = 在**不引入运行时开销**的前提下，给外部类型附加 trait 实现。

**自动解包**（用 `Deref`）：

```rust
impl std::ops::Deref for MyVec {
    type Target = Vec<i32>;
    fn deref(&self) -> &Vec<i32> { &self.0 }
}

let v = MyVec(vec![1, 2, 3]);
println!("{}", v.len());   // MyVec.len() 自动找 Vec<i32> 的方法
```

## 5.6 完全限定语法

如果方法名有歧义，用**完全限定**调用：

```rust
trait A { fn hello(&self) { println!("A"); } }
trait B { fn hello(&self) { println!("B"); } }

struct S;
impl A for S {}
impl B for S {}

let s = S;
A::hello(&s);      // 输出 A
B::hello(&s);      // 输出 B
<S as A>::hello(&s);   // 完全限定
```

> 平时不需要用，**报错信息提示"用完全限定语法"时**再用。

## 5.7 一个真实例子：`From` / `Into`

```rust
struct Celsius(f64);
struct Fahrenheit(f64);

impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self {
        Fahrenheit(c.0 * 9.0 / 5.0 + 32.0)
    }
}

fn main() {
    let c = Celsius(100.0);
    let f: Fahrenheit = c.into();   // 自动
    let f2 = Fahrenheit::from(Celsius(0.0));   // 显式
}
```

`From<T>` 自动给出 `Into<T>` 的实现——实现一个就够了。

## 5.8 完整例子：自定义迭代器

```rust
struct Counter { count: u32, max: u32 }

impl Counter {
    fn new(max: u32) -> Self { Self { count: 0, max } }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let sum: u32 = Counter::new(5).sum();
    println!("sum = {sum}");   // 1+2+3+4+5 = 15
}
```

> 实现了 `Iterator` 后，**所有迭代器方法**（map / filter / sum / collect / take）都自动可用——这就是 trait 的复利价值。

## 5.9 完全限定语法的另一种用法：关联函数

```rust
trait A { fn create() -> Self; }
trait B { fn create() -> Self; }

struct S;
impl A for S { fn create() -> Self { S } }
impl B for S { fn create() -> Self { S } }

let s = <S as A>::create();   // 调 A 的 create
```

## 5.10 一个常见反模式：滥用 trait 抽象

```rust
// ❌ 没必要
trait Addable { fn add(&self, other: &Self) -> Self; }
impl Addable for i32 { ... }

// ✅ 用标准库
fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T { a + b }
```

> 写 trait 之前先查标准库——大概率已经有了。

## 5.11 对比其他语言

| 概念 | Rust | C++ | Swift | Scala |
|------|------|-----|-------|-------|
| 关联类型 | ✅ | ❌（用模板特化） | ✅ associated type | ✅ |
| 运算符重载 | ✅（实现 trait） | ✅（operator()） | ✅ | ✅ |
| Supertrait | ✅ | ✅（多继承） | ✅ | ✅ |
| 孤儿规则 | ✅ | ❌ | ❌ | ❌ |
| newtype 模式 | 必要（绕孤儿） | 用 `using` | 用 typealias | case class |

---

## 🏋️ 本章小练习

**练习 5.1**：定义 `trait Addable`（关联类型 `Output`、方法 `add`），为 `Point { x: i32, y: i32 }` 实现。`let p3 = p1.add(p2)` 能用吗？

**练习 5.2**：实现一个 `Meters` newtype，为它实现 `Add<Meters, Output = Meters>` 和 `Display`。

**练习 5.3**：supertrait 练习：

```rust
trait Animal { fn name(&self) -> &str; }
trait Pet: Animal { fn owner(&self) -> &str; }
```

实现 `Dog`，写 `fn greet<T: Pet>(p: &T)` 调 `p.name()` 和 `p.owner()`。

**练习 5.4**：用关联类型实现一个"奇偶迭代器"：

```rust
struct Alternating { curr: bool }
impl Iterator for Alternating {
    type Item = i32;
    fn next(&mut self) -> Option<i32> { ... }
}
// 调用 take(4) → [1, 0, 2, 0] 或 [0, 1, 0, 2]
```

**练习 5.5**（真实场景）：定义 `UserId(u32)`，为它实现 `Display` / `From<u32>` / `FromStr`。再用 newtype 包装 `Vec<UserId>` 并实现自定义的 `Display`。

---

下一章：[06 · 阶段复习 →](./06-stage-review.md)
