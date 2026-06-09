// Exercise 04 · 闭包
// 任务：见 README.md

fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 { todo!() }
fn apply_mut<F: FnMut(i32) -> i32>(mut f: F, x: i32) -> i32 { todo!() }
fn apply_once<F: FnOnce(i32) -> i32>(f: F, x: i32) -> i32 { todo!() }

fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    // TODO: move |x| x + n
    move |x| x + n
}

fn sort_by<T, F: FnMut(&T, &T) -> std::cmp::Ordering>(v: &mut [T], mut cmp: F) { todo!() }

fn filter_positive(v: Vec<i32>) -> Vec<i32> { todo!() }

fn main() {
    // TODO
}
