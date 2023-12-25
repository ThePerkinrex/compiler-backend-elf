/// REGISTER NUMS:    0    1    2    3    4    5    6    7    8    9    10    11    12    13    14    15
/// 8 BYTE Registers: rax, rbx, rcx, rdx, rsi, rdi, rsp, rbp, r8,  r9,  r10,  r11,  r12,  r13,  r14,  r15
/// 4 BYTE Registers: eax, ebx, ecx, edx, esi, edi, esp, ebp, r8d, r9d, r10d, r11d, r12d, r13d, r14d, r15d
pub const ARG_REGISTERS: [u8; 6] = [5, 4, 3, 10, 8, 9];
pub const RETURN_REGISTERS: [u8; 2] = [0, 3];
pub const SYSCALL_REG: u8 = 0;
pub const REG_REPRESENTATIONS: &[&str] = &[
    "rax", "rbx", "rcx", "rdx", "rsi", "rdi", "rsp", "rbp", "r8", "r9", "r10", "r11", "r12", "r13",
    "r14", "r15",
];
