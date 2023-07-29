use crate::memory::Address;
use crate::opcode;

pub struct Instruction {
    pub address: Address,
    pub opcode: Option<opcode::Opcode>,
}

pub fn disassemble(program: &[u8]) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    let mut pc = 0;
    let mut prefixed = false;
    while pc < program.len() {
        let byte = program[pc];
        let opcode = if prefixed {
            prefixed = false;
            opcode::decode_prefixed(byte)
        } else {
            opcode::decode(byte)
        };

        let mut pc_increment = 1;
        if let Some(ref o) = opcode {
            if o.mnemonic == "PREFIX" {
                prefixed = true;
            }
            pc_increment = o.size_bytes as usize;
        }

        instructions.push(Instruction {
            address: Address(pc as u16),
            opcode,
        });
        pc += pc_increment;
    }

    instructions
}
