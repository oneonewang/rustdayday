// Exercise 05 · Ownership Quiz
// 任务：见 README.md
// 直接 cargo run 看哪些地方编译错，逐个修

fn main() {
    let s = String::from("hello");
    let s2 = s;
    println!("{s}");

    let n = 5;
    let m = n;
    println!("n = {n}, m = {m}");

    let s3 = String::from("world");
    takes(s3);
    println!("after: {s3}");

    let mut s4 = String::from("a");
    s4.push_str("b");
    println!("{s4}");

    let s5 = String::from("c");
    let s6 = s5.clone();
    println!("{s5} {s6}");

    let s7 = String::from("d");
    let s8 = s7;
    let s9 = s8;
    println!("{s8} {s9}");
}

fn takes(s: String) {
    println!("took: {s}");
}
