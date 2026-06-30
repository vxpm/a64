pub mod branch_sys;
pub mod dp_imm;
pub mod load_store;
pub mod reserved;

use core::fmt::Display;

use a64_macros::bit_match;
use bitos::{BitUtils, bitos};
use derive_more::Display;

/// Enumeration of the general purpose registers (32 bit).
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
    Unknown,
}

impl Display for Wr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl Wr {
    #[inline(always)]
    pub fn with_zr(self) -> WrZr {
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
pub enum WrZr {
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

impl Display for WrZr {
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
    SP,
}

impl Display for WrSp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

/// Enumeration of the general purpose registers (64 bit).
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
    Unknown,
}

impl Display for Xr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl Xr {
    #[inline(always)]
    pub fn with_zr(self) -> XrZr {
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
pub enum XrZr {
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

impl Display for XrZr {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum Instruction {
    Reserved(reserved::Instruction),
    DpImm(dp_imm::Instruction),
    BranchSys(branch_sys::Instruction),
    // DpReg,
    // SimdFp,
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
                // ("_", "_101") => Instruction::DpReg,
                // ("_", "_111") => Instruction::SimdFp,
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
        let value = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap());
        if let Some(ins) = Instruction::new(value) {
            println!("{value:08X} => {ins}");
        } else {
            println!("{value:08X} => UNKNOWN ({value:032b})");
            panic!();
        }

        offset += 4;
    }
}
