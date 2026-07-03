//! Data Processing - Register

use core::fmt::Display;

use a64_macros::bit_match;
use bitos::integer::{u2, u3, u6};
use bitos::{BitUtils, bitos};
use derive_more::Display;

use crate::{Condition, DataSize, LogicalOp, Reg, RegSp, RegUnk, RegWidth, ShiftKind};

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
            (LogicalOp::Ands, true) => "BICS",
            (LogicalOp::Ands, false) => "ANDS",
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

/// Add/subtract optionally-shifted register
///
/// This instruction adds/subtracts a register value and an optionally-shifted register value, and
/// writes the result to the destination register.
#[bitos(32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AddSubShifted {
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
    /// The kind of shift.
    #[bits(22..24)]
    pub shift: ShiftKind,
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

impl Display for AddSubShifted {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mnemonic = if self.sub() { "SUB" } else { "ADD" };
        let flags = if self.s() { "S" } else { "" };

        write!(
            f,
            "{}{} {}, {}, {}, {} #{}",
            mnemonic,
            flags,
            self.rd().with_width(self.sf()),
            self.rn().with_width(self.sf()),
            self.rm().with_width(self.sf()),
            self.shift(),
            self.imm()
        )
    }
}

/// Add/subtract extended and scaled register
///
/// This instruction adds/subtracts a register value and a sign or zero-extended register value,
/// followed by an optional left shift amount, and writes the result to the destination register.
#[bitos(32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AddSubExt {
    /// Destination register.
    #[bits(0..5)]
    pub rd: RegUnk,
    /// Source register 1.
    #[bits(5..10)]
    pub rn: RegSp,
    /// Shift amount.
    #[bits(10..13)]
    pub imm: u3,
    /// Size of the argument in source register 2.
    #[bits(13..15)]
    pub data_size: DataSize,
    /// Whether to perform sign-extension instead of zero-extension.
    #[bits(15)]
    pub sign_extend: bool,
    /// Source register 2.
    #[bits(16..21)]
    pub rm: Reg,
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

impl Display for AddSubExt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mnemonic = if self.sub() { "SUB" } else { "ADD" };
        let shift = if self.sign_extend() { "S" } else { "U" };
        let shift_amount = self.imm();

        if self.s() {
            write!(
                f,
                "{}S {}, {}, {}, {}XT{} #{}",
                mnemonic,
                self.rd().with_zr().with_width(self.sf()),
                self.rn().with_width(self.sf()),
                self.rm().with_width(self.sf()),
                shift,
                self.data_size(),
                shift_amount
            )
        } else {
            write!(
                f,
                "{} {}, {}, {}, {}XT{} #{}",
                mnemonic,
                self.rd().with_sp().with_width(self.sf()),
                self.rn().with_width(self.sf()),
                self.rm().with_width(self.sf()),
                shift,
                self.data_size(),
                shift_amount
            )
        }
    }
}

/// Specifies the operation a [`CondSelect`] performs on the value coming from the second source
/// register.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CondSelectOp {
    /// Keep the value unmodified.
    None,
    /// Increment the value.
    Increment,
    /// Invert the value.
    Invert,
    /// Negate the value.
    Negate,
    /// Reserved operation.
    Reserved,
}

/// Conditional select
///
/// This instruction writes the value of the first source register to the destination register if
/// the condition is TRUE. If the condition is FALSE, it writes the value of the second source
/// register to the destination register, possibly modified.
#[bitos(32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CondSelect {
    /// Destination register.
    #[bits(0..5)]
    pub rd: Reg,
    /// Source register 1.
    #[bits(5..10)]
    pub rn: Reg,
    /// Operation code 2.
    #[bits(10..12)]
    pub op2: u2,
    /// Condition to test for selection.
    #[bits(12..16)]
    pub cond: Condition,
    /// Source register 2.
    #[bits(16..21)]
    pub rm: Reg,
    /// Operation code 1.
    #[bits(30)]
    pub op1: bool,
    /// Width of the registers.
    #[bits(31)]
    pub sf: RegWidth,
}

impl CondSelect {
    pub fn op(self) -> CondSelectOp {
        bit_match! {
            match (
                self.op1() as u32,
                self.op2().value(),
            ) {
                ("0", "_0") => CondSelectOp::None,
                ("0", "_1") => CondSelectOp::Increment,
                ("1", "_0") => CondSelectOp::Invert,
                ("1", "_1") => CondSelectOp::Negate,
                _ => CondSelectOp::Reserved,
            }
        }
    }
}

impl Display for CondSelect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mnemonic = match self.op() {
            CondSelectOp::None => "CSEL",
            CondSelectOp::Increment => "CSINC",
            CondSelectOp::Invert => "CSINV",
            CondSelectOp::Negate => "CSNEG",
            CondSelectOp::Reserved => "????",
        };

        write!(
            f,
            "{} {}, {}, {}, {}",
            mnemonic,
            self.rd().with_width(self.sf()),
            self.rn().with_width(self.sf()),
            self.rm().with_width(self.sf()),
            self.cond()
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum Instruction {
    Logical(Logical),
    AddSubShifted(AddSubShifted),
    AddSubExt(AddSubExt),
    CondSelect(CondSelect),
}

impl Instruction {
    pub fn new_logical(value: u32) -> Option<Self> {
        let logical = Logical(value);
        if !logical.sf().is_64_bits() && logical.imm().value() >= 32 {
            None
        } else {
            Some(Self::Logical(logical))
        }
    }

    pub fn new_add_sub_shifted(value: u32) -> Option<Self> {
        let add_sub = AddSubShifted(value);
        if add_sub.shift() == ShiftKind::RotateRight {
            None
        } else if !add_sub.sf().is_64_bits() && add_sub.imm().value() >= 32 {
            None
        } else {
            Some(Self::AddSubShifted(add_sub))
        }
    }

    pub fn new_add_sub_ext(value: u32) -> Option<Self> {
        let add_sub = AddSubExt(value);
        if add_sub.imm().value() > 4 {
            return None;
        }

        let opt = value.bits(22, 24);
        Some(bit_match! {
            match opt {
                "00" => Self::AddSubExt(add_sub),
                "__" => return None,
            }
        })
    }

    pub fn new_cond_select(value: u32) -> Option<Self> {
        let s = value.bit(29) as u32;
        let op2 = value.bits(10, 12);

        Some(bit_match! {
            match (s, op2) {
                ("0", "1_") => return None,
                ("1", "__") => return None,
                _ => Self::CondSelect(CondSelect(value)),
            }
        })
    }

    pub fn new(value: u32) -> Option<Self> {
        let op0 = value.bit(30) as u32;
        let op1 = value.bit(28) as u32;
        let op2 = value.bits(21, 25);
        let op3 = value.bits(10, 16);

        Some(bit_match! {
            match (op0, op1, op2, op3) {
                ("0", "1", "0110", "______") => todo!("two src"),
                ("1", "1", "0110", "______") => todo!("one src"),
                ("_", "0", "0___", "______") => Self::new_logical(value)?,
                ("_", "0", "1__0", "______") => Self::new_add_sub_shifted(value)?,
                ("_", "0", "1__1", "______") => Self::new_add_sub_ext(value)?,
                ("_", "1", "0000", "000000") => todo!("add/sub (with carry)"),
                ("_", "1", "0000", "001___") => todo!("add/sub (checked ptr)"),
                ("_", "1", "0000", "_00001") => todo!("rotate right into flags"),
                ("_", "1", "0000", "__0010") => todo!("eval into flags"),
                ("_", "1", "0010", "____0_") => todo!("cond cmp (reg)"),
                ("_", "1", "0010", "____1_") => todo!("cond cmp (imm)"),
                ("_", "1", "0100", "______") => Self::new_cond_select(value)?,
                ("_", "1", "1___", "______") => todo!("three src"),
                _ => return None,
            }
        })
    }
}
