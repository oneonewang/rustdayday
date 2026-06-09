// cli.rs — 解析命令行参数

#[derive(Debug)]
pub enum Command {
    Add(String),
    List,
    Done(u32),
    Remove(u32),
}

pub fn parse_args() -> Command {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        print_usage();
        std::process::exit(1);
    }

    match args[1].as_str() {
        "add" => {
            if args.len() < 3 {
                eprintln!("用法: todo add <内容>");
                std::process::exit(1);
            }
            Command::Add(args[2..].join(" "))
        }
        "list" | "ls" => Command::List,
        "done" => {
            let id = parse_id(&args);
            Command::Done(id)
        }
        "remove" | "rm" => {
            let id = parse_id(&args);
            Command::Remove(id)
        }
        "-h" | "--help" | "help" => {
            print_usage();
            std::process::exit(0);
        }
        other => {
            eprintln!("未知子命令: {other}");
            print_usage();
            std::process::exit(1);
        }
    }
}

fn parse_id(args: &[String]) -> u32 {
    if args.len() < 3 {
        eprintln!("用法: todo {} <id>", args[1]);
        std::process::exit(1);
    }
    args[2].parse::<u32>().unwrap_or_else(|_| {
        eprintln!("id 必须是正整数，得到 '{}'", args[2]);
        std::process::exit(1);
    })
}

fn print_usage() {
    println!("CLI Todo — 任务管理工具");
    println!();
    println!("用法:");
    println!("  todo add <内容>      添加新任务");
    println!("  todo list            列出所有任务");
    println!("  todo done <id>       标记任务为完成");
    println!("  todo remove <id>     删除任务");
    println!();
    println!("环境变量:");
    println!("  TODO_FILE            数据文件路径（默认 todos.json）");
}
