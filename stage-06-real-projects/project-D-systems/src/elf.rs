//! 手解 ELF header（64-bit ELF，主要字段）
//!
//! 数据结构严格按 ELF spec 排布，注意 endianness。

use std::fmt;

use anyhow::{anyhow, Result};

/// ELF class
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElfClass {
    Elf32,
    Elf64,
}

/// ELF data encoding (endianness)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElfData {
    Little,
    Big,
}

/// ELF OS / ABI
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElfOsAbi {
    UnixSystemV,
    HpUx,
    NetBSD,
    Linux,
    Solaris,
    Aix,
    Irix,
    FreeBSD,
    OpenBSD,
    Other(u8),
}

/// ELF machine type（只列常见的）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElfMachine {
    X86,
    X86_64,
    Arm,
    AArch64,
    RiscV,
    Unknown(u16),
}

#[derive(Debug, Clone)]
pub struct ElfInfo {
    pub class: ElfClass,
    pub data: ElfData,
    pub version: u8,
    pub os_abi: ElfOsAbi,
    pub abi_version: u8,
    pub machine: ElfMachine,
    pub elf_type: u16,
    pub entry_point: u64,
    pub program_header_offset: u64,
    pub section_header_offset: u64,
    pub flags: u32,
    pub program_header_size: u16,
    pub program_header_count: u16,
    pub section_header_size: u16,
    pub section_header_count: u16,
    pub section_name_string_table_index: u16,
}

pub fn parse(bytes: &[u8]) -> Result<ElfInfo> {
    if bytes.len() < 16 {
        return Err(anyhow!("文件太小，不是 ELF（< 16 字节）"));
    }

    // 用 libc::memcmp 验证 magic（演示 FFI）
    let magic = [0x7fu8, b'E', b'L', b'F'];
    let cmp = unsafe { libc::memcmp(bytes.as_ptr() as *const _, magic.as_ptr() as *const _, 4) };
    if cmp != 0 {
        return Err(anyhow!("不是 ELF 文件（magic 不匹配）"));
    }

    let class = match bytes[4] {
        1 => ElfClass::Elf32,
        2 => ElfClass::Elf64,
        other => return Err(anyhow!("未知 ELF class: {other}")),
    };

    let data = match bytes[5] {
        1 => ElfData::Little,
        2 => ElfData::Big,
        other => return Err(anyhow!("未知 ELF data: {other}")),
    };
    let version = bytes[6];
    let os_abi = match bytes[7] {
        0 => ElfOsAbi::UnixSystemV,
        1 => ElfOsAbi::HpUx,
        2 => ElfOsAbi::NetBSD,
        3 => ElfOsAbi::Linux,
        6 => ElfOsAbi::Solaris,
        7 => ElfOsAbi::Aix,
        8 => ElfOsAbi::Irix,
        9 => ElfOsAbi::FreeBSD,
        12 => ElfOsAbi::OpenBSD,
        o  => ElfOsAbi::Other(o),
    };
    let abi_version = bytes[8];

    // 字段在 ELF32 和 ELF64 不同
    let info = match class {
        ElfClass::Elf32 => parse_elf32(bytes, data, version, os_abi, abi_version)?,
        ElfClass::Elf64 => parse_elf64(bytes, data, version, os_abi, abi_version)?,
    };
    Ok(info)
}

fn parse_elf32(
    bytes: &[u8],
    data: ElfData,
    version: u8,
    os_abi: ElfOsAbi,
    abi_version: u8,
) -> Result<ElfInfo> {
    if bytes.len() < 52 {
        return Err(anyhow!("ELF32 文件头不完整"));
    }
    // 解析时按 little-endian 读——大多数平台都 LE
    let read_u16 = |off: usize| -> u16 { u16::from_le_bytes([bytes[off], bytes[off+1]]) };
    let read_u32 = |off: usize| -> u32 {
        u32::from_le_bytes([bytes[off], bytes[off+1], bytes[off+2], bytes[off+3]])
    };

    let e_type = read_u16(16);
    let e_machine = read_u16(18);
    let e_version = read_u32(20);
    let e_entry = read_u32(24) as u64;
    let e_phoff = read_u32(28) as u64;
    let e_shoff = read_u32(32) as u64;
    let e_flags = read_u32(36);
    let e_ehsize = read_u16(40);
    let e_phentsize = read_u16(42);
    let e_phnum = read_u16(44);
    let e_shentsize = read_u16(46);
    let e_shnum = read_u16(48);
    let e_shstrndx = read_u16(50);

    Ok(ElfInfo {
        class: ElfClass::Elf32,
        data,
        version: e_version as u8,
        os_abi,
        abi_version,
        machine: classify_machine(e_machine),
        elf_type: e_type,
        entry_point: e_entry,
        program_header_offset: e_phoff,
        section_header_offset: e_shoff,
        flags: e_flags,
        program_header_size: e_ehsize,
        program_header_count: e_phnum,
        section_header_size: e_shentsize,
        section_header_count: e_shnum,
        section_name_string_table_index: e_shstrndx,
    })
}

fn parse_elf64(
    bytes: &[u8],
    data: ElfData,
    version: u8,
    os_abi: ElfOsAbi,
    abi_version: u8,
) -> Result<ElfInfo> {
    if bytes.len() < 64 {
        return Err(anyhow!("ELF64 文件头不完整（< 64 字节）"));
    }
    let read_u16 = |off: usize| -> u16 { u16::from_le_bytes([bytes[off], bytes[off+1]]) };
    let read_u32 = |off: usize| -> u32 {
        u32::from_le_bytes([bytes[off], bytes[off+1], bytes[off+2], bytes[off+3]])
    };
    let read_u64 = |off: usize| -> u64 {
        u64::from_le_bytes([
            bytes[off], bytes[off+1], bytes[off+2], bytes[off+3],
            bytes[off+4], bytes[off+5], bytes[off+6], bytes[off+7],
        ])
    };

    let e_type = read_u16(16);
    let e_machine = read_u16(18);
    let _e_version = read_u32(20);
    let e_entry = read_u64(24);
    let e_phoff = read_u64(32);
    let e_shoff = read_u64(40);
    let e_flags = read_u32(48);
    let _e_ehsize = read_u16(52);
    let e_phentsize = read_u16(54);
    let e_phnum = read_u16(56);
    let e_shentsize = read_u16(58);
    let e_shnum = read_u16(60);
    let e_shstrndx = read_u16(62);

    Ok(ElfInfo {
        class: ElfClass::Elf64,
        data,
        version,
        os_abi,
        abi_version,
        machine: classify_machine(e_machine),
        elf_type: e_type,
        entry_point: e_entry,
        program_header_offset: e_phoff,
        section_header_offset: e_shoff,
        flags: e_flags,
        program_header_size: e_phentsize,
        program_header_count: e_phnum,
        section_header_size: e_shentsize,
        section_header_count: e_shnum,
        section_name_string_table_index: e_shstrndx,
    })
}

fn classify_machine(m: u16) -> ElfMachine {
    match m {
        3   => ElfMachine::X86,
        62  => ElfMachine::X86_64,
        40  => ElfMachine::Arm,
        183 => ElfMachine::AArch64,
        243 => ElfMachine::RiscV,
        o   => ElfMachine::Unknown(o),
    }
}

const ET_EXEC: u16 = 2;
const ET_DYN: u16 = 3;

pub fn print_info(info: &ElfInfo, bytes: &[u8]) {
    println!("--- ELF Header ---");
    println!("Class:         {:?}", info.class);
    println!("Data:          {:?}", info.data);
    println!("Version:       {}", info.version);
    println!("OS/ABI:        {:?} (ver {})", info.os_abi, info.abi_version);
    println!("Type:          {} ({})", info.elf_type, elf_type_name(info.elf_type));
    println!("Machine:       {:?}", info.machine);
    println!("Entry point:   {:#x}", info.entry_point);
    println!("PH offset:     {} ({} entries × {} bytes)",
        info.program_header_offset, info.program_header_count, info.program_header_size);
    println!("SH offset:     {} ({} entries × {} bytes)",
        info.section_header_offset, info.section_header_count, info.section_header_size);
    println!("Flags:         {:#x}", info.flags);
    println!("SH strndx:     {}", info.section_name_string_table_index);

    // 第一个 16 字节的 hexdump
    println!("\n--- 文件头前 16 字节 ---");
    hex_dump_inline(&bytes[..16.min(bytes.len())]);

    // 可执行 / 动态库？
    if info.elf_type == ET_EXEC {
        println!("\n[这是一个可执行文件（ET_EXEC）]");
    } else if info.elf_type == ET_DYN {
        println!("\n[这是一个动态库 / PIE 可执行（ET_DYN）]");
    }

    // 试着读 section header 第一个
    if info.section_header_count > 0
        && info.section_header_offset as usize <= bytes.len()
    {
        println!("\n--- Section Headers (前 {} 个) ---", info.section_header_count.min(5));
        for i in 0..info.section_header_count.min(5) {
            let sh_off = info.section_header_offset as usize
                + i as usize * info.section_header_size as usize;
            if sh_off + 64 <= bytes.len() {
                let sh_name = u32::from_le_bytes([bytes[sh_off], bytes[sh_off+1], bytes[sh_off+2], bytes[sh_off+3]]);
                let sh_type = u32::from_le_bytes([bytes[sh_off+4], bytes[sh_off+5], bytes[sh_off+6], bytes[sh_off+7]]);
                println!("  [{}]  name_idx={:#x}  type={:#x}", i, sh_name, sh_type);
            }
        }
    }
}

fn elf_type_name(t: u16) -> &'static str {
    match t {
        0 => "ET_NONE",
        1 => "ET_REL (relocatable)",
        2 => "ET_EXEC (executable)",
        3 => "ET_DYN (shared object / PIE)",
        4 => "ET_CORE (core dump)",
        _ => "unknown",
    }
}

fn hex_dump_inline(bytes: &[u8]) {
    for (i, b) in bytes.iter().enumerate() {
        print!("{:02x} ", b);
        if (i + 1) % 16 == 0 { println!(); }
    }
    if bytes.len() % 16 != 0 { println!(); }
}

// 实现 Display 给 ElfInfo
impl fmt::Display for ElfInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ELF {:?} {:?} for {:?}", self.class, self.data, self.machine)
    }
}
