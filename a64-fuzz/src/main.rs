use std::io::{Read, Write as _};
use std::process::{Command, Output};
use tempfile::NamedTempFile;

struct Context {
    tmp: NamedTempFile,
    buffer: Vec<u8>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            tmp: NamedTempFile::new().unwrap(),
            buffer: Vec::with_capacity(4),
        }
    }

    fn compile(&mut self, instruction: &str) -> Result<u32, Output> {
        let (reader, mut writer) = std::io::pipe().unwrap();
        write!(&mut writer, "{instruction}").unwrap();
        std::mem::drop(writer);

        let mut cmd = Command::new("aarch64-linux-gnu-as");
        cmd.args([
            "-mcpu=cortex-a57",
            "-",
            "-o",
            self.tmp.path().to_str().unwrap(),
        ])
        .stdin(reader);

        let result = cmd.output().unwrap();
        if !result.status.success() {
            return Err(result);
        }

        std::mem::drop(cmd);

        let (mut reader, writer) = std::io::pipe().unwrap();
        let mut cmd = Command::new("aarch64-linux-gnu-objcopy");
        cmd.args([
            "-O",
            "binary",
            self.tmp.path().to_str().unwrap(),
            "/dev/stdout",
        ])
        .stdout(writer);

        let result = cmd.output().unwrap();
        assert!(result.status.success());

        std::mem::drop(cmd);

        self.buffer.clear();
        reader.read_to_end(&mut self.buffer).unwrap();
        Ok(u32::from_le_bytes(
            self.buffer.as_slice().try_into().unwrap(),
        ))
    }
}

fn main() {
    let mut ctx = Context::new();
    loop {
        let random = rand::random::<u32>();
        let Ok(instruction) = std::panic::catch_unwind(|| a64::Instruction::new(random)) else {
            continue;
        };

        if let Some(instruction) = instruction {
            // ignore because i have no clue what GAS wants
            if let a64::Instruction::DpImm(a64::dp_imm::Instruction::PcRelAddr(_)) = instruction {
                continue;
            }

            // ignore because there are still lots of unspecified system registers
            if let a64::Instruction::BranchSys(a64::branch_sys::Instruction::SysRegMove(_)) =
                instruction
            {
                continue;
            }

            let compiled = match ctx.compile(&instruction.to_string()) {
                Ok(x) => x,
                Err(output) => {
                    println!("{}", String::from_utf8_lossy(&output.stdout));
                    println!("{}", String::from_utf8_lossy(&output.stderr));
                    println!("FAILED - ASSEMBLER ERROR: {}", output.status);
                    println!("{random:08X} - {instruction} ({instruction:?})");
                    panic!();
                }
            };

            let Ok(Some(compiled_instruction)) =
                std::panic::catch_unwind(|| a64::Instruction::new(compiled))
            else {
                panic!(
                    "FAILED: cant decode compiled instruction - {random:08X} {instruction} ({instruction:?})"
                );
            };

            assert_eq!(
                instruction.to_string(),
                compiled_instruction.to_string(),
                "FAILED: {random:08X} / {compiled:08X} - expected {instruction} ({instruction:?}), got {compiled:08X} {compiled_instruction} ({compiled_instruction:?})"
            );
        }
    }
}
