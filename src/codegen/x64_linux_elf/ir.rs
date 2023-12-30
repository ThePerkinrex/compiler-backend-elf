#[derive(Debug)]
pub struct Register(u8);

pub struct RegisterAllocator {
	next_available: u8
}

pub struct RegisterProtectedAllocator<'a> {
    alloc: &'a mut RegisterAllocator
}

impl RegisterAllocator {
    pub const fn new() -> Self {
        Self {next_available: 0}
    }

    pub fn clear(&mut self) {
        self.next_available = 0;
    }

    pub fn protected(&mut self) -> RegisterProtectedAllocator<'_> {
        RegisterProtectedAllocator{
            alloc: self
        }
    }
}
impl<'a> RegisterProtectedAllocator<'a> {
    pub fn allocate(&mut self) -> Register {
        let reg = Register(self.alloc.next_available);
        self.alloc.next_available += 1;
        reg
    }
}

#[derive(Debug)]
pub enum Constant {
    Value(u64),
}

#[derive(Debug)]
pub enum Instr {
    SetConstant(Register, Constant),
    Syscall,
}
