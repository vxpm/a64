use a64_macros::bit_match;
use bitos::BitUtils;
use derive_more::Display;

pub mod add_sub;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum Instruction {
    AddSub(add_sub::Instruction),
}

impl Instruction {
    pub fn new(value: u32) -> Option<Self> {
        let op0 = value.bits(30, 32);
        let op1 = value.bits(22, 26);

        Some(bit_match! {
            match (op0, op1) {
                ("__", "010_") => Instruction::AddSub(add_sub::Instruction::new(value)?),
                _ => return None,
            }
        })
    }
}
