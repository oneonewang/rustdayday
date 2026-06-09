// Project 04 · JSON 解析器
// 任务：见 README.md

mod json;

use json::{parse, Json};

fn main() {
    println!("=== 演示 1：基本值 ===");
    let inputs = vec![
        "null",
        "true",
        "false",
        "42",
        "-3.14",
        "\"hello, rust\"",
    ];
    for s in inputs {
        match parse(s) {
            Ok(v)  => println!("{:<25} => {}", s, v),
            Err(e) => println!("{:<25} => ERR: {}", s, e),
        }
    }

    println!("\n=== 演示 2：数组 ===");
    let arr = "[1, 2, 3, 4, 5]";
    let parsed = parse(arr).unwrap();
    println!("{} => {}", arr, parsed);

    println!("\n=== 演示 3：嵌套对象 ===");
    let obj = r#"{
        "name": "alice",
        "age": 30,
        "active": true,
        "scores": [95, 87, 92],
        "address": {
            "city": "Beijing",
            "zip": "100000"
        }
    }"#;
    let parsed = parse(obj).expect("should parse");
    println!("{}", parsed);

    println!("\n=== 演示 4：错误处理 ===");
    let bad_inputs = vec![
        "nul",                    // 不完整
        "[1, 2,",                 // 缺 ]
        "{\"a\": }",              // 值缺失
        "tru",                    // 缺 e
        "[1, 2, 3",               // 缺 ]
        "{\"a\": 1,}",            // 末尾多余逗号
    ];
    for s in bad_inputs {
        match parse(s) {
            Ok(v)  => println!("{:<20} => {} (居然过了？)", s, v),
            Err(e) => println!("{:<20} => ERR: {}", s, e),
        }
    }

    println!("\n=== 演示 5：完整断言 ===");
    run_assertions();
    println!("全部断言通过 ✅");
}

fn run_assertions() {
    // 基本值
    assert_eq!(parse("null").unwrap(), Json::Null);
    assert_eq!(parse("true").unwrap(), Json::Bool(true));
    assert_eq!(parse("false").unwrap(), Json::Bool(false));
    assert_eq!(parse("42").unwrap(), Json::Number(42.0));
    assert_eq!(parse("-3.14").unwrap(), Json::Number(-3.14));
    assert_eq!(
        parse("\"hi\"").unwrap(),
        Json::String("hi".to_string())
    );

    // 数组
    assert_eq!(
        parse("[]").unwrap(),
        Json::Array(vec![])
    );
    assert_eq!(
        parse("[1, 2, 3]").unwrap(),
        Json::Array(vec![
            Json::Number(1.0),
            Json::Number(2.0),
            Json::Number(3.0),
        ])
    );

    // 嵌套
    let nested = parse("[[1, 2], [3, 4]]").unwrap();
    assert_eq!(
        nested,
        Json::Array(vec![
            Json::Array(vec![Json::Number(1.0), Json::Number(2.0)]),
            Json::Array(vec![Json::Number(3.0), Json::Number(4.0)]),
        ])
    );

    // 对象
    let obj = parse(r#"{"name": "alice", "age": 30}"#).unwrap();
    assert_eq!(
        obj,
        Json::Object(vec![
            ("name".to_string(), Json::String("alice".to_string())),
            ("age".to_string(), Json::Number(30.0)),
        ])
    );

    // 错误
    assert!(parse("nul").is_err());
    assert!(parse("[1, 2,").is_err());
    assert!(parse("{\"a\": }").is_err());
    assert!(parse("tru").is_err());
    assert!(parse("[1, 2, 3").is_err());

    // Display 输出正确
    assert_eq!(format!("{}", Json::Null), "null");
    assert_eq!(format!("{}", Json::Bool(true)), "true");
    assert_eq!(format!("{}", Json::Number(42.0)), "42");
    assert_eq!(format!("{}", Json::String("hi".to_string())), "\"hi\"");
}
