// Exercise 05 · 错误处理
// 任务：见 README.md

#[derive(Debug)]
enum AppError {
    Io(String),
    Parse(String, std::num::ParseIntError),
    Format(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AppError::Io(s) => write!(f, "I/O error: {s}"),
            AppError::Parse(line, e) => write!(f, "parse error on `{line}`: {e}"),
            AppError::Format(s) => write!(f, "format error: {s}"),
        }
    }
}

impl std::error::Error for AppError {}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self { AppError::Io(e.to_string()) }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(e: std::num::ParseIntError) -> Self { AppError::Parse("<unknown>".to_string(), e) }
}

fn read_numbers(path: &str) -> Result<Vec<i32>, AppError> {
    // TODO
    todo!()
}

fn main() {
    // TODO: 写一个 numbers.txt，跑 read_numbers，打印结果
}
