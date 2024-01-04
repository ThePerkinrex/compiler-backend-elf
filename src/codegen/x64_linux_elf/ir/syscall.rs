use super::{RegisterRequest, Register};

macro_rules! reg_list {
    [$($reg:expr),*] => {
        [$(RegisterRequest($reg)),*]
    };
}

/// REGISTER NUMS:    0    1    2    3    4    5    6    7    8    9    10    11    12    13    14    15
/// 8 BYTE Registers: rax, rcx, rdx, rbx, rsp, rbp, rsi, rdi, r8,  r9,  r10,  r11,  r12,  r13,  r14,  r15
pub const ARG_REGISTERS: [RegisterRequest; 6] = reg_list![7, 6, 2, 10, 8, 9];
pub const RETURN_REGISTERS: [RegisterRequest; 2] = reg_list![0, 1];
pub const SYSCALL_REG: RegisterRequest = RegisterRequest(0);
pub const REG_REPRESENTATIONS: [&str; 16] = [
    "rax", "rcx", "rdx", "rbx", "rsp", "rbp", "rsi", "rdi", "r8", "r9", "r10", "r11", "r12", "r13",
    "r14", "r15",
];

pub const RETURN_REG: RegisterRequest = RegisterRequest(0);
pub const STACK_FRAME_POINTER: Register = Register(5);
pub const STACK_POINTER: Register = Register(4);
