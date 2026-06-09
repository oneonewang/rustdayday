# 02 · Trait：定义行为契约

> **本章目标**：会定义 trait、为类型实现 trait、用 trait bound 约束泛型、写出可复用的多态代码。

## 2.1 Trait 是什么？

**Trait** = 一组方法的集合 + 类型**必须**实现的契约。

> 类比：
> - Java/C#：**interface**（不能有默认实现）
> - C++：**abstract class**（可以有默认）
> - Haskell：**typeclass**（最接近 Rust trait）
> - Go：**interface**（隐式实现）
> - Swift：**protocol**（最接近，可以有默认实现）

```rust
trait Summary {
    fn summarize(&self) -> String;

    // 默认实现（实现者可以 override）
    fn default_summarize(&self) -> String {
        String::from("(read more...)")
    }
}
```

> ⚠️ **Rust 没有"继承"**。trait 是"行为复用"的唯一方式。

## 2.2 为类型实现 trait

```rust
trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    title: String,
    author: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}（by {}）", self.title, self.author)
    }
}
```

**内置 trait 的实现**：

```rust
use std::fmt;

struct Point { x: i32, y: i32 }

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
```

## 2.3 默认实现

```rust
trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("（更多来自 {}）", self.summarize_author())    // 调其他方法
    }
}

impl Summary for Article {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }
    // summarize() 用默认实现
}
```

> 💡 **默认实现里可以调 trait 里没实现的方法**——实现者必须实现这些"被依赖"的方法。

## 2.4 Trait bound：约束泛型"能做什么"

```rust
fn notify<T: Summary>(item: &T) {                // T 必须实现 Summary
    println!("速报！{}", item.summarize());
}
```

**多个约束**：

```rust
fn notify<T: Summary + Display>(item: &T) { ... }
```

**更复杂的约束**——用 `where`：

```rust
fn complex<T, U>(t: &T, u: &U) -> i32
where
    T: Summary + Clone,
    U: Clone + Debug,
{ ... }
```

> `where` 子句在签名很复杂时更清晰。

## 2.5 用 trait 让函数返回"实现了某 trait 的类型"

```rust
fn make_summarizable() -> impl Summary {        // 返回"某个实现了 Summary 的类型"
    Article {
        title: "标题".to_string(),
        author: "我".to_string(),
        content: "...".to_string(),
    }
}
```

> `impl Trait` 是**静态分发**——调用方不能动态切换类型。需要动态用 `dyn Trait`（下一章）。

## 2.6 用 trait bound 条件式实现方法

```rust
impl<T: Display> Point<T> {           // 只为 T: Display 的 Point 实现方法
    fn show(&self) {
        println!("({})", self.x);
    }
}
```

## 2.7 常用标准库 trait（先认识，后面遇到再细看）

| Trait | 作用 | 常见方法 |
|-------|------|----------|
| `Display` | 用户友好的 `{}` 打印 | `fmt()` |
| `Debug` | 调试用的 `{:?}` 打印 | `fmt()` |
| `Clone` | `.clone()` 深拷贝 | `clone()` |
| `Copy` | 隐式位拷贝 | （标记 trait） |
| `PartialEq` / `Eq` | `==` / `!=` | `eq()` |
| `PartialOrd` / `Ord` | `<` / `>` / `sort` | `cmp()` |
| `Hash` | 能放进 `HashSet` / `HashMap` | `hash()` |
| `Default` | `T::default()` | `default()` |
| `Iterator` | 迭代器 | `next()` |
| `From<T>` / `Into<T>` | 类型转换 | `from()` / `into()` |
| `FromStr` | 字符串解析 | `from_str()` |
| `Send` / `Sync` | 线程安全 | （标记 trait） |
| `Sized` | 大小编译期已知 | （自动） |

> **标记 trait**（marker trait）：没有方法，只标记"类型具备某性质"。例如 `Copy` / `Send` / `Sync`。

## 2.8 `derive` 自动实现

很多常用 trait 可以让编译器自动派生：

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct Point { x: i32, y: i32 }
```

可 derive 的 trait（部分列表）：
- `Debug` `Clone` `Copy` `PartialEq` `Eq` `PartialOrd` `Ord` `Hash` `Default`

> `Display` / `From` / `Iterator` / `Summary`（自定义的）**不能** derive，必须手写。

## 2.9 trait 作为参数（多态）

```rust
fn print_summary(item: &impl Summary) {       // 简写 1
    println!("{}", item.summarize());
}

fn print_summary2<T: Summary>(item: &T) {     // 完整写法
    println!("{}", item.summarize());
}

fn print_two<T: Summary, U: Summary>(a: &T, b: &U) {  // 两个不同类型
    println!("{} 和 {}", a.summarize(), b.summarize());
}
```

## 2.10 trait 完整例子：图形

```rust
trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn describe(&self) -> String {
        format!("面积 {:.2}, 周长 {:.2}", self.area(), self.perimeter())
    }
}

struct Circle { radius: f64 }
impl Shape for Circle {
    fn area(&self) -> f64 { std::f64::consts::PI * self.radius * self.radius }
    fn perimeter(&self) -> f64 { 2.0 * std::f64::consts::PI * self.radius }
}

struct Rect { w: f64, h: f64 }
impl Shape for Rect {
    fn area(&self) -> f64 { self.w * self.h }
    fn perimeter(&self) -> f64 { 2.0 * (self.w + self.h) }
}

fn print_info(s: &impl Shape) {
    println!("{}", s.describe());
}

fn main() {
    let c = Circle { radius: 2.0 };
    let r = Rect { w: 3.0, h: 4.0 };
    print_info(&c);
    print_info(&r);
}
```

## 2.11 一个常见反模式

```rust
// ❌ 用 trait object 想"替代 if/else"
fn handle(shape: Box<dyn Shape>) {
    match shape {
        b if b.is::<Circle>() => { ... }   // 错
        _ => { ... }
    }
}
```

如果你的"shape 列表"会变长、且需要不同行为——用 **enum**（Stage 2）+ match，更清晰。

> **经验法则**：
> - 行为**一致**，类型可能多种 → 泛型 `<T: Trait>`
> - 行为**有差异**，需要在运行时切类型 → `dyn Trait`
> - 状态**有限集合**，需要 match 穷尽 → enum

## 2.12 对比其他语言

| 概念 | Rust | Java | C# | Go | TypeScript |
|------|------|------|-----|-----|------------|
| 名称 | trait | interface | interface | interface | interface |
| 默认方法 | ✅ | ✅（Java 8+） | ✅ | ❌ | ✅ |
| 静态方法 | ✅（在 trait 里） | ✅ | ✅ | ❌ | ✅ |
| 隐式实现 | ❌（必须写 `impl`） | ❌ | ❌ | ✅ | ❌ |
| 多继承 | ❌ | ✅（接口多） | ✅ | ✅ | ✅ |
| Self 类型 | ✅ | ✅ | ✅ | ❌ | ❌ |

## 2.13 与"鸭子类型"的对比

| 风格 | Rust trait | Python / JS 鸭子类型 |
|------|------------|---------------------|
| 实现时间 | 编译期决定 | 运行时决定 |
| 实现方式 | 显式 `impl` | 任何类型都"自动"实现 |
| 检查时机 | 编译期 | 运行时才崩 |
| 多态成本 | 单态化（零开销） | 反射（运行时查） |
| 灵活性 | 灵活但需注册 | 最灵活 |

> Rust 选"显式 + 编译期"——你必须告诉编译器"我实现了什么 trait"。**这反而让大型项目可控**。

---

## 🏋️ 本章小练习

**练习 2.1**：定义 `trait Animal { fn name(&self) -> &str; fn sound(&self) -> &str; }`，为 `Dog` / `Cat` / `Cow` 各实现。

**练习 2.2**：定义 `trait Draw { fn draw(&self); }`，写 `fn render_all(items: &[&dyn Draw])`（注：先写成 `&impl Draw` 也行，下一章改 dyn）。

**练习 2.3**：写 `fn largest<T: PartialOrd>(v: &[T]) -> Option<&T>`（空切片返回 None），加测试。

**练习 2.4**：用 derive + 手写混合：为 `User { name: String, age: u32 }` derive `Debug` / `Clone` / `PartialEq`，**手写**实现 `Display`（输出 `name (age 岁)`）。

**练习 2.5**：`where` 子句练习：

```rust
fn clone_and_print<T, U>(t: &T, u: &U) -> (T, U)
where
    T: Clone + Debug,
    U: Clone + Debug,
{
    let t = t.clone();
    let u = u.clone();
    println!("cloned: {:?} {:?}", t, u);
    (t, u)
}
```

---

下一章：[03 · 生命周期 ⭐ →](./03-lifetimes.md)
