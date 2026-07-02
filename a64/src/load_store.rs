//! Loads and Stores

use core::fmt::Display;

use a64_macros::bit_match;
use bitos::integer::{i7, u12};
use bitos::{BitUtils, bitos};
use derive_more::Display;

use crate::{Reg, RegSp, RegWidth};

/// Memory operation.
#[bitos(1)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Store = 0b0,
    Load = 0b1,
}

/// Data size.
#[bitos(2)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataSize {
    B8 = 0b00,
    B16 = 0b01,
    B32 = 0b10,
    B64 = 0b11,
}

impl DataSize {
    #[inline(always)]
    pub fn bits(self) -> u32 {
        match self {
            Self::B8 => 8,
            Self::B16 => 16,
            Self::B32 => 32,
            Self::B64 => 64,
        }
    }

    #[inline(always)]
    pub fn bytes(self) -> u32 {
        match self {
            Self::B8 => 1,
            Self::B16 => 2,
            Self::B32 => 4,
            Self::B64 => 8,
        }
    }
}

/// Kind of offseting done in a memory operation.
#[bitos(2)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OffsetKind {
    /// The memory access address is `reg + offset`, and this access is non-temporal.
    OffsetNonTemporal = 0b00,
    /// The memory access address is `reg` and `reg + offset` is written back to `reg`.
    PostIndexed = 0b01,
    /// The memory access address is `reg + offset`.
    Offset = 0b10,
    /// The memory access address is `reg + offset` and `reg + offset` is written back to `reg`.
    PreIndexed = 0b11,
}

/// Load/store pair of registers
///
/// This instruction calculates an address from a base register value and an immediate offset and
/// loads/stores two 32-bit words or two 64-bit doublewords from/to memory.
#[bitos(32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pair {
    /// First general-purpose register to be transferred.
    #[bits(0..5)]
    pub rt1: Reg,
    /// The general-purpose base register.
    #[bits(5..10)]
    pub rn: RegSp,
    /// Second general-purpose register to be transferred.
    #[bits(10..15)]
    pub rt2: Reg,
    /// Offset divided by register width.
    #[bits(15..22)]
    pub imm: i7,
    /// Operation to perform.
    #[bits(22)]
    pub op: Operation,
    /// Offset kind.
    #[bits(23..25)]
    pub offset_kind: OffsetKind,
    /// Width of the registers.
    #[bits(31)]
    pub sf: RegWidth,
}

impl Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mnemonic = match (
            self.op(),
            self.offset_kind() == OffsetKind::OffsetNonTemporal,
        ) {
            (Operation::Store, true) => "STNP",
            (Operation::Store, false) => "STP",
            (Operation::Load, true) => "LDNP",
            (Operation::Load, false) => "LDP",
        };

        let base = self.rn().with_width(self.sf());
        let imm = self.imm().value() as i16 * self.sf().bytes() as i16;

        let offset = match self.offset_kind() {
            OffsetKind::Offset | OffsetKind::OffsetNonTemporal => &format!("[{base}, #{imm}]"),
            OffsetKind::PreIndexed => &format!("[{base}, #{imm}]!"),
            OffsetKind::PostIndexed => &format!("[{base}], #{imm}"),
        };

        write!(
            f,
            "{} {}, {}, {}",
            mnemonic,
            self.rt1().with_width(self.sf()),
            self.rt2().with_width(self.sf()),
            offset,
        )
    }
}

/// Operation performed in an [`UnsignedImm`] instruction.
#[bitos(2)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnsignedImmOp {
    /// Store to memory.
    Store = 0b00,
    /// Load from memory and zero-extend.
    LoadZext = 0b01,
    /// Load from memory and sign extended to 64 bits (XR).
    LoadSext64 = 0b10,
    /// Load from memory and sign extended to 32 bits (WR).
    LoadSext32 = 0b11,
}

/// Load/store register (unsigned immediate)
///
/// This instruction loads/stores data of a register to/from memory. The address that is used for
/// the operation is calculated from a base register and an immediate offset.
#[bitos(32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnsignedImm {
    /// The general-purpose register to be transferred.
    #[bits(0..5)]
    pub rt: Reg,
    /// The general-purpose base register.
    #[bits(5..10)]
    pub rn: RegSp,
    /// Offset divided by data size.
    #[bits(10..22)]
    pub imm: u12,
    /// Operation to perform.
    #[bits(22..24)]
    pub op: UnsignedImmOp,
    /// Data size.
    #[bits(30..32)]
    pub size: DataSize,
}

impl UnsignedImm {
    pub fn width(self) -> RegWidth {
        match (self.op(), self.size()) {
            (UnsignedImmOp::Store, DataSize::B8) => RegWidth::W32,
            (UnsignedImmOp::Store, DataSize::B16) => RegWidth::W32,
            (UnsignedImmOp::Store, DataSize::B32) => RegWidth::W32,
            (UnsignedImmOp::Store, DataSize::B64) => RegWidth::X64,
            (UnsignedImmOp::LoadZext, DataSize::B8) => RegWidth::W32,
            (UnsignedImmOp::LoadZext, DataSize::B16) => RegWidth::W32,
            (UnsignedImmOp::LoadZext, DataSize::B32) => RegWidth::W32,
            (UnsignedImmOp::LoadZext, DataSize::B64) => RegWidth::X64,
            (UnsignedImmOp::LoadSext64, _) => RegWidth::X64,
            (UnsignedImmOp::LoadSext32, _) => RegWidth::W32,
        }
    }
}

impl Display for UnsignedImm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = self.width();
        let mnemonic = match (self.op(), self.size()) {
            (UnsignedImmOp::Store, DataSize::B8) => "STRB",
            (UnsignedImmOp::Store, DataSize::B16) => "STRH",
            (UnsignedImmOp::Store, DataSize::B32) => "STR",
            (UnsignedImmOp::Store, DataSize::B64) => "STR",
            (UnsignedImmOp::LoadZext, DataSize::B8) => "LDRB",
            (UnsignedImmOp::LoadZext, DataSize::B16) => "LDRH",
            (UnsignedImmOp::LoadZext, DataSize::B32) => "LDR",
            (UnsignedImmOp::LoadZext, DataSize::B64) => "LDR",
            (UnsignedImmOp::LoadSext64, DataSize::B8) => "LDRSB",
            (UnsignedImmOp::LoadSext64, DataSize::B16) => "LDRSH",
            (UnsignedImmOp::LoadSext64, DataSize::B32) => "LDRSH",
            (UnsignedImmOp::LoadSext64, DataSize::B64) => "LDR",
            (UnsignedImmOp::LoadSext32, DataSize::B8) => "LDRB",
            (UnsignedImmOp::LoadSext32, DataSize::B16) => "LDRH",
            (UnsignedImmOp::LoadSext32, DataSize::B32) => "LDR",
            (UnsignedImmOp::LoadSext32, DataSize::B64) => "????",
        };

        write!(
            f,
            "{} {}, [{}, #{}]",
            mnemonic,
            self.rt().with_width(width),
            self.rn().with_width(RegWidth::X64),
            self.imm().value() as u32 * self.size().bytes()
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum Instruction {
    Pair(Pair),
    UnsignedImm(UnsignedImm),
}

impl Instruction {
    fn new_pair(value: u32) -> Option<Self> {
        let opc = value.bits(30, 32);
        let vr = value.bit(26) as u32;
        let temporal = value.bits(23, 25) != OffsetKind::OffsetNonTemporal as u32;

        Some(bit_match! {
            match (opc, vr) {
                ("00", "0") => Self::Pair(Pair(value)),
                ("00", "1") => todo!("simd/fp 32 bit"),

                ("01", "0") if temporal => todo!("stgp/ldpsw"),
                ("01", "1") => todo!("simd/fp 64 bit"),

                ("10", "0") => Self::Pair(Pair(value)),
                ("10", "1") => todo!("simd/fp 128 bit"),

                ("11", "0") => todo!("sttp/ldtp"),
                ("11", "1") => todo!("sttp/ldtp simd/fp"),
                _ => return None,
            }
        })
    }

    fn new_uimm(value: u32) -> Option<Self> {
        let size = value.bits(30, 32);
        let vr = value.bit(26) as u32;
        let opc = value.bits(22, 24);

        Some(bit_match! {
            match (size, vr, opc) {
                // 8 bit
                ("00", "0", "__") => Self::UnsignedImm(UnsignedImm(value)),
                ("00", "1", "__") => todo!("simd 8 bit/128 bit"),

                // unallocated
                ("1_", "0", "11") => return None,
                ("__", "1", "1_") => return None,

                // 16 bit
                ("01", "0", "__") => Self::UnsignedImm(UnsignedImm(value)),
                ("01", "1", "__") => todo!("simd 16 bit"),

                // 32 bit
                ("10", "0", "__") => Self::UnsignedImm(UnsignedImm(value)),
                ("10", "1", "__") => todo!("simd 32 bit"),

                // prefetch
                ("11", "0", "10") => todo!("prefetch"),

                // 64 bit
                ("11", "0", "__") => Self::UnsignedImm(UnsignedImm(value)),
                ("11", "1", "__") => todo!("simd 64 bit"),

                _ => return None,
            }
        })
    }

    pub fn new(value: u32) -> Option<Self> {
        let op0 = value.bits(28, 32);
        let op1 = value.bit(26) as u32;

        // op2: 10..25, split to generate better code with bit_match
        let op2_a = value.bits(12, 25);
        let op2_b = value.bits(10, 12);

        Some(bit_match! {
            match (op0, op1, op2_a, op2_b) {
                ("0_00", "0", "00_1_________", "__") => todo!("compare and swap pair"),
                ("0_00", "0", "11_0_________", "__") => todo!("compare and swap pair (unprivileged)"),
                ("0_00", "1", "00_000000____", "__") => todo!("adv simd multiple structures"),
                ("0_00", "1", "01_0_________", "__") => todo!("adv simd multiple structures (post index)"),
                ("0_00", "1", "10__0000_____", "__") => todo!("adv simd single structure"),
                ("0_00", "1", "11___________", "__") => todo!("adv simd single structure (post index)"),

                ("0_01", "0", "1__1_____0000", "10") => todo!("rcw compare and swap"),
                ("0_01", "0", "1__1_____0000", "11") => todo!("rcw compare and swap pair"),
                ("0_01", "0", "1__1_________", "00") => todo!("128 bit atomic mem ops"),
                ("0_01", "0", "1__1_________", "01") => todo!("atomic mem ops (unprivileged)"),

                ("1101", "0", "1000111110___", "11") => todo!("gcs load/store"),
                ("1101", "0", "1__1_________", "__") => todo!("load/store mem tags"),

                ("1_00", "0", "00_1_________", "__") => todo!("load/store exclusive pair"),
                ("1_00", "0", "10_0_________", "__") => todo!("load/store exclusive reg (unprivileged)"),
                ("1_00", "0", "11_0_________", "__") => todo!("compare and swap (unprivileged)"),

                ("__00", "0", "00_0_________", "__") => todo!("load/store exclusive reg"),
                ("__00", "0", "01_0_________", "__") => todo!("load/store ordered"),
                ("__00", "0", "01_1_________", "__") => todo!("compare and swap"),

                ("__01", "0", "10_0_________", "10") => todo!("load/store ordered register pair"),
                ("__01", "0", "11_0000000000", "10") => todo!("load/store ordered (writeback)"),
                ("__01", "0", "1__0_________", "00") => todo!("load/store ordered (unscaled imm)"),
                ("__01", "1", "1__0_________", "10") => todo!("load/store ordered (simd & fp)"),
                ("__01", "_", "0____________", "__") => todo!("load reg (literal)"),
                ("__01", "_", "1__0_________", "01") => todo!("memcopy and memset"),

                ("__10", "_", "_____________", "__") => Self::new_pair(value)?,

                ("__11", "_", "0__0_________", "00") => todo!("load/store reg (unscaled imm)"),
                ("__11", "_", "0__0_________", "01") => todo!("load/store reg (imm post indexed)"),
                ("__11", "_", "0__0_________", "10") => todo!("load/store reg (unprivileged)"),
                ("__11", "_", "0__0_________", "11") => todo!("load/store reg (imm pre indexed)"),
                ("__11", "_", "0__1_________", "00") => todo!("atomic mem ops"),
                ("__11", "_", "0__1_________", "10") => todo!("load/store reg (reg offset)"),
                ("__11", "_", "0__1_________", "_1") => todo!("load/store reg (pac)"),
                ("__11", "_", "1____________", "__") => Self::new_uimm(value)?,
                _ => return None,
            }
        })
    }
}
