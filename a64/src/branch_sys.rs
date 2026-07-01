//! Branches, Exception Generating and System instructions

use core::fmt::Display;

use a64_macros::bit_match;
use bitos::integer::{i14, i26, u5};
use bitos::{BitUtils, bitos};
use derive_more::Display;

use crate::{Reg, RegWidth};

/// Branch
///
/// This instruction branches unconditionally to an address at a PC-relative offset, optionally
/// setting register X30 to PC+4.
#[bitos(32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UncondBranchImm {
    /// Offset from the address of this instruction, divided by 4.
    #[bits(0..26)]
    pub imm: i26,
    /// Whether to set X30 to PC+4.
    #[bits(31)]
    pub link: bool,
}

impl Display for UncondBranchImm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mnemonic = if self.link() { "BL" } else { "B" };
        write!(f, "{} #{}", mnemonic, self.imm().value() * 4)
    }
}

/// Test bit and branch if (non)zero
///
/// This instruction compares the value of a test bit with zero, and conditionally branches to a
/// label at a PC-relative offset if the comparison is (not) equal. This instruction provides a hint
/// that this is not a subroutine call or return. This instruction does not affect condition flags.
#[bitos(32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TestBranch {
    /// General purpose register to be tested.
    #[bits(0..5)]
    pub rt: Reg,
    /// Offset from the address of this instruction, divided by 4.
    #[bits(5..19)]
    pub imm: i14,
    /// Index of the bit to be tested. If the register width is 64 bits, this is an index into the
    /// higher half of the register.
    #[bits(19..24)]
    pub bit: u5,
    /// Whether to only branch if not zero (i.e. invert the condition).
    #[bits(24)]
    pub not: bool,
    /// Width of the register.
    #[bits(31)]
    pub sf: RegWidth,
}

impl Display for TestBranch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mnemonic = if self.not() { "TBNZ" } else { "TBZ" };
        let index = self.bit().value() + if self.sf() == RegWidth::X64 { 32 } else { 0 };

        write!(
            f,
            "{} {}, #{}, #{}",
            mnemonic,
            self.rt().with_width(self.sf()),
            index,
            self.imm().value() * 4
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum Instruction {
    UncondBranchImm(UncondBranchImm),
    TestBranch(TestBranch),
}

impl Instruction {
    pub fn new(value: u32) -> Option<Self> {
        let op0 = value.bits(29, 32);

        // op1: 12..26, split to generate better code with bit_match
        let op1_a = value.bits(16, 26);
        let op1_b = value.bits(12, 16);

        let op2 = value.bits(0, 5);

        Some(bit_match! {
            match (op0, op1_a, op1_b, op2) {
                ("010", "00________", "____", "_____") => todo!("cond branch (immediate)"),
                ("010", "01________", "____", "_____") => todo!("misc branch (immediate)"),

                ("011", "00________", "1___", "_____") => todo!("cmp u8/u16 reg and branch"),

                ("110", "00________", "____", "_____") => todo!("except"),
                ("110", "0100000011", "0001", "_____") => todo!("sys with reg"),
                ("110", "0100000011", "0010", "11111") => todo!("hints"),
                ("110", "0100000011", "0011", "_____") => todo!("barriers"),
                ("110", "0100000___", "0100", "_____") => todo!("pstate"),
                ("110", "0100_01___", "____", "_____") => todo!("sys"),
                ("110", "0100_1____", "____", "_____") => todo!("sys reg move"),
                ("110", "0101_01___", "____", "_____") => todo!("sys pair"),
                ("110", "0101_1____", "____", "_____") => todo!("sys reg pair move"),
                ("110", "1_________", "____", "_____") => todo!("uncond branch (reg)"),

                ("_00", "__________", "____", "_____") => Self::UncondBranchImm(UncondBranchImm(value)),

                ("_01", "0_________", "____", "_____") => todo!("cmp and branch (imm)"),
                ("_01", "1_________", "____", "_____") => Self::TestBranch(TestBranch(value)),

                ("_11", "00________", "00__", "_____") => todo!("cmp regs and branch"),
                ("_11", "01________", "_0__", "_____") => todo!("cmp reg with imm and branch"),
                _ => return None,
            }
        })
    }
}
