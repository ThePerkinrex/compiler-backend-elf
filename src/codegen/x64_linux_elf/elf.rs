use bitflags::bitflags;
use bytemuck::{Pod, Zeroable};
use std::{io::Write, mem::size_of};

const EI_NIDENT: usize = 16;

type Elf64Addr = u64;
type Elf64Off = u64;

const ELF_MAGIC: [u8; 4] = [0x7f, b'E', b'L', b'F'];

#[derive(Debug, bytemuck::TransparentWrapper, Pod, Zeroable, Clone, Copy)]
#[repr(transparent)]
pub struct Elf64Ident([u8; EI_NIDENT]);
impl Elf64Ident {
    pub fn new() -> Self {
        let mut data = [0; EI_NIDENT];
        data[0..4].copy_from_slice(&ELF_MAGIC);
        data[4] = 2; // CLASS64
        data[5] = 1; // LSB
        data[6] = 1; // VERSION = CURRENT
        data[7] = 3; // LINUX
                     // PADDING
        Self(data)
    }
}

#[derive(Debug, Pod, Zeroable, Clone, Copy)]
#[repr(C)]
pub struct Elf64EHdr {
    /// This  array of bytes specifies how to interpret the file, independent of the processor or the file's
    /// remaining contents.  Within this array everything is named by macros, which start  with  the  prefix
    /// EI_ and may contain values which start with the prefix ELF.  The following macros are defined:
    ///
    ///EI_MAG0  The first byte of the magic number.  It must be filled with ELFMAG0.  (0: 0x7f)
    ///
    ///EI_MAG1  The second byte of the magic number.  It must be filled with ELFMAG1.  (1: 'E')
    ///
    ///EI_MAG2  The third byte of the magic number.  It must be filled with ELFMAG2.  (2: 'L')
    ///
    ///EI_MAG3  The fourth byte of the magic number.  It must be filled with ELFMAG3.  (3: 'F')
    ///
    ///EI_CLASS The fifth byte identifies the architecture for this binary:
    ///
    ///         ELFCLASSNONE  This class is invalid.
    ///         ELFCLASS32    This  defines  the  32-bit architecture.  It supports machines with files and
    ///                       virtual address spaces up to 4 Gigabytes.
    ///         ELFCLASS64    This defines the 64-bit architecture.
    ///
    ///EI_DATA  The sixth byte specifies the data encoding of the  processor-specific  data  in  the  file.
    ///         Currently, these encodings are supported:
    ///
    ///         ELFDATANONE   Unknown data format.
    ///         ELFDATA2LSB   Two's complement, little-endian.
    ///         ELFDATA2MSB   Two's complement, big-endian.
    ///
    ///EI_VERSION
    ///         The seventh byte is the version number of the ELF specification:
    ///
    ///         EV_NONE       Invalid version.
    ///         EV_CURRENT    Current version.
    ///
    ///EI_OSABI The  eighth  byte  identifies the operating system and ABI to which the object is targeted.
    ///         Some fields in other ELF structures have flags and values that have platform-specific mean‐
    ///         ings;  the interpretation of those fields is determined by the value of this byte.  For ex‐
    ///         ample:
    ///
    ///         ELFOSABI_NONE        Same as ELFOSABI_SYSV
    ///         ELFOSABI_SYSV        UNIX System V ABI
    ///         ELFOSABI_HPUX        HP-UX ABI
    ///         ELFOSABI_NETBSD      NetBSD ABI
    ///         ELFOSABI_LINUX       Linux ABI
    ///         ELFOSABI_SOLARIS     Solaris ABI
    ///         ELFOSABI_IRIX        IRIX ABI
    ///         ELFOSABI_FREEBSD     FreeBSD ABI
    ///         ELFOSABI_TRU64       TRU64 UNIX ABI
    ///         ELFOSABI_ARM         ARM architecture ABI
    ///         ELFOSABI_STANDALONE  Stand-alone (embedded) ABI
    ///
    ///EI_ABIVERSION
    ///         The ninth byte identifies the version of the ABI to which the  object  is  targeted.   This
    ///         field  is used to distinguish among incompatible versions of an ABI.  The interpretation of
    ///         this version number is dependent on the ABI identified by the EI_OSABI field.  Applications
    ///         conforming to this specification use the value 0.
    ///
    /// EI_PAD   Start  of  padding.   These  bytes  are reserved and set to zero.  Programs which read them
    ///          should ignore them.  The value for EI_PAD will change in the  future  if  currently  unused
    ///          bytes are given meanings.
    ///
    /// EI_NIDENT
    ///          The size of the e_ident array.
    e_ident: Elf64Ident,
    e_type: u16,
    e_machine: u16,
    e_version: u32,
    e_entry: Elf64Addr,
    e_phoff: Elf64Off,
    e_shoff: Elf64Off,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16,
}

impl Elf64EHdr {
    pub fn new(
        entrypoint: Elf64Addr,
        ph_offset: Elf64Off,
        ph_num: u16,
    ) -> Self {
        Self {
            e_ident: Elf64Ident::new(),
            e_type: 2,     // EXEC
            e_machine: 62, // X86_64
            e_version: 1,  // CURRENT
            e_entry: entrypoint,
            e_phoff: ph_offset,
            e_shoff: 0,
            e_flags: 0,
            e_ehsize: size_of::<Self>() as u16, // Elf header size in bytes
            e_phentsize: size_of::<PhEntry>() as u16,
            e_phnum: ph_num,
            e_shentsize: size_of::<ShEntry>() as u16,
            e_shnum: 0,
            e_shstrndx: 0,
        }
    }
}

#[derive(Debug, Pod, Zeroable, Clone, Copy)]
#[repr(C)]
pub struct PhEntry {
    p_type: u32,
    p_flags: u32,
    p_offset: Elf64Off,
    p_vaddr: Elf64Addr,
    p_paddr: Elf64Addr,
    p_filesz: u64,
    p_memsz: u64,
    p_align: u64,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PhFlags: u32 {
        /// The value `A`, at bit position `0`.
        const R = 0x4;
        /// The value `B`, at bit position `1`.
        const W = 0x2;
        /// The value `C`, at bit position `2`.
        const X = 0x1;

        /// The combination of `A`, `B`, and `C`.
        const RW = Self::R.bits() | Self::W.bits();
        const RX = Self::R.bits() | Self::X.bits();
    }
}

impl PhEntry {
    pub const fn new(
        offset: Elf64Off,
        vaddr: Elf64Addr,
        filesz: u64,
        memsz: u64,
        flags: PhFlags,
        align: u64,
    ) -> Self {
        Self {
            p_type: 1, // PT_LOAD
            p_flags: flags.bits(),
            p_offset: offset,
            p_vaddr: vaddr,
            p_paddr: 0,
            p_filesz: filesz,
            p_memsz: memsz,
            p_align: align, // Page size
        }
    }
}

#[derive(Debug, Pod, Zeroable, Clone, Copy)]
#[repr(C)]
pub struct ShEntry {
    sh_name: u32,
    sh_type: u32,
    sh_flags: u64,
    sh_addr: Elf64Addr,
    sh_offset: Elf64Off,
    sh_size: u64,
    sh_link: u32,
    sh_info: u32,
    sh_addralign: u64,
    sh_entsize: u64,
}

#[derive(Debug)]
pub struct ElfFileSegment {
    flags: PhFlags,
    data: Vec<u8>,
    memsz: u64,
    vaddr: Elf64Addr,
    align: u64,
}

#[derive(Debug, Default)]
pub struct ElfFileBuilder {
    segments: Vec<ElfFileSegment>,
    entrypoint: Elf64Addr,
}

impl ElfFileBuilder {
    pub const fn new() -> Self {
        Self {
            segments: Vec::new(),
            entrypoint: 0
        }
    }

    pub fn add_code_segment(&mut self, code: Vec<u8>, vaddr: Elf64Addr) {
        self.segments.push(ElfFileSegment {
            flags: PhFlags::RX,
            memsz: code.len() as u64,
            data: code,
            vaddr,
            align: 16,
        })
    }

    pub fn add_rodata_segment(&mut self, data: Vec<u8>, vaddr: Elf64Addr, align: u64) {
        self.segments.push(ElfFileSegment {
            flags: PhFlags::R,
            memsz: data.len() as u64,
            data,
            vaddr,
            align,
        })
    }

    pub fn add_data_segment(&mut self, data: Vec<u8>, vaddr: Elf64Addr, align: u64) {
        self.segments.push(ElfFileSegment {
            flags: PhFlags::RW,
            memsz: data.len() as u64,
            data,
            vaddr,
            align,
        })
    }

    pub fn add_unint_segment(&mut self, vaddr: Elf64Addr, memsz: u64, align: u64) {
        self.segments.push(ElfFileSegment {
            flags: PhFlags::RW,
            data: Vec::new(),
            memsz,
            vaddr,
            align,
        })
    }

    pub fn set_entrypoint(&mut self, entrypoint: Elf64Addr) {
        self.entrypoint = entrypoint;
    }

    pub fn build<W: Write>(self, buf: &mut W) -> Result<(), std::io::Error> {
        let mut p_off = size_of::<Elf64EHdr>() as Elf64Off;
        let header = Elf64EHdr::new(self.entrypoint, p_off, self.segments.len() as u16);
        buf.write_all(bytemuck::bytes_of(&header))?;
        p_off += (size_of::<PhEntry>() * self.segments.len()) as Elf64Off;
        for (i, segment) in self.segments.iter().enumerate() {
            let filesz = segment.data.len() as u64;
            let ph_entry = PhEntry::new(p_off, segment.vaddr, filesz, segment.memsz, segment.flags, segment.align);
            buf.write_all(bytemuck::bytes_of(&ph_entry))?;
            p_off += segment.data.len() as Elf64Off;
        }
        for segment in self.segments.into_iter() {
            buf.write_all(&segment.data)?;
        }
        Ok(())
    }
}
