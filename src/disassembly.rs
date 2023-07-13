use crate::opcode;

pub struct Instruction {
    pub address: usize,
    pub opcode: Option<opcode::Opcode>,
}

pub fn disassemble(program: &[u8]) -> Vec<Instruction> {
    let mut instructions = Vec::<Instruction>::new();
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
            address: pc,
            opcode,
        });
        pc += pc_increment;
    }

    instructions
}
