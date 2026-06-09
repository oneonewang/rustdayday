// Exercise 06 · 自定义 Iterator
// 任务：见 README.md

struct FibIter { curr: u64, next: u64 }

impl Iterator for FibIter {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> { todo!() }
}

struct StrSplit<'a> { haystack: &'a str, delim: char }

impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> { todo!() }
}

struct Cycle<T> { curr: usize, items: Vec<T> }

impl<T: Clone> Iterator for Cycle<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> { todo!() }
}

struct Counter { count: u32 }
impl Counter {
    fn new() -> Self { Self { count: 0 } }
}
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> { todo!() }
}

struct RevDigits { n: u32 }
impl RevDigits {
    fn new(n: u32) -> Self { Self { n } }
}
impl Iterator for RevDigits {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> { todo!() }
}
impl DoubleEndedIterator for RevDigits {
    fn next_back(&mut self) -> Option<Self::Item> { todo!() }
}

fn main() {
    // TODO
}
