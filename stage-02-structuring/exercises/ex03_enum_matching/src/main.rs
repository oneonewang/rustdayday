// Exercise 03 · enum 与模式匹配
// 任务：见 README.md

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddr {
    fn display(&self) -> String { todo!() }
}

#[derive(Debug)]
enum Event {
    Click { x: i32, y: i32 },
    KeyPress(char),
    Resize { width: u32, height: u32 },
    Quit,
}

impl Event {
    fn handle(&self) { todo!() }
}

#[derive(Debug)]
enum Expr {
    Num(i32),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
}

impl Expr {
    fn evaluate(&self) -> i32 { todo!() }
}

fn drain_print(v: &mut Vec<i32>) { todo!() }

fn main() {
    // TODO
}
