use std::{collections::HashMap, fmt::Debug, hash::Hash};

use crate::codegen::x64_linux_elf::{x86_64_asm::{syscall, mov_regs, mov_const}, ir::Constant};

use super::ir::Instr;

pub struct Optimizer<LblId: Eq + Hash> {
	init_addr: u64,
    relocations: HashMap<LblId, Vec<usize>>,
    labels: HashMap<LblId, u64>,
    code: Vec<u8>,
}

impl<LblId: Eq + Hash + Debug> Optimizer<LblId> {
    pub fn new(init_addr: u64) -> Self {
        Self {
			init_addr,
            labels: HashMap::new(),
            code: Vec::new(),
            relocations: HashMap::new(),
        }
    }

    pub fn label(&mut self, label: LblId) {
        self.labels.insert(label, self.init_addr + self.code.len() as u64);
    }

    pub fn accept(&mut self, instr: Instr<LblId>) {
		match instr {
				Instr::SetConstant(dest, c) => {
					let val = match c {
						Constant::Value(v) => v,
						Constant::Tbd(lbl) => {
							let idx = self.get_code_len() + 2;
							self.relocations.entry(lbl).or_default().push(idx);
							0
						},
					};
					mov_const(dest, val, &mut self.code).unwrap();
				},
				Instr::MoveRegs { dest, orig } => mov_regs(dest, orig, &mut self.code).unwrap(),
				Instr::FreeRegister(_) => (),
				Instr::Syscall => syscall(&mut self.code).unwrap(),
			}
    }

    pub fn add_label(&mut self, label: LblId, addr: u64) {
        self.labels.insert(label, addr);
    }

	pub const fn get_init_addr(&self) -> u64 {
		self.init_addr
	}

    pub fn get_code_len(&self) -> usize {
        self.code.len()
    }

	pub fn get_label(&self, lbl: &LblId) -> Option<u64> {
		self.labels.get(lbl).copied()
	}

    pub fn apply_relocs(mut self) -> Vec<u8> {
        for (label, addr) in self.labels {
			println!("Relocating {label:?} with {addr:x}");
			for code_idx in self.relocations.get(&label).into_iter().flat_map(|x| x.iter().copied()) {
				self.code[code_idx..(code_idx+8)].copy_from_slice(&addr.to_ne_bytes());
			}
		}
		self.code
    }
}
