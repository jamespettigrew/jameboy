#[derive(Debug)]
pub struct Opcode {
    pub mnemonic: String,
    pub size_bytes: u8,
}

pub fn decode(byte: u8) -> Option<Opcode> {
    match byte {
        0x00 => Some(Opcode {
            mnemonic: "NOP".to_string(),
            size_bytes: 1,
        }),
        0x01 => Some(Opcode {
            mnemonic: "LD BC, n16".to_string(),
            size_bytes: 3,
        }),
        0x02 => Some(Opcode {
            mnemonic: "LD [BC], A".to_string(),
            size_bytes: 1,
        }),
        0x03 => Some(Opcode {
            mnemonic: "INC BC".to_string(),
            size_bytes: 1,
        }),
        0x04 => Some(Opcode {
            mnemonic: "INC B".to_string(),
            size_bytes: 1,
        }),
        0x05 => Some(Opcode {
            mnemonic: "DEC B".to_string(),
            size_bytes: 1,
        }),
        0x06 => Some(Opcode {
            mnemonic: "LD B, n8".to_string(),
            size_bytes: 2,
        }),
        0x07 => Some(Opcode {
            mnemonic: "RLCA".to_string(),
            size_bytes: 1,
        }),
        0x08 => Some(Opcode {
            mnemonic: "LD [a16], SP".to_string(),
            size_bytes: 3,
        }),
        0x09 => Some(Opcode {
            mnemonic: "ADD HL, BC".to_string(),
            size_bytes: 1,
        }),
        0x0A => Some(Opcode {
            mnemonic: "LD A, [BC]".to_string(),
            size_bytes: 1,
        }),
        0x0B => Some(Opcode {
            mnemonic: "DEC BC".to_string(),
            size_bytes: 1,
        }),
        0x0C => Some(Opcode {
            mnemonic: "INC C".to_string(),
            size_bytes: 1,
        }),
        0x0D => Some(Opcode {
            mnemonic: "DEC C".to_string(),
            size_bytes: 1,
        }),
        0x0E => Some(Opcode {
            mnemonic: "LD C, n8".to_string(),
            size_bytes: 2,
        }),
        0x0F => Some(Opcode {
            mnemonic: "RRCA ".to_string(),
            size_bytes: 1,
        }),
        0x10 => Some(Opcode {
            mnemonic: "STOP n8".to_string(),
            size_bytes: 2,
        }),
        0x11 => Some(Opcode {
            mnemonic: "LD DE, n16".to_string(),
            size_bytes: 3,
        }),
        0x12 => Some(Opcode {
            mnemonic: "LD [DE], A".to_string(),
            size_bytes: 1,
        }),
        0x13 => Some(Opcode {
            mnemonic: "INC DE".to_string(),
            size_bytes: 1,
        }),
        0x14 => Some(Opcode {
            mnemonic: "INC D".to_string(),
            size_bytes: 1,
        }),
        0x15 => Some(Opcode {
            mnemonic: "DEC D".to_string(),
            size_bytes: 1,
        }),
        0x16 => Some(Opcode {
            mnemonic: "LD D, n8".to_string(),
            size_bytes: 2,
        }),
        0x17 => Some(Opcode {
            mnemonic: "RLA ".to_string(),
            size_bytes: 1,
        }),
        0x18 => Some(Opcode {
            mnemonic: "JR e8".to_string(),
            size_bytes: 2,
        }),
        0x19 => Some(Opcode {
            mnemonic: "ADD HL, DE".to_string(),
            size_bytes: 1,
        }),
        0x1A => Some(Opcode {
            mnemonic: "LD A, [DE]".to_string(),
            size_bytes: 1,
        }),
        0x1B => Some(Opcode {
            mnemonic: "DEC DE".to_string(),
            size_bytes: 1,
        }),
        0x1C => Some(Opcode {
            mnemonic: "INC E".to_string(),
            size_bytes: 1,
        }),
        0x1D => Some(Opcode {
            mnemonic: "DEC E".to_string(),
            size_bytes: 1,
        }),
        0x1E => Some(Opcode {
            mnemonic: "LD E, n8".to_string(),
            size_bytes: 2,
        }),
        0x1F => Some(Opcode {
            mnemonic: "RRA ".to_string(),
            size_bytes: 1,
        }),
        0x20 => Some(Opcode {
            mnemonic: "JR NZ, e8".to_string(),
            size_bytes: 2,
        }),
        0x21 => Some(Opcode {
            mnemonic: "LD HL, n16".to_string(),
            size_bytes: 3,
        }),
        0x22 => Some(Opcode {
            mnemonic: "LD [HL+], A".to_string(),
            size_bytes: 1,
        }),
        0x23 => Some(Opcode {
            mnemonic: "INC HL".to_string(),
            size_bytes: 1,
        }),
        0x24 => Some(Opcode {
            mnemonic: "INC H".to_string(),
            size_bytes: 1,
        }),
        0x25 => Some(Opcode {
            mnemonic: "DEC H".to_string(),
            size_bytes: 1,
        }),
        0x26 => Some(Opcode {
            mnemonic: "LD H, n8".to_string(),
            size_bytes: 2,
        }),
        0x27 => Some(Opcode {
            mnemonic: "DAA ".to_string(),
            size_bytes: 1,
        }),
        0x28 => Some(Opcode {
            mnemonic: "JR Z, e8".to_string(),
            size_bytes: 2,
        }),
        0x29 => Some(Opcode {
            mnemonic: "ADD HL, HL".to_string(),
            size_bytes: 1,
        }),
        0x2A => Some(Opcode {
            mnemonic: "LD A, [HL+]".to_string(),
            size_bytes: 1,
        }),
        0x2B => Some(Opcode {
            mnemonic: "DEC HL".to_string(),
            size_bytes: 1,
        }),
        0x2C => Some(Opcode {
            mnemonic: "INC L".to_string(),
            size_bytes: 1,
        }),
        0x2D => Some(Opcode {
            mnemonic: "DEC L".to_string(),
            size_bytes: 1,
        }),
        0x2E => Some(Opcode {
            mnemonic: "LD L, n8".to_string(),
            size_bytes: 2,
        }),
        0x2F => Some(Opcode {
            mnemonic: "CPL ".to_string(),
            size_bytes: 1,
        }),
        0x30 => Some(Opcode {
            mnemonic: "JR NC, e8".to_string(),
            size_bytes: 2,
        }),
        0x31 => Some(Opcode {
            mnemonic: "LD SP, n16".to_string(),
            size_bytes: 3,
        }),
        0x32 => Some(Opcode {
            mnemonic: "LD [HL-], A".to_string(),
            size_bytes: 1,
        }),
        0x33 => Some(Opcode {
            mnemonic: "INC SP".to_string(),
            size_bytes: 1,
        }),
        0x34 => Some(Opcode {
            mnemonic: "INC [HL]".to_string(),
            size_bytes: 1,
        }),
        0x35 => Some(Opcode {
            mnemonic: "DEC [HL]".to_string(),
            size_bytes: 1,
        }),
        0x36 => Some(Opcode {
            mnemonic: "LD [HL], n8".to_string(),
            size_bytes: 2,
        }),
        0x37 => Some(Opcode {
            mnemonic: "SCF ".to_string(),
            size_bytes: 1,
        }),
        0x38 => Some(Opcode {
            mnemonic: "JR C, e8".to_string(),
            size_bytes: 2,
        }),
        0x39 => Some(Opcode {
            mnemonic: "ADD HL, SP".to_string(),
            size_bytes: 1,
        }),
        0x3A => Some(Opcode {
            mnemonic: "LD A, [HL-]".to_string(),
            size_bytes: 1,
        }),
        0x3B => Some(Opcode {
            mnemonic: "DEC SP".to_string(),
            size_bytes: 1,
        }),
        0x3C => Some(Opcode {
            mnemonic: "INC A".to_string(),
            size_bytes: 1,
        }),
        0x3D => Some(Opcode {
            mnemonic: "DEC A".to_string(),
            size_bytes: 1,
        }),
        0x3E => Some(Opcode {
            mnemonic: "LD A, n8".to_string(),
            size_bytes: 2,
        }),
        0x3F => Some(Opcode {
            mnemonic: "CCF ".to_string(),
            size_bytes: 1,
        }),
        0x40 => Some(Opcode {
            mnemonic: "LD B, B".to_string(),
            size_bytes: 1,
        }),
        0x41 => Some(Opcode {
            mnemonic: "LD B, C".to_string(),
            size_bytes: 1,
        }),
        0x42 => Some(Opcode {
            mnemonic: "LD B, D".to_string(),
            size_bytes: 1,
        }),
        0x43 => Some(Opcode {
            mnemonic: "LD B, E".to_string(),
            size_bytes: 1,
        }),
        0x44 => Some(Opcode {
            mnemonic: "LD B, H".to_string(),
            size_bytes: 1,
        }),
        0x45 => Some(Opcode {
            mnemonic: "LD B, L".to_string(),
            size_bytes: 1,
        }),
        0x46 => Some(Opcode {
            mnemonic: "LD B, [HL]".to_string(),
            size_bytes: 1,
        }),
        0x47 => Some(Opcode {
            mnemonic: "LD B, A".to_string(),
            size_bytes: 1,
        }),
        0x48 => Some(Opcode {
            mnemonic: "LD C, B".to_string(),
            size_bytes: 1,
        }),
        0x49 => Some(Opcode {
            mnemonic: "LD C, C".to_string(),
            size_bytes: 1,
        }),
        0x4A => Some(Opcode {
            mnemonic: "LD C, D".to_string(),
            size_bytes: 1,
        }),
        0x4B => Some(Opcode {
            mnemonic: "LD C, E".to_string(),
            size_bytes: 1,
        }),
        0x4C => Some(Opcode {
            mnemonic: "LD C, H".to_string(),
            size_bytes: 1,
        }),
        0x4D => Some(Opcode {
            mnemonic: "LD C, L".to_string(),
            size_bytes: 1,
        }),
        0x4E => Some(Opcode {
            mnemonic: "LD C, [HL]".to_string(),
            size_bytes: 1,
        }),
        0x4F => Some(Opcode {
            mnemonic: "LD C, A".to_string(),
            size_bytes: 1,
        }),
        0x50 => Some(Opcode {
            mnemonic: "LD D, B".to_string(),
            size_bytes: 1,
        }),
        0x51 => Some(Opcode {
            mnemonic: "LD D, C".to_string(),
            size_bytes: 1,
        }),
        0x52 => Some(Opcode {
            mnemonic: "LD D, D".to_string(),
            size_bytes: 1,
        }),
        0x53 => Some(Opcode {
            mnemonic: "LD D, E".to_string(),
            size_bytes: 1,
        }),
        0x54 => Some(Opcode {
            mnemonic: "LD D, H".to_string(),
            size_bytes: 1,
        }),
        0x55 => Some(Opcode {
            mnemonic: "LD D, L".to_string(),
            size_bytes: 1,
        }),
        0x56 => Some(Opcode {
            mnemonic: "LD D, [HL]".to_string(),
            size_bytes: 1,
        }),
        0x57 => Some(Opcode {
            mnemonic: "LD D, A".to_string(),
            size_bytes: 1,
        }),
        0x58 => Some(Opcode {
            mnemonic: "LD E, B".to_string(),
            size_bytes: 1,
        }),
        0x59 => Some(Opcode {
            mnemonic: "LD E, C".to_string(),
            size_bytes: 1,
        }),
        0x5A => Some(Opcode {
            mnemonic: "LD E, D".to_string(),
            size_bytes: 1,
        }),
        0x5B => Some(Opcode {
            mnemonic: "LD E, E".to_string(),
            size_bytes: 1,
        }),
        0x5C => Some(Opcode {
            mnemonic: "LD E, H".to_string(),
            size_bytes: 1,
        }),
        0x5D => Some(Opcode {
            mnemonic: "LD E, L".to_string(),
            size_bytes: 1,
        }),
        0x5E => Some(Opcode {
            mnemonic: "LD E, [HL]".to_string(),
            size_bytes: 1,
        }),
        0x5F => Some(Opcode {
            mnemonic: "LD E, A".to_string(),
            size_bytes: 1,
        }),
        0x60 => Some(Opcode {
            mnemonic: "LD H, B".to_string(),
            size_bytes: 1,
        }),
        0x61 => Some(Opcode {
            mnemonic: "LD H, C".to_string(),
            size_bytes: 1,
        }),
        0x62 => Some(Opcode {
            mnemonic: "LD H, D".to_string(),
            size_bytes: 1,
        }),
        0x63 => Some(Opcode {
            mnemonic: "LD H, E".to_string(),
            size_bytes: 1,
        }),
        0x64 => Some(Opcode {
            mnemonic: "LD H, H".to_string(),
            size_bytes: 1,
        }),
        0x65 => Some(Opcode {
            mnemonic: "LD H, L".to_string(),
            size_bytes: 1,
        }),
        0x66 => Some(Opcode {
            mnemonic: "LD H, [HL]".to_string(),
            size_bytes: 1,
        }),
        0x67 => Some(Opcode {
            mnemonic: "LD H, A".to_string(),
            size_bytes: 1,
        }),
        0x68 => Some(Opcode {
            mnemonic: "LD L, B".to_string(),
            size_bytes: 1,
        }),
        0x69 => Some(Opcode {
            mnemonic: "LD L, C".to_string(),
            size_bytes: 1,
        }),
        0x6A => Some(Opcode {
            mnemonic: "LD L, D".to_string(),
            size_bytes: 1,
        }),
        0x6B => Some(Opcode {
            mnemonic: "LD L, E".to_string(),
            size_bytes: 1,
        }),
        0x6C => Some(Opcode {
            mnemonic: "LD L, H".to_string(),
            size_bytes: 1,
        }),
        0x6D => Some(Opcode {
            mnemonic: "LD L, L".to_string(),
            size_bytes: 1,
        }),
        0x6E => Some(Opcode {
            mnemonic: "LD L, [HL]".to_string(),
            size_bytes: 1,
        }),
        0x6F => Some(Opcode {
            mnemonic: "LD L, A".to_string(),
            size_bytes: 1,
        }),
        0x70 => Some(Opcode {
            mnemonic: "LD [HL], B".to_string(),
            size_bytes: 1,
        }),
        0x71 => Some(Opcode {
            mnemonic: "LD [HL], C".to_string(),
            size_bytes: 1,
        }),
        0x72 => Some(Opcode {
            mnemonic: "LD [HL], D".to_string(),
            size_bytes: 1,
        }),
        0x73 => Some(Opcode {
            mnemonic: "LD [HL], E".to_string(),
            size_bytes: 1,
        }),
        0x74 => Some(Opcode {
            mnemonic: "LD [HL], H".to_string(),
            size_bytes: 1,
        }),
        0x75 => Some(Opcode {
            mnemonic: "LD [HL], L".to_string(),
            size_bytes: 1,
        }),
        0x76 => Some(Opcode {
            mnemonic: "HALT ".to_string(),
            size_bytes: 1,
        }),
        0x77 => Some(Opcode {
            mnemonic: "LD [HL], A".to_string(),
            size_bytes: 1,
        }),
        0x78 => Some(Opcode {
            mnemonic: "LD A, B".to_string(),
            size_bytes: 1,
        }),
        0x79 => Some(Opcode {
            mnemonic: "LD A, C".to_string(),
            size_bytes: 1,
        }),
        0x7A => Some(Opcode {
            mnemonic: "LD A, D".to_string(),
            size_bytes: 1,
        }),
        0x7B => Some(Opcode {
            mnemonic: "LD A, E".to_string(),
            size_bytes: 1,
        }),
        0x7C => Some(Opcode {
            mnemonic: "LD A, H".to_string(),
            size_bytes: 1,
        }),
        0x7D => Some(Opcode {
            mnemonic: "LD A, L".to_string(),
            size_bytes: 1,
        }),
        0x7E => Some(Opcode {
            mnemonic: "LD A, [HL]".to_string(),
            size_bytes: 1,
        }),
        0x7F => Some(Opcode {
            mnemonic: "LD A, A".to_string(),
            size_bytes: 1,
        }),
        0x80 => Some(Opcode {
            mnemonic: "ADD A, B".to_string(),
            size_bytes: 1,
        }),
        0x81 => Some(Opcode {
            mnemonic: "ADD A, C".to_string(),
            size_bytes: 1,
        }),
        0x82 => Some(Opcode {
            mnemonic: "ADD A, D".to_string(),
            size_bytes: 1,
        }),
        0x83 => Some(Opcode {
            mnemonic: "ADD A, E".to_string(),
            size_bytes: 1,
        }),
        0x84 => Some(Opcode {
            mnemonic: "ADD A, H".to_string(),
            size_bytes: 1,
        }),
        0x85 => Some(Opcode {
            mnemonic: "ADD A, L".to_string(),
            size_bytes: 1,
        }),
        0x86 => Some(Opcode {
            mnemonic: "ADD A, [HL]".to_string(),
            size_bytes: 1,
        }),
        0x87 => Some(Opcode {
            mnemonic: "ADD A, A".to_string(),
            size_bytes: 1,
        }),
        0x88 => Some(Opcode {
            mnemonic: "ADC A, B".to_string(),
            size_bytes: 1,
        }),
        0x89 => Some(Opcode {
            mnemonic: "ADC A, C".to_string(),
            size_bytes: 1,
        }),
        0x8A => Some(Opcode {
            mnemonic: "ADC A, D".to_string(),
            size_bytes: 1,
        }),
        0x8B => Some(Opcode {
            mnemonic: "ADC A, E".to_string(),
            size_bytes: 1,
        }),
        0x8C => Some(Opcode {
            mnemonic: "ADC A, H".to_string(),
            size_bytes: 1,
        }),
        0x8D => Some(Opcode {
            mnemonic: "ADC A, L".to_string(),
            size_bytes: 1,
        }),
        0x8E => Some(Opcode {
            mnemonic: "ADC A, [HL]".to_string(),
            size_bytes: 1,
        }),
        0x8F => Some(Opcode {
            mnemonic: "ADC A, A".to_string(),
            size_bytes: 1,
        }),
        0x90 => Some(Opcode {
            mnemonic: "SUB A, B".to_string(),
            size_bytes: 1,
        }),
        0x91 => Some(Opcode {
            mnemonic: "SUB A, C".to_string(),
            size_bytes: 1,
        }),
        0x92 => Some(Opcode {
            mnemonic: "SUB A, D".to_string(),
            size_bytes: 1,
        }),
        0x93 => Some(Opcode {
            mnemonic: "SUB A, E".to_string(),
            size_bytes: 1,
        }),
        0x94 => Some(Opcode {
            mnemonic: "SUB A, H".to_string(),
            size_bytes: 1,
        }),
        0x95 => Some(Opcode {
            mnemonic: "SUB A, L".to_string(),
            size_bytes: 1,
        }),
        0x96 => Some(Opcode {
            mnemonic: "SUB A, [HL]".to_string(),
            size_bytes: 1,
        }),
        0x97 => Some(Opcode {
            mnemonic: "SUB A, A".to_string(),
            size_bytes: 1,
        }),
        0x98 => Some(Opcode {
            mnemonic: "SBC A, B".to_string(),
            size_bytes: 1,
        }),
        0x99 => Some(Opcode {
            mnemonic: "SBC A, C".to_string(),
            size_bytes: 1,
        }),
        0x9A => Some(Opcode {
            mnemonic: "SBC A, D".to_string(),
            size_bytes: 1,
        }),
        0x9B => Some(Opcode {
            mnemonic: "SBC A, E".to_string(),
            size_bytes: 1,
        }),
        0x9C => Some(Opcode {
            mnemonic: "SBC A, H".to_string(),
            size_bytes: 1,
        }),
        0x9D => Some(Opcode {
            mnemonic: "SBC A, L".to_string(),
            size_bytes: 1,
        }),
        0x9E => Some(Opcode {
            mnemonic: "SBC A, [HL]".to_string(),
            size_bytes: 1,
        }),
        0x9F => Some(Opcode {
            mnemonic: "SBC A, A".to_string(),
            size_bytes: 1,
        }),
        0xA0 => Some(Opcode {
            mnemonic: "AND A, B".to_string(),
            size_bytes: 1,
        }),
        0xA1 => Some(Opcode {
            mnemonic: "AND A, C".to_string(),
            size_bytes: 1,
        }),
        0xA2 => Some(Opcode {
            mnemonic: "AND A, D".to_string(),
            size_bytes: 1,
        }),
        0xA3 => Some(Opcode {
            mnemonic: "AND A, E".to_string(),
            size_bytes: 1,
        }),
        0xA4 => Some(Opcode {
            mnemonic: "AND A, H".to_string(),
            size_bytes: 1,
        }),
        0xA5 => Some(Opcode {
            mnemonic: "AND A, L".to_string(),
            size_bytes: 1,
        }),
        0xA6 => Some(Opcode {
            mnemonic: "AND A, [HL]".to_string(),
            size_bytes: 1,
        }),
        0xA7 => Some(Opcode {
            mnemonic: "AND A, A".to_string(),
            size_bytes: 1,
        }),
        0xA8 => Some(Opcode {
            mnemonic: "XOR A, B".to_string(),
            size_bytes: 1,
        }),
        0xA9 => Some(Opcode {
            mnemonic: "XOR A, C".to_string(),
            size_bytes: 1,
        }),
        0xAA => Some(Opcode {
            mnemonic: "XOR A, D".to_string(),
            size_bytes: 1,
        }),
        0xAB => Some(Opcode {
            mnemonic: "XOR A, E".to_string(),
            size_bytes: 1,
        }),
        0xAC => Some(Opcode {
            mnemonic: "XOR A, H".to_string(),
            size_bytes: 1,
        }),
        0xAD => Some(Opcode {
            mnemonic: "XOR A, L".to_string(),
            size_bytes: 1,
        }),
        0xAE => Some(Opcode {
            mnemonic: "XOR A, [HL]".to_string(),
            size_bytes: 1,
        }),
        0xAF => Some(Opcode {
            mnemonic: "XOR A, A".to_string(),
            size_bytes: 1,
        }),
        0xB0 => Some(Opcode {
            mnemonic: "OR A, B".to_string(),
            size_bytes: 1,
        }),
        0xB1 => Some(Opcode {
            mnemonic: "OR A, C".to_string(),
            size_bytes: 1,
        }),
        0xB2 => Some(Opcode {
            mnemonic: "OR A, D".to_string(),
            size_bytes: 1,
        }),
        0xB3 => Some(Opcode {
            mnemonic: "OR A, E".to_string(),
            size_bytes: 1,
        }),
        0xB4 => Some(Opcode {
            mnemonic: "OR A, H".to_string(),
            size_bytes: 1,
        }),
        0xB5 => Some(Opcode {
            mnemonic: "OR A, L".to_string(),
            size_bytes: 1,
        }),
        0xB6 => Some(Opcode {
            mnemonic: "OR A, [HL]".to_string(),
            size_bytes: 1,
        }),
        0xB7 => Some(Opcode {
            mnemonic: "OR A, A".to_string(),
            size_bytes: 1,
        }),
        0xB8 => Some(Opcode {
            mnemonic: "CP A, B".to_string(),
            size_bytes: 1,
        }),
        0xB9 => Some(Opcode {
            mnemonic: "CP A, C".to_string(),
            size_bytes: 1,
        }),
        0xBA => Some(Opcode {
            mnemonic: "CP A, D".to_string(),
            size_bytes: 1,
        }),
        0xBB => Some(Opcode {
            mnemonic: "CP A, E".to_string(),
            size_bytes: 1,
        }),
        0xBC => Some(Opcode {
            mnemonic: "CP A, H".to_string(),
            size_bytes: 1,
        }),
        0xBD => Some(Opcode {
            mnemonic: "CP A, L".to_string(),
            size_bytes: 1,
        }),
        0xBE => Some(Opcode {
            mnemonic: "CP A, [HL]".to_string(),
            size_bytes: 1,
        }),
        0xBF => Some(Opcode {
            mnemonic: "CP A, A".to_string(),
            size_bytes: 1,
        }),
        0xC0 => Some(Opcode {
            mnemonic: "RET NZ".to_string(),
            size_bytes: 1,
        }),
        0xC1 => Some(Opcode {
            mnemonic: "POP BC".to_string(),
            size_bytes: 1,
        }),
        0xC2 => Some(Opcode {
            mnemonic: "JP NZ, a16".to_string(),
            size_bytes: 3,
        }),
        0xC3 => Some(Opcode {
            mnemonic: "JP a16".to_string(),
            size_bytes: 3,
        }),
        0xC4 => Some(Opcode {
            mnemonic: "CALL NZ, a16".to_string(),
            size_bytes: 3,
        }),
        0xC5 => Some(Opcode {
            mnemonic: "PUSH BC".to_string(),
            size_bytes: 1,
        }),
        0xC6 => Some(Opcode {
            mnemonic: "ADD A, n8".to_string(),
            size_bytes: 2,
        }),
        0xC7 => Some(Opcode {
            mnemonic: "RST $00".to_string(),
            size_bytes: 1,
        }),
        0xC8 => Some(Opcode {
            mnemonic: "RET Z".to_string(),
            size_bytes: 1,
        }),
        0xC9 => Some(Opcode {
            mnemonic: "RET ".to_string(),
            size_bytes: 1,
        }),
        0xCA => Some(Opcode {
            mnemonic: "JP Z, a16".to_string(),
            size_bytes: 3,
        }),
        0xCB => Some(Opcode {
            mnemonic: "PREFIX ".to_string(),
            size_bytes: 1,
        }),
        0xCC => Some(Opcode {
            mnemonic: "CALL Z, a16".to_string(),
            size_bytes: 3,
        }),
        0xCD => Some(Opcode {
            mnemonic: "CALL a16".to_string(),
            size_bytes: 3,
        }),
        0xCE => Some(Opcode {
            mnemonic: "ADC A, n8".to_string(),
            size_bytes: 2,
        }),
        0xCF => Some(Opcode {
            mnemonic: "RST $08".to_string(),
            size_bytes: 1,
        }),
        0xD0 => Some(Opcode {
            mnemonic: "RET NC".to_string(),
            size_bytes: 1,
        }),
        0xD1 => Some(Opcode {
            mnemonic: "POP DE".to_string(),
            size_bytes: 1,
        }),
        0xD2 => Some(Opcode {
            mnemonic: "JP NC, a16".to_string(),
            size_bytes: 3,
        }),
        0xD4 => Some(Opcode {
            mnemonic: "CALL NC, a16".to_string(),
            size_bytes: 3,
        }),
        0xD5 => Some(Opcode {
            mnemonic: "PUSH DE".to_string(),
            size_bytes: 1,
        }),
        0xD6 => Some(Opcode {
            mnemonic: "SUB A, n8".to_string(),
            size_bytes: 2,
        }),
        0xD7 => Some(Opcode {
            mnemonic: "RST $10".to_string(),
            size_bytes: 1,
        }),
        0xD8 => Some(Opcode {
            mnemonic: "RET C".to_string(),
            size_bytes: 1,
        }),
        0xD9 => Some(Opcode {
            mnemonic: "RETI ".to_string(),
            size_bytes: 1,
        }),
        0xDA => Some(Opcode {
            mnemonic: "JP C, a16".to_string(),
            size_bytes: 3,
        }),
        0xDC => Some(Opcode {
            mnemonic: "CALL C, a16".to_string(),
            size_bytes: 3,
        }),
        0xDE => Some(Opcode {
            mnemonic: "SBC A, n8".to_string(),
            size_bytes: 2,
        }),
        0xDF => Some(Opcode {
            mnemonic: "RST $18".to_string(),
            size_bytes: 1,
        }),
        0xE0 => Some(Opcode {
            mnemonic: "LDH [a8], A".to_string(),
            size_bytes: 2,
        }),
        0xE1 => Some(Opcode {
            mnemonic: "POP HL".to_string(),
            size_bytes: 1,
        }),
        0xE2 => Some(Opcode {
            mnemonic: "LD [C], A".to_string(),
            size_bytes: 1,
        }),
        0xE5 => Some(Opcode {
            mnemonic: "PUSH HL".to_string(),
            size_bytes: 1,
        }),
        0xE6 => Some(Opcode {
            mnemonic: "AND A, n8".to_string(),
            size_bytes: 2,
        }),
        0xE7 => Some(Opcode {
            mnemonic: "RST $20".to_string(),
            size_bytes: 1,
        }),
        0xE8 => Some(Opcode {
            mnemonic: "ADD SP, e8".to_string(),
            size_bytes: 2,
        }),
        0xE9 => Some(Opcode {
            mnemonic: "JP HL".to_string(),
            size_bytes: 1,
        }),
        0xEA => Some(Opcode {
            mnemonic: "LD [a16], A".to_string(),
            size_bytes: 3,
        }),
        0xEE => Some(Opcode {
            mnemonic: "XOR A, n8".to_string(),
            size_bytes: 2,
        }),
        0xEF => Some(Opcode {
            mnemonic: "RST $28".to_string(),
            size_bytes: 1,
        }),
        0xF0 => Some(Opcode {
            mnemonic: "LDH A, [a8]".to_string(),
            size_bytes: 2,
        }),
        0xF1 => Some(Opcode {
            mnemonic: "POP AF".to_string(),
            size_bytes: 1,
        }),
        0xF2 => Some(Opcode {
            mnemonic: "LD A, [C]".to_string(),
            size_bytes: 1,
        }),
        0xF3 => Some(Opcode {
            mnemonic: "DI".to_string(),
            size_bytes: 1,
        }),
        0xF5 => Some(Opcode {
            mnemonic: "PUSH AF".to_string(),
            size_bytes: 1,
        }),
        0xF6 => Some(Opcode {
            mnemonic: "OR A, n8".to_string(),
            size_bytes: 2,
        }),
        0xF7 => Some(Opcode {
            mnemonic: "RST $30".to_string(),
            size_bytes: 1,
        }),
        0xF8 => Some(Opcode {
            mnemonic: "LD HL, SP + e8".to_string(),
            size_bytes: 2,
        }),
        0xF9 => Some(Opcode {
            mnemonic: "LD SP, HL".to_string(),
            size_bytes: 1,
        }),
        0xFA => Some(Opcode {
            mnemonic: "LD A, [a16]".to_string(),
            size_bytes: 3,
        }),
        0xFB => Some(Opcode {
            mnemonic: "EI ".to_string(),
            size_bytes: 1,
        }),
        0xFE => Some(Opcode {
            mnemonic: "CP A, n8".to_string(),
            size_bytes: 2,
        }),
        0xFF => Some(Opcode {
            mnemonic: "RST $38".to_string(),
            size_bytes: 1,
        }),
        _ => None,
    }
}

pub fn decode_prefixed(byte: u8) -> Option<Opcode> {
    match byte {
        0x00 => Some(Opcode {
            mnemonic: "RLC B".to_string(),
            size_bytes: 2,
        }),
        0x01 => Some(Opcode {
            mnemonic: "RLC C".to_string(),
            size_bytes: 2,
        }),
        0x02 => Some(Opcode {
            mnemonic: "RLC D".to_string(),
            size_bytes: 2,
        }),
        0x03 => Some(Opcode {
            mnemonic: "RLC E".to_string(),
            size_bytes: 2,
        }),
        0x04 => Some(Opcode {
            mnemonic: "RLC H".to_string(),
            size_bytes: 2,
        }),
        0x05 => Some(Opcode {
            mnemonic: "RLC L".to_string(),
            size_bytes: 2,
        }),
        0x06 => Some(Opcode {
            mnemonic: "RLC [HL]".to_string(),
            size_bytes: 2,
        }),
        0x07 => Some(Opcode {
            mnemonic: "RLC A".to_string(),
            size_bytes: 2,
        }),
        0x08 => Some(Opcode {
            mnemonic: "RRC B".to_string(),
            size_bytes: 2,
        }),
        0x09 => Some(Opcode {
            mnemonic: "RRC C".to_string(),
            size_bytes: 2,
        }),
        0x0A => Some(Opcode {
            mnemonic: "RRC D".to_string(),
            size_bytes: 2,
        }),
        0x0B => Some(Opcode {
            mnemonic: "RRC E".to_string(),
            size_bytes: 2,
        }),
        0x0C => Some(Opcode {
            mnemonic: "RRC H".to_string(),
            size_bytes: 2,
        }),
        0x0D => Some(Opcode {
            mnemonic: "RRC L".to_string(),
            size_bytes: 2,
        }),
        0x0E => Some(Opcode {
            mnemonic: "RRC [HL]".to_string(),
            size_bytes: 2,
        }),
        0x0F => Some(Opcode {
            mnemonic: "RRC A".to_string(),
            size_bytes: 2,
        }),
        0x10 => Some(Opcode {
            mnemonic: "RL B".to_string(),
            size_bytes: 2,
        }),
        0x11 => Some(Opcode {
            mnemonic: "RL C".to_string(),
            size_bytes: 2,
        }),
        0x12 => Some(Opcode {
            mnemonic: "RL D".to_string(),
            size_bytes: 2,
        }),
        0x13 => Some(Opcode {
            mnemonic: "RL E".to_string(),
            size_bytes: 2,
        }),
        0x14 => Some(Opcode {
            mnemonic: "RL H".to_string(),
            size_bytes: 2,
        }),
        0x15 => Some(Opcode {
            mnemonic: "RL L".to_string(),
            size_bytes: 2,
        }),
        0x16 => Some(Opcode {
            mnemonic: "RL [HL]".to_string(),
            size_bytes: 2,
        }),
        0x17 => Some(Opcode {
            mnemonic: "RL A".to_string(),
            size_bytes: 2,
        }),
        0x18 => Some(Opcode {
            mnemonic: "RR B".to_string(),
            size_bytes: 2,
        }),
        0x19 => Some(Opcode {
            mnemonic: "RR C".to_string(),
            size_bytes: 2,
        }),
        0x1A => Some(Opcode {
            mnemonic: "RR D".to_string(),
            size_bytes: 2,
        }),
        0x1B => Some(Opcode {
            mnemonic: "RR E".to_string(),
            size_bytes: 2,
        }),
        0x1C => Some(Opcode {
            mnemonic: "RR H".to_string(),
            size_bytes: 2,
        }),
        0x1D => Some(Opcode {
            mnemonic: "RR L".to_string(),
            size_bytes: 2,
        }),
        0x1E => Some(Opcode {
            mnemonic: "RR [HL]".to_string(),
            size_bytes: 2,
        }),
        0x1F => Some(Opcode {
            mnemonic: "RR A".to_string(),
            size_bytes: 2,
        }),
        0x20 => Some(Opcode {
            mnemonic: "SLA B".to_string(),
            size_bytes: 2,
        }),
        0x21 => Some(Opcode {
            mnemonic: "SLA C".to_string(),
            size_bytes: 2,
        }),
        0x22 => Some(Opcode {
            mnemonic: "SLA D".to_string(),
            size_bytes: 2,
        }),
        0x23 => Some(Opcode {
            mnemonic: "SLA E".to_string(),
            size_bytes: 2,
        }),
        0x24 => Some(Opcode {
            mnemonic: "SLA H".to_string(),
            size_bytes: 2,
        }),
        0x25 => Some(Opcode {
            mnemonic: "SLA L".to_string(),
            size_bytes: 2,
        }),
        0x26 => Some(Opcode {
            mnemonic: "SLA [HL]".to_string(),
            size_bytes: 2,
        }),
        0x27 => Some(Opcode {
            mnemonic: "SLA A".to_string(),
            size_bytes: 2,
        }),
        0x28 => Some(Opcode {
            mnemonic: "SRA B".to_string(),
            size_bytes: 2,
        }),
        0x29 => Some(Opcode {
            mnemonic: "SRA C".to_string(),
            size_bytes: 2,
        }),
        0x2A => Some(Opcode {
            mnemonic: "SRA D".to_string(),
            size_bytes: 2,
        }),
        0x2B => Some(Opcode {
            mnemonic: "SRA E".to_string(),
            size_bytes: 2,
        }),
        0x2C => Some(Opcode {
            mnemonic: "SRA H".to_string(),
            size_bytes: 2,
        }),
        0x2D => Some(Opcode {
            mnemonic: "SRA L".to_string(),
            size_bytes: 2,
        }),
        0x2E => Some(Opcode {
            mnemonic: "SRA [HL]".to_string(),
            size_bytes: 2,
        }),
        0x2F => Some(Opcode {
            mnemonic: "SRA A".to_string(),
            size_bytes: 2,
        }),
        0x30 => Some(Opcode {
            mnemonic: "SWAP B".to_string(),
            size_bytes: 2,
        }),
        0x31 => Some(Opcode {
            mnemonic: "SWAP C".to_string(),
            size_bytes: 2,
        }),
        0x32 => Some(Opcode {
            mnemonic: "SWAP D".to_string(),
            size_bytes: 2,
        }),
        0x33 => Some(Opcode {
            mnemonic: "SWAP E".to_string(),
            size_bytes: 2,
        }),
        0x34 => Some(Opcode {
            mnemonic: "SWAP H".to_string(),
            size_bytes: 2,
        }),
        0x35 => Some(Opcode {
            mnemonic: "SWAP L".to_string(),
            size_bytes: 2,
        }),
        0x36 => Some(Opcode {
            mnemonic: "SWAP [HL]".to_string(),
            size_bytes: 2,
        }),
        0x37 => Some(Opcode {
            mnemonic: "SWAP A".to_string(),
            size_bytes: 2,
        }),
        0x38 => Some(Opcode {
            mnemonic: "SRL B".to_string(),
            size_bytes: 2,
        }),
        0x39 => Some(Opcode {
            mnemonic: "SRL C".to_string(),
            size_bytes: 2,
        }),
        0x3A => Some(Opcode {
            mnemonic: "SRL D".to_string(),
            size_bytes: 2,
        }),
        0x3B => Some(Opcode {
            mnemonic: "SRL E".to_string(),
            size_bytes: 2,
        }),
        0x3C => Some(Opcode {
            mnemonic: "SRL H".to_string(),
            size_bytes: 2,
        }),
        0x3D => Some(Opcode {
            mnemonic: "SRL L".to_string(),
            size_bytes: 2,
        }),
        0x3E => Some(Opcode {
            mnemonic: "SRL [HL]".to_string(),
            size_bytes: 2,
        }),
        0x3F => Some(Opcode {
            mnemonic: "SRL A".to_string(),
            size_bytes: 2,
        }),
        0x40 => Some(Opcode {
            mnemonic: "BIT 0, B".to_string(),
            size_bytes: 2,
        }),
        0x41 => Some(Opcode {
            mnemonic: "BIT 0, C".to_string(),
            size_bytes: 2,
        }),
        0x42 => Some(Opcode {
            mnemonic: "BIT 0, D".to_string(),
            size_bytes: 2,
        }),
        0x43 => Some(Opcode {
            mnemonic: "BIT 0, E".to_string(),
            size_bytes: 2,
        }),
        0x44 => Some(Opcode {
            mnemonic: "BIT 0, H".to_string(),
            size_bytes: 2,
        }),
        0x45 => Some(Opcode {
            mnemonic: "BIT 0, L".to_string(),
            size_bytes: 2,
        }),
        0x46 => Some(Opcode {
            mnemonic: "BIT 0, [HL]".to_string(),
            size_bytes: 2,
        }),
        0x47 => Some(Opcode {
            mnemonic: "BIT 0, A".to_string(),
            size_bytes: 2,
        }),
        0x48 => Some(Opcode {
            mnemonic: "BIT 1, B".to_string(),
            size_bytes: 2,
        }),
        0x49 => Some(Opcode {
            mnemonic: "BIT 1, C".to_string(),
            size_bytes: 2,
        }),
        0x4A => Some(Opcode {
            mnemonic: "BIT 1, D".to_string(),
            size_bytes: 2,
        }),
        0x4B => Some(Opcode {
            mnemonic: "BIT 1, E".to_string(),
            size_bytes: 2,
        }),
        0x4C => Some(Opcode {
            mnemonic: "BIT 1, H".to_string(),
            size_bytes: 2,
        }),
        0x4D => Some(Opcode {
            mnemonic: "BIT 1, L".to_string(),
            size_bytes: 2,
        }),
        0x4E => Some(Opcode {
            mnemonic: "BIT 1, [HL]".to_string(),
            size_bytes: 2,
        }),
        0x4F => Some(Opcode {
            mnemonic: "BIT 1, A".to_string(),
            size_bytes: 2,
        }),
        0x50 => Some(Opcode {
            mnemonic: "BIT 2, B".to_string(),
            size_bytes: 2,
        }),
        0x51 => Some(Opcode {
            mnemonic: "BIT 2, C".to_string(),
            size_bytes: 2,
        }),
        0x52 => Some(Opcode {
            mnemonic: "BIT 2, D".to_string(),
            size_bytes: 2,
        }),
        0x53 => Some(Opcode {
            mnemonic: "BIT 2, E".to_string(),
            size_bytes: 2,
        }),
        0x54 => Some(Opcode {
            mnemonic: "BIT 2, H".to_string(),
            size_bytes: 2,
        }),
        0x55 => Some(Opcode {
            mnemonic: "BIT 2, L".to_string(),
            size_bytes: 2,
        }),
        0x56 => Some(Opcode {
            mnemonic: "BIT 2, [HL]".to_string(),
            size_bytes: 2,
        }),
        0x57 => Some(Opcode {
            mnemonic: "BIT 2, A".to_string(),
            size_bytes: 2,
        }),
        0x58 => Some(Opcode {
            mnemonic: "BIT 3, B".to_string(),
            size_bytes: 2,
        }),
        0x59 => Some(Opcode {
            mnemonic: "BIT 3, C".to_string(),
            size_bytes: 2,
        }),
        0x5A => Some(Opcode {
            mnemonic: "BIT 3, D".to_string(),
            size_bytes: 2,
        }),
        0x5B => Some(Opcode {
            mnemonic: "BIT 3, E".to_string(),
            size_bytes: 2,
        }),
        0x5C => Some(Opcode {
            mnemonic: "BIT 3, H".to_string(),
            size_bytes: 2,
        }),
        0x5D => Some(Opcode {
            mnemonic: "BIT 3, L".to_string(),
            size_bytes: 2,
        }),
        0x5E => Some(Opcode {
            mnemonic: "BIT 3, [HL]".to_string(),
            size_bytes: 2,
        }),
        0x5F => Some(Opcode {
            mnemonic: "BIT 3, A".to_string(),
            size_bytes: 2,
        }),
        0x60 => Some(Opcode {
            mnemonic: "BIT 4, B".to_string(),
            size_bytes: 2,
        }),
        0x61 => Some(Opcode {
            mnemonic: "BIT 4, C".to_string(),
            size_bytes: 2,
        }),
        0x62 => Some(Opcode {
            mnemonic: "BIT 4, D".to_string(),
            size_bytes: 2,
        }),
        0x63 => Some(Opcode {
            mnemonic: "BIT 4, E".to_string(),
            size_bytes: 2,
        }),
        0x64 => Some(Opcode {
            mnemonic: "BIT 4, H".to_string(),
            size_bytes: 2,
        }),
        0x65 => Some(Opcode {
            mnemonic: "BIT 4, L".to_string(),
            size_bytes: 2,
        }),
        0x66 => Some(Opcode {
            mnemonic: "BIT 4, [HL]".to_string(),
            size_bytes: 2,
        }),
        0x67 => Some(Opcode {
            mnemonic: "BIT 4, A".to_string(),
            size_bytes: 2,
        }),
        0x68 => Some(Opcode {
            mnemonic: "BIT 5, B".to_string(),
            size_bytes: 2,
        }),
        0x69 => Some(Opcode {
            mnemonic: "BIT 5, C".to_string(),
            size_bytes: 2,
        }),
        0x6A => Some(Opcode {
            mnemonic: "BIT 5, D".to_string(),
            size_bytes: 2,
        }),
        0x6B => Some(Opcode {
            mnemonic: "BIT 5, E".to_string(),
            size_bytes: 2,
        }),
        0x6C => Some(Opcode {
            mnemonic: "BIT 5, H".to_string(),
            size_bytes: 2,
        }),
        0x6D => Some(Opcode {
            mnemonic: "BIT 5, L".to_string(),
            size_bytes: 2,
        }),
        0x6E => Some(Opcode {
            mnemonic: "BIT 5, [HL]".to_string(),
            size_bytes: 2,
        }),
        0x6F => Some(Opcode {
            mnemonic: "BIT 5, A".to_string(),
            size_bytes: 2,
        }),
        0x70 => Some(Opcode {
            mnemonic: "BIT 6, B".to_string(),
            size_bytes: 2,
        }),
        0x71 => Some(Opcode {
            mnemonic: "BIT 6, C".to_string(),
            size_bytes: 2,
        }),
        0x72 => Some(Opcode {
            mnemonic: "BIT 6, D".to_string(),
            size_bytes: 2,
        }),
        0x73 => Some(Opcode {
            mnemonic: "BIT 6, E".to_string(),
            size_bytes: 2,
        }),
        0x74 => Some(Opcode {
            mnemonic: "BIT 6, H".to_string(),
            size_bytes: 2,
        }),
        0x75 => Some(Opcode {
            mnemonic: "BIT 6, L".to_string(),
            size_bytes: 2,
        }),
        0x76 => Some(Opcode {
            mnemonic: "BIT 6, [HL]".to_string(),
            size_bytes: 2,
        }),
        0x77 => Some(Opcode {
            mnemonic: "BIT 6, A".to_string(),
            size_bytes: 2,
        }),
        0x78 => Some(Opcode {
            mnemonic: "BIT 7, B".to_string(),
            size_bytes: 2,
        }),
        0x79 => Some(Opcode {
            mnemonic: "BIT 7, C".to_string(),
            size_bytes: 2,
        }),
        0x7A => Some(Opcode {
            mnemonic: "BIT 7, D".to_string(),
            size_bytes: 2,
        }),
        0x7B => Some(Opcode {
            mnemonic: "BIT 7, E".to_string(),
            size_bytes: 2,
        }),
        0x7C => Some(Opcode {
            mnemonic: "BIT 7, H".to_string(),
            size_bytes: 2,
        }),
        0x7D => Some(Opcode {
            mnemonic: "BIT 7, L".to_string(),
            size_bytes: 2,
        }),
        0x7E => Some(Opcode {
            mnemonic: "BIT 7, [HL]".to_string(),
            size_bytes: 2,
        }),
        0x7F => Some(Opcode {
            mnemonic: "BIT 7, A".to_string(),
            size_bytes: 2,
        }),
        0x80 => Some(Opcode {
            mnemonic: "RES 0, B".to_string(),
            size_bytes: 2,
        }),
        0x81 => Some(Opcode {
            mnemonic: "RES 0, C".to_string(),
            size_bytes: 2,
        }),
        0x82 => Some(Opcode {
            mnemonic: "RES 0, D".to_string(),
            size_bytes: 2,
        }),
        0x83 => Some(Opcode {
            mnemonic: "RES 0, E".to_string(),
            size_bytes: 2,
        }),
        0x84 => Some(Opcode {
            mnemonic: "RES 0, H".to_string(),
            size_bytes: 2,
        }),
        0x85 => Some(Opcode {
            mnemonic: "RES 0, L".to_string(),
            size_bytes: 2,
        }),
        0x86 => Some(Opcode {
            mnemonic: "RES 0, [HL]".to_string(),
            size_bytes: 2,
        }),
        0x87 => Some(Opcode {
            mnemonic: "RES 0, A".to_string(),
            size_bytes: 2,
        }),
        0x88 => Some(Opcode {
            mnemonic: "RES 1, B".to_string(),
            size_bytes: 2,
        }),
        0x89 => Some(Opcode {
            mnemonic: "RES 1, C".to_string(),
            size_bytes: 2,
        }),
        0x8A => Some(Opcode {
            mnemonic: "RES 1, D".to_string(),
            size_bytes: 2,
        }),
        0x8B => Some(Opcode {
            mnemonic: "RES 1, E".to_string(),
            size_bytes: 2,
        }),
        0x8C => Some(Opcode {
            mnemonic: "RES 1, H".to_string(),
            size_bytes: 2,
        }),
        0x8D => Some(Opcode {
            mnemonic: "RES 1, L".to_string(),
            size_bytes: 2,
        }),
        0x8E => Some(Opcode {
            mnemonic: "RES 1, [HL]".to_string(),
            size_bytes: 2,
        }),
        0x8F => Some(Opcode {
            mnemonic: "RES 1, A".to_string(),
            size_bytes: 2,
        }),
        0x90 => Some(Opcode {
            mnemonic: "RES 2, B".to_string(),
            size_bytes: 2,
        }),
        0x91 => Some(Opcode {
            mnemonic: "RES 2, C".to_string(),
            size_bytes: 2,
        }),
        0x92 => Some(Opcode {
            mnemonic: "RES 2, D".to_string(),
            size_bytes: 2,
        }),
        0x93 => Some(Opcode {
            mnemonic: "RES 2, E".to_string(),
            size_bytes: 2,
        }),
        0x94 => Some(Opcode {
            mnemonic: "RES 2, H".to_string(),
            size_bytes: 2,
        }),
        0x95 => Some(Opcode {
            mnemonic: "RES 2, L".to_string(),
            size_bytes: 2,
        }),
        0x96 => Some(Opcode {
            mnemonic: "RES 2, [HL]".to_string(),
            size_bytes: 2,
        }),
        0x97 => Some(Opcode {
            mnemonic: "RES 2, A".to_string(),
            size_bytes: 2,
        }),
        0x98 => Some(Opcode {
            mnemonic: "RES 3, B".to_string(),
            size_bytes: 2,
        }),
        0x99 => Some(Opcode {
            mnemonic: "RES 3, C".to_string(),
            size_bytes: 2,
        }),
        0x9A => Some(Opcode {
            mnemonic: "RES 3, D".to_string(),
            size_bytes: 2,
        }),
        0x9B => Some(Opcode {
            mnemonic: "RES 3, E".to_string(),
            size_bytes: 2,
        }),
        0x9C => Some(Opcode {
            mnemonic: "RES 3, H".to_string(),
            size_bytes: 2,
        }),
        0x9D => Some(Opcode {
            mnemonic: "RES 3, L".to_string(),
            size_bytes: 2,
        }),
        0x9E => Some(Opcode {
            mnemonic: "RES 3, [HL]".to_string(),
            size_bytes: 2,
        }),
        0x9F => Some(Opcode {
            mnemonic: "RES 3, A".to_string(),
            size_bytes: 2,
        }),
        0xA0 => Some(Opcode {
            mnemonic: "RES 4, B".to_string(),
            size_bytes: 2,
        }),
        0xA1 => Some(Opcode {
            mnemonic: "RES 4, C".to_string(),
            size_bytes: 2,
        }),
        0xA2 => Some(Opcode {
            mnemonic: "RES 4, D".to_string(),
            size_bytes: 2,
        }),
        0xA3 => Some(Opcode {
            mnemonic: "RES 4, E".to_string(),
            size_bytes: 2,
        }),
        0xA4 => Some(Opcode {
            mnemonic: "RES 4, H".to_string(),
            size_bytes: 2,
        }),
        0xA5 => Some(Opcode {
            mnemonic: "RES 4, L".to_string(),
            size_bytes: 2,
        }),
        0xA6 => Some(Opcode {
            mnemonic: "RES 4, [HL]".to_string(),
            size_bytes: 2,
        }),
        0xA7 => Some(Opcode {
            mnemonic: "RES 4, A".to_string(),
            size_bytes: 2,
        }),
        0xA8 => Some(Opcode {
            mnemonic: "RES 5, B".to_string(),
            size_bytes: 2,
        }),
        0xA9 => Some(Opcode {
            mnemonic: "RES 5, C".to_string(),
            size_bytes: 2,
        }),
        0xAA => Some(Opcode {
            mnemonic: "RES 5, D".to_string(),
            size_bytes: 2,
        }),
        0xAB => Some(Opcode {
            mnemonic: "RES 5, E".to_string(),
            size_bytes: 2,
        }),
        0xAC => Some(Opcode {
            mnemonic: "RES 5, H".to_string(),
            size_bytes: 2,
        }),
        0xAD => Some(Opcode {
            mnemonic: "RES 5, L".to_string(),
            size_bytes: 2,
        }),
        0xAE => Some(Opcode {
            mnemonic: "RES 5, [HL]".to_string(),
            size_bytes: 2,
        }),
        0xAF => Some(Opcode {
            mnemonic: "RES 5, A".to_string(),
            size_bytes: 2,
        }),
        0xB0 => Some(Opcode {
            mnemonic: "RES 6, B".to_string(),
            size_bytes: 2,
        }),
        0xB1 => Some(Opcode {
            mnemonic: "RES 6, C".to_string(),
            size_bytes: 2,
        }),
        0xB2 => Some(Opcode {
            mnemonic: "RES 6, D".to_string(),
            size_bytes: 2,
        }),
        0xB3 => Some(Opcode {
            mnemonic: "RES 6, E".to_string(),
            size_bytes: 2,
        }),
        0xB4 => Some(Opcode {
            mnemonic: "RES 6, H".to_string(),
            size_bytes: 2,
        }),
        0xB5 => Some(Opcode {
            mnemonic: "RES 6, L".to_string(),
            size_bytes: 2,
        }),
        0xB6 => Some(Opcode {
            mnemonic: "RES 6, [HL]".to_string(),
            size_bytes: 2,
        }),
        0xB7 => Some(Opcode {
            mnemonic: "RES 6, A".to_string(),
            size_bytes: 2,
        }),
        0xB8 => Some(Opcode {
            mnemonic: "RES 7, B".to_string(),
            size_bytes: 2,
        }),
        0xB9 => Some(Opcode {
            mnemonic: "RES 7, C".to_string(),
            size_bytes: 2,
        }),
        0xBA => Some(Opcode {
            mnemonic: "RES 7, D".to_string(),
            size_bytes: 2,
        }),
        0xBB => Some(Opcode {
            mnemonic: "RES 7, E".to_string(),
            size_bytes: 2,
        }),
        0xBC => Some(Opcode {
            mnemonic: "RES 7, H".to_string(),
            size_bytes: 2,
        }),
        0xBD => Some(Opcode {
            mnemonic: "RES 7, L".to_string(),
            size_bytes: 2,
        }),
        0xBE => Some(Opcode {
            mnemonic: "RES 7, [HL]".to_string(),
            size_bytes: 2,
        }),
        0xBF => Some(Opcode {
            mnemonic: "RES 7, A".to_string(),
            size_bytes: 2,
        }),
        0xC0 => Some(Opcode {
            mnemonic: "SET 0, B".to_string(),
            size_bytes: 2,
        }),
        0xC1 => Some(Opcode {
            mnemonic: "SET 0, C".to_string(),
            size_bytes: 2,
        }),
        0xC2 => Some(Opcode {
            mnemonic: "SET 0, D".to_string(),
            size_bytes: 2,
        }),
        0xC3 => Some(Opcode {
            mnemonic: "SET 0, E".to_string(),
            size_bytes: 2,
        }),
        0xC4 => Some(Opcode {
            mnemonic: "SET 0, H".to_string(),
            size_bytes: 2,
        }),
        0xC5 => Some(Opcode {
            mnemonic: "SET 0, L".to_string(),
            size_bytes: 2,
        }),
        0xC6 => Some(Opcode {
            mnemonic: "SET 0, [HL]".to_string(),
            size_bytes: 2,
        }),
        0xC7 => Some(Opcode {
            mnemonic: "SET 0, A".to_string(),
            size_bytes: 2,
        }),
        0xC8 => Some(Opcode {
            mnemonic: "SET 1, B".to_string(),
            size_bytes: 2,
        }),
        0xC9 => Some(Opcode {
            mnemonic: "SET 1, C".to_string(),
            size_bytes: 2,
        }),
        0xCA => Some(Opcode {
            mnemonic: "SET 1, D".to_string(),
            size_bytes: 2,
        }),
        0xCB => Some(Opcode {
            mnemonic: "SET 1, E".to_string(),
            size_bytes: 2,
        }),
        0xCC => Some(Opcode {
            mnemonic: "SET 1, H".to_string(),
            size_bytes: 2,
        }),
        0xCD => Some(Opcode {
            mnemonic: "SET 1, L".to_string(),
            size_bytes: 2,
        }),
        0xCE => Some(Opcode {
            mnemonic: "SET 1, [HL]".to_string(),
            size_bytes: 2,
        }),
        0xCF => Some(Opcode {
            mnemonic: "SET 1, A".to_string(),
            size_bytes: 2,
        }),
        0xD0 => Some(Opcode {
            mnemonic: "SET 2, B".to_string(),
            size_bytes: 2,
        }),
        0xD1 => Some(Opcode {
            mnemonic: "SET 2, C".to_string(),
            size_bytes: 2,
        }),
        0xD2 => Some(Opcode {
            mnemonic: "SET 2, D".to_string(),
            size_bytes: 2,
        }),
        0xD3 => Some(Opcode {
            mnemonic: "SET 2, E".to_string(),
            size_bytes: 2,
        }),
        0xD4 => Some(Opcode {
            mnemonic: "SET 2, H".to_string(),
            size_bytes: 2,
        }),
        0xD5 => Some(Opcode {
            mnemonic: "SET 2, L".to_string(),
            size_bytes: 2,
        }),
        0xD6 => Some(Opcode {
            mnemonic: "SET 2, [HL]".to_string(),
            size_bytes: 2,
        }),
        0xD7 => Some(Opcode {
            mnemonic: "SET 2, A".to_string(),
            size_bytes: 2,
        }),
        0xD8 => Some(Opcode {
            mnemonic: "SET 3, B".to_string(),
            size_bytes: 2,
        }),
        0xD9 => Some(Opcode {
            mnemonic: "SET 3, C".to_string(),
            size_bytes: 2,
        }),
        0xDA => Some(Opcode {
            mnemonic: "SET 3, D".to_string(),
            size_bytes: 2,
        }),
        0xDB => Some(Opcode {
            mnemonic: "SET 3, E".to_string(),
            size_bytes: 2,
        }),
        0xDC => Some(Opcode {
            mnemonic: "SET 3, H".to_string(),
            size_bytes: 2,
        }),
        0xDD => Some(Opcode {
            mnemonic: "SET 3, L".to_string(),
            size_bytes: 2,
        }),
        0xDE => Some(Opcode {
            mnemonic: "SET 3, [HL]".to_string(),
            size_bytes: 2,
        }),
        0xDF => Some(Opcode {
            mnemonic: "SET 3, A".to_string(),
            size_bytes: 2,
        }),
        0xE0 => Some(Opcode {
            mnemonic: "SET 4, B".to_string(),
            size_bytes: 2,
        }),
        0xE1 => Some(Opcode {
            mnemonic: "SET 4, C".to_string(),
            size_bytes: 2,
        }),
        0xE2 => Some(Opcode {
            mnemonic: "SET 4, D".to_string(),
            size_bytes: 2,
        }),
        0xE3 => Some(Opcode {
            mnemonic: "SET 4, E".to_string(),
            size_bytes: 2,
        }),
        0xE4 => Some(Opcode {
            mnemonic: "SET 4, H".to_string(),
            size_bytes: 2,
        }),
        0xE5 => Some(Opcode {
            mnemonic: "SET 4, L".to_string(),
            size_bytes: 2,
        }),
        0xE6 => Some(Opcode {
            mnemonic: "SET 4, [HL]".to_string(),
            size_bytes: 2,
        }),
        0xE7 => Some(Opcode {
            mnemonic: "SET 4, A".to_string(),
            size_bytes: 2,
        }),
        0xE8 => Some(Opcode {
            mnemonic: "SET 5, B".to_string(),
            size_bytes: 2,
        }),
        0xE9 => Some(Opcode {
            mnemonic: "SET 5, C".to_string(),
            size_bytes: 2,
        }),
        0xEA => Some(Opcode {
            mnemonic: "SET 5, D".to_string(),
            size_bytes: 2,
        }),
        0xEB => Some(Opcode {
            mnemonic: "SET 5, E".to_string(),
            size_bytes: 2,
        }),
        0xEC => Some(Opcode {
            mnemonic: "SET 5, H".to_string(),
            size_bytes: 2,
        }),
        0xED => Some(Opcode {
            mnemonic: "SET 5, L".to_string(),
            size_bytes: 2,
        }),
        0xEE => Some(Opcode {
            mnemonic: "SET 5, [HL]".to_string(),
            size_bytes: 2,
        }),
        0xEF => Some(Opcode {
            mnemonic: "SET 5, A".to_string(),
            size_bytes: 2,
        }),
        0xF0 => Some(Opcode {
            mnemonic: "SET 6, B".to_string(),
            size_bytes: 2,
        }),
        0xF1 => Some(Opcode {
            mnemonic: "SET 6, C".to_string(),
            size_bytes: 2,
        }),
        0xF2 => Some(Opcode {
            mnemonic: "SET 6, D".to_string(),
            size_bytes: 2,
        }),
        0xF3 => Some(Opcode {
            mnemonic: "SET 6, E".to_string(),
            size_bytes: 2,
        }),
        0xF4 => Some(Opcode {
            mnemonic: "SET 6, H".to_string(),
            size_bytes: 2,
        }),
        0xF5 => Some(Opcode {
            mnemonic: "SET 6, L".to_string(),
            size_bytes: 2,
        }),
        0xF6 => Some(Opcode {
            mnemonic: "SET 6, [HL]".to_string(),
            size_bytes: 2,
        }),
        0xF7 => Some(Opcode {
            mnemonic: "SET 6, A".to_string(),
            size_bytes: 2,
        }),
        0xF8 => Some(Opcode {
            mnemonic: "SET 7, B".to_string(),
            size_bytes: 2,
        }),
        0xF9 => Some(Opcode {
            mnemonic: "SET 7, C".to_string(),
            size_bytes: 2,
        }),
        0xFA => Some(Opcode {
            mnemonic: "SET 7, D".to_string(),
            size_bytes: 2,
        }),
        0xFB => Some(Opcode {
            mnemonic: "SET 7, E".to_string(),
            size_bytes: 2,
        }),
        0xFC => Some(Opcode {
            mnemonic: "SET 7, H".to_string(),
            size_bytes: 2,
        }),
        0xFD => Some(Opcode {
            mnemonic: "SET 7, L".to_string(),
            size_bytes: 2,
        }),
        0xFE => Some(Opcode {
            mnemonic: "SET 7, [HL]".to_string(),
            size_bytes: 2,
        }),
        0xFF => Some(Opcode {
            mnemonic: "SET 7, A".to_string(),
            size_bytes: 2,
        }),
    }
}
