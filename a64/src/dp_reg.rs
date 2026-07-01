//! Data Processing - Register

use core::fmt::Display;

use a64_macros::bit_match;
use bitos::integer::u6;
use bitos::{BitUtils, bitos};
use derive_more::Display;

use crate::{LogicalOp, Reg, RegWidth, ShiftKind};

/// Logical operation
///
/// This instruction performs a bitwise operation between a register value and an optionally-shifted
/// register value, and writes the result to the destination register.
#[bitos(32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Logical {
    /// Destination register.
    #[bits(0..5)]
    pub rd: Reg,
    /// Source register 1.
    #[bits(5..10)]
    pub rn: Reg,
    /// Shift amount.
    #[bits(10..16)]
    pub imm: u6,
    /// Source register 2.
    #[bits(16..21)]
    pub rm: Reg,
    /// Whether to invert source register 2 after shifting.
    #[bits(21)]
    pub n: bool,
    /// The kind of shift.
    #[bits(22..24)]
    pub shift: ShiftKind,
    /// The bitwise operation.
    #[bits(29..31)]
    pub op: LogicalOp,
    /// Width of the registers.
    #[bits(31)]
    pub sf: RegWidth,
}

impl Display for Logical {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mnemonic = match (self.op(), self.n()) {
            (LogicalOp::And, true) => "BIC",
            (LogicalOp::And, false) => "AND",
            (LogicalOp::Or, true) => "ORN",
            (LogicalOp::Or, false) => "ORR",
            (LogicalOp::Xor, true) => "EON",
            (LogicalOp::Xor, false) => "EOR",
            (LogicalOp::Ands, true) => "ANDS",
            (LogicalOp::Ands, false) => "BICS",
        };

        write!(
            f,
            "{} {}, {}, {}, {} #{}",
            mnemonic,
            self.rd().with_width(self.sf()),
            self.rn().with_width(self.sf()),
            self.rm().with_width(self.sf()),
            self.shift(),
            self.imm()
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum Instruction {
    Logical(Logical),
}

impl Instruction {
    pub fn new(value: u32) -> Option<Self> {
        let op0 = value.bit(30) as u32;
        let op1 = value.bit(28) as u32;
        let op2 = value.bits(21, 25);
        let op3 = value.bits(10, 16);

        Some(bit_match! {
            match (op0, op1, op2, op3) {
                ("0", "1", "0110", "______") => todo!("two src"),
                ("1", "1", "0110", "______") => todo!("one src"),
                ("_", "0", "0___", "______") => Self::Logical(Logical(value)),
                ("_", "0", "1__0", "______") => todo!("add/sub (shifted reg)"),
                ("_", "0", "1__1", "______") => todo!("add/sub (extended reg)"),
                ("_", "1", "0000", "000000") => todo!("add/sub (with carry)"),
                ("_", "1", "0000", "001___") => todo!("add/sub (checked ptr)"),
                ("_", "1", "0000", "_00001") => todo!("rotate right into flags"),
                ("_", "1", "0000", "__0010") => todo!("eval into flags"),
                ("_", "1", "0010", "____0_") => todo!("cond cmp (reg)"),
                ("_", "1", "0010", "____1_") => todo!("cond cmp (imm)"),
                ("_", "1", "0100", "______") => todo!("cond select"),
                ("_", "1", "1___", "______") => todo!("three src"),
                _ => todo!(),
            }
        })
    }
}
