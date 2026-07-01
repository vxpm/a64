//! Data Processing - Immediate

pub mod logical;
pub mod mov_wide;
pub mod pc_rel_addr;

use core::fmt::Display;

use a64_macros::bit_match;
use bitos::integer::u12;
use bitos::{BitUtils, bitos};
use derive_more::Display;

use crate::{RegSp, RegUnk, RegWidth};

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
    PcRelAddr(pc_rel_addr::Instruction),
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
                ("__", "00__") => Self::PcRelAddr(pc_rel_addr::Instruction::new(value)?),
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
