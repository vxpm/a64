//! Loads and Stores

use core::fmt::Display;

use a64_macros::bit_match;
use bitos::integer::{i7, i9, u3, u12};
use bitos::{BitUtils, bitos};
use derive_more::Display;

use crate::{DataSize, MemOp, MemOpExtended, Reg, RegWidth, SimdReg, SimdRegScalarKind, XrSp};

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
    pub rn: XrSp,
    /// Second general-purpose register to be transferred.
    #[bits(10..15)]
    pub rt2: Reg,
    /// Offset divided by register width.
    #[bits(15..22)]
    pub imm: i7,
    /// Operation to perform.
    #[bits(22)]
    pub op: MemOp,
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
            (MemOp::Store, true) => "STNP",
            (MemOp::Store, false) => "STP",
            (MemOp::Load, true) => "LDNP",
            (MemOp::Load, false) => "LDP",
        };

        let base = self.rn();
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

/// Width of the scalar used in SIMD pair instructions.
#[bitos(2)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimdPairWidth {
    /// 32 bit
    S32 = 0b00,
    /// 64 bit
    D64 = 0b01,
    /// 128 bit
    Q128 = 0b10,
    /// Reserved.
    Reserved = 0b11,
}

impl SimdPairWidth {
    pub fn kind(self) -> SimdRegScalarKind {
        match self {
            Self::S32 => SimdRegScalarKind::S32,
            Self::D64 => SimdRegScalarKind::D64,
            Self::Q128 => SimdRegScalarKind::Q128,
            Self::Reserved => SimdRegScalarKind::Q128, // whatever
        }
    }
}

/// Load/store pair of SIMD & FP registers
///
/// This instruction stores a pair of SIMD & FP registers to memory. The address used for the store
/// is calculated from a base register value and an immediate offset.
#[bitos(32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimdPair {
    /// First SIMD register to be transferred.
    #[bits(0..5)]
    pub rt1: SimdReg,
    /// The general-purpose base register.
    #[bits(5..10)]
    pub rn: XrSp,
    /// Second SIMD register to be transferred.
    #[bits(10..15)]
    pub rt2: SimdReg,
    /// Offset divided by register width.
    #[bits(15..22)]
    pub imm: i7,
    /// Operation to perform.
    #[bits(22)]
    pub op: MemOp,
    /// Offset kind.
    #[bits(23..25)]
    pub offset_kind: OffsetKind,
    /// Width of the scalar registers.
    #[bits(30..32)]
    pub opc: SimdPairWidth,
}

impl Display for SimdPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mnemonic = match (
            self.op(),
            self.offset_kind() == OffsetKind::OffsetNonTemporal,
        ) {
            (MemOp::Store, true) => "STNP",
            (MemOp::Store, false) => "STP",
            (MemOp::Load, true) => "LDNP",
            (MemOp::Load, false) => "LDP",
        };

        let scalar_kind = self.opc().kind();
        let base = self.rn();
        let imm = self.imm().value() as i16 * scalar_kind.bytes() as i16;

        let offset = match self.offset_kind() {
            OffsetKind::Offset | OffsetKind::OffsetNonTemporal => &format!("[{base}, #{imm}]"),
            OffsetKind::PreIndexed => &format!("[{base}, #{imm}]!"),
            OffsetKind::PostIndexed => &format!("[{base}], #{imm}"),
        };

        write!(
            f,
            "{} {}, {}, {}",
            mnemonic,
            self.rt1().scalar(scalar_kind),
            self.rt2().scalar(scalar_kind),
            offset,
        )
    }
}

/// Load/store register (unscaled immediate)
///
/// This instruction loads/stores data of a register to/from memory. The address that is used for
/// the operation is calculated from a base register and an immediate offset that is unscaled.
#[bitos(32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnscaledImm {
    /// The general-purpose register to be transferred.
    #[bits(0..5)]
    pub rt: Reg,
    /// The general-purpose base register.
    #[bits(5..10)]
    pub rn: XrSp,
    /// Offset in bytes.
    #[bits(12..21)]
    pub imm: i9,
    /// Operation to perform.
    #[bits(22..24)]
    pub op: MemOpExtended,
    /// Data size.
    #[bits(30..32)]
    pub size: DataSize,
}

impl UnscaledImm {
    pub fn width(self) -> RegWidth {
        match (self.op(), self.size()) {
            (MemOpExtended::Store, DataSize::B8) => RegWidth::W32,
            (MemOpExtended::Store, DataSize::B16) => RegWidth::W32,
            (MemOpExtended::Store, DataSize::B32) => RegWidth::W32,
            (MemOpExtended::Store, DataSize::B64) => RegWidth::X64,
            (MemOpExtended::LoadZext, DataSize::B8) => RegWidth::W32,
            (MemOpExtended::LoadZext, DataSize::B16) => RegWidth::W32,
            (MemOpExtended::LoadZext, DataSize::B32) => RegWidth::W32,
            (MemOpExtended::LoadZext, DataSize::B64) => RegWidth::X64,
            (MemOpExtended::LoadSext64, _) => RegWidth::X64,
            (MemOpExtended::LoadSext32, _) => RegWidth::W32,
        }
    }
}

impl Display for UnscaledImm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = self.width();
        let mnemonic = match (self.op(), self.size()) {
            (MemOpExtended::Store, DataSize::B8) => "STURB",
            (MemOpExtended::Store, DataSize::B16) => "STURH",
            (MemOpExtended::Store, DataSize::B32) => "STUR",
            (MemOpExtended::Store, DataSize::B64) => "STUR",
            (MemOpExtended::LoadZext, DataSize::B8) => "LDURB",
            (MemOpExtended::LoadZext, DataSize::B16) => "LDURH",
            (MemOpExtended::LoadZext, DataSize::B32) => "LDUR",
            (MemOpExtended::LoadZext, DataSize::B64) => "LDUR",
            (MemOpExtended::LoadSext64, DataSize::B8) => "LDURSB",
            (MemOpExtended::LoadSext64, DataSize::B16) => "LDURSH",
            (MemOpExtended::LoadSext64, DataSize::B32) => "LDURSW",
            (MemOpExtended::LoadSext64, DataSize::B64) => "LDUR",
            (MemOpExtended::LoadSext32, DataSize::B8) => "LDURSB",
            (MemOpExtended::LoadSext32, DataSize::B16) => "LDURSH",
            (MemOpExtended::LoadSext32, DataSize::B32) => "LDUR",
            (MemOpExtended::LoadSext32, DataSize::B64) => "????",
        };

        write!(
            f,
            "{} {}, [{}, #{}]",
            mnemonic,
            self.rt().with_width(width),
            self.rn(),
            self.imm()
        )
    }
}

#[bitos(2)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegOffsetIndexOp {
    ZeroExt32,
    LeftShift,
    SignExt32,
    SignExt64,
    Reserved,
}

impl Display for RegOffsetIndexOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let frag = match self {
            Self::ZeroExt32 => "UXTW",
            Self::LeftShift => "LSL",
            Self::SignExt32 => "SXTW",
            Self::SignExt64 => "SXTX",
            Self::Reserved => "????",
        };

        write!(f, "{frag}")
    }
}

impl RegOffsetIndexOp {
    pub fn width(self) -> RegWidth {
        match self {
            Self::ZeroExt32 => RegWidth::W32,
            Self::LeftShift => RegWidth::X64,
            Self::SignExt32 => RegWidth::W32,
            Self::SignExt64 => RegWidth::X64,
            Self::Reserved => RegWidth::X64,
        }
    }
}

/// Load/store register (register offset)
///
/// This instruction loads/stores data of a register to/from memory. The address that is used for
/// the operation is calculated from a base register and an offset register, which can be shifted
/// or extended.
#[bitos(32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegOffset {
    /// The general-purpose register to be transferred.
    #[bits(0..5)]
    pub rt: Reg,
    /// The general-purpose base register.
    #[bits(5..10)]
    pub rn: XrSp,
    /// Whether to shift left by log2 of data size in bytes.
    #[bits(12)]
    pub s: bool,
    /// Index op info.
    #[bits(13..16)]
    pub option: u3,
    /// Offset register.
    #[bits(16..21)]
    pub rm: Reg,
    /// Operation to perform.
    #[bits(22..24)]
    pub op: MemOpExtended,
    /// Data size.
    #[bits(30..32)]
    pub size: DataSize,
}

impl RegOffset {
    pub fn width(self) -> RegWidth {
        match (self.op(), self.size()) {
            (MemOpExtended::Store, DataSize::B8) => RegWidth::W32,
            (MemOpExtended::Store, DataSize::B16) => RegWidth::W32,
            (MemOpExtended::Store, DataSize::B32) => RegWidth::W32,
            (MemOpExtended::Store, DataSize::B64) => RegWidth::X64,
            (MemOpExtended::LoadZext, DataSize::B8) => RegWidth::W32,
            (MemOpExtended::LoadZext, DataSize::B16) => RegWidth::W32,
            (MemOpExtended::LoadZext, DataSize::B32) => RegWidth::W32,
            (MemOpExtended::LoadZext, DataSize::B64) => RegWidth::X64,
            (MemOpExtended::LoadSext64, _) => RegWidth::X64,
            (MemOpExtended::LoadSext32, _) => RegWidth::W32,
        }
    }

    pub fn index_op(self) -> RegOffsetIndexOp {
        let option = self.option().value();
        bit_match! {
            match option {
                "010" => RegOffsetIndexOp::ZeroExt32,
                "011" => RegOffsetIndexOp::LeftShift,
                "110" => RegOffsetIndexOp::SignExt32,
                "111" => RegOffsetIndexOp::SignExt64,
                _ => RegOffsetIndexOp::Reserved,
            }
        }
    }
}

impl Display for RegOffset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = self.width();
        let mnemonic = match (self.op(), self.size()) {
            (MemOpExtended::Store, DataSize::B8) => "STRB",
            (MemOpExtended::Store, DataSize::B16) => "STRH",
            (MemOpExtended::Store, DataSize::B32) => "STR",
            (MemOpExtended::Store, DataSize::B64) => "STR",
            (MemOpExtended::LoadZext, DataSize::B8) => "LDRB",
            (MemOpExtended::LoadZext, DataSize::B16) => "LDRH",
            (MemOpExtended::LoadZext, DataSize::B32) => "LDR",
            (MemOpExtended::LoadZext, DataSize::B64) => "LDR",
            (MemOpExtended::LoadSext64, DataSize::B8) => "LDRSB",
            (MemOpExtended::LoadSext64, DataSize::B16) => "LDRSH",
            (MemOpExtended::LoadSext64, DataSize::B32) => "LDRSW",
            (MemOpExtended::LoadSext64, DataSize::B64) => "LDR",
            (MemOpExtended::LoadSext32, DataSize::B8) => "LDRSB",
            (MemOpExtended::LoadSext32, DataSize::B16) => "LDRSH",
            (MemOpExtended::LoadSext32, DataSize::B32) => "LDR",
            (MemOpExtended::LoadSext32, DataSize::B64) => "????",
        };

        let index_op = self.index_op();
        let amount = if self.s() {
            self.size().bytes().ilog2()
        } else {
            0
        };

        write!(
            f,
            "{} {}, [{}, {}, {} #{}]",
            mnemonic,
            self.rt().with_width(width),
            self.rn(),
            self.rm().with_width(index_op.width()),
            self.index_op(),
            amount,
        )
    }
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
    pub rn: XrSp,
    /// Offset divided by data size.
    #[bits(10..22)]
    pub imm: u12,
    /// Operation to perform.
    #[bits(22..24)]
    pub op: MemOpExtended,
    /// Data size.
    #[bits(30..32)]
    pub size: DataSize,
}

impl UnsignedImm {
    pub fn width(self) -> RegWidth {
        match (self.op(), self.size()) {
            (MemOpExtended::Store, DataSize::B8) => RegWidth::W32,
            (MemOpExtended::Store, DataSize::B16) => RegWidth::W32,
            (MemOpExtended::Store, DataSize::B32) => RegWidth::W32,
            (MemOpExtended::Store, DataSize::B64) => RegWidth::X64,
            (MemOpExtended::LoadZext, DataSize::B8) => RegWidth::W32,
            (MemOpExtended::LoadZext, DataSize::B16) => RegWidth::W32,
            (MemOpExtended::LoadZext, DataSize::B32) => RegWidth::W32,
            (MemOpExtended::LoadZext, DataSize::B64) => RegWidth::X64,
            (MemOpExtended::LoadSext64, _) => RegWidth::X64,
            (MemOpExtended::LoadSext32, _) => RegWidth::W32,
        }
    }
}

impl Display for UnsignedImm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = self.width();
        let mnemonic = match (self.op(), self.size()) {
            (MemOpExtended::Store, DataSize::B8) => "STRB",
            (MemOpExtended::Store, DataSize::B16) => "STRH",
            (MemOpExtended::Store, DataSize::B32) => "STR",
            (MemOpExtended::Store, DataSize::B64) => "STR",
            (MemOpExtended::LoadZext, DataSize::B8) => "LDRB",
            (MemOpExtended::LoadZext, DataSize::B16) => "LDRH",
            (MemOpExtended::LoadZext, DataSize::B32) => "LDR",
            (MemOpExtended::LoadZext, DataSize::B64) => "LDR",
            (MemOpExtended::LoadSext64, DataSize::B8) => "LDRSB",
            (MemOpExtended::LoadSext64, DataSize::B16) => "LDRSH",
            (MemOpExtended::LoadSext64, DataSize::B32) => "LDRSW",
            (MemOpExtended::LoadSext64, DataSize::B64) => "LDR",
            (MemOpExtended::LoadSext32, DataSize::B8) => "LDRSB",
            (MemOpExtended::LoadSext32, DataSize::B16) => "LDRSH",
            (MemOpExtended::LoadSext32, DataSize::B32) => "LDR",
            (MemOpExtended::LoadSext32, DataSize::B64) => "????",
        };

        write!(
            f,
            "{} {}, [{}, #{}]",
            mnemonic,
            self.rt().with_width(width),
            self.rn(),
            self.imm().value() as u32 * self.size().bytes()
        )
    }
}

/// Load/store SIMD register (unsigned immediate)
///
/// This instruction loads/stores data of a SIMD register to/from memory. The address that is used
/// for the operation is calculated from a base register and an immediate offset.
#[bitos(32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimdUnsignedImm {
    /// The general-purpose register to be transferred.
    #[bits(0..5)]
    pub rt: SimdReg,
    /// The general-purpose base register.
    #[bits(5..10)]
    pub rn: XrSp,
    /// Offset divided by data size.
    #[bits(10..22)]
    pub imm: u12,
    /// Operation to perform.
    #[bits(22)]
    pub op: MemOp,
    /// Whether to use Q128.
    #[bits(23)]
    pub opc: bool,
    /// Data size.
    #[bits(30..32)]
    pub size: DataSize,
}

impl SimdUnsignedImm {
    pub fn scalar_kind(self) -> SimdRegScalarKind {
        match (self.size(), self.opc()) {
            (DataSize::B8, false) => SimdRegScalarKind::B8,
            (DataSize::B16, false) => SimdRegScalarKind::H16,
            (DataSize::B32, false) => SimdRegScalarKind::S32,
            (DataSize::B64, false) => SimdRegScalarKind::D64,
            (_, true) => SimdRegScalarKind::Q128,
        }
    }
}

impl Display for SimdUnsignedImm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let scalar_kind = self.scalar_kind();
        let mnemonic = match self.op() {
            MemOp::Store => "STR",
            MemOp::Load => "LDR",
        };

        write!(
            f,
            "{} {}, [{}, #{}]",
            mnemonic,
            self.rt().scalar(scalar_kind),
            self.rn(),
            self.imm().value() as u32 * scalar_kind.bytes()
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum Instruction {
    Pair(Pair),
    SimdPair(SimdPair),
    UnscaledImm(UnscaledImm),
    RegOffset(RegOffset),
    UnsignedImm(UnsignedImm),
    SimdUnsignedImm(SimdUnsignedImm),
}

impl Instruction {
    fn new_pair(value: u32) -> Option<Self> {
        let opc = value.bits(30, 32);
        let vr = value.bit(26) as u32;
        let temporal = value.bits(23, 25) != OffsetKind::OffsetNonTemporal as u32;

        Some(bit_match! {
            match (opc, vr) {
                ("00", "0") => Self::Pair(Pair(value)),
                ("00", "1") => Self::SimdPair(SimdPair(value)),

                ("01", "0") if temporal => todo!("stgp/ldpsw"),
                ("01", "1") => Self::SimdPair(SimdPair(value)),

                ("10", "0") => Self::Pair(Pair(value)),
                ("10", "1") => Self::SimdPair(SimdPair(value)),

                ("11", "0") => todo!("sttp/ldtp"),
                ("11", "1") => todo!("sttp/ldtp simd/fp"),
                _ => return None,
            }
        })
    }

    fn new_unscaled_imm(value: u32) -> Option<Self> {
        let size = value.bits(30, 32);
        let vr = value.bit(26) as u32;
        let opc = value.bits(22, 24);

        Some(bit_match! {
            match (size, vr, opc) {
                // 8 bit
                ("00", "0", "__") => Self::UnscaledImm(UnscaledImm(value)),
                ("00", "1", "__") => todo!("simd 8 bit/128 bit"),

                // unallocated
                ("1_", "0", "11") => return None,
                ("__", "1", "1_") => return None,

                // 16 bit
                ("01", "0", "__") => Self::UnscaledImm(UnscaledImm(value)),
                ("01", "1", "__") => todo!("simd 16 bit"),

                // 32 bit
                ("10", "0", "__") => Self::UnscaledImm(UnscaledImm(value)),
                ("10", "1", "__") => todo!("simd 32 bit"),

                // prefetch
                ("11", "0", "10") => todo!("prefetch"),

                // 64 bit
                ("11", "0", "__") => Self::UnscaledImm(UnscaledImm(value)),
                ("11", "1", "__") => todo!("simd 64 bit"),

                _ => return None,
            }
        })
    }

    fn new_reg_offset(value: u32) -> Option<Self> {
        let size = value.bits(30, 32);
        let vr = value.bit(26) as u32;
        let opc = value.bits(22, 24);
        let option = value.bits(13, 16);

        Some(bit_match! {
            match (size, vr, opc) {
                // 8 bit
                ("00", "0", "__") => Self::RegOffset(RegOffset(value)),
                ("00", "1", "__") => todo!("simd 8/128 bit"),

                // unallocated
                ("1_", "0", "11") => return None,
                ("__", "1", "1_") => return None,

                // 16 bit
                ("01", "0", "__") => Self::RegOffset(RegOffset(value)),
                ("01", "1", "__") => todo!("simd 16 bit"),

                // 32 bit
                ("10", "0", "__") => Self::RegOffset(RegOffset(value)),
                ("10", "1", "__") => todo!("simd 32 bit"),

                // prefetch
                ("11", "0", "10") => todo!("prefetch"),

                // 64 bit
                ("11", "0", "__") => Self::RegOffset(RegOffset(value)),
                ("11", "1", "__") => todo!("simd 64 bit"),

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
                ("00", "1", "__") => Self::SimdUnsignedImm(SimdUnsignedImm(value)),

                // unallocated
                ("1_", "0", "11") => return None,
                ("__", "1", "1_") => return None,

                // 16 bit
                ("01", "0", "__") => Self::UnsignedImm(UnsignedImm(value)),
                ("01", "1", "__") => Self::SimdUnsignedImm(SimdUnsignedImm(value)),

                // 32 bit
                ("10", "0", "__") => Self::UnsignedImm(UnsignedImm(value)),
                ("10", "1", "__") => Self::SimdUnsignedImm(SimdUnsignedImm(value)),

                // prefetch
                ("11", "0", "10") => todo!("prefetch"),

                // 64 bit
                ("11", "0", "__") => Self::UnsignedImm(UnsignedImm(value)),
                ("11", "1", "__") => Self::SimdUnsignedImm(SimdUnsignedImm(value)),

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

                ("__11", "_", "0__0_________", "00") => Self::new_unscaled_imm(value)?,
                ("__11", "_", "0__0_________", "01") => todo!("load/store reg (imm post indexed)"),
                ("__11", "_", "0__0_________", "10") => todo!("load/store reg (unprivileged)"),
                ("__11", "_", "0__0_________", "11") => todo!("load/store reg (imm pre indexed)"),
                ("__11", "_", "0__1_________", "00") => todo!("atomic mem ops"),
                ("__11", "_", "0__1_________", "10") => Self::new_reg_offset(value)?,
                ("__11", "_", "0__1_________", "_1") => todo!("load/store reg (pac)"),
                ("__11", "_", "1____________", "__") => Self::new_uimm(value)?,
                _ => return None,
            }
        })
    }
}
