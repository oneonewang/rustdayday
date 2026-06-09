// Project 02 · CLI Todo
// 多文件组织：main 入口、todo 业务、storage 持久化、error 错误

mod cli;
mod error;
mod storage;
mod todo;

use error::AppError;

fn main() -> Result<(), AppError> {
    let path = std::env::var("TODO_FILE").unwrap_or_else(|_| "todos.json".to_string());

    let cmd = cli::parse_args();
    let mut list = storage::load(&path)?;

    match cmd {
        cli::Command::Add(text) => {
            list.add(text);
        }
        cli::Command::List => {
            list.print();
        }
        cli::Command::Done(id) => {
            list.complete(id)?;
        }
        cli::Command::Remove(id) => {
            list.remove(id)?;
        }
    }

    storage::save(&path, &list)?;
    Ok(())
}
