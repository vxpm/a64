//! Data Processing - Immediate

pub mod add_sub;
pub mod logical;
pub mod mov_wide;
pub mod pc_rel_addr;

use a64_macros::bit_match;
use bitos::BitUtils;
use derive_more::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum Instruction {
    PcRelAddr(pc_rel_addr::Instruction),
    AddSub(add_sub::Instruction),
    Logical(logical::Instruction),
    MovWide(mov_wide::Instruction),
}

impl Instruction {
    pub fn new(value: u32) -> Option<Self> {
        let op0 = value.bits(30, 32);
        let op1 = value.bits(22, 26);

        Some(bit_match! {
            match (op0, op1) {
                ("11", "111_") => todo!("one src"),
                ("__", "00__") => Self::PcRelAddr(pc_rel_addr::Instruction::new(value)?),
                ("__", "010_") => Self::AddSub(add_sub::Instruction::new(value)?),
                ("__", "0110") => todo!("add sub with tags"),
                ("__", "0111") => todo!("min max"),
                ("__", "100_") => Self::Logical(logical::Instruction::new(value)?),
                ("__", "101_") => Self::MovWide(mov_wide::Instruction::new(value)?),
                ("__", "110_") => todo!("bitfield"),
                ("__", "111_") => todo!("extract"),
                _ => return None,
            }
        })
    }
}
