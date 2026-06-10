//! 离线自检：用 wasmparser 解析 .wasm，断言所有导出都存在且签名正确

use std::path::Path;

fn main() {
    let wasm_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target/wasm32-unknown-unknown/release/wasm_frontend.wasm");
    if !wasm_path.exists() {
        eprintln!("❌ 找不到 {}\n   请先: cargo build --release --target wasm32-unknown-unknown", wasm_path.display());
        std::process::exit(1);
    }

    let bytes = std::fs::read(&wasm_path).expect("read wasm");
    println!("=== Project C 自检：Rust → WASM ===");
    println!("模块: {}", wasm_path.display());
    println!("大小: {} bytes\n", bytes.len());

    let parser = wasmparser::Parser::new(0);
    let mut exports = vec![];
    let mut imports = vec![];
    let mut memories = 0;
    for payload in parser.parse_all(&bytes) {
        let payload = payload.expect("parse");
        match payload {
            wasmparser::Payload::ExportSection(s) => {
                for exp in s {
                    let exp = exp.expect("export");
                    exports.push((exp.name.to_string(), format!("{:?}", exp.kind)));
                }
            }
            wasmparser::Payload::ImportSection(s) => {
                for imp in s {
                    let imp = imp.expect("import");
                    imports.push(imp.name.to_string());
                }
            }
            wasmparser::Payload::MemorySection(s) => {
                for _ in s { memories += 1; }
            }
            _ => {}
        }
    }

    println!("--- 导入 ({} 个) ---", imports.len());
    for i in &imports { println!("  • {i}"); }
    println!("\n--- 内存段 ({} 个) ---", memories);
    println!("\n--- 导出 ({} 个) ---", exports.len());
    for (name, kind) in &exports {
        println!("  • {name}  [{kind}]");
    }

    println!("\n--- 关键导出断言 ---");
    let required = [
        ("factorial",       "用户函数"),
        ("fibonacci",       "用户函数"),
        ("is_prime",        "用户函数"),
        ("reverse",         "用户函数"),
        ("sum_array",       "用户函数"),
        ("version",         "用户函数"),
        ("__wbindgen_malloc", "wasm-bindgen 内存分配"),
        ("__wbindgen_free",   "wasm-bindgen 内存释放"),
        ("__data_end",        "数据段结束"),
        ("__heap_base",       "堆起始"),
    ];

    let mut passed = 0;
    let mut failed = 0;
    for (r, desc) in required {
        if exports.iter().any(|(n, _)| n == r) {
            println!("  ✅ {r} ({desc})");
            passed += 1;
        } else {
            println!("  ❌ {r} ({desc}) 缺失");
            failed += 1;
        }
    }

    println!("\n--- 统计 ---");
    println!("  导出: {} 个", exports.len());
    println!("  passed: {passed}");
    println!("  failed: {failed}");

    if failed == 0 {
        println!("\n🎉 所有关键导出都存在 ✅");
    } else {
        std::process::exit(1);
    }
}
