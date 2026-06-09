# Exercise 02 · trait 基础

> 难度：⭐⭐  涉及：第 2 章

## 任务

### 1) 定义一个 trait

```rust
trait Animal {
    fn name(&self) -> &str;
    fn sound(&self) -> &str;

    // 默认实现
    fn describe(&self) -> String {
        format!("{} 叫 {}", self.name(), self.sound())
    }
}
```

为 `Dog` / `Cat` / `Cow` 各实现，sound 自己挑。

### 2) 自定义 Display

```rust
struct User {
    name: String,
    age: u32,
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // 格式：Alice (30 岁)
    }
}
```

> 注意 `User` 不能 derive `Display`（标准库没有自动派生），必须手写。

### 3) trait 作为参数

```rust
fn announce<T: Animal>(animal: &T) {
    println!("看！一只 {}！", animal.name());
}

fn announce_dyn(animal: &dyn Animal) {            // 同上另一种写法
    println!("看！一只 {}！", animal.name());
}
```

写一个 `Vec<Box<dyn Animal>>`，装 3 种动物，遍历 `announce_dyn` 调用。

## 验收

```rust
let dog = Dog { name: "旺财".to_string() };
let cat = Cat { name: "咪咪".to_string() };
let cow = Cow { name: "牛牛".to_string() };

assert_eq!(dog.sound(), "汪汪");
assert_eq!(cat.sound(), "喵喵");
assert_eq!(cow.sound(), "哞");

println!("{}", dog.describe());

let u = User { name: "Alice".to_string(), age: 30 };
assert_eq!(format!("{u}"), "Alice (30 岁)");

let zoo: Vec<Box<dyn Animal>> = vec![
    Box::new(dog),
    Box::new(cat),
    Box::new(cow),
];
for a in &zoo {
    announce_dyn(a.as_ref());
}
```

## 提示

- `Box<dyn Animal>::as_ref()` 把 `&Box<dyn Animal>` 转成 `&dyn Animal`
- `format!("{u}")` 用的是 `Display`

## 进阶

写一个 `trait Shape`（`area` / `perimeter`），为 `Circle` / `Rectangle` / `Triangle` 实现，然后写一个**总周长**函数：

```rust
fn total_perimeter(shapes: &[Box<dyn Shape>]) -> f64
```

完成 → [ex03_trait_bounds](../ex03_trait_bounds)
