//! Loads and Stores

use a64_macros::bit_match;
use bitos::BitUtils;
use derive_more::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum Instruction {}

impl Instruction {
    pub fn new(value: u32) -> Option<Self> {
        let op0 = value.bits(28, 32);
        let op1 = value.bit(26) as u32;
        let op2 = value.bits(10, 25);

        Some(bit_match! {
            match (op0, op1, op2) {
                ("0_00", "0", "00_1___________") => todo!("compare and swap pair"),
                ("0_00", "0", "11_0___________") => todo!("compare and swap pair (unprivileged)"),
                ("0_00", "1", "00_000000______") => todo!("adv simd multiple structures"),
                ("0_00", "1", "01_0___________") => todo!("adv simd multiple structures (post index)"),
                ("0_00", "1", "10__0000_______") => todo!("adv simd single structure"),
                ("0_00", "1", "11_____________") => todo!("adv simd single structure (post index)"),

                ("0_01", "0", "1__1_____000010") => todo!("rcw compare and swap"),
                ("0_01", "0", "1__1_____000011") => todo!("rcw compare and swap pair"),
                ("0_01", "0", "1__1_________00") => todo!("128 bit atomic mem ops"),
                ("0_01", "0", "1__1_________01") => todo!("atomic mem ops (unprivileged)"),

                ("1101", "0", "1000111110___11") => todo!("gcs load/store"),
                ("1101", "0", "1__1___________") => todo!("load/store mem tags"),

                ("1_00", "0", "00_1___________") => todo!("load/store exclusive pair"),
                ("1_00", "0", "10_0___________") => todo!("load/store exclusive reg (unprivileged)"),
                ("1_00", "0", "11_0___________") => todo!("compare and swap (unprivileged)"),

                ("__00", "0", "00_0___________") => todo!("load/store exclusive reg"),
                ("__00", "0", "01_0___________") => todo!("load/store ordered"),
                ("__00", "0", "01_1___________") => todo!("compare and swap"),

                ("__01", "0", "10_0_________10") => todo!("load/store ordered register pair"),
                ("__01", "0", "11_000000000010") => todo!("load/store ordered (writeback)"),
                ("__01", "0", "1__0_________00") => todo!("load/store ordered (unscaled imm)"),
                ("__01", "1", "1__0_________10") => todo!("load/store ordered (simd & fp)"),
                ("__01", "_", "0______________") => todo!("load reg (literal)"),
                ("__01", "_", "1__0_________01") => todo!("memcopy and memset"),

                ("__10", "_", "00_____________") => todo!("load/store no-alloc pair (offset)"),
                ("__10", "_", "01_____________") => todo!("load/store reg pair (post-indexed)"),
                ("__10", "_", "10_____________") => todo!("load/store reg pair (offset)"),
                ("__10", "_", "11_____________") => todo!("load/store reg pair (pre-indexed)"),

                ("__11", "_", "0__0_________00") => todo!("load/store reg (unscaled imm)"),
                ("__11", "_", "0__0_________01") => todo!("load/store reg (imm post indexed)"),
                ("__11", "_", "0__0_________10") => todo!("load/store reg (unprivileged)"),
                ("__11", "_", "0__0_________11") => todo!("load/store reg (imm pre indexed)"),
                ("__11", "_", "0__1_________00") => todo!("atomic mem ops"),
                ("__11", "_", "0__1_________10") => todo!("load/store reg (reg offset)"),
                ("__11", "_", "0__1__________1") => todo!("load/store reg (pac)"),
                ("__11", "_", "1______________") => todo!("load/store reg (unsigned imm)"),
                _ => return None,
            }
        })
    }
}
