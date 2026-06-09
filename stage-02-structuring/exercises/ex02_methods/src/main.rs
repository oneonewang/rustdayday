// Exercise 02 · 方法
// 任务：见 README.md

#[derive(Debug, Clone, Copy)]
struct Rectangle { width: f64, height: f64 }

impl Rectangle {
    fn new(w: f64, h: f64) -> Self { todo!() }
    fn square(size: f64) -> Self { todo!() }
    fn area(&self) -> f64 { todo!() }
    fn perimeter(&self) -> f64 { todo!() }
    fn is_square(&self) -> bool { todo!() }
    fn can_contain(&self, other: &Rectangle) -> bool { todo!() }
    fn scale(&mut self, factor: f64) { todo!() }
    fn shrink_to_fit(self, target: &Rectangle) -> Self { todo!() }
}

fn main() {
    // TODO
}
