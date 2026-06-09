# 01 · Struct：自定义数据类型

> **本章目标**：会用 struct 把相关数据组织起来，理解三种 struct 形态和字段初始化简写。

## 1.1 为什么需要 struct？

Stage 1 我们用过的"组合数据"——元组 `(String, i32, bool)`——能存数据，但**没有命名**，取字段要记索引：

```rust
let user: (String, i32, bool) = (String::from("alice"), 30, true);
println!("{} is {} years old", user.0, user.1);   // 0, 1 是什么？
```

struct 给字段**命名**：

```rust
struct User {
    name: String,
    age: u32,
    active: bool,
}

let u = User { name: String::from("alice"), age: 30, active: true };
println!("{} is {} years old", u.name, u.age);    // 自解释
```

## 1.2 三种 struct 形态

### A. 具名字段 struct（最常用）

```rust
struct Account {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

> ⚠️ 字段名是 `snake_case` 还是别的？**Rust 编译器只对 `snake_case` 函数 / 变量给警告，对 struct 字段没硬性要求**，但社区惯例是 `snake_case`。

### B. Tuple struct

字段没有名字、只有类型——适合"想有类型名 + 元组"：

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);

println!("black = ({}, {}, {})", black.0, black.1, black.2);
```

> 💡 **类型上的区别**：`Color` 和 `Point` 即使字段类型完全一样，**也是不同类型**——不能互相赋值。普通元组 `(i32, i32, i32)` 则和它们都不一样。

### C. Unit struct（空 struct）

```rust
struct AlwaysEqual;     // 没有任何字段

let subject = AlwaysEqual;
```

用途：给类型打 tag、用于 trait 约束（Stage 3 详解）。

## 1.3 struct 所有权约定

定义 struct 时，**优先用 `String` / `Vec<T>` 等拥有所有权的类型**，**避免**用 `&str` / `&[T]`（引用）。

原因：Stage 1 学过，引用需要生命周期参数；初学先全部"拥有"。

```rust
// 推荐
struct User {
    name: String,        // 拥有
    email: String,       // 拥有
}

// 等 Stage 3 学了生命周期再写：
// struct User<'a> {
//     name: &'a str,
//     email: &'a str,
// }
```

## 1.4 实例化与字段初始化简写

```rust
fn build_user(name: String, email: String) -> User {
    User {
        name,             // 字段初始化简写：name: name → name
        email,            // email: email → email
        active: true,
        sign_in_count: 1,
    }
}
```

当**参数名 = 字段名**时，可以省 `field: `。Stage 1 见过，但这里再强调，因为 struct 里很常用。

## 1.5 从其他实例更新：struct update 语法

```rust
let u1 = User { name: String::from("alice"), email: String::from("a@x"), active: true, sign_in_count: 1 };
let u2 = User { name: String::from("bob"), ..u1 };   // 其他字段都从 u1 拷贝
// u2.name = "bob", u2.email = u1.email 的 clone, u2.active = u1.active, ...
```

> ⚠️ `..u1` 转移了**非 Copy 字段的所有权**（这里 `email: String` 被 move 走）。`u1` 之后只剩 `active` 和 `sign_in_count` 可用，`email` 不能再访问。

## 1.6 调试输出：derive(Debug)

`println!("{:?}", u)` 编译错——struct 默认没实现 `Display` / `Debug`：

```rust
struct User { name: String, age: u32 }    // 直接 println! u 会编译错
```

两种解决：

### A. 临时加 `#[derive(Debug)]`

```rust
#[derive(Debug)]
struct User { name: String, age: u32 }

println!("{:?}", u);          // User { name: "alice", age: 30 }
println!("{:#?}", u);         // 多行格式
```

### B. 自己实现 `Display`

```rust
use std::fmt;

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.age)
    }
}

println!("{}", u);              // alice (30)
```

> `#[derive(Debug)]` 几行搞定；`impl Display` 写起来多。**调试阶段都用 `#[derive(Debug)]`**。

## 1.7 一些常见 derive

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point { x: i32, y: i32 }
```

| Trait | 作用 |
|-------|------|
| `Debug` | `{:?}` 打印 |
| `Clone` | `x.clone()` 深拷贝 |
| `PartialEq` / `Eq` | `==` 比较 |
| `Hash` | 能放进 `HashSet` / `HashMap` |
| `Default` | `T::default()` |
| `Copy` | 编译期位拷贝（**前提：所有字段都是 Copy**） |

## 1.8 完整示例

```rust
#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn area(&self) -> f64 {                  // 方法：第一个参数 &self
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: f64) -> Rectangle {     // 关联函数：没有 self，类似"静态方法"
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let r = Rectangle { width: 30.0, height: 50.0 };
    println!("面积 = {}", r.area());

    let s = Rectangle::square(20.0);
    println!("20x20 正方形 = {}x{}", s.width, s.height);

    let bigger = Rectangle { width: 40.0, height: 60.0 };
    println!("r 装得下 bigger 吗？{}", r.can_hold(&bigger));
}
```

> 方法 / 关联函数详细在 [02-methods.md](./02-methods.md)。

## 1.9 对比其他语言

| 概念 | Rust struct | C struct | Java class | Python class | Go struct |
|------|-------------|----------|------------|--------------|-----------|
| 字段命名 | ✅ | ✅ | ✅ | ✅ | ✅ |
| 默认可见性 | 模块私有 | public（无访问控制） | private | public | 包外不可见 |
| 继承 | ❌（用 trait） | ❌ | ✅ | ✅ | ❌ |
| 方法 | 通过 `impl` 块 | ❌ | ✅ | ✅ | 通过 receiver |
| 字段可变性 | 默认不可变，struct 本身 `let mut` 才可变 | 自由 | 默认 private，引用控制 | 自由 | 自由 |

> 💡 **核心区别**：Rust struct **不**带方法本体，方法必须通过 `impl` 块"附加"——这样数据和数据的行为可以分开组织。

## 1.10 一个常见坑

```rust
let r = Rectangle { width: 30.0, height: 50.0 };
r.width = 40.0;       // ❌ cannot assign to field
```

要改字段，整个 `r` 必须 `mut`：

```rust
let mut r = Rectangle { width: 30.0, height: 50.0 };
r.width = 40.0;       // ✅
```

---

## 🏋️ 本章小练习

**练习 1.1**：定义 `Book` struct（`title: String`, `author: String`, `pages: u32`, `price: f64`），实例化两本不同的书并打印。

**练习 1.2**：给 `Book` 加 `#[derive(Debug, Clone)]`，试着 `clone` 一本再改字段，看原始书是否受影响。

**练习 1.3**：写 `build_book(title, author) -> Book`，使用字段初始化简写（`title,` 而不是 `title: title,`），`pages` 默认 100，`price` 默认 0.0。

**练习 1.4**：tuple struct `Meters(f64)` 和 `Kilograms(f64)`，证明它们是不同类型：

```rust
let m = Meters(1.0);
let k = Kilograms(1.0);
// m = k;   // 应该是编译错
```

---

下一章：[02 · 方法与关联函数 →](./02-methods.md)
