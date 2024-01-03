use std::io::Write;

use bitflags::bitflags;

use super::ir::Register;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Rex: u8 {
        const W = 0b1000;
        const R = 0b0100;
        const X = 0b0010;
        const B = 0b0001;
    }
}

impl Rex {
    const fn as_rex(&self) -> u8 {
        0b0100_0000u8 | self.bits()
    }
}

pub fn mov_regs(dest: Register, origin: Register, buf: &mut impl Write) -> std::io::Result<()> {
    // REX.W + 89 /r
    // MOV r/m64, r64
    // Move r64 to r/m64
    let mut rex = Rex::W;
    if dest.0 >= 8 {
        rex |= Rex::R
    }
    if origin.0 >= 8 {
        rex |= Rex::B
    }
    let opcode = 0x89u8;
    let mod_rm = 0b11_000_000u8 | ((origin.0 & 0b111) << 3) | (dest.0 & 0b111);
    buf.write_all(&[rex.as_rex(), opcode, mod_rm])?;
    Ok(())
}

/// imm64 offset to start: 2 bytes
pub fn mov_const(dest: Register, origin: u64, buf: &mut impl Write) -> std::io::Result<()> {
    // REX.W + B8+ rd io
    // MOV r64, imm64
    // Move imm64 to r64.
    let mut rex = Rex::W;
    if dest.0 >= 8 {
        rex |= Rex::R
    }
    let opcode = 0xB8_u8 | (dest.0 & 0b111);
    buf.write_all(&[rex.as_rex(), opcode])?;
    buf.write_all(&origin.to_ne_bytes())?;
    Ok(())
}

pub fn syscall(buf: &mut impl Write) -> std::io::Result<()> {
    // 0F 05
    // SYSCALL
    // Fast call to privilege level 0 system procedures.
    buf.write_all(&[0x0F, 0x05])
}
