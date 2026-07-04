//! Scalar Floating-Point and Advanced SIMD

use a64_macros::bit_match;
use bitos::BitUtils;
use derive_more::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum Instruction {}

impl Instruction {
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
                ("0__0", "10", "0000", "_______", "_1") => todo!("simd modified imm"),
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
