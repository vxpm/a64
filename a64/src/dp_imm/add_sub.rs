//! Add/subtract (immediate)

use crate::{WrSp, WrUnk, XrSp, XrUnk};
use a64_macros::bit_match;
use bitos::integer::u12;
use bitos::{BitUtils, bitos};
use core::fmt::Display;
use derive_more::Display;

macro_rules! display {
    ($base:literal) => {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if self.s() {
                write!(
                    f,
                    "{}S {}, {}, {}, #LSL {}",
                    $base,
                    self.rd().with_zr(),
                    self.rn(),
                    self.imm(),
                    if self.sh() { "12" } else { "0" }
                )
            } else {
                write!(
                    f,
                    "{} {}, {}, {}, #LSL {}",
                    $base,
                    self.rd().with_sp(),
                    self.rn(),
                    self.imm(),
                    if self.sh() { "12" } else { "0" }
                )
            }
        }
    };
}

/// Add immediate value (32 bits)
///
/// This instruction adds a register value and an optionally-shifted immediate value, and writes the
/// result to the destination register.
#[bitos(32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Add32 {
    /// Destination register.
    #[bits(0..5)]
    pub rd: WrUnk,
    /// Source register.
    #[bits(5..10)]
    pub rn: WrSp,
    /// Immediate value.
    #[bits(10..22)]
    pub imm: u12,
    /// Whether to left-shift the immediate by 12.
    #[bits(22)]
    pub sh: bool,
    /// Whether to update condition flags based on the result. If set, `rd` uses ZR, otherwise SP.
    #[bits(29)]
    pub s: bool,
}

impl Display for Add32 {
    display!("ADD");
}

/// Subtract immediate value (32 bits)
///
/// This instruction subtracts an optionally-shifted immediate value from a register value, and
/// writes the result to the destination register.
#[bitos(32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sub32 {
    /// Destination register.
    #[bits(0..5)]
    pub rd: WrUnk,
    /// Source register.
    #[bits(5..10)]
    pub rn: WrSp,
    /// Immediate value.
    #[bits(10..22)]
    pub imm: u12,
    /// Whether to left-shift the immediate by 12.
    #[bits(22)]
    pub sh: bool,
    /// Whether to update condition flags based on the result. If set, `rd` uses ZR instead of SP.
    #[bits(29)]
    pub s: bool,
}

impl Display for Sub32 {
    display!("SUB");
}

/// Add immediate value (64 bits)
///
/// This instruction adds a register value and an optionally-shifted immediate value, and writes the
/// result to the destination register.
#[bitos(32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Add64 {
    /// Destination register.
    #[bits(0..5)]
    pub rd: XrUnk,
    /// Source register.
    #[bits(5..10)]
    pub rn: XrSp,
    /// Immediate value.
    #[bits(10..22)]
    pub imm: u12,
    /// Whether to left-shift the immediate by 12.
    #[bits(22)]
    pub sh: bool,
    /// Whether to update condition flags based on the result. If set, `rd` uses ZR instead of SP.
    #[bits(29)]
    pub s: bool,
}

impl Display for Add64 {
    display!("ADD");
}

/// Subtract immediate value (64 bits)
///
/// This instruction subtracts an optionally-shifted immediate value from a register value, and
/// writes the result to the destination register.
#[bitos(32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sub64 {
    /// Destination register.
    #[bits(0..5)]
    pub rd: XrUnk,
    /// Source register.
    #[bits(5..10)]
    pub rn: XrSp,
    /// Immediate value.
    #[bits(10..22)]
    pub imm: u12,
    /// Whether to left-shift the immediate by 12.
    #[bits(22)]
    pub sh: bool,
    /// Whether to update condition flags based on the result. If set, `rd` uses ZR instead of SP.
    #[bits(29)]
    pub s: bool,
}

impl Display for Sub64 {
    display!("SUB");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum Instruction {
    Add32(Add32),
    Sub32(Sub32),
    Add64(Add64),
    Sub64(Sub64),
}

impl Instruction {
    pub fn new(value: u32) -> Option<Self> {
        let op = value.bit(30) as u32;
        let sf = value.bit(31) as u32;

        Some(bit_match! {
            match (op, sf) {
                ("0", "0") => Self::Add32(Add32(value)),
                ("0", "1") => Self::Add64(Add64(value)),
                ("1", "0") => Self::Sub32(Sub32(value)),
                ("1", "1") => Self::Sub64(Sub64(value)),
                _ => return None,
            }
        })
    }
}
