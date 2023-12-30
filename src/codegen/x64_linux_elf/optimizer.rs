use crate::data::St;

use super::ir::Instr;

pub struct Optimizer {}

impl Optimizer {
	pub const fn new() -> Self {
		Self {}
	}

	pub fn accept(&mut self, instr: Instr) {
		dbg!(instr);
	}
}