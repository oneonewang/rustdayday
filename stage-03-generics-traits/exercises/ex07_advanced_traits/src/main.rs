// Exercise 07 · 高级 Trait
// 任务：见 README.md

struct Counter { max: u32, count: u32 }

impl Counter {
    fn new(max: u32) -> Self { Self { max, count: 0 } }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> { todo!() }
}

trait Person { fn name(&self) -> &str; }
trait Student: Person { fn school(&self) -> &str; }

struct Alice;
impl Person for Alice { fn name(&self) -> &str { "Alice" } }
impl Student for Alice { fn school(&self) -> &str { "MIT" } }

fn greet_student<S: Student>(s: &S) { todo!() }

use std::num::Wrapping;
struct Wrap(Wrapping<u32>);

impl std::fmt::Display for Wrap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { todo!() }
}

struct Alternating { curr: bool }
impl Iterator for Alternating {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> { todo!() }
}

use std::ops::Add;
#[derive(Debug, PartialEq)]
struct Money(u64);
impl Add<Money> for i32 { type Output = Money; fn add(self, rhs: Money) -> Money { todo!() } }
impl Add for Money { type Output = Money; fn add(self, rhs: Money) -> Money { todo!() } }

fn main() {
    // TODO
}
