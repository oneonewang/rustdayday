//! hex dump（仿 xxd / hexdump -C）

pub fn print_hex(bytes: &[u8], limit: usize) {
    let n = bytes.len().min(limit);
    println!("--- hex dump (前 {} / {} 字节) ---\n", n, bytes.len());
    for chunk in bytes[..n].chunks(16) {
        // 偏移
        let off = (chunk.as_ptr() as usize) - (bytes.as_ptr() as usize);
        print!("{:08x}  ", off);

        // 16 个字节（不足补空格）
        for (i, b) in chunk.iter().enumerate() {
            print!("{:02x} ", b);
            if i == 7 { print!(" "); }   // 中间多空一格
        }
        // 不足 16 字节时补
        let missing = 16 - chunk.len();
        for _ in 0..missing {
            print!("   ");
            if chunk.len() + missing > 8 && chunk.len() <= 8 { print!(" "); }
        }

        // ASCII 侧
        print!(" |");
        for b in chunk {
            if (0x20..=0x7e).contains(b) {
                print!("{}", *b as char);
            } else {
                print!(".");
            }
        }
        println!("|");
    }
}
