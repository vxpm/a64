//! Branches, Exception Generating and System instructions

use a64_macros::bit_match;
use bitos::BitUtils;
use derive_more::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum Instruction {}

impl Instruction {
    pub fn new(value: u32) -> Option<Self> {
        let op0 = value.bits(29, 32);
        let op1 = value.bits(5, 25);
        let op2 = value.bits(0, 5);

        Some(bit_match! {
            match (op0, op1, op2) {
                ("010", "00____________", "_____") => todo!("cond branch (immediate)"),
                ("010", "01____________", "_____") => todo!("misc branch (immediate)"),

                ("011", "00________1___", "_____") => todo!("cmp u8/u16 reg and branch"),

                ("110", "00____________", "_____") => todo!("except"),
                ("110", "01000000110001", "_____") => todo!("sys with reg"),
                ("110", "01000000110010", "11111") => todo!("hints"),
                ("110", "01000000110011", "_____") => todo!("barriers"),
                ("110", "0100000___0100", "_____") => todo!("pstate"),
                ("110", "0100_01_______", "_____") => todo!("sys"),
                ("110", "0100_1________", "_____") => todo!("sys reg move"),
                ("110", "0101_01_______", "_____") => todo!("sys pair"),
                ("110", "0101_1________", "_____") => todo!("sys reg pair move"),
                ("110", "1_____________", "_____") => todo!("uncond branch (reg)"),

                ("_00", "______________", "_____") => todo!("uncond branch (imm)"),

                ("_01", "0_____________", "_____") => todo!("cmp and branch (imm)"),
                ("_01", "1_____________", "_____") => todo!("test and branch (imm)"),

                ("_11", "00________00__", "_____") => todo!("cmp regs and branch"),
                ("_11", "01_________0__", "_____") => todo!("cmp reg with imm and branch"),
                _ => todo!(),
            }
        })
    }
}
