use std::{fmt::Debug, fs::File, hash::Hash};

use crate::{
    codegen::generic::Codegen,
    data::St,
    json::{Expression, Statement},
};

use super::{
    elf::ElfFileBuilder,
    ir::{
        syscall::{ARG_REGISTERS, SYSCALL_REG, STACK_FRAME_POINTER, STACK_POINTER},
        Constant, Instr, RegAllocation, RegisterAllocator, RegisterRequest, Register,
    },
    optimizer::Optimizer,
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

    fn function_enter_harness(&mut self) {
        // push rbp // push stack frame
        // mov rbp, rsp // save new frame
        self.opt.accept(Instr::Push(STACK_FRAME_POINTER));
        self.opt.accept(Instr::MoveRegs { dest: STACK_FRAME_POINTER, orig: STACK_POINTER });
    }

    fn function_exit_harness(&mut self) {
        // mov rsp, rbp ; go to stack frame start
        // pop rbp ; pop the previous stack frame
        // ret
        self.opt.accept(Instr::MoveRegs { dest: STACK_POINTER, orig: STACK_FRAME_POINTER });
        self.opt.accept(Instr::Pop(STACK_FRAME_POINTER));
        self.opt.accept(Instr::Ret)

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
        self.function_enter_harness()
    }

    fn exit_fn(&mut self) {
        
    }

    fn gen_statement(&mut self, statement: Statement) {
        match statement {
            Statement::Syscall { syscall, args } => {
                let mut reg = self.gen_expression(syscall);
                println!("{reg:?} != {SYSCALL_REG:?} is {}", reg != SYSCALL_REG);
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
            Statement::Return { inner } => {
                if let Some(expr) = inner {
                    let reg = self.gen_expression(expr);
                    let rax = RegisterRequest(0);
                    if reg != rax {
                        let rax = self.registers.allocate(rax);
                        self.opt.accept(Instr::MoveRegs {
                            dest: rax.reg(),
                            orig: reg.reg(),
                        });
                        self.free(reg);
                    }
                }
                self.function_exit_harness()
            }
        }
    }

    fn finish(mut self) {
        let mut elf_hdr = ElfFileBuilder::new();
        let mut entrypoint = None;
        // Add run harness
        if let Some(main) = self.main {
            let start = LabelId::CustomLabel(0); // TODO Allocate a label
            self.opt.label(start);
            entrypoint = Some(start);
            self.opt.accept(Instr::Call(main));
            let arg = self.registers.allocate(ARG_REGISTERS[0]);
            self.opt.accept(Instr::MoveRegs { dest: arg.reg(), orig: Register(0) });
            self.opt.accept(Instr::SetConstant(Register(0), Constant::Value(60)));
            self.opt.accept(Instr::Syscall);
        }

        let init_addr = self.opt.get_init_addr();
        let mut data_dir = init_addr + self.opt.get_code_len() as u64;
        if data_dir % PAGE_SIZE != 0 {
            data_dir = data_dir + PAGE_SIZE - (data_dir % PAGE_SIZE);
        }
        for lbl in self.data_labels {
            self.opt.add_label(LabelId::DataAddr(lbl), lbl + data_dir);
        }
        let entrypoint = entrypoint.and_then(|lbl| self.opt.get_label(&lbl)).unwrap();
        elf_hdr.set_entrypoint(entrypoint);
        let code = self.opt.apply_relocs();
        elf_hdr.add_code_segment(code, init_addr);
        elf_hdr.add_rodata_segment(self.data, data_dir, 4);
        let mut f = File::create("res.elf").unwrap();
        elf_hdr.build(&mut f).unwrap();
    }
}
