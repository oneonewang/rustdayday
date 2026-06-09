// Exercise 06 · Send / Sync
// 任务：见 README.md

use std::rc::Rc;

fn assert_send<T: Send>() {}
fn assert_sync<T: Sync>() {}

async fn some_op() {}

async fn uses_rc() {
    let r = Rc::new(5);
    println!("before: {r}");
    some_op().await;
}

async fn bad_uses_rc() {
    let r = Rc::new(5);
    some_op().await;
    println!("{r}");
}

struct MyPtr(*mut u8);
unsafe impl Send for MyPtr {}
unsafe impl Sync for MyPtr {}

fn main() {
    // TODO
}
