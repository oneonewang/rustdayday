// Exercise 07 · 智能指针深入
// 任务：见 README.md

use std::sync::{Arc, Mutex, OnceLock, RwLock};
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::borrow::Cow;
use std::collections::HashMap;
use std::thread;

fn parallel_increment(n: usize) -> i32 { todo!() }

struct Config { map: Arc<RwLock<HashMap<String, String>>> }
impl Config {
    fn new() -> Self { todo!() }
    fn get(&self, key: &str) -> Option<String> { todo!() }
    fn set(&self, key: String, value: String) { todo!() }
}

fn trim_if_long(s: &str, max_len: usize) -> Cow<str> { todo!() }

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Option<Weak<Node>>>,
    children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    fn new(v: i32) -> Rc<Self> { todo!() }
    fn child_of(self: &Rc<Self>, v: i32) -> Rc<Node> { todo!() }
    fn parent_value(&self) -> Option<i32> { todo!() }
}

static GREETING: OnceLock<String> = OnceLock::new();
fn get_greeting() -> &'static String { todo!() }

fn main() {
    // TODO
}
