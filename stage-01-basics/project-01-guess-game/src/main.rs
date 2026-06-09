// Project 01 · 猜数字游戏
// 任务：见 README.md

use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("=== 猜数字游戏 ===");
    println!("我已经想好了一个 1 到 100 之间的整数。");
    println!("输入你的猜测，我会告诉你是大了、小了还是猜中了。");
    println!("输入 'quit' 退出游戏。\n");

    let secret = rand::thread_rng().gen_range(1..=100);
    let mut attempts = 0;

    loop {
        println!("请输入你的猜测：");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("读取输入失败");

        let guess = guess.trim();

        // 退出命令
        if guess.eq_ignore_ascii_case("quit") || guess.eq_ignore_ascii_case("exit") {
            println!("下次再玩！秘密数字是 {secret}。");
            break;
        }

        // 解析输入
        let guess: u32 = match guess.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("'{guess}' 不是合法数字，请重新输入。\n");
                continue;
            }
        };

        attempts += 1;
        println!("你猜的是：{guess}");

        // 比较
        match guess.cmp(&secret) {
            Ordering::Less    => println!("太小了，再大一点！\n"),
            Ordering::Greater => println!("太大了，再小一点！\n"),
            Ordering::Equal   => {
                println!("\n🎉 恭喜！你在 {attempts} 次尝试后猜中了 {secret}！");
                break;
            }
        }
    }
}
