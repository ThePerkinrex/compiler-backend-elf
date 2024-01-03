use std::{fmt::Debug, hash::Hash, fs::File};

use crate::{
    codegen::generic::Codegen,
    data::St,
    json::{Expression, Statement},
};

use super::{
    ir::{
        syscall::{ARG_REGISTERS, SYSCALL_REG},
        Constant, Instr, RegAllocation, RegisterAllocator,
    },
    optimizer::Optimizer, elf::ElfFileBuilder,
};

pub const PAGE_SIZE: u64 = 4096; // 4kb

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum LabelId<StEntryId> {
    StLabel(StEntryId),
    CustomLabel(usize),
    DataAddr(u64),
}

pub struct X64LinuxElfCodegen<SE: PartialEq + Eq + Hash, S: St<StEntryId = SE>> {
    data: Vec<u8>,
    opt: Optimizer<LabelId<S::StEntryId>>,
    st: S,
    registers: RegisterAllocator,
    main: Option<LabelId<S::StEntryId>>,
    data_labels: Vec<u64>,
}

impl<SE: PartialEq + Eq + Hash + Debug, S: St<StEntryId = SE>> X64LinuxElfCodegen<SE, S> {
    pub fn new(st: S) -> Self {
        Self {
            data: Vec::new(),
            opt: Optimizer::new(0x10000),
            st,
            registers: RegisterAllocator::new(),
            main: None,
            data_labels: Vec::new(),
        }
    }

    fn gen_expression(&mut self, expr: Expression) -> RegAllocation {
        match expr {
            Expression::IntConst { val } => {
                let reg = self.registers.allocate_any();
                self.opt
                    .accept(Instr::SetConstant(reg.reg(), Constant::Value(val)));
                reg
            }
            Expression::StrConst { val } => {
                let reg = self.registers.allocate_any();
                let addr = self.data.len() as u64;
                self.data_labels.push(addr);
                self.opt.accept(Instr::SetConstant(
                    reg.reg(),
                    Constant::Tbd(LabelId::DataAddr(addr)),
                ));
                self.data.extend_from_slice(val.as_bytes());
                self.data.push(0);
                reg
            }
        }
    }

    fn free(&mut self, reg: RegAllocation) {
        self.opt.accept(Instr::FreeRegister(reg.reg()));
        self.registers.free(reg);
    }
}

impl<SE: PartialEq + Eq + Hash + Clone + Copy + Debug, S: St<StEntryId = SE>> Codegen<S>
    for X64LinuxElfCodegen<SE, S>
{
    fn enter_fn(&mut self, entry: SE) {
        let label = LabelId::StLabel(entry);
        self.opt.label(label);
        let entry = self.st.get(entry);
        if entry.lexeme == "main" {
            self.main = Some(label);
        }
    }

    fn exit_fn(&mut self) {
    }

    fn gen_statement(&mut self, statement: Statement) {
        match statement {
            Statement::Syscall { syscall, args } => {
                let mut reg = self.gen_expression(syscall);
                if reg != SYSCALL_REG {
                    let old_reg = reg;
                    reg = self.registers.allocate(SYSCALL_REG);
                    self.opt.accept(Instr::MoveRegs {
                        dest: reg.reg(),
                        orig: old_reg.reg(),
                    });
                    self.free(old_reg);
                }
                let regs = args
                    .into_iter()
                    .enumerate()
                    .map(|(i, expr)| {
                        let expected = ARG_REGISTERS[i];
                        let gotten = self.gen_expression(expr);
                        if gotten == expected {
                            gotten
                        } else {
                            let expected = self.registers.allocate(expected);
                            self.opt.accept(Instr::MoveRegs {
                                dest: expected.reg(),
                                orig: gotten.reg(),
                            });
                            self.free(gotten);
                            expected
                        }
                    })
                    .collect::<Vec<_>>();
                self.opt.accept(Instr::Syscall);
                self.free(reg);
                for reg in regs {
                    self.free(reg);
                }
                // TODO Keep Return registers if needed
            }
        }
    }

    fn finish(mut self) {
		let init_addr = self.opt.get_init_addr();
		let mut data_dir = init_addr + self.opt.get_code_len() as u64;
		if data_dir % PAGE_SIZE != 0 {
			data_dir = data_dir + PAGE_SIZE - (data_dir % PAGE_SIZE);
		}
		for lbl in self.data_labels {
			self.opt.add_label(LabelId::DataAddr(lbl), lbl + data_dir);
		}
		let entrypoint = self.main.and_then(|lbl| self.opt.get_label(&lbl)).unwrap();
        let code = self.opt.apply_relocs();
		let mut elf_hdr = ElfFileBuilder::new();
		elf_hdr.add_code_segment(code, init_addr);
		elf_hdr.add_rodata_segment(self.data, data_dir, 4);
		elf_hdr.set_entrypoint(entrypoint);
		let mut f = File::create("res.elf").unwrap();
		elf_hdr.build(&mut f).unwrap();
    }
}
