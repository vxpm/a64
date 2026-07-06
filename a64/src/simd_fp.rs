//! Scalar Floating-Point and Advanced SIMD

use core::fmt::Display;

use a64_macros::bit_match;
use bitos::integer::{u3, u4, u5};
use bitos::{BitUtils, bitos};
use derive_more::Display;

use crate::{SimdReg, SimdRegScalar, SimdRegWidth};

/// Move immediate (vector)
///
/// This instruction places an immediate constant into every vector element of the destination SIMD
/// & FP register.
#[bitos(32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveImm {
    /// The SIMD destination register.
    #[bits(0..5)]
    pub rd: SimdReg,
    /// Lower bits of the immediate (d:e:f:g:h).
    #[bits(5..10)]
    pub immlo: u5,
    /// Operation info.
    #[bits(12..16)]
    pub cmode: u4,
    /// Higher bits of the immediate (a:b:c).
    #[bits(16..19)]
    pub immhi: u3,
    /// Operation info.
    #[bits(29)]
    pub op: bool,
    /// Width of the SIMD register.
    #[bits(30)]
    pub q: SimdRegWidth,
}

pub enum MoveImmOp {
    B8,
    B16,
    B32,
    B32MSL,
    B64,
    Reserved,
}

impl MoveImm {
    pub fn operation(self) -> MoveImmOp {
        let op = self.op() as u32;
        let cmode = self.cmode().value();

        bit_match! {
            match (op, cmode) {
                ("0", "1110") => MoveImmOp::B8,
                ("0", "10_0") => MoveImmOp::B16,
                ("0", "0__0") => MoveImmOp::B32,
                ("0", "110_") => MoveImmOp::B32MSL,
                ("1", "1110") => MoveImmOp::B64,
                _ => MoveImmOp::Reserved,
            }
        }
    }

    pub fn imm8(self) -> u8 {
        0.with_bits(0, 5, self.immlo().value())
            .with_bits(5, 8, self.immhi().value())
    }

    pub fn imm64(self) -> u64 {
        let imm = self.imm8();

        let mut result = 0;
        for i in 0..8 {
            let byte = if imm.bit(i) { !0 } else { 0 };
            result.set_bits(i * 8, (i + 1) * 8, byte);
        }

        result
    }

    pub fn shift_amount(self) -> u8 {
        match self.operation() {
            MoveImmOp::B8 => 0,
            MoveImmOp::B16 => {
                if self.cmode().bit(1) {
                    8
                } else {
                    0
                }
            }
            MoveImmOp::B32 => self.cmode().bits(1, 3).value() * 8,
            MoveImmOp::B32MSL => {
                if self.cmode().bit(0) {
                    16
                } else {
                    8
                }
            }
            MoveImmOp::B64 => 0,
            MoveImmOp::Reserved => 0,
        }
    }
}

impl Display for MoveImm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.operation() {
            MoveImmOp::B8 => {
                let format = if self.q().is_128_bits() { "16B" } else { "8B" };
                write!(f, "MOVI {}.{}, #{}", self.rd(), format, self.imm8())
            }
            MoveImmOp::B16 => {
                let format = if self.q().is_128_bits() { "8H" } else { "4H" };
                write!(
                    f,
                    "MOVI {}.{}, #{}, LSL #{}",
                    self.rd(),
                    format,
                    self.imm8(),
                    self.shift_amount()
                )
            }
            MoveImmOp::B32 => {
                let format = if self.q().is_128_bits() { "4S" } else { "2S" };
                write!(
                    f,
                    "MOVI {}.{}, #{}, LSL #{}",
                    self.rd(),
                    format,
                    self.imm8(),
                    self.shift_amount()
                )
            }
            MoveImmOp::B32MSL => {
                let format = if self.q().is_128_bits() { "4S" } else { "2S" };
                write!(
                    f,
                    "MOVI {}.{}, #{}, MSL #{}",
                    self.rd(),
                    format,
                    self.imm8(),
                    self.shift_amount()
                )
            }
            MoveImmOp::B64 => {
                let format = if self.q().is_128_bits() { ".2D" } else { "" };
                write!(
                    f,
                    "MOVI {}{}, #{}",
                    SimdRegScalar::D64(self.rd()),
                    format,
                    self.imm64(),
                )
            }
            MoveImmOp::Reserved => write!(f, "MOVI ????"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum Instruction {
    MoveImm(MoveImm),
}

impl Instruction {
    fn new_simd_modified_imm(value: u32) -> Option<Self> {
        let q = value.bit(30) as u32;
        let op1 = value.bit(29) as u32;
        let cmode = value.bits(12, 16);
        let op2 = value.bit(11) as u32;

        Some(bit_match! {
            match (q, op1, cmode, op2) {
                ("_", "0", "0__0", "0") => Self::MoveImm(MoveImm(value)),
                ("_", "0", "0__1", "0") => todo!("orr vector imm 32 bit"),
                ("_", "0", "10_0", "0") => Self::MoveImm(MoveImm(value)),
                ("_", "0", "10_1", "0") => todo!("orr vector imm 16 bit"),
                ("_", "0", "110_", "0") => Self::MoveImm(MoveImm(value)),
                ("_", "0", "1110", "0") => Self::MoveImm(MoveImm(value)),
                ("_", "0", "1111", "0") => todo!("fmov vector imm 32 bit (single)"),
                ("_", "0", "1111", "1") => todo!("fmov vector imm 16 bit (half)"),
                ("_", "0", "____", "1") => return None,
                ("_", "1", "____", "1") => return None,
                ("_", "1", "0__0", "0") => todo!("mvni 32 bit shifted imm"),
                ("_", "1", "0__1", "0") => todo!("bic vector imm 32 bit"),
                ("_", "1", "10_0", "0") => todo!("mvni 16 bit shifted imm"),
                ("_", "1", "10_1", "0") => todo!("bic vector imm 16 bit"),
                ("_", "1", "110_", "0") => todo!("mvni 32 bit shifting ones"),
                ("0", "1", "1110", "0") => Self::MoveImm(MoveImm(value)),
                ("0", "1", "1111", "0") => return None,
                ("1", "1", "1110", "0") => Self::MoveImm(MoveImm(value)),
                ("1", "1", "1111", "0") => todo!("fmov vector imm 64 bit (double)"),
                _ => return None,
            }
        })
    }

    pub fn new(value: u32) -> Option<Self> {
        let op0 = value.bits(28, 32);
        let op1 = value.bits(23, 25);
        let op2 = value.bits(19, 23);

        // op3: 10..19, split to generate better code with bit_match
        let op3_a = value.bits(12, 19);
        let op3_b = value.bits(10, 12);

        Some(bit_match! {
            match (op0, op1, op2, op3_a, op3_b) {
                ("0100", "0_", "_101", "00_____", "10") => todo!("crypto AES"),

                ("0101", "0_", "_0__", "___0___", "00") => todo!("crypto three reg SHA"),
                ("0101", "0_", "_101", "00_____", "10") => todo!("crypto two reg SHA"),

                ("01_1", "00", "00__", "___0___", "_1") => todo!("simd scalar copy"),
                ("01_1", "0_", "10__", "___00__", "_1") => todo!("simd scalar three same fp16"),
                ("01_1", "0_", "1111", "00_____", "10") => todo!("simd scalar two reg misc fp16"),
                ("01_1", "0_", "_0__", "___1___", "_1") => todo!("simd scalar three same extra"),
                ("01_1", "0_", "_100", "00_____", "10") => todo!("simd scalar two reg misc"),
                ("01_1", "0_", "_110", "00_____", "10") => todo!("simd scalar pairwise"),
                ("01_1", "0_", "_1__", "_______", "00") => todo!("simd scalar three diff"),
                ("01_1", "0_", "_1__", "_______", "_1") => todo!("simd scalar three same"),
                ("01_1", "10", "____", "_______", "_1") => todo!("simd scalar shift by imm"),
                ("01_1", "1_", "____", "_______", "_0") => todo!("simd scalar x indexed elem"),

                ("0_00", "0_", "_0__", "___0___", "00") => todo!("simd table lookup"),
                ("0_00", "0_", "_0__", "___0___", "10") => todo!("simd permute"),

                ("0_10", "0_", "_0__", "___0___", "_0") => todo!("simd extract"),

                ("0__0", "00", "00__", "___0___", "_1") => todo!("simd copy"),
                ("0__0", "0_", "10__", "___0___", "_1") => todo!("simd three same fp16"),
                ("0__0", "0_", "1111", "00_____", "10") => todo!("simd two reg misc fp16"),
                ("0__0", "0_", "_0__", "___1___", "_1") => todo!("simd three reg extension"),
                ("0__0", "0_", "_100", "00_____", "10") => todo!("simd two reg misc"),
                ("0__0", "0_", "_110", "00_____", "10") => todo!("simd across lanes"),
                ("0__0", "0_", "_1__", "_______", "00") => todo!("simd three diff"),
                ("0__0", "0_", "_1__", "_______", "_1") => todo!("simd three same"),
                ("0__0", "10", "0000", "_______", "_1") => Self::new_simd_modified_imm(value)?,
                ("0__0", "10", "____", "_______", "_1") => todo!("simd shift by imm"),
                ("0__0", "1_", "____", "_______", "_0") => todo!("simd vector x indexed elem"),

                ("1100", "00", "10__", "___10__", "__") => todo!("crypto three reg imm2"),
                ("1100", "00", "11__", "___1_00", "__") => todo!("crypto three reg SHA 512"),
                ("1100", "00", "____", "___0___", "__") => todo!("crypto four reg"),
                ("1100", "01", "00__", "_______", "__") => todo!("xar"),
                ("1100", "01", "1000", "0001000", "__") => todo!("crypto two reg SHA 512"),

                ("_0_1", "0_", "_0__", "_______", "__") => todo!("conv float - fixed"),
                ("_0_1", "0_", "_1__", "___0000", "00") => todo!("conv float - int"),
                ("_0_1", "0_", "_1__", "____100", "00") => todo!("float dp one src"),
                ("_0_1", "0_", "_1__", "_____10", "00") => todo!("float cmp"),
                ("_0_1", "0_", "_1__", "______1", "00") => todo!("float imm"),
                ("_0_1", "0_", "_1__", "_______", "01") => todo!("float cond cmp"),
                ("_0_1", "0_", "_1__", "_______", "10") => todo!("float dp two src"),
                ("_0_1", "0_", "_1__", "_______", "11") => todo!("float cond select"),
                ("_0_1", "1_", "____", "_______", "__") => todo!("float dp three src"),
                _ => return None,
            }
        })
    }
}
