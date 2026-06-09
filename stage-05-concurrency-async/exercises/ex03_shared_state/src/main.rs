// Exercise 03 · 共享状态
// 任务：见 README.md

use std::sync::{Arc, Mutex, RwLock, OnceLock};

fn parallel_count(n: usize) -> i32 { todo!() }
fn reader_writer() { todo!() }

static CONFIG: OnceLock<String> = OnceLock::new();
fn get_config() -> &'static String { todo!() }

fn deadlock_fix() { todo!() }
fn poisoned_recover() { todo!() }

fn main() {
    // TODO
}
