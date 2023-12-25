use std::{fs::File, io::Write};

use codegen::x64_linux_elf::elf::Elf64EHdr;
// use codegen::{LiteralAllocator, RegAllocator, codegen_body, codegen_code};
use json::{Code, JsonSt};

mod codegen;
mod data;
mod ir;
mod json;
mod syscall;

fn main() {
    let code: Code =
        serde_json::from_str(include_str!("../examples/helloworld/code.json")).unwrap();
    let st: JsonSt = serde_json::from_str(include_str!("../examples/helloworld/st.json")).unwrap();
    // let mut lit = LiteralAllocator::new(0);
    // let mut regs = RegAllocator::new();
    // codegen_code(&code, &st, &mut lit, &mut regs);
    let elf_hdr = Elf64EHdr::new(0, 0, 0, 0);
    let mut f = File::create("res.elf").unwrap();
    f.write_all(bytemuck::bytes_of(&elf_hdr)).unwrap();
    
}
