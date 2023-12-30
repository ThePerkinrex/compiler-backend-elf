use crate::{data::St, codegen::generic::Codegen, json::{Statement, Expression}};

use super::{optimizer::Optimizer, ir::{RegisterProtectedAllocator, Register}};

pub struct X64LinuxElfCodegen<S> {
	data: Vec<u8>,
	opt: Optimizer,
	st: S,
}

impl<S> X64LinuxElfCodegen<S> {
	pub const fn new(st: S) -> Self {
		Self {
			data: Vec::new(),
			opt: Optimizer::new(),
			st,
		}
	}

	fn gen_expression(&mut self, available_regs: &mut RegisterProtectedAllocator, expr: Expression) -> Register {
		todo!()
	}
}

impl<S: St> Codegen<S> for X64LinuxElfCodegen<S> {
    fn enter_fn(&mut self, entry: <S as St>::StEntryId) {
        todo!()
    }

    fn exit_fn(&mut self) {
        todo!()
    }

    fn gen_statement(&mut self, statement: Statement) {
        match statement {
            Statement::Syscall { syscall, args } => {
				
			},
        }
    }
}