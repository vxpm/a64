//! Unconditional branch (immediate)

use a64_macros::bit_match;
use bitos::integer::i26;
use bitos::{BitUtils, bitos};
use core::fmt::Display;
use derive_more::Display;

/// Branch
///
/// This instruction branches unconditionally to a label at a PC-relative offset. This instruction
/// provides a hint that this is not a subroutine call or return.
#[bitos(32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Branch {
    /// Offset from the address of this instruction, divided by 4.
    #[bits(0..26)]
    pub imm: i26,
}

impl Display for Branch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "B .{:+}", self.imm().value() * 4)
    }
}

/// Branch with link
///
/// This instruction branches to a PC-relative offset, setting register X30 to PC+4. This instruction
/// provides a hint that this is a subroutine call.
#[bitos(32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BranchLink {
    /// Offset from the address of this instruction, divided by 4.
    #[bits(0..26)]
    pub imm: i26,
}

impl Display for BranchLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BL .{:+}", self.imm().value() * 4)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum Instruction {
    Branch(Branch),
    BranchLink(BranchLink),
}

impl Instruction {
    pub fn new(value: u32) -> Option<Self> {
        let op = value.bit(31) as u32;

        Some(bit_match! {
            match op {
                "0" => Self::Branch(Branch(value)),
                "1" => Self::BranchLink(BranchLink(value)),
                _ => return None,
            }
        })
    }
}
