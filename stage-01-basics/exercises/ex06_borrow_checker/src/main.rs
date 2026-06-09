// Exercise 06 · Borrow Checker Quiz
// 任务：见 README.md
// 把下面 5 段代码分别编译，看哪些错，依次修

fn main() {
    // ===== 段 1 =====
    // let mut s = String::from("hi");
    // let r1 = &s;
    // let r2 = &mut s;
    // println!("{r1} {r2}");

    // ===== 段 2 =====
    // let s = String::from("hi");
    // change(&s);

    // ===== 段 3 =====
    // let mut s = String::from("hi");
    // let r1 = &mut s;
    // let r2 = &s;
    // println!("{r1} {r2}");

    // ===== 段 4 =====
    // println!("{}", dangle());

    // ===== 段 5 =====
    // let mut v = vec![1, 2, 3];
    // for x in &v {
    //     v.push(*x + 10);
    // }
}

// fn change(s: &String) {
//     s.push_str("!");
// }

// fn dangle() -> &String {
//     let s = String::from("d");
//     &s
// }
