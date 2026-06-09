// Exercise 02 · trait 基础
// 任务：见 README.md

trait Animal {
    fn name(&self) -> &str;
    fn sound(&self) -> &str;
    fn describe(&self) -> String {
        format!("{} 叫 {}", self.name(), self.sound())
    }
}

struct Dog { name: String }
struct Cat { name: String }
struct Cow { name: String }

impl Animal for Dog { fn name(&self) -> &str { &self.name } fn sound(&self) -> &str { "汪汪" } }
impl Animal for Cat { fn name(&self) -> &str { &self.name } fn sound(&self) -> &str { "喵喵" } }
impl Animal for Cow { fn name(&self) -> &str { &self.name } fn sound(&self) -> &str { "哞" } }

struct User { name: String, age: u32 }

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // TODO
        todo!()
    }
}

fn announce_dyn(animal: &dyn Animal) {
    println!("看！一只 {}！", animal.name());
}

fn main() {
    // TODO
}
