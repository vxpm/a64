//! Branches, Exception Generating and System instructions

mod system_regs;

use core::fmt::Display;

use a64_macros::bit_match;
use bitos::integer::{i14, i19, i26, u5};
use bitos::{BitUtils, bitos};
use derive_more::Display;

use crate::{MemOp, Reg, RegWidth, Xr};

pub use system_regs::SystemReg;

/// A kind of hint instruction.
#[bitos(7)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HintKind {
    Nop = 0b0000_000,
    Yield = 0b0000_001,
    WaitForEvent = 0b0000_010,
    WaitForInterrupt = 0b0000_011,
    SendEvent = 0b0000_100,
    SendEventLocal = 0b0000_101,
    DataGathering = 0b0000_110,
    SpeculativeDataBarrier = 0b0010_100,
}

impl Display for HintKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mnemonic = match self {
            Self::Nop => "NOP",
            Self::Yield => "YIELD",
            Self::WaitForEvent => "WFE",
            Self::WaitForInterrupt => "WFI",
            Self::SendEvent => "SEV",
            Self::SendEventLocal => "SEVL",
            Self::DataGathering => "DGH",
            Self::SpeculativeDataBarrier => "CSDB",
        };

        write!(f, "{mnemonic}")
    }
}

/// Hint instruction
#[bitos(32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hint {
    /// The kind of hint.
    #[bits(5..12)]
    pub kind: Option<HintKind>,
}

impl Display for Hint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(kind) = self.kind() {
            write!(f, "{}", kind)
        } else {
            write!(f, "????")
        }
    }
}

/// Condition of a branch.
#[bitos(4)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BranchCondition {
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

impl Display for BranchCondition {
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

/// Branch conditionally
///
/// This instruction branches conditionally to a label at a PC-relative offset. This instruction
/// provides a hint that this is not a subroutine call or return.
#[bitos(32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CondBranch {
    /// The condition of the branch.
    #[bits(0..4)]
    pub cond: BranchCondition,
    /// Offset from the address of this instruction, divided by 4.
    #[bits(5..24)]
    pub imm: i19,
}

impl Display for CondBranch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "B.{} #{}", self.cond(), self.imm().value() * 4)
    }
}

/// Move general-purpose register to/from system register
///
/// This instruction allows the PE to write/read an AArch64 System register from/to a general-purpose
/// register.
#[bitos(32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SysRegMove {
    /// General purpose register to use in the transfer.
    #[bits(0..5)]
    pub rt: Xr,
    /// The system register to use in the transfer.
    #[bits(5..21)]
    pub system_reg: Option<SystemReg>,
    /// Operation to perform.
    #[bits(21)]
    pub op: MemOp,
}

impl Display for SysRegMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.op() {
            MemOp::Store => {
                write!(f, "MSR ")?;

                if let Some(reg) = self.system_reg() {
                    write!(f, "{reg:?}, ")?;
                } else {
                    write!(f, "????, ")?;
                }

                write!(f, "{}", self.rt())
            }
            MemOp::Load => {
                write!(f, "MRS {}, ", self.rt())?;

                if let Some(reg) = self.system_reg() {
                    write!(f, "{reg:?}")
                } else {
                    write!(f, "????")
                }
            }
        }
    }
}

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

/// Branch
///
/// This instruction branches unconditionally to an address in a register.
#[bitos(32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UncondBranchReg {
    /// General purpose register containing the address to branch to.
    #[bits(5..10)]
    pub rn: Xr,
    /// Whether to set X30 to PC+4. If set, also provides a hint that this instruction is a call.
    #[bits(21)]
    pub link: bool,
    /// If set, provides a hint that this instruction is a return.
    #[bits(22)]
    pub ret: bool,
}

impl Display for UncondBranchReg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mnemonic = match (self.link(), self.ret()) {
            (true, true) => "????",
            (true, false) => "BLR",
            (false, true) => "RET",
            (false, false) => "BR",
        };

        write!(f, "{} {}", mnemonic, self.rn())
    }
}

/// Compare and branch on (non)zero
///
/// This instruction compares the value in a register with zero, and conditionally branches to a
/// label at a PC-relative offset if the comparison is (not )equal.
#[bitos(32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmpBranchImm {
    /// General purpose register to be tested.
    #[bits(0..5)]
    pub rt: Reg,
    /// Offset from the address of this instruction, divided by 4.
    #[bits(5..24)]
    pub imm: i19,
    /// Whether to only branch if not zero (i.e. invert the condition).
    #[bits(24)]
    pub not: bool,
    /// Width of the register.
    #[bits(31)]
    pub sf: RegWidth,
}

impl Display for CmpBranchImm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mnemonic = if self.not() { "CBNZ" } else { "CBZ" };

        write!(
            f,
            "{} {}, #{}",
            mnemonic,
            self.rt().with_width(self.sf()),
            self.imm().value() * 4
        )
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
    CondBranch(CondBranch),
    Hint(Hint),
    SysRegMove(SysRegMove),
    UncondBranchReg(UncondBranchReg),
    UncondBranchImm(UncondBranchImm),
    CmpBranchImm(CmpBranchImm),
    TestBranch(TestBranch),
}

impl Instruction {
    fn new_uncond_branch_reg(value: u32) -> Option<Self> {
        let opc = value.bits(21, 25);
        let op2 = value.bits(16, 21);
        let op3 = value.bits(10, 16);
        let rn = value.bits(5, 10);
        let op4 = value.bits(0, 5);

        Some(bit_match! {
            match (opc, op2, op3, rn, op4) {
                ("00__", "11111", "000000", "_____", "00000") => Self::UncondBranchReg(UncondBranchReg(value)),
                ("0100", "11111", "000000", "11111", "00000") => todo!("eret"),
                ("0101", "11111", "000000", "11111", "00000") => todo!("drps"),
                _ => return None,
            }
        })
    }

    pub fn new(value: u32) -> Option<Self> {
        let op0 = value.bits(29, 32);

        // op1: 12..26, split to generate better code with bit_match
        let op1_a = value.bits(16, 26);
        let op1_b = value.bits(12, 16);

        let op2 = value.bits(0, 5);

        Some(bit_match! {
            match (op0, op1_a, op1_b, op2) {
                ("010", "00________", "____", "_____") => Self::CondBranch(CondBranch(value)),
                ("010", "01________", "____", "_____") => todo!("misc branch (immediate)"),

                ("011", "00________", "1___", "_____") => todo!("cmp u8/u16 reg and branch"),

                ("110", "00________", "____", "_____") => todo!("except"),
                ("110", "0100000011", "0001", "_____") => todo!("sys with reg"),
                ("110", "0100000011", "0010", "11111") => Self::Hint(Hint(value)),
                ("110", "0100000011", "0011", "_____") => todo!("barriers"),
                ("110", "0100000___", "0100", "_____") => todo!("pstate"),
                ("110", "0100_01___", "____", "_____") => todo!("sys"),
                ("110", "0100_1____", "____", "_____") => Self::SysRegMove(SysRegMove(value)),
                ("110", "0101_01___", "____", "_____") => todo!("sys pair"),
                ("110", "0101_1____", "____", "_____") => todo!("sys reg pair move"),
                ("110", "1_________", "____", "_____") => Self::new_uncond_branch_reg(value)?,

                ("_00", "__________", "____", "_____") => Self::UncondBranchImm(UncondBranchImm(value)),

                ("_01", "0_________", "____", "_____") => Self::CmpBranchImm(CmpBranchImm(value)),
                ("_01", "1_________", "____", "_____") => Self::TestBranch(TestBranch(value)),

                ("_11", "00________", "00__", "_____") => todo!("cmp regs and branch"),
                ("_11", "01________", "_0__", "_____") => todo!("cmp reg with imm and branch"),
                _ => return None,
            }
        })
    }
}
