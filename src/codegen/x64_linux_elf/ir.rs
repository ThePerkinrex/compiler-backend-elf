use bitflags::bitflags;

use self::syscall::REG_REPRESENTATIONS;

pub mod syscall;

type InternalRegister = u8;

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RegAllocation(InternalRegister);
#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Register(pub InternalRegister);

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
pub struct RegisterRequest(pub InternalRegister);

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

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct RegisterAllocatorInternal: u16 {
        const rax = 1 << 0;
        const rcx = 1 << 1;
        const rdx = 1 << 2;
        const rbx = 1 << 3;
        const rsp = 1 << 4;
        const rbp = 1 << 5;
        const rsi = 1 << 6;
        const rdi = 1 << 7;
        const r8  = 1 << 8;
        const r9  = 1 << 9;
        const r10 = 1 << 10;
        const r11 = 1 << 11;
        const r12 = 1 << 12;
        const r13 = 1 << 13;
        const r14 = 1 << 14;
        const r15 = 1 << 15;
    }
}

pub struct RegisterAllocator {
    available: RegisterAllocatorInternal,
}
impl RegisterAllocator {
    pub fn new() -> Self {
        Self {
            available: !(RegisterAllocatorInternal::rsp | RegisterAllocatorInternal::rbp),
        }
    }

    pub fn free(&mut self, reg: RegAllocation) {
        self.available ^= RegisterAllocatorInternal::from_bits_truncate(1 << reg.0);
    }

    pub fn allocate_any(&mut self) -> RegAllocation {
        let mut mask = 1;
        for i in 0..16u8 {
            {
                let mask = RegisterAllocatorInternal::from_bits_truncate(mask);
                if !((self.available & mask).is_empty()) {
                    self.available ^= mask;
                    return RegAllocation(i);
                }
            }
            mask <<= 1;
        }
        panic!("No registers available")
    }

    pub fn allocate(&mut self, req: RegisterRequest) -> RegAllocation {
        // println!("Requesting register {} ({})", req.0, REG_REPRESENTATIONS[req.0 as usize]);
        let mask = RegisterAllocatorInternal::from_bits_truncate(1 << req.0);
        if !((self.available & mask).is_empty()) {
            self.available ^= mask;
            return RegAllocation(req.0);
        }
        panic!(
            "Register {} ({}) isn't available",
            req.0, REG_REPRESENTATIONS[req.0 as usize]
        )
    }
}

#[derive(Debug)]
pub enum Constant<Lbl> {
    Value(u64),
    Tbd(Lbl),
}

#[derive(Debug)]
pub enum Instr<Lbl> {
    SetConstant(Register, Constant<Lbl>),
    MoveRegs { dest: Register, orig: Register },
    Push(Register),
    Pop(Register),
    FreeRegister(Register),
    Syscall,
    Ret,
    Call(Lbl)
}
