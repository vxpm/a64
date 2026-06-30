//! Logical (immediate)

use core::fmt::Display;

use a64_macros::bit_match;
use bitos::integer::u6;
use bitos::{BitUtils, bitos};
use derive_more::Display;

use crate::{Xr, XrSp};

pub fn decode_logical_imm(_n: bool, imms: u6, immr: u6) -> u64 {
    let imms = imms.value();
    let immr = immr.value();

    let elem_size: u8 = bit_match! {
        match imms {
            "11110_" => 2,
            "1110__" => 4,
            "110___" => 8,
            "10____" => 16,
            "0_____" => 32,
            "______" => 64,
        }
    };

    let elem_ones = imms.bits(0, elem_size.ilog2() as u8) + 1;
    let elem_pattern = (1u64 << elem_ones) - 1;
    let elem_count = 64 / elem_size;

    let mut pattern = 0;
    for i in 0..elem_count {
        pattern.set_bits(i * elem_size, (i + 1) * elem_size, elem_pattern);
    }

    let immr_mask = 0.with_bits(0, elem_size, !0);
    pattern.rotate_right(immr as u32 & immr_mask)
}

/// Bitwise OR (immediate)
///
/// This instruction performs a bitwise (inclusive) OR between a register value and an immediate
/// value, and writes the result to the destination register.
#[bitos(32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Orr64 {
    /// Destination register.
    #[bits(0..5)]
    pub rd: XrSp,
    /// Source register.
    #[bits(5..10)]
    pub rn: Xr,
    /// Bitmask immediate pattern.
    #[bits(10..16)]
    pub imms: u6,
    /// Bitmask immediate rotation.
    #[bits(16..22)]
    pub immr: u6,
    /// Whether the immediate is 64 bit wide instead of 32 bit.
    #[bits(22)]
    pub n: bool,
}

impl Display for Orr64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let imm = decode_logical_imm(self.n(), self.imms(), self.immr());
        write!(f, "ORR {}, {}, #{}", self.rd(), self.rn(), imm)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum Instruction {
    Orr64(Orr64),
}

impl Instruction {
    pub fn new(value: u32) -> Option<Self> {
        let sf = value.bit(31) as u32;
        let opc = value.bits(29, 31);
        let n = value.bit(22) as u32;

        Some(bit_match! {
            match (sf, opc, n) {
                ("0", "00", "0") => todo!("and 32 bit"),
                ("0", "01", "0") => todo!("orr 32 bit"),
                ("0", "10", "0") => todo!("eor 32 bit"),
                ("0", "11", "0") => todo!("ands 32 bit"),

                ("1", "00", "0") => todo!("and 64 bit"),
                ("1", "01", "0") => Self::Orr64(Orr64(value)),
                ("1", "10", "0") => todo!("eor 64 bit"),
                ("1", "11", "0") => todo!("ands 64 bit"),
                _ => return None,
            }
        })
    }
}
