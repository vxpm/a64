//! PC-rel. addressing

use core::fmt::Display;

use bitos::integer::{u2, u19};
use bitos::{BitUtils, bitos};
use derive_more::Display;

use crate::Xr;

/// Form PC-relative address
///
/// This instruction adds an immediate value to the PC value to form a PC-relative address, and
/// writes the result to the destination register.
#[bitos(32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Adr {
    /// Destination register.
    #[bits(0..5)]
    pub rd: Xr,
    /// Immediate value high.
    #[bits(5..24)]
    pub immhi: u19,
    /// Immediate value low.
    #[bits(29..31)]
    pub immlo: u2,
}

impl Display for Adr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let imm = 0.with_bits(0, 2, self.immlo().value() as u64).with_bits(
            2,
            21,
            self.immhi().value() as u64,
        );

        write!(f, "ADR {}, #{}", self.rd(), imm)
    }
}

/// Form PC-relative address to 4KB page
///
/// This instruction adds an immediate value that is shifted left by 12 bits, to the PC value to
/// form a PC-relative address, with the bottom 12 bits masked out, and writes the result to the
/// destination register.
#[bitos(32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Adrp {
    /// Destination register.
    #[bits(0..5)]
    pub rd: Xr,
    /// Immediate value high.
    #[bits(5..24)]
    pub immhi: u19,
    /// Immediate value low.
    #[bits(29..31)]
    pub immlo: u2,
}

impl Display for Adrp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let imm = 0.with_bits(0, 2, self.immlo().value() as u64).with_bits(
            2,
            21,
            self.immhi().value() as u64,
        ) << 12;

        write!(f, "ADRP {}, #{}", self.rd(), imm)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum Instruction {
    Adr(Adr),
    Adrp(Adrp),
}

impl Instruction {
    pub fn new(value: u32) -> Option<Self> {
        let op = value.bit(31);

        Some(if op {
            Self::Adrp(Adrp(value))
        } else {
            Self::Adr(Adr(value))
        })
    }
}
