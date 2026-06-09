// Exercise 01 · struct 基础
// 任务：见 README.md

#[derive(Debug, Clone, PartialEq)]
struct Book {
    title: String,
    author: String,
    pages: u32,
    price: f64,
}

impl Book {
    fn new(title: &str, author: &str, pages: u32, price: f64) -> Self {
        // TODO
        todo!()
    }

    fn free_sample(title: &str, author: &str) -> Self {
        // TODO
        todo!()
    }
}

fn main() {
    // TODO
}
