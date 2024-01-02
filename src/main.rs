use std::{fs::File, io::Write};

use codegen::x64_linux_elf::{elf::{Elf64EHdr, ElfFileBuilder}, ir_gen::X64LinuxElfCodegen};
// use codegen::{LiteralAllocator, RegAllocator, codegen_body, codegen_code};
use json::{Code, JsonSt, run};

mod codegen;
mod data;
mod json;

fn main() {
    let code: Code =
        serde_json::from_str(include_str!("../examples/helloworld/code.json")).unwrap();
    let st: JsonSt = serde_json::from_str(include_str!("../examples/helloworld/st.json")).unwrap();
    let codegen = X64LinuxElfCodegen::new(st);
    run(code, codegen);
    // let mut lit = LiteralAllocator::new(0);
    // let mut regs = RegAllocator::new();
    // codegen_code(&code, &st, &mut lit, &mut regs);
    let mut elf_hdr = ElfFileBuilder::new();
    elf_hdr.add_data_segment(b"hola mundo!\0".to_vec(), 0, 4);
    let mut f = File::create("res.elf").unwrap();
    elf_hdr.build(&mut f).unwrap();
}
