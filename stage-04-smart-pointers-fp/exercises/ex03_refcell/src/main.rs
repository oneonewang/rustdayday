// Exercise 03 · RefCell
// 任务：见 README.md

use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};

struct LazyCache<T> {
    value: RefCell<Option<T>>,
    init: Box<dyn Fn() -> T>,
}

impl<T> LazyCache<T> {
    fn new(init: impl Fn() -> T + 'static) -> Self { todo!() }
    fn get_or_init(&self) -> &T { todo!() }
    fn reset(&self) { todo!() }
}

struct Counter { value: Cell<u32> }
impl Counter {
    fn new() -> Self { Self { value: Cell::new(0) } }
    fn increment(&self) { todo!() }
    fn get(&self) -> u32 { todo!() }
}

#[derive(Debug)]
struct TreeNode {
    value: i32,
    children: RefCell<Vec<Rc<TreeNode>>>,
}

impl TreeNode {
    fn new(v: i32) -> Rc<Self> { todo!() }
    fn add_child(self: &Rc<Self>, v: i32) -> Rc<Self> { todo!() }
    fn count(&self) -> usize { todo!() }
    fn depth(&self) -> usize { todo!() }
}

#[derive(Debug)]
struct Parent { children: RefCell<Vec<Rc<Child>>> }
#[derive(Debug)]
struct Child { parent: RefCell<Option<Weak<Parent>>> }

fn main() {
    // TODO
}
