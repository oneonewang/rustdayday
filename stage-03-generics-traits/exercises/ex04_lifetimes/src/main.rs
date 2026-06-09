// Exercise 04 · 生命周期
// 任务：见 README.md

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { todo!() }

fn first_word(s: &str) -> &str { todo!() }

struct Row<'a> {
    id: u32,
    name: &'a str,
}

impl<'a> Row<'a> {
    fn new(id: u32, name: &'a str) -> Self { todo!() }
    fn name(&self) -> &str { todo!() }
}

fn mix<'a, 'b>(x: &'a str, y: &'b str) -> &'a str { todo!() }

fn app_name() -> &'static str { todo!() }

struct LongestWith<'a, 'b> {
    s1: &'a str,
    s2: &'b str,
}

impl<'a, 'b> LongestWith<'a, 'b> {
    fn new(s1: &'a str, s2: &'b str) -> Self { todo!() }
    fn longest(&self) -> &str { todo!() }
    fn first(&self) -> &str { todo!() }
}

fn main() {
    // TODO
}
