use self::syscall::REG_REPRESENTATIONS;


pub mod syscall;

type InternalRegister = u8;

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RegAllocation(InternalRegister);
#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Register(InternalRegister);

impl RegAllocation {
    pub const fn repr(&self) -> &'static str {
        REG_REPRESENTATIONS[self.0 as usize]
    }

    pub const fn reg(&self) -> Register {
        Register(self.0)
    }
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct RegisterRequest(InternalRegister);

impl PartialEq<RegisterRequest> for RegAllocation {
    fn eq(&self, other: &RegisterRequest) -> bool {
        self.0 == other.0
    }
}

impl PartialEq<RegAllocation> for RegisterRequest {
    fn eq(&self, other: &RegAllocation) -> bool {
        self.0 == other.0
    }
}

impl PartialEq<RegisterRequest> for Register {
    fn eq(&self, other: &RegisterRequest) -> bool {
        self.0 == other.0
    }
}

impl PartialEq<Register> for RegisterRequest {
    fn eq(&self, other: &Register) -> bool {
        self.0 == other.0
    }
}

impl PartialEq<Register> for RegAllocation {
    fn eq(&self, other: &Register) -> bool {
        self.0 == other.0
    }
}

impl PartialEq<RegAllocation> for Register {
    fn eq(&self, other: &RegAllocation) -> bool {
        self.0 == other.0
    }
}

pub struct RegisterAllocator {
	available: u16
}
impl RegisterAllocator {
    pub const fn new() -> Self {
        Self {available: u16::MAX}
    }

    pub fn free(&mut self, reg: RegAllocation) {
        self.available ^= 1 << reg.0;
    }

    pub fn allocate_any(&mut self) -> RegAllocation {
        let mut mask = 1;
        for i in 0..16u8 {
            if self.available & mask != 0 {
                self.available ^= mask;
                return RegAllocation(i);
            }
            mask <<= 1;
        }
        panic!("No registers available")
    }

    pub fn allocate(&mut self, req: RegisterRequest) -> RegAllocation {
        let mask = 1 << req.0;
        if self.available & mask != 0 {
            self.available ^= mask;
            return RegAllocation(req.0);
        }
        panic!("No registers available")
    }
}

#[derive(Debug)]
pub enum Constant {
    Value(u64),
    DataAddr(u64)
}

#[derive(Debug)]
pub enum Instr {
    SetConstant(Register, Constant),
    MoveRegs{dest: Register, orig: Register},
    FreeRegister(Register),
    Syscall,
}
