//! Data Processing - Register

use core::fmt::Display;

use a64_macros::bit_match;
use bitos::integer::{u2, u3, u4, u5, u6};
use bitos::{BitUtils, bitos};
use derive_more::Display;

use crate::{Condition, DataSize, LogicalOp, Reg, RegSp, RegUnk, RegWidth, ShiftKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TwoSrcOp {
    UnsignedDiv,
    SignedDiv,
    Shift,
    Crc32,
    Crc32C,
    // These are all FEAT_CSSC (v8.7):
    // SignedMax,
    // UnsignedMax,
    // SignedMin,
    // UnsignedMin,
    Reserved,
}

/// Two source operation
///
/// This instruction performs an operation on two source registers and writes the result to a
/// destination register.
#[bitos(32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TwoSrc {
    /// Destination register.
    #[bits(0..5)]
    pub rd: Reg,
    /// Source register 1.
    #[bits(5..10)]
    pub rn: Reg,
    /// Operation info.
    #[bits(10..16)]
    pub opcode: u6,
    /// Shift kind for shift operation.
    #[bits(10..12)]
    pub shift_kind: ShiftKind,
    /// Size for CRC32 operation.
    #[bits(10..12)]
    pub sz: DataSize,
    /// Source register 2.
    #[bits(16..21)]
    pub rm: Reg,
    /// Operation info.
    #[bits(29)]
    pub s: bool,
    /// Width of the registers.
    #[bits(31)]
    pub sf: RegWidth,
}

impl TwoSrc {
    /// Operation to perform.
    pub fn op(self) -> TwoSrcOp {
        if self.s() {
            return TwoSrcOp::Reserved;
        }

        let opcode = self.opcode().value();
        bit_match! {
            match opcode {
                "000010" => TwoSrcOp::UnsignedDiv,
                "000011" => TwoSrcOp::SignedDiv,
                "0010__" => TwoSrcOp::Shift,
                "0100__" => TwoSrcOp::Crc32,
                "0101__" => TwoSrcOp::Crc32C,
                // "011000" => TwoSrcOp::SignedMax,
                // "011001" => TwoSrcOp::UnsignedMax,
                // "011010" => TwoSrcOp::SignedMin,
                // "011011" => TwoSrcOp::UnsignedMin,
                _ => TwoSrcOp::Reserved,
            }
        }
    }
}

impl Display for TwoSrc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mnemonic = match self.op() {
            TwoSrcOp::UnsignedDiv => "UDIV",
            TwoSrcOp::SignedDiv => "SDIV",
            TwoSrcOp::Shift => match self.shift_kind() {
                ShiftKind::LogicLeft => "LSLV",
                ShiftKind::LogicRight => "LSRV",
                ShiftKind::ArithRight => "ASRV",
                ShiftKind::RotateRight => "RORV",
            },
            TwoSrcOp::Crc32 => &format!("CRC32{}", self.sz()),
            TwoSrcOp::Crc32C => &format!("CRC32C{}", self.sz()),
            // TwoSrcOp::SignedMax => "SMAX",
            // TwoSrcOp::UnsignedMax => "UMAX",
            // TwoSrcOp::SignedMin => "SMIN",
            // TwoSrcOp::UnsignedMin => "UMIN",
            TwoSrcOp::Reserved => "????",
        };

        let rd_rn_width = match self.op() {
            TwoSrcOp::Crc32 | TwoSrcOp::Crc32C => RegWidth::W32,
            _ => self.sf(),
        };

        write!(
            f,
            "{} {}, {}, {}",
            mnemonic,
            self.rd().with_width(rd_rn_width),
            self.rn().with_width(rd_rn_width),
            self.rm().with_width(self.sf()),
        )
    }
}

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

/// Conditional compare (immediate)
///
/// This instruction sets the value of the condition flags to the result of the comparison of a
/// register value and a (possibly negated) immediate value if the condition is TRUE, and an
/// immediate value otherwise.
#[bitos(32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CondCmpImm {
    /// Flag bit specifier.
    #[bits(0..4)]
    pub nzcv: u4,
    /// Source register.
    #[bits(5..10)]
    pub rn: Reg,
    /// Condition to test in comparison.
    #[bits(12..16)]
    pub cond: Condition,
    /// Source register 2.
    #[bits(16..21)]
    pub imm: u5,
    /// If set, don't negate the immediate.
    #[bits(30)]
    pub not_neg: bool,
    /// Width of the registers.
    #[bits(31)]
    pub sf: RegWidth,
}

impl Display for CondCmpImm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mnemonic = if self.not_neg() { "CCMP" } else { "CCMN" };
        write!(
            f,
            "{} {}, #{}, #{}, {}",
            mnemonic,
            self.rn().with_width(self.sf()),
            self.imm(),
            self.nzcv(),
            self.cond()
        )
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreeSrcOp {
    MulAdd,
    MulSub,
    SignedMulAddLong,
    SignedMulSubLong,
    SignedMulHigh,
    UnsignedMulAddLong,
    UnsignedMulSubLong,
    UnsignedMulHigh,
    Reserved,
}

impl Display for ThreeSrcOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mnemonic = match self {
            Self::MulAdd => "MADD",
            Self::MulSub => "MSUB",
            Self::SignedMulAddLong => "SMADDL",
            Self::SignedMulSubLong => "SMSUBL",
            Self::SignedMulHigh => "SMULH",
            Self::UnsignedMulAddLong => "UMADDL",
            Self::UnsignedMulSubLong => "UMSUBL",
            Self::UnsignedMulHigh => "UMULH",
            Self::Reserved => "????",
        };

        write!(f, "{mnemonic}")
    }
}

impl ThreeSrcOp {
    pub fn uses_ra(self) -> bool {
        match self {
            Self::SignedMulHigh => false,
            Self::UnsignedMulHigh => false,
            _ => true,
        }
    }
}

/// Three source operation
///
/// This instruction performs an operation on three source registers and writes the result to a
/// destination register.
#[bitos(32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThreeSrc {
    /// Destination register.
    #[bits(0..5)]
    pub rd: Reg,
    /// Source register 1.
    #[bits(5..10)]
    pub rn: Reg,
    /// Source register 3.
    #[bits(10..15)]
    pub ra: Reg,
    /// Operation info.
    #[bits(15)]
    pub op0: bool,
    /// Source register 2.
    #[bits(16..21)]
    pub rm: Reg,
    /// Operation info.
    #[bits(21..24)]
    pub op31: u3,
    /// Width of the registers.
    #[bits(31)]
    pub sf: RegWidth,
}

impl ThreeSrc {
    pub fn op(self) -> ThreeSrcOp {
        let op31 = self.op31().value();
        let op0 = self.op0() as u32;

        bit_match! {
            match (op31, op0) {
                ("000", "0") => ThreeSrcOp::MulAdd,
                ("000", "1") => ThreeSrcOp::MulSub,
                ("001", "0") => ThreeSrcOp::SignedMulAddLong,
                ("001", "1") => ThreeSrcOp::SignedMulSubLong,
                ("010", "0") => ThreeSrcOp::SignedMulHigh,
                ("101", "0") => ThreeSrcOp::UnsignedMulAddLong,
                ("101", "1") => ThreeSrcOp::UnsignedMulSubLong,
                ("110", "0") => ThreeSrcOp::UnsignedMulHigh,
                _ => ThreeSrcOp::Reserved,
            }
        }
    }
}

impl Display for ThreeSrc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = self.op();

        let n_m_width = match op {
            ThreeSrcOp::SignedMulAddLong => RegWidth::W32,
            ThreeSrcOp::SignedMulSubLong => RegWidth::W32,
            ThreeSrcOp::UnsignedMulAddLong => RegWidth::W32,
            ThreeSrcOp::UnsignedMulSubLong => RegWidth::W32,
            _ => self.sf(),
        };

        write!(
            f,
            "{} {}, {}, {}",
            self.op(),
            self.rd().with_width(self.sf()),
            self.rn().with_width(n_m_width),
            self.rm().with_width(n_m_width),
        )?;

        if op.uses_ra() {
            write!(f, ", {}", self.ra().with_width(self.sf()))?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum Instruction {
    TwoSrc(TwoSrc),
    Logical(Logical),
    AddSubShifted(AddSubShifted),
    AddSubExt(AddSubExt),
    CondCmpImm(CondCmpImm),
    CondSelect(CondSelect),
    ThreeSrc(ThreeSrc),
}

impl Instruction {
    pub fn new_two_src(value: u32) -> Option<Self> {
        let s = value.bit(29);
        if s {
            return None;
        }

        let sf = value.bit(31) as u32;
        let opcode = value.bits(10, 16);
        Some(bit_match! {
            match (sf, opcode) {
                // udiv/sdiv
                ("_", "000010") => Self::TwoSrc(TwoSrc(value)),
                ("_", "000011") => Self::TwoSrc(TwoSrc(value)),

                // shift
                ("_", "0010__") => Self::TwoSrc(TwoSrc(value)),

                // crc32
                ("0", "010011") => return None,
                ("0", "0100__") => Self::TwoSrc(TwoSrc(value)),
                ("1", "010011") => Self::TwoSrc(TwoSrc(value)),

                // crc32c
                ("0", "010111") => return None,
                ("0", "0101__") => Self::TwoSrc(TwoSrc(value)),
                ("1", "010111") => Self::TwoSrc(TwoSrc(value)),
                _ => return None,
            }
        })
    }

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

    pub fn new_cond_cmp_imm(value: u32) -> Option<Self> {
        let s = value.bit(29) as u32;
        let op2 = value.bit(10) as u32;
        let op3 = value.bit(4) as u32;

        Some(bit_match! {
            match (s, op2, op3) {
                ("0", "_", "_") => return None,
                ("1", "0", "1") => return None,
                ("1", "1", "_") => return None,
                _ => Self::CondCmpImm(CondCmpImm(value)),
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

    pub fn new_three_src(value: u32) -> Option<Self> {
        let sf = value.bit(31) as u32;
        let op54 = value.bits(29, 31);
        let op31 = value.bits(21, 24);
        let op0 = value.bit(15) as u32;

        if op54 != 0 {
            return None;
        }

        Some(bit_match! {
            match (sf, op31, op0) {
                ("_", "000", "_") => Self::ThreeSrc(ThreeSrc(value)),
                ("1", "001", "_") => Self::ThreeSrc(ThreeSrc(value)),
                ("1", "010", "0") => Self::ThreeSrc(ThreeSrc(value)),
                // ("1", "011", "_") => todo!("maddpt/msubpt"),
                ("1", "101", "_") => Self::ThreeSrc(ThreeSrc(value)),
                ("1", "110", "0") => Self::ThreeSrc(ThreeSrc(value)),
                _ => return None,
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
                ("0", "1", "0110", "______") => Self::new_two_src(value)?,
                ("1", "1", "0110", "______") => todo!("one src"),
                ("_", "0", "0___", "______") => Self::new_logical(value)?,
                ("_", "0", "1__0", "______") => Self::new_add_sub_shifted(value)?,
                ("_", "0", "1__1", "______") => Self::new_add_sub_ext(value)?,
                ("_", "1", "0000", "000000") => todo!("add/sub (with carry)"),
                ("_", "1", "0000", "001___") => todo!("add/sub (checked ptr)"),
                ("_", "1", "0000", "_00001") => todo!("rotate right into flags"),
                ("_", "1", "0000", "__0010") => todo!("eval into flags"),
                ("_", "1", "0010", "____0_") => todo!("cond cmp (reg)"),
                ("_", "1", "0010", "____1_") => Self::new_cond_cmp_imm(value)?,
                ("_", "1", "0100", "______") => Self::new_cond_select(value)?,
                ("_", "1", "1___", "______") => Self::new_three_src(value)?,
                _ => return None,
            }
        })
    }
}
