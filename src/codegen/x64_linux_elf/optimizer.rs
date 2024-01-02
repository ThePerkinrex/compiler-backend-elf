use super::ir::Instr;

pub struct CodeLabel(usize);

pub struct Optimizer {
	labels: Vec<u64>
}

impl Optimizer {
	pub const fn new() -> Self {
		Self {labels: Vec::new()}
	}

	pub fn label(&mut self) -> CodeLabel {
		let label = CodeLabel(self.labels.len());
		self.labels.push(0 /* TODO current address */);
		label
	}

	pub fn accept(&mut self, instr: Instr) {
		println!("\t{instr:?}");
	}
}