//! Load/store register pair (offset)

use core::fmt::Display;

use a64_macros::bit_match;
use bitos::BitUtils;
use derive_more::{Deref, Display};

use crate::load_store::Repr64;

/// Load pair of registers
///
/// This instruction calculates an address from a base register value and an immediate offset, loads
/// two 32-bit words or two 64-bit doublewords from memory, and writes them to two registers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deref)]
pub struct Ldp64(pub Repr64);

impl Display for Ldp64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LDP {}, {}, [{}, #{}]",
            self.rt1(),
            self.rt2(),
            self.rtn(),
            self.imm().value() as i16 * 8
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum Instruction {
    Ldp64(Ldp64),
}

impl Instruction {
    pub fn new(value: u32) -> Option<Self> {
        let opc = value.bits(30, 32);
        let vr = value.bit(26) as u32;
        let l = value.bit(22) as u32;

        Some(bit_match! {
            match (opc, vr, l) {
                ("00", "0", "0") => todo!("stp 32 bit"),
                ("00", "0", "1") => todo!("ldp 32 bit"),
                ("00", "1", "0") => todo!("stp simd/fp 32 bit"),
                ("00", "1", "1") => todo!("ldp simd/fp 32 bit"),

                ("01", "0", "0") => todo!("stgp"),
                ("01", "0", "1") => todo!("ldpsw"),
                ("01", "1", "0") => todo!("stp simd/fp 64 bit"),
                ("01", "1", "1") => todo!("ldp simd/fp 64 bit"),

                ("10", "0", "0") => todo!("stp 64 bit"),
                ("10", "0", "1") => Self::Ldp64(Ldp64(Repr64(value))),
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
