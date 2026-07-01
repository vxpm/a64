//! Data Processing - Immediate

pub mod logical;
pub mod mov_wide;

use core::fmt::Display;

use a64_macros::bit_match;
use bitos::integer::{u2, u12, u19};
use bitos::{BitUtils, bitos};
use derive_more::Display;

use crate::{RegSp, RegUnk, RegWidth, Xr};

/// Form PC-relative address, possibly to 4 KiB page
///
/// This instruction adds an immediate value to the PC value to form a PC-relative address, and
/// writes the result to the destination register. If the target is a page, the immediate is shifted
/// left by 12 and the 12 lower bits of the result are masked.
#[bitos(32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PcRelAddr {
    /// Destination register.
    #[bits(0..5)]
    pub rd: Xr,
    /// Immediate value high.
    #[bits(5..24)]
    pub immhi: u19,
    /// Immediate value low.
    #[bits(29..31)]
    pub immlo: u2,
    /// Whether the address targets a 4 KiB page.
    #[bits(31)]
    pub to_page: bool,
}

impl Display for PcRelAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let imm = 0.with_bits(0, 2, self.immlo().value() as u64).with_bits(
            2,
            21,
            self.immhi().value() as u64,
        );

        if self.to_page() {
            write!(f, "ADRP {}, #{}", self.rd(), imm << 12)
        } else {
            write!(f, "ADR {}, #{}", self.rd(), imm)
        }
    }
}

/// Add or subtract immediate value
///
/// This instruction adds/subtracts a register value and an optionally-shifted immediate value, and
/// writes the result to the destination register.
#[bitos(32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AddSub {
    /// Destination register.
    #[bits(0..5)]
    pub rd: RegUnk,
    /// Source register.
    #[bits(5..10)]
    pub rn: RegSp,
    /// Immediate value.
    #[bits(10..22)]
    pub imm: u12,
    /// Whether to left-shift the immediate by 12.
    #[bits(22)]
    pub sh: bool,
    /// Whether to update condition flags based on the result. If set, `rd` uses ZR, otherwise SP.
    #[bits(29)]
    pub s: bool,
    /// Whether this is a subtract operation.
    #[bits(30)]
    pub sub: bool,
    /// Width of the registers.
    #[bits(31)]
    pub sf: RegWidth,
}

impl Display for AddSub {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let base = if self.sub() { "SUB" } else { "ADD" };
        let shift = if self.sh() { "12" } else { "0" };

        if self.s() {
            write!(
                f,
                "{}S {}, {}, #{}, LSL #{}",
                base,
                self.rd().with_zr().with_width(self.sf()),
                self.rn().with_width(self.sf()),
                self.imm(),
                shift
            )
        } else {
            write!(
                f,
                "{} {}, {}, #{}, LSL #{}",
                base,
                self.rd().with_sp().with_width(self.sf()),
                self.rn().with_width(self.sf()),
                self.imm(),
                shift,
            )
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum Instruction {
    PcRelAddr(PcRelAddr),
    AddSub(AddSub),
    Logical(logical::Instruction),
    MovWide(mov_wide::Instruction),
}

impl Instruction {
    pub fn new(value: u32) -> Option<Self> {
        let op0 = value.bits(30, 32);
        let op1 = value.bits(22, 26);

        Some(bit_match! {
            match (op0, op1) {
                ("11", "111_") => todo!("one src"),
                ("__", "00__") => Self::PcRelAddr(PcRelAddr(value)),
                ("__", "010_") => Self::AddSub(AddSub(value)),
                ("__", "0110") => todo!("add sub with tags"),
                ("__", "0111") => todo!("min max"),
                ("__", "100_") => Self::Logical(logical::Instruction::new(value)?),
                ("__", "101_") => Self::MovWide(mov_wide::Instruction::new(value)?),
                ("__", "110_") => todo!("bitfield"),
                ("__", "111_") => todo!("extract"),
                _ => return None,
            }
        })
    }
}
