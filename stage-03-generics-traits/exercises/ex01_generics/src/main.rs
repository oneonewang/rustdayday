// Exercise 01 · 泛型基础
// 任务：见 README.md

fn swap<T>(pair: (T, T)) -> (T, T) { todo!() }

fn largest<T: PartialOrd + Copy>(v: &[T]) -> Option<T> { todo!() }

#[derive(Debug)]
struct Pair<T> {
    first: T,
    second: T,
}

impl<T> Pair<T> {
    fn new(a: T, b: T) -> Self { todo!() }
    fn swap(self) -> Self { todo!() }
    fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Pair<U> { todo!() }
}

#[derive(Debug)]
enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Either<L, R> {
    fn is_left(&self) -> bool { todo!() }
    fn into_left(self) -> Option<L> { todo!() }
    fn map_left<L2, F: FnOnce(L) -> L2>(self, f: F) -> Either<L2, R> { todo!() }
}

fn main() {
    // TODO
}
