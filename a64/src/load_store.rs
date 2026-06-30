//! Loads and Stores

pub mod pair_pre;

use a64_macros::bit_match;
use bitos::BitUtils;
use derive_more::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum Instruction {
    PairPre(pair_pre::Instruction),
}

impl Instruction {
    pub fn new(value: u32) -> Option<Self> {
        let op0 = value.bits(28, 32);
        let op1 = value.bit(26) as u32;

        // op2: 10..25, split to generate better code with bit_match
        let op2_a = value.bits(12, 25);
        let op2_b = value.bits(10, 12);

        Some(bit_match! {
            match (op0, op1, op2_a, op2_b) {
                ("0_00", "0", "00_1_________", "__") => todo!("compare and swap pair"),
                ("0_00", "0", "11_0_________", "__") => todo!("compare and swap pair (unprivileged)"),
                ("0_00", "1", "00_000000____", "__") => todo!("adv simd multiple structures"),
                ("0_00", "1", "01_0_________", "__") => todo!("adv simd multiple structures (post index)"),
                ("0_00", "1", "10__0000_____", "__") => todo!("adv simd single structure"),
                ("0_00", "1", "11___________", "__") => todo!("adv simd single structure (post index)"),

                ("0_01", "0", "1__1_____0000", "10") => todo!("rcw compare and swap"),
                ("0_01", "0", "1__1_____0000", "11") => todo!("rcw compare and swap pair"),
                ("0_01", "0", "1__1_________", "00") => todo!("128 bit atomic mem ops"),
                ("0_01", "0", "1__1_________", "01") => todo!("atomic mem ops (unprivileged)"),

                ("1101", "0", "1000111110___", "11") => todo!("gcs load/store"),
                ("1101", "0", "1__1_________", "__") => todo!("load/store mem tags"),

                ("1_00", "0", "00_1_________", "__") => todo!("load/store exclusive pair"),
                ("1_00", "0", "10_0_________", "__") => todo!("load/store exclusive reg (unprivileged)"),
                ("1_00", "0", "11_0_________", "__") => todo!("compare and swap (unprivileged)"),

                ("__00", "0", "00_0_________", "__") => todo!("load/store exclusive reg"),
                ("__00", "0", "01_0_________", "__") => todo!("load/store ordered"),
                ("__00", "0", "01_1_________", "__") => todo!("compare and swap"),

                ("__01", "0", "10_0_________", "10") => todo!("load/store ordered register pair"),
                ("__01", "0", "11_0000000000", "10") => todo!("load/store ordered (writeback)"),
                ("__01", "0", "1__0_________", "00") => todo!("load/store ordered (unscaled imm)"),
                ("__01", "1", "1__0_________", "10") => todo!("load/store ordered (simd & fp)"),
                ("__01", "_", "0____________", "__") => todo!("load reg (literal)"),
                ("__01", "_", "1__0_________", "01") => todo!("memcopy and memset"),

                ("__10", "_", "00___________", "__") => todo!("load/store no-alloc pair (offset)"),
                ("__10", "_", "01___________", "__") => todo!("load/store reg pair (post-indexed)"),
                ("__10", "_", "10___________", "__") => todo!("load/store reg pair (offset)"),
                ("__10", "_", "11___________", "__") => Self::PairPre(pair_pre::Instruction::new(value)?),

                ("__11", "_", "0__0_________", "00") => todo!("load/store reg (unscaled imm)"),
                ("__11", "_", "0__0_________", "01") => todo!("load/store reg (imm post indexed)"),
                ("__11", "_", "0__0_________", "10") => todo!("load/store reg (unprivileged)"),
                ("__11", "_", "0__0_________", "11") => todo!("load/store reg (imm pre indexed)"),
                ("__11", "_", "0__1_________", "00") => todo!("atomic mem ops"),
                ("__11", "_", "0__1_________", "10") => todo!("load/store reg (reg offset)"),
                ("__11", "_", "0__1_________", "_1") => todo!("load/store reg (pac)"),
                ("__11", "_", "1____________", "__") => todo!("load/store reg (unsigned imm)"),
                _ => return None,
            }
        })
    }
}
