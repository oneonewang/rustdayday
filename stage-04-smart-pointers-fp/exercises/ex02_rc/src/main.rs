// Exercise 02 · Rc
// 任务：见 README.md

use std::rc::Rc;

#[derive(Debug)]
struct IntNode {
    value: i32,
    children: Vec<Rc<IntNode>>,
}

#[derive(Debug)]
struct List<T> {
    head: Option<Rc<ListNode<T>>>,
}

#[derive(Debug)]
struct ListNode<T> {
    elem: T,
    next: Option<Rc<ListNode<T>>>,
}

impl<T> List<T> {
    fn new() -> Self { Self { head: None } }
    fn prepend(&self, elem: T) -> List<T> { todo!() }
    fn head(&self) -> Option<&T> { todo!() }
    fn tail(&self) -> List<T> { todo!() }
}

fn count_rc<T>(r: &Rc<T>) -> usize { todo!() }

#[derive(Debug)]
struct Parent { name: String, child: Rc<Child> }
#[derive(Debug)]
struct Child { name: String }

fn main() {
    // TODO
}
