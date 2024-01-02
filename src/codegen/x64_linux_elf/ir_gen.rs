use crate::{data::St, codegen::generic::Codegen, json::{Statement, Expression}};

use super::{optimizer::{Optimizer, CodeLabel}, ir::{RegAllocation, RegisterAllocator, Instr, Constant, syscall::{SYSCALL_REG, ARG_REGISTERS}}};

pub struct X64LinuxElfCodegen<S> {
	data: Vec<u8>,
	opt: Optimizer,
	st: S,
	registers: RegisterAllocator,
	main: Option<CodeLabel>
}

impl<S> X64LinuxElfCodegen<S> {
	pub const fn new(st: S) -> Self {
		Self {
			data: Vec::new(),
			opt: Optimizer::new(),
			st,
			registers: RegisterAllocator::new(),
			main: None
		}
	}

	fn gen_expression(&mut self, expr: Expression) -> RegAllocation {
		match expr {
			Expression::IntConst { val } => {
				let reg = self.registers.allocate_any();
				self.opt.accept(Instr::SetConstant(reg.reg(), Constant::Value(val)));
				reg
			},
			Expression::StrConst { val } => {
				let reg = self.registers.allocate_any();
				
				self.opt.accept(Instr::SetConstant(reg.reg(), Constant::DataAddr(self.data.len() as u64)));
				self.data.extend_from_slice(val.as_bytes());
				self.data.push(0);
				reg
			},
		}
	}

	fn free(&mut self, reg: RegAllocation) {
		self.opt.accept(Instr::FreeRegister(reg.reg()));
		self.registers.free(reg);
	}
}

impl<S: St> Codegen<S> for X64LinuxElfCodegen<S> {
    fn enter_fn(&mut self, entry: <S as St>::StEntryId) {
		let entry = self.st.get(entry);
		let label = self.opt.label();
		if entry.lexeme == "main" {
			self.main = Some(label);
		}
        println!("Enter fn: {}", entry.lexeme)
    }

    fn exit_fn(&mut self) {
        println!("Exit fn")
    }

    fn gen_statement(&mut self, statement: Statement) {
        match statement {
            Statement::Syscall { syscall, args } => {
				let mut reg = self.gen_expression(syscall);
				if reg != SYSCALL_REG {
					let old_reg = reg;
					reg = self.registers.allocate(SYSCALL_REG);
					self.opt.accept(Instr::MoveRegs { dest: reg.reg(), orig: old_reg.reg() });
					self.free(old_reg);
				}
				let regs = args.into_iter().enumerate().map(|(i, expr)| {
					let expected = ARG_REGISTERS[i];
					let gotten = self.gen_expression(expr);
					if gotten == expected {gotten} else{
						let expected = self.registers.allocate(expected);
						self.opt.accept(Instr::MoveRegs { dest: expected.reg(), orig: gotten.reg() });
						self.free(gotten);
						expected
					}
				}).collect::<Vec<_>>();
				self.opt.accept(Instr::Syscall);
				self.free(reg);
				for reg in regs {
					self.free(reg);
				}
				// TODO Keep Return registers if needed
			},
        }
    }
}