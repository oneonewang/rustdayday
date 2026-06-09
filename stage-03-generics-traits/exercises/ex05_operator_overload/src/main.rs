// Exercise 05 · 运算符重载与标准 trait
// 任务：见 README.md

use std::ops::{Add, AddAssign, Mul};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point { x: i32, y: i32 }

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point { todo!() }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Point) { todo!() }
}

impl Mul<i32> for Point {
    type Output = Point;
    fn mul(self, scalar: i32) -> Point { todo!() }
}

#[derive(Debug, PartialEq, Eq)]
struct Money { cents: u64 }

impl std::fmt::Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // TODO
        todo!()
    }
}

impl std::str::FromStr for Money {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO
        todo!()
    }
}

impl From<i32> for Money {
    fn from(cents: i32) -> Self { todo!() }
}

fn main() {
    // TODO
}
