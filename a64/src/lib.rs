#![allow(clippy::unusual_byte_groupings)]

pub mod branch_sys;
pub mod dp_imm;
pub mod dp_reg;
pub mod load_store;
pub mod reserved;
pub mod simd_fp;

use core::fmt::Display;

use a64_macros::bit_match;
use bitos::{BitUtils, bitos};
use derive_more::Display;
use either::Either;

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
        self.bits() / 8
    }
}

impl Display for DataSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let frag = match self {
            Self::B8 => "B",
            Self::B16 => "H",
            Self::B32 => "W",
            Self::B64 => "X",
        };

        write!(f, "{frag}")
    }
}

/// Memory operation.
#[bitos(1)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemOp {
    Store = 0b0,
    Load = 0b1,
}

/// Extended memory operation.
#[bitos(2)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemOpExtended {
    /// Store to memory.
    Store = 0b00,
    /// Load from memory and zero-extend.
    LoadZext = 0b01,
    /// Load from memory and sign extended to 64 bits (XR).
    LoadSext64 = 0b10,
    /// Load from memory and sign extended to 32 bits (WR).
    LoadSext32 = 0b11,
}

/// Width used for accessing and manipulating the general purpose registers.
#[bitos(1)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegWidth {
    W32,
    X64,
}

impl RegWidth {
    #[inline(always)]
    pub fn is_32_bits(self) -> bool {
        match self {
            Self::W32 => true,
            Self::X64 => false,
        }
    }

    #[inline(always)]
    pub fn is_64_bits(self) -> bool {
        match self {
            Self::W32 => false,
            Self::X64 => true,
        }
    }

    #[inline(always)]
    pub fn bits(self) -> u32 {
        match self {
            Self::W32 => 32,
            Self::X64 => 64,
        }
    }

    #[inline(always)]
    pub fn bytes(self) -> u32 {
        self.bits() / 8
    }
}

/// Enumeration of the general purpose registers. The 32nd register is considered unknown.
#[bitos(5)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegUnk {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    R16,
    R17,
    R18,
    R19,
    R20,
    R21,
    R22,
    R23,
    R24,
    R25,
    R26,
    R27,
    R28,
    R29,
    R30,
    Unknown,
}

impl RegUnk {
    #[inline(always)]
    pub fn with_width(self, width: RegWidth) -> Either<WrUnk, XrUnk> {
        match width {
            RegWidth::W32 => Either::Left(unsafe { std::mem::transmute(self) }),
            RegWidth::X64 => Either::Right(unsafe { std::mem::transmute(self) }),
        }
    }

    #[inline(always)]
    pub fn with_zr(self) -> Reg {
        unsafe { std::mem::transmute(self) }
    }

    #[inline(always)]
    pub fn with_sp(self) -> RegSp {
        unsafe { std::mem::transmute(self) }
    }
}

/// Enumeration of the general purpose registers and ZR as the 32nd value.
#[bitos(5)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reg {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    R16,
    R17,
    R18,
    R19,
    R20,
    R21,
    R22,
    R23,
    R24,
    R25,
    R26,
    R27,
    R28,
    R29,
    R30,
    RZR,
}

impl Reg {
    #[inline(always)]
    pub fn with_width(self, width: RegWidth) -> Either<Wr, Xr> {
        match width {
            RegWidth::W32 => Either::Left(unsafe { std::mem::transmute(self) }),
            RegWidth::X64 => Either::Right(unsafe { std::mem::transmute(self) }),
        }
    }
}

/// Enumeration of the general purpose registers and SP as the 32nd value.
#[bitos(5)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegSp {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    R16,
    R17,
    R18,
    R19,
    R20,
    R21,
    R22,
    R23,
    R24,
    R25,
    R26,
    R27,
    R28,
    R29,
    R30,
    SP,
}

impl RegSp {
    #[inline(always)]
    pub fn with_width(self, width: RegWidth) -> Either<WrSp, XrSp> {
        match width {
            RegWidth::W32 => Either::Left(unsafe { std::mem::transmute(self) }),
            RegWidth::X64 => Either::Right(unsafe { std::mem::transmute(self) }),
        }
    }
}

/// Enumeration of the general purpose registers (32 bit). The 32nd register is considered unknown.
#[bitos(5)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WrUnk {
    W0,
    W1,
    W2,
    W3,
    W4,
    W5,
    W6,
    W7,
    W8,
    W9,
    W10,
    W11,
    W12,
    W13,
    W14,
    W15,
    W16,
    W17,
    W18,
    W19,
    W20,
    W21,
    W22,
    W23,
    W24,
    W25,
    W26,
    W27,
    W28,
    W29,
    W30,
    Unknown,
}

impl WrUnk {
    #[inline(always)]
    pub fn with_zr(self) -> Wr {
        // SAFETY: both are C-like enums with the same amount of variants
        unsafe { std::mem::transmute(self) }
    }

    #[inline(always)]
    pub fn with_sp(self) -> WrSp {
        // SAFETY: both are C-like enums with the same amount of variants
        unsafe { std::mem::transmute(self) }
    }
}

/// Enumeration of the general purpose registers (32 bit) and ZR as the 32nd value.
#[bitos(5)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Wr {
    W0,
    W1,
    W2,
    W3,
    W4,
    W5,
    W6,
    W7,
    W8,
    W9,
    W10,
    W11,
    W12,
    W13,
    W14,
    W15,
    W16,
    W17,
    W18,
    W19,
    W20,
    W21,
    W22,
    W23,
    W24,
    W25,
    W26,
    W27,
    W28,
    W29,
    W30,
    WZR,
}

impl Display for Wr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

/// Enumeration of the general purpose registers (32 bit) and SP as the 32nd value.
#[bitos(5)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WrSp {
    W0,
    W1,
    W2,
    W3,
    W4,
    W5,
    W6,
    W7,
    W8,
    W9,
    W10,
    W11,
    W12,
    W13,
    W14,
    W15,
    W16,
    W17,
    W18,
    W19,
    W20,
    W21,
    W22,
    W23,
    W24,
    W25,
    W26,
    W27,
    W28,
    W29,
    W30,
    WSP,
}

impl Display for WrSp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

/// Enumeration of the general purpose registers (64 bit). The 32nd register is considered unknown.
#[bitos(5)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XrUnk {
    X0,
    X1,
    X2,
    X3,
    X4,
    X5,
    X6,
    X7,
    X8,
    X9,
    X10,
    X11,
    X12,
    X13,
    X14,
    X15,
    X16,
    X17,
    X18,
    X19,
    X20,
    X21,
    X22,
    X23,
    X24,
    X25,
    X26,
    X27,
    X28,
    X29,
    X30,
    Unknown,
}

impl XrUnk {
    #[inline(always)]
    pub fn with_zr(self) -> Xr {
        // SAFETY: both are C-like enums with the same amount of variants
        unsafe { std::mem::transmute(self) }
    }

    #[inline(always)]
    pub fn with_sp(self) -> XrSp {
        // SAFETY: both are C-like enums with the same amount of variants
        unsafe { std::mem::transmute(self) }
    }
}

/// Enumeration of the general purpose registers (64 bit) and ZR as the 32nd value.
#[bitos(5)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Xr {
    X0,
    X1,
    X2,
    X3,
    X4,
    X5,
    X6,
    X7,
    X8,
    X9,
    X10,
    X11,
    X12,
    X13,
    X14,
    X15,
    X16,
    X17,
    X18,
    X19,
    X20,
    X21,
    X22,
    X23,
    X24,
    X25,
    X26,
    X27,
    X28,
    X29,
    X30,
    XZR,
}

impl Display for Xr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

/// Enumeration of the general purpose registers (64 bit) and SP as the 32nd value.
#[bitos(5)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XrSp {
    X0,
    X1,
    X2,
    X3,
    X4,
    X5,
    X6,
    X7,
    X8,
    X9,
    X10,
    X11,
    X12,
    X13,
    X14,
    X15,
    X16,
    X17,
    X18,
    X19,
    X20,
    X21,
    X22,
    X23,
    X24,
    X25,
    X26,
    X27,
    X28,
    X29,
    X30,
    SP,
}

impl Display for XrSp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

/// Width used for accessing and manipulating the SIMD registers.
#[bitos(1)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimdRegWidth {
    V64,
    V128,
}

impl SimdRegWidth {
    #[inline(always)]
    pub fn is_64_bits(self) -> bool {
        match self {
            Self::V64 => true,
            Self::V128 => false,
        }
    }

    #[inline(always)]
    pub fn is_128_bits(self) -> bool {
        match self {
            Self::V64 => false,
            Self::V128 => true,
        }
    }

    #[inline(always)]
    pub fn bits(self) -> u32 {
        match self {
            Self::V64 => 64,
            Self::V128 => 128,
        }
    }

    #[inline(always)]
    pub fn bytes(self) -> u32 {
        self.bits() / 8
    }
}

/// Enumeration of scalar kinds within a SIMD register.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimdRegScalarKind {
    /// Lower 8 bits.
    B8,
    /// Lower 16 bits.
    H16,
    /// Lower 32 bits.
    S32,
    /// Lower 64 bits.
    D64,
    /// All 128 bits.
    Q128,
}

impl SimdRegScalarKind {
    #[inline(always)]
    pub fn bits(self) -> u32 {
        match self {
            Self::B8 => 8,
            Self::H16 => 16,
            Self::S32 => 32,
            Self::D64 => 64,
            Self::Q128 => 128,
        }
    }

    #[inline(always)]
    pub fn bytes(self) -> u32 {
        self.bits() / 8
    }
}

/// Enumeration of the SIMD & FP registers.
#[bitos(5)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimdReg {
    V0,
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    V9,
    V10,
    V11,
    V12,
    V13,
    V14,
    V15,
    V16,
    V17,
    V18,
    V19,
    V20,
    V21,
    V22,
    V23,
    V24,
    V25,
    V26,
    V27,
    V28,
    V29,
    V30,
    V31,
}

impl Display for SimdReg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

/// Enumeration of scalars within a SIMD register.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimdRegScalar {
    /// Lower 8 bits.
    B8(SimdReg),
    /// Lower 16 bits.
    H16(SimdReg),
    /// Lower 32 bits.
    S32(SimdReg),
    /// Lower 64 bits.
    D64(SimdReg),
    /// All 128 bits.
    Q128(SimdReg),
}

impl Display for SimdRegScalar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::B8(r) => write!(f, "B{}", r as u32),
            Self::H16(r) => write!(f, "H{}", r as u32),
            Self::S32(r) => write!(f, "S{}", r as u32),
            Self::D64(r) => write!(f, "D{}", r as u32),
            Self::Q128(r) => write!(f, "Q{}", r as u32),
        }
    }
}

impl SimdReg {
    /// Returns the scalar of the given kind contained within this register.
    #[inline(always)]
    pub fn scalar(self, kind: SimdRegScalarKind) -> SimdRegScalar {
        match kind {
            SimdRegScalarKind::B8 => SimdRegScalar::B8(self),
            SimdRegScalarKind::H16 => SimdRegScalar::H16(self),
            SimdRegScalarKind::S32 => SimdRegScalar::S32(self),
            SimdRegScalarKind::D64 => SimdRegScalar::D64(self),
            SimdRegScalarKind::Q128 => SimdRegScalar::Q128(self),
        }
    }
}

/// Specifies the operation of a logical operation.
#[bitos(2)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicalOp {
    /// Bitwise and.
    And = 0b00,
    /// Bitwise or.
    Or = 0b01,
    /// Exclusive or.
    Xor = 0b10,
    /// Bitwise and, but also updates condition flags.
    Ands = 0b11,
}

impl Display for LogicalOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mnemonic = match self {
            Self::And => "AND",
            Self::Or => "ORR",
            Self::Xor => "EOR",
            Self::Ands => "ANDS",
        };

        write!(f, "{mnemonic}")
    }
}

/// Specifies a kind of shifting operation.
#[bitos(2)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShiftKind {
    /// Logic left shift.
    LogicLeft = 0b00,
    /// Logic right shift.
    LogicRight = 0b01,
    /// Arithmethic shift right.
    ArithRight = 0b10,
    /// Rotate right.
    RotateRight = 0b11,
}

impl Display for ShiftKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let frag = match self {
            Self::LogicLeft => "LSL",
            Self::LogicRight => "LSR",
            Self::ArithRight => "ASR",
            Self::RotateRight => "ROR",
        };

        write!(f, "{frag}")
    }
}

/// A condition.
#[bitos(4)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Condition {
    Equal = 0b0000,
    NotEqual = 0b0001,
    CarrySet = 0b0010,
    CarryClear = 0b0011,
    Negative = 0b0100,
    PositiveOrZero = 0b0101,
    Overflow = 0b0110,
    NoOverflow = 0b0111,
    UnsignedGreater = 0b1000,
    UnsignedLessEqual = 0b1001,
    SignedGreaterEqual = 0b1010,
    SignedLess = 0b1011,
    SignedGreater = 0b1100,
    SignedLessEqual = 0b1101,
    Always = 0b1110,
    Never = 0b1111,
}

impl Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let frag = match self {
            Self::Equal => "EQ",
            Self::NotEqual => "NE",
            Self::CarrySet => "CS",
            Self::CarryClear => "CC",
            Self::Negative => "MI",
            Self::PositiveOrZero => "PL",
            Self::Overflow => "VS",
            Self::NoOverflow => "VC",
            Self::UnsignedGreater => "HI",
            Self::UnsignedLessEqual => "LS",
            Self::SignedGreaterEqual => "GE",
            Self::SignedLess => "LT",
            Self::SignedGreater => "GT",
            Self::SignedLessEqual => "LE",
            Self::Always => "AL",
            Self::Never => "NV",
        };

        write!(f, "{frag}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum Instruction {
    Reserved(reserved::Instruction),
    DpImm(dp_imm::Instruction),
    BranchSys(branch_sys::Instruction),
    DpReg(dp_reg::Instruction),
    SimdFp(simd_fp::Instruction),
    LoadStore(load_store::Instruction),
}

impl Instruction {
    #[inline(never)]
    pub fn new(value: u32) -> Option<Self> {
        let op0 = value.bit(31) as u32;
        let op1 = value.bits(25, 29);

        Some(bit_match! {
            match (op0, op1) {
                ("0", "0000") => Self::Reserved(reserved::Instruction::new(value)?),
                ("_", "100_") => Self::DpImm(dp_imm::Instruction::new(value)?),
                ("_", "101_") => Self::BranchSys(branch_sys::Instruction::new(value)?),
                ("_", "_101") => Self::DpReg(dp_reg::Instruction::new(value)?),
                ("_", "_111") => Self::SimdFp(simd_fp::Instruction::new(value)?),
                ("_", "_1_0") => Self::LoadStore(load_store::Instruction::new(value)?),
                _ => return None,
            }
        })
    }
}

#[test]
fn test_app() {
    let data = std::fs::read("/home/vxpm/dev/nx_hello.elf").unwrap();
    let main_offset = 0x10180;

    let mut offset = main_offset;
    loop {
        let index = offset - main_offset;
        let value = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap());
        if let Some(ins) = Instruction::new(value) {
            println!("[{offset:08X} / {index:04}] {value:08X} => {ins}");
        } else {
            println!("[{offset:08X} / {index:04}] {value:08X} => UNKNOWN ({value:032b})");
            panic!();
        }

        offset += 4;
    }
}
