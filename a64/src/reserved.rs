//! Reserved

use a64_macros::bit_match;
use bitos::{BitUtils, bitos};
use core::fmt::Display;
use derive_more::Display;

#[bitos(32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Undefined {
    #[bits(0..16)]
    pub imm: u16,
}

impl Display for Undefined {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UDF #{}", self.imm())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum Instruction {
    Undefined(Undefined),
}

impl Instruction {
    pub fn new(value: u32) -> Option<Self> {
        let op0 = value.bits(29, 31);
        let op1 = value.bits(15, 24);

        Some(bit_match! {
            match (op0, op1) {
                ("0", "000000000") => Self::Undefined(Undefined(value)),
                _ => return None,
            }
        })
    }
}
