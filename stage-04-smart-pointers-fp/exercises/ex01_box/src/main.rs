// Exercise 01 · Box
// 任务：见 README.md

use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> Self { MyBox(x) }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T { &self.0 }
}

enum IntList {
    Cons(i32, Box<IntList>),
    Nil,
}

impl IntList {
    fn new() -> Self { IntList::Nil }
    fn prepend(self, elem: i32) -> Self { todo!() }
    fn sum(&self) -> i32 { todo!() }
    fn len(&self) -> usize { todo!() }
    fn to_vec(&self) -> Vec<i32> { todo!() }
}

trait Animal { fn name(&self) -> &str; }

struct Dog;
struct Cat;

impl Animal for Dog { fn name(&self) -> &str { "Dog" } }
impl Animal for Cat { fn name(&self) -> &str { "Cat" } }

fn make_zoo() -> Vec<Box<dyn Animal>> { todo!() }

enum BinaryTree {
    Empty,
    Node(i32, Box<BinaryTree>, Box<BinaryTree>),
}

impl BinaryTree {
    fn leaf(v: i32) -> Self { todo!() }
    fn node(v: i32, l: BinaryTree, r: BinaryTree) -> Self { todo!() }
    fn sum(&self) -> i32 { todo!() }
    fn height(&self) -> usize { todo!() }
    fn contains(&self, v: i32) -> bool { todo!() }
}

fn main() {
    // TODO
}
