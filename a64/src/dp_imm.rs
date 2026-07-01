//! Data Processing - Immediate

pub mod logical;

use core::fmt::Display;

use a64_macros::bit_match;
use bitos::integer::{u2, u12, u19};
use bitos::{BitUtils, bitos};
use derive_more::Display;

use crate::{Reg, RegSp, RegUnk, RegWidth, Xr};

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

/// Specifies the behaviour of a MovWide.
#[bitos(2)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MovWideKind {
    /// Same as [`Zero`](MovWideKind::Zero), but inverts the register at the end.
    Not = 0b00,
    Reserved = 0b01,
    /// Moves the 16-bit immediate and zeroes other bits.
    Zero = 0b10,
    /// Moves the 16-bit immediate and leaves other bits untouched.
    Keep = 0b11,
}

/// Move wide
///
/// This instruction moves an optionally-shifted 16-bit immediate value to a register.
#[bitos(32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MovWide {
    /// Destination register.
    #[bits(0..5)]
    pub rd: Reg,
    /// Immediate value.
    #[bits(5..21)]
    pub imm: u16,
    /// Amount by which to shift the immediate left, divided by 16.
    #[bits(21..23)]
    pub hw: u2,
    /// Specifies the behaviour.
    #[bits(29..31)]
    pub kind: MovWideKind,
    /// Width of the registers.
    #[bits(31)]
    pub sf: RegWidth,
}

impl Display for MovWide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let suffix = match self.kind() {
            MovWideKind::Not => "N",
            MovWideKind::Reserved => "?",
            MovWideKind::Zero => "Z",
            MovWideKind::Keep => "K",
        };

        write!(
            f,
            "MOV{} {}, #{}, LSL #{}",
            suffix,
            self.rd().with_width(self.sf()),
            self.imm(),
            self.hw().value() * 16,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum Instruction {
    PcRelAddr(PcRelAddr),
    AddSub(AddSub),
    Logical(logical::Instruction),
    MovWide(MovWide),
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
                ("__", "101_") => Self::MovWide(MovWide(value)),
                ("__", "110_") => todo!("bitfield"),
                ("__", "111_") => todo!("extract"),
                _ => return None,
            }
        })
    }
}
