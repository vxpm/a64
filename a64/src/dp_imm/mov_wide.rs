//! Move wide (immediate)

use core::fmt::Display;

use a64_macros::bit_match;
use bitos::integer::u2;
use bitos::{BitUtils, bitos};
use derive_more::Display;

use crate::Xr;

/// Move wide with zero
///
/// This instruction moves an optionally-shifted 16-bit immediate value to a register.
#[bitos(32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Movz64 {
    /// Destination register.
    #[bits(0..5)]
    pub rd: Xr,
    /// Immediate value.
    #[bits(5..21)]
    pub imm: u16,
    /// Amount by which to shift the immediate left, divided by 16.
    #[bits(21..23)]
    pub hw: u2,
}

impl Display for Movz64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MOVZ {}, #{}, LSL #{}",
            self.rd(),
            self.imm(),
            self.hw().value() * 16,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum Instruction {
    Movz64(Movz64),
}

impl Instruction {
    pub fn new(value: u32) -> Option<Self> {
        let sf = value.bit(31) as u32;
        let opc = value.bits(29, 31);
        let hw = value.bits(21, 23);

        Some(bit_match! {
            match (sf, opc, hw) {
                ("0", "00", "0_") => todo!("movn 32 bit"),
                ("0", "10", "0_") => todo!("movz 32 bit"),
                ("0", "11", "0_") => todo!("movk 32 bit"),

                ("1", "00", "__") => todo!("movn 64 bit"),
                ("1", "10", "__") => Self::Movz64(Movz64(value)),
                ("1", "11", "__") => todo!("movk 64 bit"),
                _ => return None,
            }
        })
    }
}
