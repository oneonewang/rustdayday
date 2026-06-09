// Exercise 03 · trait bound
// 任务：见 README.md

use std::fmt::Display;

fn print_twice<T: Display + Clone>(x: T) {
    // TODO
    todo!()
}

fn complex<T, U>(t: &T, u: &U) -> String
where
    T: std::fmt::Display + std::fmt::Debug + Clone,
    U: std::fmt::Debug + Clone,
{
    // TODO
    todo!()
}

struct Wrapper<T> { value: T }

impl<T> Wrapper<T> {
    fn new(value: T) -> Self { Self { value } }
}

impl<T: std::fmt::Display> Wrapper<T> {
    fn print(&self) {
        // TODO
        todo!()
    }
}

impl<T: Clone> Wrapper<T> {
    fn duplicate(&self) -> Self {
        // TODO
        todo!()
    }
}

fn make_iter() -> impl Iterator<Item = i32> {
    // TODO: 返回 1..=5 这样的迭代器
    1..=5
}

fn main() {
    // TODO
}
