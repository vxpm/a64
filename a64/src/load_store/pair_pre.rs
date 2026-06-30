//! Load/store register pair (pre-indexed)

use core::fmt::Display;

use a64_macros::bit_match;
use bitos::integer::i7;
use bitos::{BitUtils, bitos};
use derive_more::Display;

use crate::{Xr, XrSp};

/// Store pair of registers
///
/// This instruction calculates an address from a base register value and an immediate offset, and
/// stores two 64-bit doublewords to the calculated address, from two registers.
#[bitos(32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Stp64 {
    /// First general-purpose register to be stored.
    #[bits(0..5)]
    pub rt1: Xr,
    /// The general-purpose base register.
    #[bits(5..10)]
    pub rtn: XrSp,
    /// Second general-purpose register to be stored.
    #[bits(10..15)]
    pub rt2: Xr,
    /// Offset divided by 8.
    #[bits(15..22)]
    pub imm: i7,
}

impl Display for Stp64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "STP {}, {}, [{}, #{}]!",
            self.rt1(),
            self.rt2(),
            self.rtn(),
            self.imm().value() as i16 * 8
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum Instruction {
    Stp64(Stp64),
}

impl Instruction {
    pub fn new(value: u32) -> Option<Self> {
        let opc = value.bits(30, 32);
        let vec = value.bit(26) as u32;
        let load = value.bit(22) as u32;

        Some(bit_match! {
            match (opc, vec, load) {
                ("00", "0", "0") => todo!("stp 32 bit"),
                ("00", "0", "1") => todo!("ldp 32 bit"),
                ("00", "1", "0") => todo!("stp simd/fp 32 bit"),
                ("00", "1", "1") => todo!("ldp simd/fp 32 bit"),

                ("01", "0", "0") => todo!("stgp"),
                ("01", "0", "1") => todo!("ldpsw"),
                ("01", "1", "0") => todo!("stp simd/fp 64 bit"),
                ("01", "1", "1") => todo!("ldp simd/fp 64 bit"),

                ("10", "0", "0") => Self::Stp64(Stp64(value)),
                ("10", "0", "1") => todo!("ldp 64 bit"),
                ("10", "1", "0") => todo!("stp simd/fp 128 bit"),
                ("10", "1", "1") => todo!("ldp simd/fp 128 bit"),

                ("11", "0", "0") => todo!("sttp"),
                ("11", "0", "1") => todo!("ldtp"),
                ("11", "1", "0") => todo!("sttp simd/fp"),
                ("11", "1", "1") => todo!("ldtp simd/fp"),
                _ => return None,
            }
        })
    }
}
