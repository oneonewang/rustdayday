// Exercise 06 · 模块化
// 任务：见 README.md

// === 第一步：把下面的代码放在 src/main.rs 里，验证能跑通 ===
// === 第二步：拆成 src/lib.rs + src/geometry/{mod.rs, shape.rs, utils.rs} ===

#[derive(Debug)]
pub struct Circle { pub radius: f64 }
impl Circle {
    pub fn new(r: f64) -> Self { Self { radius: r } }
    pub fn area(&self) -> f64 { std::f64::consts::PI * self.radius * self.radius }
    pub fn perimeter(&self) -> f64 { 2.0 * std::f64::consts::PI * self.radius }
}

#[derive(Debug)]
pub struct Rectangle { pub width: f64, pub height: f64 }
impl Rectangle {
    pub fn new(w: f64, h: f64) -> Self { Self { width: w, height: h } }
    pub fn area(&self) -> f64 { self.width * self.height }
    pub fn perimeter(&self) -> f64 { 2.0 * (self.width + self.height) }
}

pub fn describe(s: &str) -> String { format!("这是一个 {s} 模块") }
pub fn version() -> &'static str { "0.1.0" }

fn main() {
    let c = Circle::new(2.0);
    let r = Rectangle::new(3.0, 4.0);
    println!("{} 面积 = {}, 形状版本 = {}", describe("circle"), c.area(), version());
    println!("{} 面积 = {}", describe("rectangle"), r.area());
}
