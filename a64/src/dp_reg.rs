//! Data Processing - Register

use a64_macros::bit_match;
use bitos::BitUtils;
use derive_more::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum Instruction {}

impl Instruction {
    pub fn new(value: u32) -> Option<Self> {
        let op0 = value.bit(30) as u32;
        let op1 = value.bit(28) as u32;
        let op2 = value.bits(21, 25);
        let op3 = value.bits(10, 16);

        Some(bit_match! {
            match (op0, op1, op2, op3) {
                ("0", "1", "0110", "______") => todo!("two src"),
                ("1", "1", "0110", "______") => todo!("one src"),
                ("_", "0", "0___", "______") => todo!("logical (shifted reg)"),
                ("_", "0", "1__0", "______") => todo!("add/sub (shifted reg)"),
                ("_", "0", "1__1", "______") => todo!("add/sub (extended reg)"),
                ("_", "1", "0000", "000000") => todo!("add/sub (with carry)"),
                ("_", "1", "0000", "001___") => todo!("add/sub (checked ptr)"),
                ("_", "1", "0000", "_00001") => todo!("rotate right into flags"),
                ("_", "1", "0000", "__0010") => todo!("eval into flags"),
                ("_", "1", "0010", "____0_") => todo!("cond cmp (reg)"),
                ("_", "1", "0010", "____1_") => todo!("cond cmp (imm)"),
                ("_", "1", "0100", "______") => todo!("cond select"),
                ("_", "1", "1___", "______") => todo!("three src"),
                _ => todo!(),
            }
        })
    }
}
