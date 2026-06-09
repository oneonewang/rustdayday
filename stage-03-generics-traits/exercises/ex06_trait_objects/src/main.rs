// Exercise 06 · Trait Object
// 任务：见 README.md

trait Command {
    fn name(&self) -> &str;
    fn execute(&self);
}

struct AddCommand { delta: i32 }
impl Command for AddCommand {
    fn name(&self) -> &str { "add" }
    fn execute(&self) { println!("+{}", self.delta); }
}

struct PrintCommand { prefix: String }
impl Command for PrintCommand {
    fn name(&self) -> &str { "print" }
    fn execute(&self) { println!("{}{{prefix}}", self.prefix); }   // TODO
}

struct Runner { commands: Vec<Box<dyn Command>> }

impl Runner {
    fn new() -> Self { todo!() }
    fn add(&mut self, cmd: Box<dyn Command>) { todo!() }
    fn run_all(&self) { todo!() }
}

trait Shape { fn area(&self) -> f64; }

struct Circle { radius: f64 }
struct Square { side: f64 }

impl Shape for Circle { fn area(&self) -> f64 { todo!() } }
impl Shape for Square { fn area(&self) -> f64 { todo!() } }

fn total_area(shapes: &[Box<dyn Shape>]) -> f64 { todo!() }

fn main() {
    // TODO
}
