use crate::cpu::{Cpu, Register, RegisterWide, WriteFlags};
use crate::memory::{Address, Memory};
use crate::util::{self, half_carried_add16, half_carried_add8, half_carried_sub8, u16_to_u8, u8_to_u16};

type OpcodeHandler = fn(cpu: &mut Cpu, memory: &mut Memory);

#[derive(Debug)]
pub struct Opcode {
    pub mnemonic: String,
    pub size_bytes: u8,
    pub handler: Option<OpcodeHandler>,
}

impl Opcode {
    pub fn execute(&self, cpu: &mut Cpu, memory: &mut Memory) {
        match self.handler {
            Some(handler) => handler(cpu, memory),
            None => println!("Unimplemented opcode: {:?}", self)
        };
    }
}

pub fn decode(byte: u8) -> Option<Opcode> {
    match byte {
        0x00 => Some(Opcode {
            mnemonic: "NOP".to_string(),
            size_bytes: 1,
            handler: Some(|_, _| {}),
        }),
        0x01 => Some(Opcode {
            mnemonic: "LD BC, n16".to_string(),
            size_bytes: 3,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                ld_r16_n16(cpu, memory, RegisterWide::BC)
            }),
        }),
        0x02 => Some(Opcode {
            mnemonic: "LD [BC], A".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                ld_indirect_r16_r8(
                    cpu,
                    memory,
                    RegisterWide::BC,
                    Register::A,
                    IndirectAddressingMode::Retain,
                )
            }),
        }),
        0x03 => Some(Opcode {
            mnemonic: "INC BC".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| inc_r16(cpu, RegisterWide::BC)),
        }),
        0x04 => Some(Opcode {
            mnemonic: "INC B".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| inc_r8(cpu, Register::B)),
        }),
        0x05 => Some(Opcode {
            mnemonic: "DEC B".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| dec_r8(cpu, Register::B)),
        }),
        0x06 => Some(Opcode {
            mnemonic: "LD B, n8".to_string(),
            size_bytes: 2,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| ld_r8_n8(cpu, memory, Register::B)),
        }),
        0x07 => Some(Opcode {
            mnemonic: "RLCA".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| {
                let value = cpu.read_register(Register::A);
                let result = value.rotate_left(1);

                cpu.write_flags(WriteFlags {
                    zero: Some(false),
                    subtract: Some(false),
                    half_carry: Some(false),
                    carry: Some(value & 0b1000_0000 != 0),
                });
                cpu.write_register(Register::A, result);
            }),
        }),
        0x08 => Some(Opcode {
            mnemonic: "LD [a16], SP".to_string(),
            size_bytes: 3,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                let pc = cpu.read_register_wide(RegisterWide::PC);
                let lsb = memory.read(Address(pc - 2));
                let msb = memory.read(Address(pc - 1));
                let address = util::u8_to_u16(msb, lsb);
                let sp = cpu.read_register_wide(RegisterWide::SP);
                let (sp_msb, sp_lsb) = util::u16_to_u8(sp);
                memory.write(Address(address), sp_lsb);
                memory.write(Address(address + 1), sp_msb);
            }),
        }),
        0x09 => Some(Opcode {
            mnemonic: "ADD HL, BC".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| add_hl_r16(cpu, RegisterWide::BC)),
        }),
        0x0A => Some(Opcode {
            mnemonic: "LD A, [BC]".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                ld_r8_indirect_r16(
                    cpu,
                    memory,
                    Register::A,
                    RegisterWide::BC,
                    IndirectAddressingMode::Retain,
                )
            }),
        }),
        0x0B => Some(Opcode {
            mnemonic: "DEC BC".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| dec_r16(cpu, RegisterWide::BC)),
        }),
        0x0C => Some(Opcode {
            mnemonic: "INC C".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| inc_r8(cpu, Register::C)),
        }),
        0x0D => Some(Opcode {
            mnemonic: "DEC C".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| dec_r8(cpu, Register::C)),
        }),
        0x0E => Some(Opcode {
            mnemonic: "LD C, n8".to_string(),
            size_bytes: 2,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| ld_r8_n8(cpu, memory, Register::C)),
        }),
        0x0F => Some(Opcode {
            mnemonic: "RRCA ".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0x10 => Some(Opcode {
            mnemonic: "STOP n8".to_string(),
            size_bytes: 2,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                // This should stop the CPU and LCD but I don't think this is important right now,
                // if ever.
            }),
        }),
        0x11 => Some(Opcode {
            mnemonic: "LD DE, n16".to_string(),
            size_bytes: 3,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                ld_r16_n16(cpu, memory, RegisterWide::DE)
            }),
        }),
        0x12 => Some(Opcode {
            mnemonic: "LD [DE], A".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                ld_indirect_r16_r8(
                    cpu,
                    memory,
                    RegisterWide::DE,
                    Register::A,
                    IndirectAddressingMode::Retain,
                )
            }),
        }),
        0x13 => Some(Opcode {
            mnemonic: "INC DE".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| inc_r16(cpu, RegisterWide::DE)),
        }),
        0x14 => Some(Opcode {
            mnemonic: "INC D".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| inc_r8(cpu, Register::D)),
        }),
        0x15 => Some(Opcode {
            mnemonic: "DEC D".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| dec_r8(cpu, Register::D)),
        }),
        0x16 => Some(Opcode {
            mnemonic: "LD D, n8".to_string(),
            size_bytes: 2,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| ld_r8_n8(cpu, memory, Register::D)),
        }),
        0x17 => Some(Opcode {
            mnemonic: "RLA".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                let value = cpu.read_register(Register::A);
                let mut result = value << 1;
                if cpu.read_flags().carry {
                    result |= 0b0000_0001;
                }

                cpu.write_flags(WriteFlags {
                    zero: Some(false),
                    subtract: Some(false),
                    half_carry: Some(false),
                    carry: Some(value & 0b1000_0000 != 0),
                });
                cpu.write_register(Register::A, result);
            })
        }),
        0x18 => Some(Opcode {
            mnemonic: "JR e8".to_string(),
            size_bytes: 2,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| jump_relative(cpu, memory)),
        }),
        0x19 => Some(Opcode {
            mnemonic: "ADD HL, DE".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| add_hl_r16(cpu, RegisterWide::DE)),
        }),
        0x1A => Some(Opcode {
            mnemonic: "LD A, [DE]".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                ld_r8_indirect_r16(
                    cpu,
                    memory,
                    Register::A,
                    RegisterWide::DE,
                    IndirectAddressingMode::Retain,
                )
            }),
        }),
        0x1B => Some(Opcode {
            mnemonic: "DEC DE".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| dec_r16(cpu, RegisterWide::DE)),
        }),
        0x1C => Some(Opcode {
            mnemonic: "INC E".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| inc_r8(cpu, Register::E)),
        }),
        0x1D => Some(Opcode {
            mnemonic: "DEC E".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| dec_r8(cpu, Register::E)),
        }),
        0x1E => Some(Opcode {
            mnemonic: "LD E, n8".to_string(),
            size_bytes: 2,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| ld_r8_n8(cpu, memory, Register::E)),
        }),
        0x1F => Some(Opcode {
            mnemonic: "RRA ".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                let value = cpu.read_register(Register::A);
                let mut result = value >> 1;
                if cpu.read_flags().carry {
                    result |= 0b1000_0000;
                }

                cpu.write_flags(WriteFlags {
                    zero: Some(false),
                    subtract: Some(false),
                    half_carry: Some(false),
                    carry: Some(value & 0b0000_0001 != 0),
                });
                cpu.write_register(Register::A, result);
            })
        }),
        0x20 => Some(Opcode {
            mnemonic: "JR NZ, e8".to_string(),
            size_bytes: 2,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                let flags = cpu.read_flags();
                if !flags.zero {
                    jump_relative(cpu, memory);
                }
            }),
        }),
        0x21 => Some(Opcode {
            mnemonic: "LD HL, n16".to_string(),
            size_bytes: 3,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                ld_r16_n16(cpu, memory, RegisterWide::HL)
            }),
        }),
        0x22 => Some(Opcode {
            mnemonic: "LD [HL+], A".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                ld_indirect_r16_r8(
                    cpu,
                    memory,
                    RegisterWide::HL,
                    Register::A,
                    IndirectAddressingMode::Increment,
                )
            }),
        }),
        0x23 => Some(Opcode {
            mnemonic: "INC HL".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| inc_r16(cpu, RegisterWide::HL)),
        }),
        0x24 => Some(Opcode {
            mnemonic: "INC H".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| inc_r8(cpu, Register::H)),
        }),
        0x25 => Some(Opcode {
            mnemonic: "DEC H".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| dec_r8(cpu, Register::H)),
        }),
        0x26 => Some(Opcode {
            mnemonic: "LD H, n8".to_string(),
            size_bytes: 2,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| ld_r8_n8(cpu, memory, Register::H)),
        }),
        0x27 => Some(Opcode {
            mnemonic: "DAA ".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0x28 => Some(Opcode {
            mnemonic: "JR Z, e8".to_string(),
            size_bytes: 2,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                let flags = cpu.read_flags();
                if !flags.zero {
                    return;
                }

                jump_relative(cpu, memory);
            }),
        }),
        0x29 => Some(Opcode {
            mnemonic: "ADD HL, HL".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| add_hl_r16(cpu, RegisterWide::HL)),
        }),
        0x2A => Some(Opcode {
            mnemonic: "LD A, [HL+]".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                ld_r8_indirect_r16(
                    cpu,
                    memory,
                    Register::A,
                    RegisterWide::HL,
                    IndirectAddressingMode::Increment,
                )
            }),
        }),
        0x2B => Some(Opcode {
            mnemonic: "DEC HL".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| dec_r16(cpu, RegisterWide::HL)),
        }),
        0x2C => Some(Opcode {
            mnemonic: "INC L".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| inc_r8(cpu, Register::L)),
        }),
        0x2D => Some(Opcode {
            mnemonic: "DEC L".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| dec_r8(cpu, Register::L)),
        }),
        0x2E => Some(Opcode {
            mnemonic: "LD L, n8".to_string(),
            size_bytes: 2,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| ld_r8_n8(cpu, memory, Register::L)),
        }),
        0x2F => Some(Opcode {
            mnemonic: "CPL ".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| {
                let a = cpu.read_register(Register::A);
                cpu.write_register(Register::A, !a);
                cpu.write_flags(WriteFlags {
                    subtract: Some(true),
                    half_carry: Some(true),
                    ..Default::default()
                });
            }),
        }),
        0x30 => Some(Opcode {
            mnemonic: "JR NC, e8".to_string(),
            size_bytes: 2,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                if cpu.read_flags().carry {
                    return;
                }

                jump_relative(cpu, memory);
            }),
        }),
        0x31 => Some(Opcode {
            mnemonic: "LD SP, n16".to_string(),
            size_bytes: 3,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                ld_r16_n16(cpu, memory, RegisterWide::SP)
            }),
        }),
        0x32 => Some(Opcode {
            mnemonic: "LD [HL-], A".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                ld_indirect_r16_r8(
                    cpu,
                    memory,
                    RegisterWide::HL,
                    Register::A,
                    IndirectAddressingMode::Decrement,
                )
            }),
        }),
        0x33 => Some(Opcode {
            mnemonic: "INC SP".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| inc_r16(cpu, RegisterWide::SP)),
        }),
        0x34 => Some(Opcode {
            mnemonic: "INC [HL]".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                let hl = cpu.read_register_wide(RegisterWide::HL);
                let value = memory.read(Address(hl));
                let result = value.wrapping_add(1);
                memory.write(Address(hl), result);
                cpu.write_flags(WriteFlags {
                    zero: Some(result == 0),
                    subtract: Some(false),
                    half_carry: Some(half_carried_add8(value, 1)),
                    carry: None,
                });
            }),
        }),
        0x35 => Some(Opcode {
            mnemonic: "DEC [HL]".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                let hl = cpu.read_register_wide(RegisterWide::HL);
                let value = memory.read(Address(hl));
                let (result, _) = value.overflowing_sub(1);
                memory.write(Address(hl), result);
                cpu.write_flags(WriteFlags {
                    zero: Some(result == 0),
                    subtract: Some(true),
                    half_carry: Some(half_carried_sub8(value, 1)),
                    carry: None,
                });
            }),
        }),
        0x36 => Some(Opcode {
            mnemonic: "LD [HL], n8".to_string(),
            size_bytes: 2,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                let pc = cpu.read_register_wide(RegisterWide::SP);
                let dst_value = cpu.read_register_wide(RegisterWide::HL);
                let src_value = memory.read(Address(pc - 1));
                memory.write(Address(dst_value), src_value);
            }),
        }),
        0x37 => Some(Opcode {
            mnemonic: "SCF ".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0x38 => Some(Opcode {
            mnemonic: "JR C, e8".to_string(),
            size_bytes: 2,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                if !cpu.read_flags().carry {
                    return;
                }

                jump_relative(cpu, memory);
            }),
        }),
        0x39 => Some(Opcode {
            mnemonic: "ADD HL, SP".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| add_hl_r16(cpu, RegisterWide::SP)),
        }),
        0x3A => Some(Opcode {
            mnemonic: "LD A, [HL-]".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                ld_r8_indirect_r16(
                    cpu,
                    memory,
                    Register::A,
                    RegisterWide::HL,
                    IndirectAddressingMode::Decrement,
                )
            }),
        }),
        0x3B => Some(Opcode {
            mnemonic: "DEC SP".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| dec_r16(cpu, RegisterWide::SP)),
        }),
        0x3C => Some(Opcode {
            mnemonic: "INC A".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| inc_r8(cpu, Register::A)),
        }),
        0x3D => Some(Opcode {
            mnemonic: "DEC A".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| dec_r8(cpu, Register::A)),
        }),
        0x3E => Some(Opcode {
            mnemonic: "LD A, n8".to_string(),
            size_bytes: 2,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| ld_r8_n8(cpu, memory, Register::A)),
        }),
        0x3F => Some(Opcode {
            mnemonic: "CCF ".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0x40 => Some(Opcode {
            mnemonic: "LD B, B".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _: &mut Memory| ld_r8_r8(cpu, Register::B, Register::B)),
        }),
        0x41 => Some(Opcode {
            mnemonic: "LD B, C".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _: &mut Memory| ld_r8_r8(cpu, Register::B, Register::C)),
        }),
        0x42 => Some(Opcode {
            mnemonic: "LD B, D".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _: &mut Memory| ld_r8_r8(cpu, Register::B, Register::D)),
        }),
        0x43 => Some(Opcode {
            mnemonic: "LD B, E".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _: &mut Memory| ld_r8_r8(cpu, Register::B, Register::E)),
        }),
        0x44 => Some(Opcode {
            mnemonic: "LD B, H".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _: &mut Memory| ld_r8_r8(cpu, Register::B, Register::H)),
        }),
        0x45 => Some(Opcode {
            mnemonic: "LD B, L".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _: &mut Memory| ld_r8_r8(cpu, Register::B, Register::L)),
        }),
        0x46 => Some(Opcode {
            mnemonic: "LD B, [HL]".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                ld_r8_indirect_r16(
                    cpu,
                    memory,
                    Register::B,
                    RegisterWide::HL,
                    IndirectAddressingMode::Retain,
                )
            }),
        }),
        0x47 => Some(Opcode {
            mnemonic: "LD B, A".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _: &mut Memory| ld_r8_r8(cpu, Register::B, Register::A)),
        }),
        0x48 => Some(Opcode {
            mnemonic: "LD C, B".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _: &mut Memory| ld_r8_r8(cpu, Register::C, Register::B)),
        }),
        0x49 => Some(Opcode {
            mnemonic: "LD C, C".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _: &mut Memory| ld_r8_r8(cpu, Register::C, Register::C)),
        }),
        0x4A => Some(Opcode {
            mnemonic: "LD C, D".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _: &mut Memory| ld_r8_r8(cpu, Register::C, Register::D)),
        }),
        0x4B => Some(Opcode {
            mnemonic: "LD C, E".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _: &mut Memory| ld_r8_r8(cpu, Register::C, Register::E)),
        }),
        0x4C => Some(Opcode {
            mnemonic: "LD C, H".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _: &mut Memory| ld_r8_r8(cpu, Register::C, Register::H)),
        }),
        0x4D => Some(Opcode {
            mnemonic: "LD C, L".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _: &mut Memory| ld_r8_r8(cpu, Register::C, Register::L)),
        }),
        0x4E => Some(Opcode {
            mnemonic: "LD C, [HL]".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                ld_r8_indirect_r16(
                    cpu,
                    memory,
                    Register::C,
                    RegisterWide::HL,
                    IndirectAddressingMode::Retain,
                )
            }),
        }),
        0x4F => Some(Opcode {
            mnemonic: "LD C, A".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _: &mut Memory| ld_r8_r8(cpu, Register::C, Register::A)),
        }),
        0x50 => Some(Opcode {
            mnemonic: "LD D, B".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _: &mut Memory| ld_r8_r8(cpu, Register::D, Register::B)),
        }),
        0x51 => Some(Opcode {
            mnemonic: "LD D, C".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _: &mut Memory| ld_r8_r8(cpu, Register::D, Register::C)),
        }),
        0x52 => Some(Opcode {
            mnemonic: "LD D, D".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _: &mut Memory| ld_r8_r8(cpu, Register::D, Register::D)),
        }),
        0x53 => Some(Opcode {
            mnemonic: "LD D, E".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _: &mut Memory| ld_r8_r8(cpu, Register::D, Register::E)),
        }),
        0x54 => Some(Opcode {
            mnemonic: "LD D, H".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _: &mut Memory| ld_r8_r8(cpu, Register::D, Register::H)),
        }),
        0x55 => Some(Opcode {
            mnemonic: "LD D, L".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _: &mut Memory| ld_r8_r8(cpu, Register::D, Register::L)),
        }),
        0x56 => Some(Opcode {
            mnemonic: "LD D, [HL]".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                ld_r8_indirect_r16(
                    cpu,
                    memory,
                    Register::D,
                    RegisterWide::HL,
                    IndirectAddressingMode::Retain,
                )
            }),
        }),
        0x57 => Some(Opcode {
            mnemonic: "LD D, A".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _: &mut Memory| ld_r8_r8(cpu, Register::D, Register::A)),
        }),
        0x58 => Some(Opcode {
            mnemonic: "LD E, B".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _: &mut Memory| ld_r8_r8(cpu, Register::E, Register::B)),
        }),
        0x59 => Some(Opcode {
            mnemonic: "LD E, C".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _: &mut Memory| ld_r8_r8(cpu, Register::E, Register::C)),
        }),
        0x5A => Some(Opcode {
            mnemonic: "LD E, D".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _: &mut Memory| ld_r8_r8(cpu, Register::E, Register::D)),
        }),
        0x5B => Some(Opcode {
            mnemonic: "LD E, E".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _: &mut Memory| ld_r8_r8(cpu, Register::E, Register::E)),
        }),
        0x5C => Some(Opcode {
            mnemonic: "LD E, H".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _: &mut Memory| ld_r8_r8(cpu, Register::E, Register::H)),
        }),
        0x5D => Some(Opcode {
            mnemonic: "LD E, L".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _: &mut Memory| ld_r8_r8(cpu, Register::E, Register::L)),
        }),
        0x5E => Some(Opcode {
            mnemonic: "LD E, [HL]".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                ld_r8_indirect_r16(
                    cpu,
                    memory,
                    Register::E,
                    RegisterWide::HL,
                    IndirectAddressingMode::Retain,
                )
            }),
        }),
        0x5F => Some(Opcode {
            mnemonic: "LD E, A".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _: &mut Memory| ld_r8_r8(cpu, Register::E, Register::A)),
        }),
        0x60 => Some(Opcode {
            mnemonic: "LD H, B".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _: &mut Memory| ld_r8_r8(cpu, Register::H, Register::B)),
        }),
        0x61 => Some(Opcode {
            mnemonic: "LD H, C".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _: &mut Memory| ld_r8_r8(cpu, Register::H, Register::C)),
        }),
        0x62 => Some(Opcode {
            mnemonic: "LD H, D".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _: &mut Memory| ld_r8_r8(cpu, Register::H, Register::D)),
        }),
        0x63 => Some(Opcode {
            mnemonic: "LD H, E".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _: &mut Memory| ld_r8_r8(cpu, Register::H, Register::E)),
        }),
        0x64 => Some(Opcode {
            mnemonic: "LD H, H".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _: &mut Memory| ld_r8_r8(cpu, Register::H, Register::H)),
        }),
        0x65 => Some(Opcode {
            mnemonic: "LD H, L".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _: &mut Memory| ld_r8_r8(cpu, Register::H, Register::L)),
        }),
        0x66 => Some(Opcode {
            mnemonic: "LD H, [HL]".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                ld_r8_indirect_r16(
                    cpu,
                    memory,
                    Register::H,
                    RegisterWide::HL,
                    IndirectAddressingMode::Retain,
                )
            }),
        }),
        0x67 => Some(Opcode {
            mnemonic: "LD H, A".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _: &mut Memory| ld_r8_r8(cpu, Register::H, Register::A)),
        }),
        0x68 => Some(Opcode {
            mnemonic: "LD L, B".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _: &mut Memory| ld_r8_r8(cpu, Register::L, Register::B)),
        }),
        0x69 => Some(Opcode {
            mnemonic: "LD L, C".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _: &mut Memory| ld_r8_r8(cpu, Register::L, Register::C)),
        }),
        0x6A => Some(Opcode {
            mnemonic: "LD L, D".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _: &mut Memory| ld_r8_r8(cpu, Register::L, Register::D)),
        }),
        0x6B => Some(Opcode {
            mnemonic: "LD L, E".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _: &mut Memory| ld_r8_r8(cpu, Register::L, Register::E)),
        }),
        0x6C => Some(Opcode {
            mnemonic: "LD L, H".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _: &mut Memory| ld_r8_r8(cpu, Register::L, Register::H)),
        }),
        0x6D => Some(Opcode {
            mnemonic: "LD L, L".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _: &mut Memory| ld_r8_r8(cpu, Register::L, Register::L)),
        }),
        0x6E => Some(Opcode {
            mnemonic: "LD L, [HL]".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                ld_r8_indirect_r16(
                    cpu,
                    memory,
                    Register::L,
                    RegisterWide::HL,
                    IndirectAddressingMode::Retain,
                )
            }),
        }),
        0x6F => Some(Opcode {
            mnemonic: "LD L, A".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _: &mut Memory| ld_r8_r8(cpu, Register::L, Register::A)),
        }),
        0x70 => Some(Opcode {
            mnemonic: "LD [HL], B".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                ld_indirect_r16_r8(
                    cpu,
                    memory,
                    RegisterWide::HL,
                    Register::B,
                    IndirectAddressingMode::Retain,
                )
            }),
        }),
        0x71 => Some(Opcode {
            mnemonic: "LD [HL], C".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                ld_indirect_r16_r8(
                    cpu,
                    memory,
                    RegisterWide::HL,
                    Register::C,
                    IndirectAddressingMode::Retain,
                )
            }),
        }),
        0x72 => Some(Opcode {
            mnemonic: "LD [HL], D".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                ld_indirect_r16_r8(
                    cpu,
                    memory,
                    RegisterWide::HL,
                    Register::D,
                    IndirectAddressingMode::Retain,
                )
            }),
        }),
        0x73 => Some(Opcode {
            mnemonic: "LD [HL], E".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                ld_indirect_r16_r8(
                    cpu,
                    memory,
                    RegisterWide::HL,
                    Register::E,
                    IndirectAddressingMode::Retain,
                )
            }),
        }),
        0x74 => Some(Opcode {
            mnemonic: "LD [HL], H".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                ld_indirect_r16_r8(
                    cpu,
                    memory,
                    RegisterWide::HL,
                    Register::H,
                    IndirectAddressingMode::Retain,
                )
            }),
        }),
        0x75 => Some(Opcode {
            mnemonic: "LD [HL], L".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                ld_indirect_r16_r8(
                    cpu,
                    memory,
                    RegisterWide::HL,
                    Register::L,
                    IndirectAddressingMode::Retain,
                )
            }),
        }),
        0x76 => Some(Opcode {
            mnemonic: "HALT ".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0x77 => Some(Opcode {
            mnemonic: "LD [HL], A".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                ld_indirect_r16_r8(
                    cpu,
                    memory,
                    RegisterWide::HL,
                    Register::A,
                    IndirectAddressingMode::Retain,
                )
            }),
        }),
        0x78 => Some(Opcode {
            mnemonic: "LD A, B".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _: &mut Memory| ld_r8_r8(cpu, Register::A, Register::B)),
        }),
        0x79 => Some(Opcode {
            mnemonic: "LD A, C".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _: &mut Memory| ld_r8_r8(cpu, Register::A, Register::C)),
        }),
        0x7A => Some(Opcode {
            mnemonic: "LD A, D".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _: &mut Memory| ld_r8_r8(cpu, Register::A, Register::D)),
        }),
        0x7B => Some(Opcode {
            mnemonic: "LD A, E".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _: &mut Memory| ld_r8_r8(cpu, Register::A, Register::E)),
        }),
        0x7C => Some(Opcode {
            mnemonic: "LD A, H".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _: &mut Memory| ld_r8_r8(cpu, Register::A, Register::H)),
        }),
        0x7D => Some(Opcode {
            mnemonic: "LD A, L".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _: &mut Memory| ld_r8_r8(cpu, Register::A, Register::L)),
        }),
        0x7E => Some(Opcode {
            mnemonic: "LD A, [HL]".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                ld_r8_indirect_r16(
                    cpu,
                    memory,
                    Register::A,
                    RegisterWide::HL,
                    IndirectAddressingMode::Retain,
                )
            }),
        }),
        0x7F => Some(Opcode {
            mnemonic: "LD A, A".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _: &mut Memory| ld_r8_r8(cpu, Register::A, Register::A)),
        }),
        0x80 => Some(Opcode {
            mnemonic: "ADD A, B".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| add_r8(cpu, Register::B)),
        }),
        0x81 => Some(Opcode {
            mnemonic: "ADD A, C".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| add_r8(cpu, Register::C)),
        }),
        0x82 => Some(Opcode {
            mnemonic: "ADD A, D".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| add_r8(cpu, Register::D)),
        }),
        0x83 => Some(Opcode {
            mnemonic: "ADD A, E".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| add_r8(cpu, Register::E)),
        }),
        0x84 => Some(Opcode {
            mnemonic: "ADD A, H".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| add_r8(cpu, Register::H)),
        }),
        0x85 => Some(Opcode {
            mnemonic: "ADD A, L".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| add_r8(cpu, Register::L)),
        }),
        0x86 => Some(Opcode {
            mnemonic: "ADD A, [HL]".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                let a = cpu.read_register(Register::A);
                let hl = cpu.read_register_wide(RegisterWide::HL);
                let b = memory.read(Address(hl));
                let (result, overflowed) = a.overflowing_add(b);
                cpu.write_register(Register::A, result);
                cpu.write_flags(WriteFlags {
                    zero: Some(result == 0),
                    subtract: Some(false),
                    half_carry: Some(half_carried_add8(a, b)),
                    carry: Some(overflowed),
                });
            }),
        }),
        0x87 => Some(Opcode {
            mnemonic: "ADD A, A".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| add_r8(cpu, Register::A)),
        }),
        0x88 => Some(Opcode {
            mnemonic: "ADC A, B".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| adc_r8(cpu, Register::B)),
        }),
        0x89 => Some(Opcode {
            mnemonic: "ADC A, C".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| adc_r8(cpu, Register::C)),
        }),
        0x8A => Some(Opcode {
            mnemonic: "ADC A, D".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| adc_r8(cpu, Register::D)),
        }),
        0x8B => Some(Opcode {
            mnemonic: "ADC A, E".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| adc_r8(cpu, Register::E)),
        }),
        0x8C => Some(Opcode {
            mnemonic: "ADC A, H".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| adc_r8(cpu, Register::H)),
        }),
        0x8D => Some(Opcode {
            mnemonic: "ADC A, L".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| adc_r8(cpu, Register::L)),
        }),
        0x8E => Some(Opcode {
            mnemonic: "ADC A, [HL]".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                let a = cpu.read_register(Register::A);
                let carry_bit = cpu.read_flags().carry as u8;
                let hl = cpu.read_register_wide(RegisterWide::HL);
                let hl_value = memory.read(Address(hl));
                let b = hl_value.wrapping_add(carry_bit);
                let (mut result, mut overflowed) = a.overflowing_add(b);

                if cpu.read_flags().carry {
                    let (carry_result, carry_overflowed) = result.overflowing_add(1);
                    result = carry_result;
                    overflowed |= carry_overflowed;
                }

                cpu.write_register(Register::A, result);
                cpu.write_flags(WriteFlags {
                    zero: Some(result == 0),
                    subtract: Some(false),
                    half_carry: Some(half_carried_add8(a, b)),
                    carry: Some(overflowed),
                });
            }),
        }),
        0x8F => Some(Opcode {
            mnemonic: "ADC A, A".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| adc_r8(cpu, Register::A)),
        }),
        0x90 => Some(Opcode {
            mnemonic: "SUB A, B".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| sub_r8(cpu, Register::B)),
        }),
        0x91 => Some(Opcode {
            mnemonic: "SUB A, C".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| sub_r8(cpu, Register::C)),
        }),
        0x92 => Some(Opcode {
            mnemonic: "SUB A, D".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| sub_r8(cpu, Register::D)),
        }),
        0x93 => Some(Opcode {
            mnemonic: "SUB A, E".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| sub_r8(cpu, Register::E)),
        }),
        0x94 => Some(Opcode {
            mnemonic: "SUB A, H".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| sub_r8(cpu, Register::H)),
        }),
        0x95 => Some(Opcode {
            mnemonic: "SUB A, L".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| sub_r8(cpu, Register::L)),
        }),
        0x96 => Some(Opcode {
            mnemonic: "SUB A, [HL]".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0x97 => Some(Opcode {
            mnemonic: "SUB A, A".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| sub_r8(cpu, Register::A)),
        }),
        0x98 => Some(Opcode {
            mnemonic: "SBC A, B".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| sbc_r8(cpu, Register::B)),
        }),
        0x99 => Some(Opcode {
            mnemonic: "SBC A, C".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| sbc_r8(cpu, Register::C)),
        }),
        0x9A => Some(Opcode {
            mnemonic: "SBC A, D".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| sbc_r8(cpu, Register::D)),
        }),
        0x9B => Some(Opcode {
            mnemonic: "SBC A, E".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| sbc_r8(cpu, Register::E)),
        }),
        0x9C => Some(Opcode {
            mnemonic: "SBC A, H".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| sbc_r8(cpu, Register::H)),
        }),
        0x9D => Some(Opcode {
            mnemonic: "SBC A, L".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| sbc_r8(cpu, Register::L)),
        }),
        0x9E => Some(Opcode {
            mnemonic: "SBC A, [HL]".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0x9F => Some(Opcode {
            mnemonic: "SBC A, A".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| sbc_r8(cpu, Register::A)),
        }),
        0xA0 => Some(Opcode {
            mnemonic: "AND A, B".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| and_r8(cpu, Register::B)),
        }),
        0xA1 => Some(Opcode {
            mnemonic: "AND A, C".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| and_r8(cpu, Register::C)),
        }),
        0xA2 => Some(Opcode {
            mnemonic: "AND A, D".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| and_r8(cpu, Register::D)),
        }),
        0xA3 => Some(Opcode {
            mnemonic: "AND A, E".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| and_r8(cpu, Register::E)),
        }),
        0xA4 => Some(Opcode {
            mnemonic: "AND A, H".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| and_r8(cpu, Register::H)),
        }),
        0xA5 => Some(Opcode {
            mnemonic: "AND A, L".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| and_r8(cpu, Register::L)),
        }),
        0xA6 => Some(Opcode {
            mnemonic: "AND A, [HL]".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xA7 => Some(Opcode {
            mnemonic: "AND A, A".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| and_r8(cpu, Register::A)),
        }),
        0xA8 => Some(Opcode {
            mnemonic: "XOR A, B".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| xor_r8(cpu, Register::B)),
        }),
        0xA9 => Some(Opcode {
            mnemonic: "XOR A, C".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| xor_r8(cpu, Register::C)),
        }),
        0xAA => Some(Opcode {
            mnemonic: "XOR A, D".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| xor_r8(cpu, Register::D)),
        }),
        0xAB => Some(Opcode {
            mnemonic: "XOR A, E".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| xor_r8(cpu, Register::E)),
        }),
        0xAC => Some(Opcode {
            mnemonic: "XOR A, H".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| xor_r8(cpu, Register::H)),
        }),
        0xAD => Some(Opcode {
            mnemonic: "XOR A, L".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| xor_r8(cpu, Register::L)),
        }),
        0xAE => Some(Opcode {
            mnemonic: "XOR A, [HL]".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                let a = cpu.read_register(Register::A);
                let hl = cpu.read_register_wide(RegisterWide::HL);
                let value = memory.read(Address(hl));
                let result = a ^ value;
                cpu.write_register(Register::A, result);
                cpu.write_flags(WriteFlags {
                    zero: Some(result == 0),
                    subtract: Some(false),
                    half_carry: Some(false),
                    carry: Some(false),
                });
            })
        }),
        0xAF => Some(Opcode {
            mnemonic: "XOR A, A".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| xor_r8(cpu, Register::A)),
        }),
        0xB0 => Some(Opcode {
            mnemonic: "OR A, B".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| or_r8(cpu, Register::B)),
        }),
        0xB1 => Some(Opcode {
            mnemonic: "OR A, C".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| or_r8(cpu, Register::C)),
        }),
        0xB2 => Some(Opcode {
            mnemonic: "OR A, D".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| or_r8(cpu, Register::D)),
        }),
        0xB3 => Some(Opcode {
            mnemonic: "OR A, E".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| or_r8(cpu, Register::E)),
        }),
        0xB4 => Some(Opcode {
            mnemonic: "OR A, H".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| or_r8(cpu, Register::H)),
        }),
        0xB5 => Some(Opcode {
            mnemonic: "OR A, L".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| or_r8(cpu, Register::L)),
        }),
        0xB6 => Some(Opcode {
            mnemonic: "OR A, [HL]".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                let a = cpu.read_register(Register::A);
                let hl = cpu.read_register_wide(RegisterWide::HL);
                let b = memory.read(Address(hl));
                let result = a | b;
                cpu.write_register(Register::A, result);
                cpu.write_flags(WriteFlags {
                    zero: Some(result == 0),
                    subtract: Some(false),
                    half_carry: Some(false),
                    carry: Some(false),
                });
            }),
        }),
        0xB7 => Some(Opcode {
            mnemonic: "OR A, A".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| or_r8(cpu, Register::A)),
        }),
        0xB8 => Some(Opcode {
            mnemonic: "CP A, B".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| cp_r8(cpu, Register::B)),
        }),
        0xB9 => Some(Opcode {
            mnemonic: "CP A, C".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| cp_r8(cpu, Register::C)),
        }),
        0xBA => Some(Opcode {
            mnemonic: "CP A, D".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| cp_r8(cpu, Register::D)),
        }),
        0xBB => Some(Opcode {
            mnemonic: "CP A, E".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| cp_r8(cpu, Register::E)),
        }),
        0xBC => Some(Opcode {
            mnemonic: "CP A, H".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| cp_r8(cpu, Register::H)),
        }),
        0xBD => Some(Opcode {
            mnemonic: "CP A, L".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| cp_r8(cpu, Register::L)),
        }),
        0xBE => Some(Opcode {
            mnemonic: "CP A, [HL]".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                let a = cpu.read_register(Register::A);
                let hl = cpu.read_register_wide(RegisterWide::HL);
                let b = memory.read(Address(hl));
                let (result, overflowed) = a.overflowing_sub(b);
                cpu.write_flags(WriteFlags {
                    zero: Some(result == 0),
                    subtract: Some(true),
                    half_carry: Some(half_carried_sub8(a, b)),
                    carry: Some(overflowed),
                });
            }),
        }),
        0xBF => Some(Opcode {
            mnemonic: "CP A, A".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| cp_r8(cpu, Register::A)),
        }),
        0xC0 => Some(Opcode {
            mnemonic: "RET NZ".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                if cpu.read_flags().zero {
                    return;
                }

                pop(cpu, memory, RegisterWide::PC);
            }),
        }),
        0xC1 => Some(Opcode {
            mnemonic: "POP BC".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| pop(cpu, memory, RegisterWide::BC)),
        }),
        0xC2 => Some(Opcode {
            mnemonic: "JP NZ, a16".to_string(),
            size_bytes: 3,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                if cpu.read_flags().zero {
                    return;
                }

                jump(cpu, memory);
            }),
        }),
        0xC3 => Some(Opcode {
            mnemonic: "JP a16".to_string(),
            size_bytes: 3,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                let pc = cpu.read_register_wide(RegisterWide::PC);
                let msb = memory.read(Address(pc - 1));
                let lsb = memory.read(Address(pc - 2));
                let nn = u8_to_u16(msb, lsb);
                cpu.write_register_wide(RegisterWide::PC, nn);
            }),
        }),
        0xC4 => Some(Opcode {
            mnemonic: "CALL NZ, a16".to_string(),
            size_bytes: 3,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                if cpu.read_flags().zero {
                    return;
                }

                call_a16(cpu, memory);
            }),
        }),
        0xC5 => Some(Opcode {
            mnemonic: "PUSH BC".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| push(cpu, memory, RegisterWide::BC)),
        }),
        0xC6 => Some(Opcode {
            mnemonic: "ADD A, n8".to_string(),
            size_bytes: 2,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                let a = cpu.read_register(Register::A);
                let pc = cpu.read_register_wide(RegisterWide::PC);
                let imm = memory.read(Address(pc - 1));
                let (result, overflowed) = a.overflowing_add(imm);
                cpu.write_register(Register::A, result);
                cpu.write_flags(WriteFlags {
                    zero: Some(result == 0),
                    subtract: Some(false),
                    half_carry: Some(half_carried_add8(a, imm)),
                    carry: Some(overflowed),
                });
            })
        }),
        0xC7 => Some(Opcode {
            mnemonic: "RST $00".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| rst(cpu, memory, 0x00)),
        }),
        0xC8 => Some(Opcode {
            mnemonic: "RET Z".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                if !cpu.read_flags().zero {
                    return;
                }

                pop(cpu, memory, RegisterWide::PC);
            }),
        }),
        0xC9 => Some(Opcode {
            mnemonic: "RET".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| pop(cpu, memory, RegisterWide::PC)),
        }),
        0xCA => Some(Opcode {
            mnemonic: "JP Z, a16".to_string(),
            size_bytes: 3,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                if !cpu.read_flags().zero {
                    return;
                }

                jump(cpu, memory);
            }),
        }),
        0xCB => Some(Opcode {
            mnemonic: "PREFIX".to_string(),
            size_bytes: 1,
            handler: Some(|_, _| {}),
        }),
        0xCC => Some(Opcode {
            mnemonic: "CALL Z, a16".to_string(),
            size_bytes: 3,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                let zero = cpu.read_flags().zero;
                if !zero {
                    return;
                }

                call_a16(cpu, memory);
            }),
        }),
        0xCD => Some(Opcode {
            mnemonic: "CALL a16".to_string(),
            size_bytes: 3,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| call_a16(cpu, memory)),
        }),
        0xCE => Some(Opcode {
            mnemonic: "ADC A, n8".to_string(),
            size_bytes: 2,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                let a = cpu.read_register(Register::A);
                let pc = cpu.read_register_wide(RegisterWide::PC);
                let n = memory.read(Address(pc - 1));
                let (mut result, mut overflowed) = a.overflowing_add(n);
                let mut half_carried = util::half_carried_add8(a, n);

                if cpu.read_flags().carry {
                    let (carry_result, carry_overflowed) = result.overflowing_add(1);
                    result = carry_result;
                    overflowed |= carry_overflowed;
                    half_carried = util::half_carried_add8(a, n + 1);
                }

                cpu.write_register(Register::A, result);
                cpu.write_flags(WriteFlags {
                    zero: Some(result == 0),
                    subtract: Some(false),
                    half_carry: Some(half_carried),
                    carry: Some(overflowed),
                });
            }),
        }),
        0xCF => Some(Opcode {
            mnemonic: "RST $08".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| rst(cpu, memory, 0x08)),
        }),
        0xD0 => Some(Opcode {
            mnemonic: "RET NC".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                if cpu.read_flags().carry {
                    return;
                }

                pop(cpu, memory, RegisterWide::PC);
            }),
        }),
        0xD1 => Some(Opcode {
            mnemonic: "POP DE".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| pop(cpu, memory, RegisterWide::DE)),
        }),
        0xD2 => Some(Opcode {
            mnemonic: "JP NC, a16".to_string(),
            size_bytes: 3,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                if cpu.read_flags().carry {
                    return;
                }

                jump(cpu, memory);
            }),
        }),
        0xD4 => Some(Opcode {
            mnemonic: "CALL NC, a16".to_string(),
            size_bytes: 3,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                if cpu.read_flags().carry {
                    return;
                }

                call_a16(cpu, memory);
            }),
        }),
        0xD5 => Some(Opcode {
            mnemonic: "PUSH DE".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| push(cpu, memory, RegisterWide::DE)),
        }),
        0xD6 => Some(Opcode {
            mnemonic: "SUB A, n8".to_string(),
            size_bytes: 2,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                let a = cpu.read_register(Register::A);
                let pc = cpu.read_register_wide(RegisterWide::PC);
                let imm = memory.read(Address(pc - 1));
                let (result, overflowed) = a.overflowing_sub(imm);
                cpu.write_register(Register::A, result);
                cpu.write_flags(WriteFlags {
                    zero: Some(result == 0),
                    subtract: Some(true),
                    half_carry: Some(half_carried_sub8(a, imm)),
                    carry: Some(overflowed),
                });
            })
        }),
        0xD7 => Some(Opcode {
            mnemonic: "RST $10".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| rst(cpu, memory, 0x10)),
        }),
        0xD8 => Some(Opcode {
            mnemonic: "RET C".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                if !cpu.read_flags().carry {
                    return;
                }

                pop(cpu, memory, RegisterWide::PC);
            }),
        }),
        0xD9 => Some(Opcode {
            mnemonic: "RETI ".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                cpu.request_ime_enable();
                pop(cpu, memory, RegisterWide::PC);
            })
        }),
        0xDA => Some(Opcode {
            mnemonic: "JP C, a16".to_string(),
            size_bytes: 3,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                if !cpu.read_flags().carry {
                    return;
                }

                jump(cpu, memory);
            }),
        }),
        0xDC => Some(Opcode {
            mnemonic: "CALL C, a16".to_string(),
            size_bytes: 3,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                if !cpu.read_flags().carry {
                    return;
                }

                call_a16(cpu, memory);
            }),
        }),
        0xDE => Some(Opcode {
            mnemonic: "SBC A, n8".to_string(),
            size_bytes: 2,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                let a = cpu.read_register(Register::A);
                let carry_bit = cpu.read_flags().carry as u8;
                let pc = cpu.read_register_wide(RegisterWide::PC);
                let b = memory.read(Address(pc)).wrapping_add(carry_bit);

                let (result, overflowed) = a.overflowing_sub(b);
                cpu.write_register(Register::A, result);
                cpu.write_flags(WriteFlags {
                    zero: Some(result == 0),
                    subtract: Some(true),
                    half_carry: Some(half_carried_sub8(a, b)),
                    carry: Some(overflowed),
                });
            }),
        }),
        0xDF => Some(Opcode {
            mnemonic: "RST $18".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| rst(cpu, memory, 0x18)),
        }),
        0xE0 => Some(Opcode {
            mnemonic: "LDH [a8], A".to_string(),
            size_bytes: 2,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                let pc = cpu.read_register_wide(RegisterWide::PC);
                let imm = memory.read(Address(pc - 1));
                let address = u8_to_u16(0xFF, imm);
                let a = cpu.read_register(Register::A);
                memory.write(Address(address), a);
            }),
        }),
        0xE1 => Some(Opcode {
            mnemonic: "POP HL".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| pop(cpu, memory, RegisterWide::HL)),
        }),
        0xE2 => Some(Opcode {
            mnemonic: "LD [C], A".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                let a = cpu.read_register(Register::A);
                let c = cpu.read_register(Register::C);
                let address = Address(util::u8_to_u16(0xFF, c));
                memory.write(address, a);
            }),
        }),
        0xE5 => Some(Opcode {
            mnemonic: "PUSH HL".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| push(cpu, memory, RegisterWide::HL)),
        }),
        0xE6 => Some(Opcode {
            mnemonic: "AND A, n8".to_string(),
            size_bytes: 2,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                let a = cpu.read_register(Register::A);
                let pc = cpu.read_register_wide(RegisterWide::PC);
                let imm = memory.read(Address(pc - 1));
                let result = a & imm;
                cpu.write_register(Register::A, result);
                cpu.write_flags(WriteFlags {
                    zero: Some(result == 0),
                    subtract: Some(false),
                    half_carry: Some(true),
                    carry: Some(false),
                });
            })
        }),
        0xE7 => Some(Opcode {
            mnemonic: "RST $20".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| rst(cpu, memory, 0x20)),
        }),
        0xE8 => Some(Opcode {
            mnemonic: "ADD SP, e8".to_string(),
            size_bytes: 2,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                let pc = cpu.read_register_wide(RegisterWide::PC);
                let imm = memory.read(Address(pc - 1));
                let sp = cpu.read_register_wide(RegisterWide::SP);

                // Unintuitively, even though we're adding to a 16 bit integer, the half-carry
                // flag should be based on the low byte i.e. set when carry occurs from bit 3
                // to bit 4.
                //
                // See:
                // https://stackoverflow.com/questions/57958631/game-boy-half-carry-flag-and-16-bit-instructions-especially-opcode-0xe8/57978555#57978555
                let half_carried = util::half_carried_add8(sp as u8, imm);

                // Similar to the half-carry, for the carry we need to look at only the low byte
                let sp_low_byte = (sp & 0xFF) as u8;
                let (_, carried) = sp_low_byte.overflowing_add(imm);

                let sp = sp.wrapping_add_signed((imm as i8).into());

                cpu.write_register_wide(RegisterWide::SP, sp);
                cpu.write_flags(WriteFlags {
                    zero: Some(false),
                    subtract: Some(false),
                    half_carry: Some(half_carried),
                    carry: Some(carried),
                });
            })
        }),
        0xE9 => Some(Opcode {
            mnemonic: "JP HL".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| {
                let hl = cpu.read_register_wide(RegisterWide::HL);
                cpu.write_register_wide(RegisterWide::PC, hl);
            }),
        }),
        0xEA => Some(Opcode {
            mnemonic: "LD [a16], A".to_string(),
            size_bytes: 3,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                let pc = cpu.read_register_wide(RegisterWide::PC);
                let lsb = memory.read(Address(pc - 2));
                let msb = memory.read(Address(pc - 1));
                let address = util::u8_to_u16(msb, lsb);
                let a = cpu.read_register(Register::A);
                memory.write(Address(address), a);
            }),
        }),
        0xEE => Some(Opcode {
            mnemonic: "XOR A, n8".to_string(),
            size_bytes: 2,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                let a = cpu.read_register(Register::A);
                let pc = cpu.read_register_wide(RegisterWide::PC);
                let imm = memory.read(Address(pc - 1));
                let result = a ^ imm;
                cpu.write_register(Register::A, result);
                cpu.write_flags(WriteFlags {
                    zero: Some(result == 0),
                    subtract: Some(false),
                    half_carry: Some(false),
                    carry: Some(false),
                });
            })
        }),
        0xEF => Some(Opcode {
            mnemonic: "RST $28".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| rst(cpu, memory, 0x28)),
        }),
        0xF0 => Some(Opcode {
            mnemonic: "LDH A, [a8]".to_string(),
            size_bytes: 2,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                let pc = cpu.read_register_wide(RegisterWide::PC);
                let imm = memory.read(Address(pc - 1));
                let address = u8_to_u16(0xFF, imm);
                let value = memory.read(Address(address));
                cpu.write_register(Register::A, value);
            }),
        }),
        0xF1 => Some(Opcode {
            mnemonic: "POP AF".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| pop(cpu, memory, RegisterWide::AF)),
        }),
        0xF2 => Some(Opcode {
            mnemonic: "LD A, [C]".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xF3 => Some(Opcode {
            mnemonic: "DI".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| {
                cpu.request_ime_disable();
            })
        }),
        0xF5 => Some(Opcode {
            mnemonic: "PUSH AF".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| push(cpu, memory, RegisterWide::AF)),
        }),
        0xF6 => Some(Opcode {
            mnemonic: "OR A, n8".to_string(),
            size_bytes: 2,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                let a = cpu.read_register(Register::A);
                let pc = cpu.read_register_wide(RegisterWide::PC);
                let imm = memory.read(Address(pc - 1));
                let result = a | imm;
                cpu.write_register(Register::A, result);
                cpu.write_flags(WriteFlags {
                    zero: Some(result == 0),
                    subtract: Some(false),
                    half_carry: Some(false),
                    carry: Some(false),
                });
            }),
        }),
        0xF7 => Some(Opcode {
            mnemonic: "RST $30".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| rst(cpu, memory, 0x30)),
        }),
        0xF8 => Some(Opcode {
            mnemonic: "LD HL, SP + e8".to_string(),
            size_bytes: 2,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                let pc = cpu.read_register_wide(RegisterWide::PC);
                let imm = memory.read(Address(pc - 1));
                let sp = cpu.read_register_wide(RegisterWide::SP);

                // Unintuitively, even though we're adding to a 16 bit integer, the half-carry
                // flag should be based on the low byte i.e. set when carry occurs from bit 3
                // to bit 4.
                //
                // See:
                // https://stackoverflow.com/questions/57958631/game-boy-half-carry-flag-and-16-bit-instructions-especially-opcode-0xe8/57978555#57978555
                let half_carried = util::half_carried_add8(sp as u8, imm);

                // Similar to the half-carry, for the carry we need to look at only the low byte
                let sp_low_byte = (sp & 0xFF) as u8;
                let (_, carried) = sp_low_byte.overflowing_add(imm);

                let sp = sp.wrapping_add_signed((imm as i8).into());

                cpu.write_register_wide(RegisterWide::HL, sp);
                cpu.write_flags(WriteFlags {
                    zero: Some(false),
                    subtract: Some(false),
                    half_carry: Some(half_carried),
                    carry: Some(carried),
                });
            })
        }),
        0xF9 => Some(Opcode {
            mnemonic: "LD SP, HL".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| {
                let hl = cpu.read_register_wide(RegisterWide::HL);
                cpu.write_register_wide(RegisterWide::SP, hl);
            }),
        }),
        0xFA => Some(Opcode {
            mnemonic: "LD A, [a16]".to_string(),
            size_bytes: 3,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                let pc = cpu.read_register_wide(RegisterWide::PC);
                let lsb = memory.read(Address(pc - 2));
                let msb = memory.read(Address(pc - 1));
                let address = util::u8_to_u16(msb, lsb);
                let value = memory.read(Address(address));
                cpu.write_register(Register::A, value);
            })
        }),
        0xFB => Some(Opcode {
            mnemonic: "EI ".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| {
                cpu.request_ime_enable();
            })
        }),
        0xFE => Some(Opcode {
            mnemonic: "CP A, n8".to_string(),
            size_bytes: 2,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| {
                let a = cpu.read_register(Register::A);
                let pc = cpu.read_register_wide(RegisterWide::PC);
                let imm = memory.read(Address(pc - 1));
                let (result, overflowed) = a.overflowing_sub(imm);
                cpu.write_flags(WriteFlags {
                    zero: Some(result == 0),
                    subtract: Some(true),
                    half_carry: Some(half_carried_sub8(a, imm)),
                    carry: Some(overflowed),
                });
            }),
        }),
        0xFF => Some(Opcode {
            mnemonic: "RST $38".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| rst(cpu, memory, 0x38)),
        }),
        _ => None,
    }
}

pub fn decode_prefixed(byte: u8) -> Option<Opcode> {
    match byte {
        0x00 => Some(Opcode {
            mnemonic: "RLC B".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0x01 => Some(Opcode {
            mnemonic: "RLC C".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0x02 => Some(Opcode {
            mnemonic: "RLC D".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0x03 => Some(Opcode {
            mnemonic: "RLC E".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0x04 => Some(Opcode {
            mnemonic: "RLC H".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0x05 => Some(Opcode {
            mnemonic: "RLC L".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0x06 => Some(Opcode {
            mnemonic: "RLC [HL]".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0x07 => Some(Opcode {
            mnemonic: "RLC A".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0x08 => Some(Opcode {
            mnemonic: "RRC B".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0x09 => Some(Opcode {
            mnemonic: "RRC C".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0x0A => Some(Opcode {
            mnemonic: "RRC D".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0x0B => Some(Opcode {
            mnemonic: "RRC E".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0x0C => Some(Opcode {
            mnemonic: "RRC H".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0x0D => Some(Opcode {
            mnemonic: "RRC L".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0x0E => Some(Opcode {
            mnemonic: "RRC [HL]".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0x0F => Some(Opcode {
            mnemonic: "RRC A".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0x10 => Some(Opcode {
            mnemonic: "RL B".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| rl_r8(cpu, Register::B)),
        }),
        0x11 => Some(Opcode {
            mnemonic: "RL C".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| rl_r8(cpu, Register::C)),
        }),
        0x12 => Some(Opcode {
            mnemonic: "RL D".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| rl_r8(cpu, Register::D)),
        }),
        0x13 => Some(Opcode {
            mnemonic: "RL E".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| rl_r8(cpu, Register::E)),
        }),
        0x14 => Some(Opcode {
            mnemonic: "RL H".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| rl_r8(cpu, Register::H)),
        }),
        0x15 => Some(Opcode {
            mnemonic: "RL L".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| rl_r8(cpu, Register::L)),
        }),
        0x16 => Some(Opcode {
            mnemonic: "RL [HL]".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0x17 => Some(Opcode {
            mnemonic: "RL A".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| rl_r8(cpu, Register::A)),
        }),
        0x18 => Some(Opcode {
            mnemonic: "RR B".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| rr_r8(cpu, Register::B)),
        }),
        0x19 => Some(Opcode {
            mnemonic: "RR C".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| rr_r8(cpu, Register::C)),
        }),
        0x1A => Some(Opcode {
            mnemonic: "RR D".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| rr_r8(cpu, Register::D)),
        }),
        0x1B => Some(Opcode {
            mnemonic: "RR E".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| rr_r8(cpu, Register::E)),
        }),
        0x1C => Some(Opcode {
            mnemonic: "RR H".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| rr_r8(cpu, Register::H)),
        }),
        0x1D => Some(Opcode {
            mnemonic: "RR L".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| rr_r8(cpu, Register::L)),
        }),
        0x1E => Some(Opcode {
            mnemonic: "RR [HL]".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0x1F => Some(Opcode {
            mnemonic: "RR A".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| rr_r8(cpu, Register::A)),
        }),
        0x20 => Some(Opcode {
            mnemonic: "SLA B".to_string(),
            size_bytes: 1,
            handler: Some(|cpu, _| sla_r8(cpu, Register::B)),
        }),
        0x21 => Some(Opcode {
            mnemonic: "SLA C".to_string(),
            size_bytes: 1,
            handler: Some(|cpu, _| sla_r8(cpu, Register::C)),
        }),
        0x22 => Some(Opcode {
            mnemonic: "SLA D".to_string(),
            size_bytes: 1,
            handler: Some(|cpu, _| sla_r8(cpu, Register::D)),
        }),
        0x23 => Some(Opcode {
            mnemonic: "SLA E".to_string(),
            size_bytes: 1,
            handler: Some(|cpu, _| sla_r8(cpu, Register::E)),
        }),
        0x24 => Some(Opcode {
            mnemonic: "SLA H".to_string(),
            size_bytes: 1,
            handler: Some(|cpu, _| sla_r8(cpu, Register::H)),
        }),
        0x25 => Some(Opcode {
            mnemonic: "SLA L".to_string(),
            size_bytes: 1,
            handler: Some(|cpu, _| sla_r8(cpu, Register::L)),
        }),
        0x26 => Some(Opcode {
            mnemonic: "SLA [HL]".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0x27 => Some(Opcode {
            mnemonic: "SLA A".to_string(),
            size_bytes: 1,
            handler: Some(|cpu, _| sla_r8(cpu, Register::A)),
        }),
        0x28 => Some(Opcode {
            mnemonic: "SRA B".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0x29 => Some(Opcode {
            mnemonic: "SRA C".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0x2A => Some(Opcode {
            mnemonic: "SRA D".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0x2B => Some(Opcode {
            mnemonic: "SRA E".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0x2C => Some(Opcode {
            mnemonic: "SRA H".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0x2D => Some(Opcode {
            mnemonic: "SRA L".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0x2E => Some(Opcode {
            mnemonic: "SRA [HL]".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0x2F => Some(Opcode {
            mnemonic: "SRA A".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0x30 => Some(Opcode {
            mnemonic: "SWAP B".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| swap_r8(cpu, Register::B)),
        }),
        0x31 => Some(Opcode {
            mnemonic: "SWAP C".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| swap_r8(cpu, Register::C)),
        }),
        0x32 => Some(Opcode {
            mnemonic: "SWAP D".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| swap_r8(cpu, Register::D)),
        }),
        0x33 => Some(Opcode {
            mnemonic: "SWAP E".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| swap_r8(cpu, Register::E)),
        }),
        0x34 => Some(Opcode {
            mnemonic: "SWAP H".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| swap_r8(cpu, Register::H)),
        }),
        0x35 => Some(Opcode {
            mnemonic: "SWAP L".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| swap_r8(cpu, Register::L)),
        }),
        0x36 => Some(Opcode {
            mnemonic: "SWAP [HL]".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0x37 => Some(Opcode {
            mnemonic: "SWAP A".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| swap_r8(cpu, Register::A)),
        }),
        0x38 => Some(Opcode {
            mnemonic: "SRL B".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| srl_r8(cpu, Register::B)),
        }),
        0x39 => Some(Opcode {
            mnemonic: "SRL C".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| srl_r8(cpu, Register::C)),
        }),
        0x3A => Some(Opcode {
            mnemonic: "SRL D".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| srl_r8(cpu, Register::D)),
        }),
        0x3B => Some(Opcode {
            mnemonic: "SRL E".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| srl_r8(cpu, Register::E)),
        }),
        0x3C => Some(Opcode {
            mnemonic: "SRL H".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| srl_r8(cpu, Register::H)),
        }),
        0x3D => Some(Opcode {
            mnemonic: "SRL L".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| srl_r8(cpu, Register::L)),
        }),
        0x3E => Some(Opcode {
            mnemonic: "SRL [HL]".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0x3F => Some(Opcode {
            mnemonic: "SRL A".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| srl_r8(cpu, Register::A)),
        }),
        0x40 => Some(Opcode {
            mnemonic: "BIT 0, B".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::Zero, Register::B)),
        }),
        0x41 => Some(Opcode {
            mnemonic: "BIT 0, C".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::Zero, Register::C)),
        }),
        0x42 => Some(Opcode {
            mnemonic: "BIT 0, D".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::Zero, Register::D)),
        }),
        0x43 => Some(Opcode {
            mnemonic: "BIT 0, E".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::Zero, Register::E)),
        }),
        0x44 => Some(Opcode {
            mnemonic: "BIT 0, H".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::Zero, Register::H)),
        }),
        0x45 => Some(Opcode {
            mnemonic: "BIT 0, L".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::Zero, Register::L)),
        }),
        0x46 => Some(Opcode {
            mnemonic: "BIT 0, [HL]".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| bit_indirect_hl(cpu, memory, Bit::Zero)),
        }),
        0x47 => Some(Opcode {
            mnemonic: "BIT 0, A".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::Zero, Register::A)),
        }),
        0x48 => Some(Opcode {
            mnemonic: "BIT 1, B".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::One, Register::B)),
        }),
        0x49 => Some(Opcode {
            mnemonic: "BIT 1, C".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::One, Register::C)),
        }),
        0x4A => Some(Opcode {
            mnemonic: "BIT 1, D".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::One, Register::D)),
        }),
        0x4B => Some(Opcode {
            mnemonic: "BIT 1, E".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::One, Register::E)),
        }),
        0x4C => Some(Opcode {
            mnemonic: "BIT 1, H".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::One, Register::H)),
        }),
        0x4D => Some(Opcode {
            mnemonic: "BIT 1, L".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::One, Register::L)),
        }),
        0x4E => Some(Opcode {
            mnemonic: "BIT 1, [HL]".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| bit_indirect_hl(cpu, memory, Bit::One)),
        }),
        0x4F => Some(Opcode {
            mnemonic: "BIT 1, A".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::One, Register::A)),
        }),
        0x50 => Some(Opcode {
            mnemonic: "BIT 2, B".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::Two, Register::B)),
        }),
        0x51 => Some(Opcode {
            mnemonic: "BIT 2, C".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::Two, Register::C)),
        }),
        0x52 => Some(Opcode {
            mnemonic: "BIT 2, D".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::Two, Register::D)),
        }),
        0x53 => Some(Opcode {
            mnemonic: "BIT 2, E".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::Two, Register::E)),
        }),
        0x54 => Some(Opcode {
            mnemonic: "BIT 2, H".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::Two, Register::H)),
        }),
        0x55 => Some(Opcode {
            mnemonic: "BIT 2, L".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::Two, Register::L)),
        }),
        0x56 => Some(Opcode {
            mnemonic: "BIT 2, [HL]".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| bit_indirect_hl(cpu, memory, Bit::Two)),
        }),
        0x57 => Some(Opcode {
            mnemonic: "BIT 2, A".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::Two, Register::A)),
        }),
        0x58 => Some(Opcode {
            mnemonic: "BIT 3, B".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::Three, Register::B)),
        }),
        0x59 => Some(Opcode {
            mnemonic: "BIT 3, C".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::Three, Register::C)),
        }),
        0x5A => Some(Opcode {
            mnemonic: "BIT 3, D".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::Three, Register::D)),
        }),
        0x5B => Some(Opcode {
            mnemonic: "BIT 3, E".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::Three, Register::E)),
        }),
        0x5C => Some(Opcode {
            mnemonic: "BIT 3, H".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::Three, Register::H)),
        }),
        0x5D => Some(Opcode {
            mnemonic: "BIT 3, L".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::Three, Register::L)),
        }),
        0x5E => Some(Opcode {
            mnemonic: "BIT 3, [HL]".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| bit_indirect_hl(cpu, memory, Bit::Three)),
        }),
        0x5F => Some(Opcode {
            mnemonic: "BIT 3, A".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::Three, Register::A)),
        }),
        0x60 => Some(Opcode {
            mnemonic: "BIT 4, B".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::Four, Register::B)),
        }),
        0x61 => Some(Opcode {
            mnemonic: "BIT 4, C".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::Four, Register::C)),
        }),
        0x62 => Some(Opcode {
            mnemonic: "BIT 4, D".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::Four, Register::D)),
        }),
        0x63 => Some(Opcode {
            mnemonic: "BIT 4, E".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::Four, Register::E)),
        }),
        0x64 => Some(Opcode {
            mnemonic: "BIT 4, H".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::Four, Register::H)),
        }),
        0x65 => Some(Opcode {
            mnemonic: "BIT 4, L".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::Four, Register::L)),
        }),
        0x66 => Some(Opcode {
            mnemonic: "BIT 4, [HL]".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| bit_indirect_hl(cpu, memory, Bit::Four)),
        }),
        0x67 => Some(Opcode {
            mnemonic: "BIT 4, A".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::Four, Register::A)),
        }),
        0x68 => Some(Opcode {
            mnemonic: "BIT 5, B".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::Five, Register::B)),
        }),
        0x69 => Some(Opcode {
            mnemonic: "BIT 5, C".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::Five, Register::C)),
        }),
        0x6A => Some(Opcode {
            mnemonic: "BIT 5, D".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::Five, Register::D)),
        }),
        0x6B => Some(Opcode {
            mnemonic: "BIT 5, E".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::Five, Register::E)),
        }),
        0x6C => Some(Opcode {
            mnemonic: "BIT 5, H".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::Five, Register::H)),
        }),
        0x6D => Some(Opcode {
            mnemonic: "BIT 5, L".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::Five, Register::L)),
        }),
        0x6E => Some(Opcode {
            mnemonic: "BIT 5, [HL]".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| bit_indirect_hl(cpu, memory, Bit::Five)),
        }),
        0x6F => Some(Opcode {
            mnemonic: "BIT 5, A".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::Five, Register::A)),
        }),
        0x70 => Some(Opcode {
            mnemonic: "BIT 6, B".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::Six, Register::B)),
        }),
        0x71 => Some(Opcode {
            mnemonic: "BIT 6, C".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::Six, Register::C)),
        }),
        0x72 => Some(Opcode {
            mnemonic: "BIT 6, D".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::Six, Register::D)),
        }),
        0x73 => Some(Opcode {
            mnemonic: "BIT 6, E".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::Six, Register::E)),
        }),
        0x74 => Some(Opcode {
            mnemonic: "BIT 6, H".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::Six, Register::H)),
        }),
        0x75 => Some(Opcode {
            mnemonic: "BIT 6, L".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::Six, Register::L)),
        }),
        0x76 => Some(Opcode {
            mnemonic: "BIT 6, [HL]".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| bit_indirect_hl(cpu, memory, Bit::Six)),
        }),
        0x77 => Some(Opcode {
            mnemonic: "BIT 6, A".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::Six, Register::A)),
        }),
        0x78 => Some(Opcode {
            mnemonic: "BIT 7, B".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::Seven, Register::B)),
        }),
        0x79 => Some(Opcode {
            mnemonic: "BIT 7, C".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::Seven, Register::C)),
        }),
        0x7A => Some(Opcode {
            mnemonic: "BIT 7, D".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::Seven, Register::D)),
        }),
        0x7B => Some(Opcode {
            mnemonic: "BIT 7, E".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::Seven, Register::E)),
        }),
        0x7C => Some(Opcode {
            mnemonic: "BIT 7, H".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::Seven, Register::H)),
        }),
        0x7D => Some(Opcode {
            mnemonic: "BIT 7, L".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::Seven, Register::L)),
        }),
        0x7E => Some(Opcode {
            mnemonic: "BIT 7, [HL]".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| bit_indirect_hl(cpu, memory, Bit::Seven)),
        }),
        0x7F => Some(Opcode {
            mnemonic: "BIT 7, A".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| bit_r8(cpu, Bit::Seven, Register::A)),
        }),
        0x80 => Some(Opcode {
            mnemonic: "RES 0, B".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::Zero, Register::B)),
        }),
        0x81 => Some(Opcode {
            mnemonic: "RES 0, C".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::Zero, Register::C)),
        }),
        0x82 => Some(Opcode {
            mnemonic: "RES 0, D".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::Zero, Register::D)),
        }),
        0x83 => Some(Opcode {
            mnemonic: "RES 0, E".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::Zero, Register::E)),
        }),
        0x84 => Some(Opcode {
            mnemonic: "RES 0, H".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::Zero, Register::H)),
        }),
        0x85 => Some(Opcode {
            mnemonic: "RES 0, L".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::Zero, Register::L)),
        }),
        0x86 => Some(Opcode {
            mnemonic: "RES 0, [HL]".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| res_indirect_hl(cpu, memory, Bit::Zero)),
        }),
        0x87 => Some(Opcode {
            mnemonic: "RES 0, A".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::Zero, Register::A)),
        }),
        0x88 => Some(Opcode {
            mnemonic: "RES 1, B".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::One, Register::B)),
        }),
        0x89 => Some(Opcode {
            mnemonic: "RES 1, C".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::One, Register::C)),
        }),
        0x8A => Some(Opcode {
            mnemonic: "RES 1, D".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::One, Register::D)),
        }),
        0x8B => Some(Opcode {
            mnemonic: "RES 1, E".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::One, Register::E)),
        }),
        0x8C => Some(Opcode {
            mnemonic: "RES 1, H".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::One, Register::H)),
        }),
        0x8D => Some(Opcode {
            mnemonic: "RES 1, L".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::One, Register::L)),
        }),
        0x8E => Some(Opcode {
            mnemonic: "RES 1, [HL]".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| res_indirect_hl(cpu, memory, Bit::One)),
        }),
        0x8F => Some(Opcode {
            mnemonic: "RES 1, A".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::One, Register::A)),
        }),
        0x90 => Some(Opcode {
            mnemonic: "RES 2, B".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::Two, Register::B)),
        }),
        0x91 => Some(Opcode {
            mnemonic: "RES 2, C".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::Two, Register::C)),
        }),
        0x92 => Some(Opcode {
            mnemonic: "RES 2, D".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::Two, Register::D)),
        }),
        0x93 => Some(Opcode {
            mnemonic: "RES 2, E".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::Two, Register::E)),
        }),
        0x94 => Some(Opcode {
            mnemonic: "RES 2, H".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::Two, Register::H)),
        }),
        0x95 => Some(Opcode {
            mnemonic: "RES 2, L".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::Two, Register::L)),
        }),
        0x96 => Some(Opcode {
            mnemonic: "RES 2, [HL]".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| res_indirect_hl(cpu, memory, Bit::Two)),
        }),
        0x97 => Some(Opcode {
            mnemonic: "RES 2, A".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::Two, Register::A)),
        }),
        0x98 => Some(Opcode {
            mnemonic: "RES 3, B".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::Three, Register::B)),
        }),
        0x99 => Some(Opcode {
            mnemonic: "RES 3, C".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::Three, Register::C)),
        }),
        0x9A => Some(Opcode {
            mnemonic: "RES 3, D".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::Three, Register::D)),
        }),
        0x9B => Some(Opcode {
            mnemonic: "RES 3, E".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::Three, Register::E)),
        }),
        0x9C => Some(Opcode {
            mnemonic: "RES 3, H".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::Three, Register::H)),
        }),
        0x9D => Some(Opcode {
            mnemonic: "RES 3, L".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::Three, Register::L)),
        }),
        0x9E => Some(Opcode {
            mnemonic: "RES 3, [HL]".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| res_indirect_hl(cpu, memory, Bit::Three)),
        }),
        0x9F => Some(Opcode {
            mnemonic: "RES 3, A".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::Three, Register::A)),
        }),
        0xA0 => Some(Opcode {
            mnemonic: "RES 4, B".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::Four, Register::B)),
        }),
        0xA1 => Some(Opcode {
            mnemonic: "RES 4, C".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::Four, Register::C)),
        }),
        0xA2 => Some(Opcode {
            mnemonic: "RES 4, D".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::Four, Register::D)),
        }),
        0xA3 => Some(Opcode {
            mnemonic: "RES 4, E".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::Four, Register::E)),
        }),
        0xA4 => Some(Opcode {
            mnemonic: "RES 4, H".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::Four, Register::H)),
        }),
        0xA5 => Some(Opcode {
            mnemonic: "RES 4, L".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::Four, Register::L)),
        }),
        0xA6 => Some(Opcode {
            mnemonic: "RES 4, [HL]".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| res_indirect_hl(cpu, memory, Bit::Four)),
        }),
        0xA7 => Some(Opcode {
            mnemonic: "RES 4, A".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::Four, Register::A)),
        }),
        0xA8 => Some(Opcode {
            mnemonic: "RES 5, B".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::Five, Register::B)),
        }),
        0xA9 => Some(Opcode {
            mnemonic: "RES 5, C".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::Five, Register::C)),
        }),
        0xAA => Some(Opcode {
            mnemonic: "RES 5, D".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::Five, Register::D)),
        }),
        0xAB => Some(Opcode {
            mnemonic: "RES 5, E".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::Five, Register::E)),
        }),
        0xAC => Some(Opcode {
            mnemonic: "RES 5, H".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::Five, Register::H)),
        }),
        0xAD => Some(Opcode {
            mnemonic: "RES 5, L".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::Five, Register::L)),
        }),
        0xAE => Some(Opcode {
            mnemonic: "RES 5, [HL]".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| res_indirect_hl(cpu, memory, Bit::Five)),
        }),
        0xAF => Some(Opcode {
            mnemonic: "RES 5, A".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::Five, Register::A)),
        }),
        0xB0 => Some(Opcode {
            mnemonic: "RES 6, B".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::Six, Register::B)),
        }),
        0xB1 => Some(Opcode {
            mnemonic: "RES 6, C".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::Six, Register::C)),
        }),
        0xB2 => Some(Opcode {
            mnemonic: "RES 6, D".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::Six, Register::D)),
        }),
        0xB3 => Some(Opcode {
            mnemonic: "RES 6, E".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::Six, Register::E)),
        }),
        0xB4 => Some(Opcode {
            mnemonic: "RES 6, H".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::Six, Register::H)),
        }),
        0xB5 => Some(Opcode {
            mnemonic: "RES 6, L".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::Six, Register::L)),
        }),
        0xB6 => Some(Opcode {
            mnemonic: "RES 6, [HL]".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| res_indirect_hl(cpu, memory, Bit::Six)),
        }),
        0xB7 => Some(Opcode {
            mnemonic: "RES 6, A".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::Six, Register::A)),
        }),
        0xB8 => Some(Opcode {
            mnemonic: "RES 7, B".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::Seven, Register::B)),
        }),
        0xB9 => Some(Opcode {
            mnemonic: "RES 7, C".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::Seven, Register::C)),
        }),
        0xBA => Some(Opcode {
            mnemonic: "RES 7, D".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::Seven, Register::D)),
        }),
        0xBB => Some(Opcode {
            mnemonic: "RES 7, E".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::Seven, Register::E)),
        }),
        0xBC => Some(Opcode {
            mnemonic: "RES 7, H".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::Seven, Register::H)),
        }),
        0xBD => Some(Opcode {
            mnemonic: "RES 7, L".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::Seven, Register::L)),
        }),
        0xBE => Some(Opcode {
            mnemonic: "RES 7, [HL]".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| res_indirect_hl(cpu, memory, Bit::Seven)),
        }),
        0xBF => Some(Opcode {
            mnemonic: "RES 7, A".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, _| res_r8(cpu, Bit::Seven, Register::A)),
        }),
        0xC0 => Some(Opcode {
            mnemonic: "SET 0, B".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xC1 => Some(Opcode {
            mnemonic: "SET 0, C".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xC2 => Some(Opcode {
            mnemonic: "SET 0, D".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xC3 => Some(Opcode {
            mnemonic: "SET 0, E".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xC4 => Some(Opcode {
            mnemonic: "SET 0, H".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xC5 => Some(Opcode {
            mnemonic: "SET 0, L".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xC6 => Some(Opcode {
            mnemonic: "SET 0, [HL]".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| set_indirect_hl(cpu, memory, Bit::Zero)),
        }),
        0xC7 => Some(Opcode {
            mnemonic: "SET 0, A".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xC8 => Some(Opcode {
            mnemonic: "SET 1, B".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xC9 => Some(Opcode {
            mnemonic: "SET 1, C".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xCA => Some(Opcode {
            mnemonic: "SET 1, D".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xCB => Some(Opcode {
            mnemonic: "SET 1, E".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xCC => Some(Opcode {
            mnemonic: "SET 1, H".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xCD => Some(Opcode {
            mnemonic: "SET 1, L".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xCE => Some(Opcode {
            mnemonic: "SET 1, [HL]".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| set_indirect_hl(cpu, memory, Bit::One)),
        }),
        0xCF => Some(Opcode {
            mnemonic: "SET 1, A".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xD0 => Some(Opcode {
            mnemonic: "SET 2, B".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xD1 => Some(Opcode {
            mnemonic: "SET 2, C".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xD2 => Some(Opcode {
            mnemonic: "SET 2, D".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xD3 => Some(Opcode {
            mnemonic: "SET 2, E".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xD4 => Some(Opcode {
            mnemonic: "SET 2, H".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xD5 => Some(Opcode {
            mnemonic: "SET 2, L".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xD6 => Some(Opcode {
            mnemonic: "SET 2, [HL]".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| set_indirect_hl(cpu, memory, Bit::Two)),
        }),
        0xD7 => Some(Opcode {
            mnemonic: "SET 2, A".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xD8 => Some(Opcode {
            mnemonic: "SET 3, B".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xD9 => Some(Opcode {
            mnemonic: "SET 3, C".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xDA => Some(Opcode {
            mnemonic: "SET 3, D".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xDB => Some(Opcode {
            mnemonic: "SET 3, E".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xDC => Some(Opcode {
            mnemonic: "SET 3, H".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xDD => Some(Opcode {
            mnemonic: "SET 3, L".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xDE => Some(Opcode {
            mnemonic: "SET 3, [HL]".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| set_indirect_hl(cpu, memory, Bit::Three)),
        }),
        0xDF => Some(Opcode {
            mnemonic: "SET 3, A".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xE0 => Some(Opcode {
            mnemonic: "SET 4, B".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xE1 => Some(Opcode {
            mnemonic: "SET 4, C".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xE2 => Some(Opcode {
            mnemonic: "SET 4, D".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xE3 => Some(Opcode {
            mnemonic: "SET 4, E".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xE4 => Some(Opcode {
            mnemonic: "SET 4, H".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xE5 => Some(Opcode {
            mnemonic: "SET 4, L".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xE6 => Some(Opcode {
            mnemonic: "SET 4, [HL]".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| set_indirect_hl(cpu, memory, Bit::Four)),
        }),
        0xE7 => Some(Opcode {
            mnemonic: "SET 4, A".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xE8 => Some(Opcode {
            mnemonic: "SET 5, B".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xE9 => Some(Opcode {
            mnemonic: "SET 5, C".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xEA => Some(Opcode {
            mnemonic: "SET 5, D".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xEB => Some(Opcode {
            mnemonic: "SET 5, E".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xEC => Some(Opcode {
            mnemonic: "SET 5, H".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xED => Some(Opcode {
            mnemonic: "SET 5, L".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xEE => Some(Opcode {
            mnemonic: "SET 5, [HL]".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| set_indirect_hl(cpu, memory, Bit::Five)),
        }),
        0xEF => Some(Opcode {
            mnemonic: "SET 5, A".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xF0 => Some(Opcode {
            mnemonic: "SET 6, B".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xF1 => Some(Opcode {
            mnemonic: "SET 6, C".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xF2 => Some(Opcode {
            mnemonic: "SET 6, D".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xF3 => Some(Opcode {
            mnemonic: "SET 6, E".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xF4 => Some(Opcode {
            mnemonic: "SET 6, H".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xF5 => Some(Opcode {
            mnemonic: "SET 6, L".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xF6 => Some(Opcode {
            mnemonic: "SET 6, [HL]".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| set_indirect_hl(cpu, memory, Bit::Six)),
        }),
        0xF7 => Some(Opcode {
            mnemonic: "SET 6, A".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xF8 => Some(Opcode {
            mnemonic: "SET 7, B".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xF9 => Some(Opcode {
            mnemonic: "SET 7, C".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xFA => Some(Opcode {
            mnemonic: "SET 7, D".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xFB => Some(Opcode {
            mnemonic: "SET 7, E".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xFC => Some(Opcode {
            mnemonic: "SET 7, H".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xFD => Some(Opcode {
            mnemonic: "SET 7, L".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        0xFE => Some(Opcode {
            mnemonic: "SET 7, [HL]".to_string(),
            size_bytes: 1,
            handler: Some(|cpu: &mut Cpu, memory: &mut Memory| set_indirect_hl(cpu, memory, Bit::Seven)),
        }),
        0xFF => Some(Opcode {
            mnemonic: "SET 7, A".to_string(),
            size_bytes: 1,
            handler: None,
        }),
        _ => None,
    }
}

fn add_hl_r16(cpu: &mut Cpu, r: RegisterWide) {
    let hl = cpu.read_register_wide(RegisterWide::HL);
    let value = cpu.read_register_wide(r);
    let (result, overflowed) = hl.overflowing_add(value);
    cpu.write_register_wide(RegisterWide::HL, result);
    cpu.write_flags(WriteFlags {
        subtract: Some(false),
        half_carry: Some(half_carried_add16(hl, value)),
        carry: Some(overflowed),
        ..Default::default()
    });
}

fn add_r8(cpu: &mut Cpu, r: Register) {
    let a = cpu.read_register(Register::A);
    let b = cpu.read_register(r);
    let (result, overflowed) = a.overflowing_add(b);
    cpu.write_register(Register::A, result);
    cpu.write_flags(WriteFlags {
        zero: Some(result == 0),
        subtract: Some(false),
        half_carry: Some(half_carried_add8(a, b)),
        carry: Some(overflowed),
    });
}

fn adc_r8(cpu: &mut Cpu, r: Register) {
    let a = cpu.read_register(Register::A);
    let carry_bit = cpu.read_flags().carry as u8;
    let b = cpu.read_register(r).wrapping_add(carry_bit);
    let (mut result, mut overflowed) = a.overflowing_add(b);

    if cpu.read_flags().carry {
        let (carry_result, carry_overflowed) = result.overflowing_add(1);
        result = carry_result;
        overflowed |= carry_overflowed;
    }

    cpu.write_register(Register::A, result);
    cpu.write_flags(WriteFlags {
        zero: Some(result == 0),
        subtract: Some(false),
        half_carry: Some(half_carried_add8(a, b)),
        carry: Some(overflowed),
    });
}

fn and_r8(cpu: &mut Cpu, r: Register) {
    let a = cpu.read_register(Register::A);
    let b = cpu.read_register(r);
    let result = a & b;
    cpu.write_register(Register::A, result);
    cpu.write_flags(WriteFlags {
        zero: Some(result == 0),
        subtract: Some(false),
        half_carry: Some(true),
        carry: Some(false),
    });
}

enum Bit {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
}

fn bit_indirect_hl(cpu: &mut Cpu, memory: &mut Memory, b: Bit) {
    let hl = cpu.read_register_wide(RegisterWide::HL);
    let value = memory.read(Address(hl));
    let bit = value & (1 << b as u8);
    cpu.write_flags(WriteFlags {
        zero: Some(bit == 0),
        subtract: Some(false),
        half_carry: Some(true),
        ..Default::default()
    })
}

fn bit_r8(cpu: &mut Cpu, b: Bit, r: Register) {
    let bit = cpu.read_register(r) & (1 << b as u8);
    cpu.write_flags(WriteFlags {
        zero: Some(bit == 0),
        subtract: Some(false),
        half_carry: Some(true),
        ..Default::default()
    })
}

fn call_a16(cpu: &mut Cpu, memory: &mut Memory) {
    let pc = cpu.read_register_wide(RegisterWide::PC);
    let msb = memory.read(Address(pc - 1));
    let lsb = memory.read(Address(pc - 2));
    let address = u8_to_u16(msb, lsb);
    let sp = cpu.read_register_wide(RegisterWide::SP);
    let new_sp = sp - 2;
    let (msb, lsb) = u16_to_u8(pc);
    memory.write(Address(new_sp), lsb);
    memory.write(Address(new_sp + 1), msb);
    cpu.write_register_wide(RegisterWide::SP, new_sp);
    cpu.write_register_wide(RegisterWide::PC, address);
}

fn cp_r8(cpu: &mut Cpu, r: Register) {
    let a = cpu.read_register(Register::A);
    let b = cpu.read_register(r);
    let (result, overflowed) = a.overflowing_sub(b);
    cpu.write_flags(WriteFlags {
        zero: Some(result == 0),
        subtract: Some(true),
        half_carry: Some(half_carried_sub8(a, b)),
        carry: Some(overflowed),
    });
}

fn dec_r8(cpu: &mut Cpu, r: Register) {
    let a = cpu.read_register(r);
    let (result, _) = a.overflowing_sub(1);
    cpu.write_flags(WriteFlags {
        zero: Some(result == 0),
        subtract: Some(true),
        half_carry: Some(half_carried_sub8(a, 1)),
        carry: None,
    });
    cpu.write_register(r, result);
}

fn dec_r16(cpu: &mut Cpu, r: RegisterWide) {
    let value = cpu.read_register_wide(r);
    cpu.write_register_wide(r, value - 1);
}

fn inc_r8(cpu: &mut Cpu, r: Register) {
    let a = cpu.read_register(r);
    let result = a.wrapping_add(1);
    cpu.write_register(r, result);
    cpu.write_flags(WriteFlags {
        zero: Some(result == 0),
        subtract: Some(false),
        half_carry: Some(half_carried_add8(a, 1)),
        carry: None,
    });
}

fn inc_r16(cpu: &mut Cpu, r: RegisterWide) {
    let value = cpu.read_register_wide(r);
    cpu.write_register_wide(r, value + 1);
}

fn jump(cpu: &mut Cpu, memory: &mut Memory) {
    let pc = cpu.read_register_wide(RegisterWide::PC);
    let msb = memory.read(Address(pc - 1));
    let lsb = memory.read(Address(pc - 2));
    let new_pc = util::u8_to_u16(msb, lsb);
    cpu.write_register_wide(RegisterWide::PC, new_pc);
}

fn jump_relative(cpu: &mut Cpu, memory: &mut Memory) {
    let mut pc = cpu.read_register_wide(RegisterWide::PC);
    let imm = memory.read(Address(pc - 1)) as i8;
    pc = pc.wrapping_add_signed(imm.into());
    cpu.write_register_wide(RegisterWide::PC, pc);
}

fn ld_r8_r8(cpu: &mut Cpu, dst_register: Register, src_register: Register) {
    cpu.write_register(dst_register, cpu.read_register(src_register));
}

enum IndirectAddressingMode {
    // Leaves the register untouched after the operation
    Retain,
    // Decrements the register after the operation
    Decrement,
    // Increments the register after the operation
    Increment,
}

fn ld_r8_indirect_r16(
    cpu: &mut Cpu,
    memory: &mut Memory,
    dst_register: Register,
    src_register: RegisterWide,
    mode: IndirectAddressingMode,
) {
    let src_address = cpu.read_register_wide(src_register);
    let value = memory.read(Address(src_address));
    cpu.write_register(dst_register, value);
    
    let register_value = cpu.read_register_wide(src_register);
    match mode {
        IndirectAddressingMode::Retain => {},
        IndirectAddressingMode::Increment => cpu.write_register_wide(src_register, register_value + 1),
        IndirectAddressingMode::Decrement => cpu.write_register_wide(src_register, register_value - 1),
    };
}

fn ld_indirect_r16_r8(
    cpu: &mut Cpu,
    memory: &mut Memory,
    dst_register: RegisterWide,
    src_register: Register,
    mode: IndirectAddressingMode,
) {
    let dst_value = cpu.read_register_wide(dst_register);
    let src_value = cpu.read_register(src_register);
    memory.write(Address(dst_value), src_value);

    let new_dst_register = match mode {
        IndirectAddressingMode::Retain => dst_value,
        IndirectAddressingMode::Increment => dst_value + 1,
        IndirectAddressingMode::Decrement => dst_value - 1,
    };
    cpu.write_register_wide(dst_register, new_dst_register);
}

fn ld_r8_n8(cpu: &mut Cpu, memory: &mut Memory, r: Register) {
    let pc = cpu.read_register_wide(RegisterWide::PC);
    let imm = memory.read(Address(pc - 1));
    cpu.write_register(r, imm);
}

fn ld_r16_n16(cpu: &mut Cpu, memory: &mut Memory, r: RegisterWide) {
    let pc = cpu.read_register_wide(RegisterWide::PC);
    let msb = memory.read(Address(pc - 1));
    let lsb = memory.read(Address(pc - 2));
    let value = u8_to_u16(msb, lsb);
    cpu.write_register_wide(r, value);
}

fn or_r8(cpu: &mut Cpu, r: Register) {
    let a = cpu.read_register(Register::A);
    let b = cpu.read_register(r);
    let result = a | b;
    cpu.write_register(Register::A, result);
    cpu.write_flags(WriteFlags {
        zero: Some(result == 0),
        subtract: Some(false),
        half_carry: Some(false),
        carry: Some(false),
    });
}

fn pop(cpu: &mut Cpu, memory: &mut Memory, r: RegisterWide) {
    let sp = cpu.read_register_wide(RegisterWide::SP);
    let lsb = memory.read(Address(sp));
    let msb = memory.read(Address(sp + 1));
    cpu.write_register_wide(RegisterWide::SP, sp + 2);
    let value = u8_to_u16(msb, lsb);
    cpu.write_register_wide(r, value);
}

fn push(cpu: &mut Cpu, memory: &mut Memory, r: RegisterWide) {
    let value = cpu.read_register_wide(r);
    let (msb, lsb) = util::u16_to_u8(value);
    let sp = cpu.read_register_wide(RegisterWide::SP);
    memory.write(Address(sp - 1), msb);
    memory.write(Address(sp - 2), lsb);

    cpu.write_register_wide(RegisterWide::SP, sp - 2);
}

fn res_indirect_hl(cpu: &mut Cpu, memory: &mut Memory, b: Bit) {
    let hl = cpu.read_register_wide(RegisterWide::HL);
    let value = memory.read(Address(hl));
    let new_value = util::set_bits(value, 1, 1 << b as u8);
    memory.write(Address(hl), new_value)
}

fn res_r8(cpu: &mut Cpu, b: Bit, r: Register) {
    let value = cpu.read_register(r);
    let new_value = util::set_bits(value, 1, 1 << b as u8);
    cpu.write_register(r, new_value)
}

fn rl_r8(cpu: &mut Cpu, r: Register) {
    let register_value = cpu.read_register(r);
    let mut result = register_value << 1;
    if cpu.read_flags().carry {
        result |= 0b0000_0001;
    }

    cpu.write_flags(WriteFlags {
        zero: Some(result == 0),
        subtract: Some(false),
        half_carry: Some(false),
        carry: Some(register_value & 0b1000_0000 != 0),
    });
    cpu.write_register(r, result);
}

fn rr_r8(cpu: &mut Cpu, r: Register) {
    let value = cpu.read_register(r);
    let mut result = value >> 1;
    if cpu.read_flags().carry {
        result |= 0b1000_0000;
    }

    cpu.write_flags(WriteFlags {
        zero: Some(result == 0),
        subtract: Some(false),
        half_carry: Some(false),
        carry: Some(value & 0b0000_0001 != 0),
    });
    cpu.write_register(r, result);
}

fn rst(cpu: &mut Cpu, memory: &mut Memory, address: u16) {
    let pc = cpu.read_register_wide(RegisterWide::PC);
    let (msb, lsb) = util::u16_to_u8(pc);
    let sp = cpu.read_register_wide(RegisterWide::SP);
    memory.write(Address(sp - 1), msb);
    memory.write(Address(sp - 2), lsb);

    cpu.write_register_wide(RegisterWide::SP, sp - 2);
    cpu.write_register_wide(RegisterWide::PC, address);
}

fn sub_r8(cpu: &mut Cpu, r: Register) {
    let a = cpu.read_register(Register::A);
    let b = cpu.read_register(r);
    let (result, overflowed) = a.overflowing_sub(b);
    cpu.write_register(Register::A, result);
    cpu.write_flags(WriteFlags {
        zero: Some(result == 0),
        subtract: Some(true),
        half_carry: Some(half_carried_sub8(a, b)),
        carry: Some(overflowed),
    });
}

fn sbc_r8(cpu: &mut Cpu, r: Register) {
    let a = cpu.read_register(Register::A);
    let carry_bit = cpu.read_flags().carry as u8;
    let b = cpu.read_register(r).wrapping_add(carry_bit);
    let (result, overflowed) = a.overflowing_sub(b);
    cpu.write_register(Register::A, result);
    cpu.write_flags(WriteFlags {
        zero: Some(result == 0),
        subtract: Some(true),
        half_carry: Some(half_carried_sub8(a, b)),
        carry: Some(overflowed),
    });
}

fn set_indirect_hl(cpu: &mut Cpu, memory: &mut Memory, bit: Bit) {
    let hl = cpu.read_register_wide(RegisterWide::HL); 
    let value = memory.read(Address(hl));
    let mask = 1 << bit as u8;
    memory.write(Address(hl), util::set_bits(value, mask, mask));
}

fn sla_r8(cpu: &mut Cpu, r: Register) {
    let value = cpu.read_register(r);
    let result = value << 1;
    cpu.write_register(r, result);
    cpu.write_flags(WriteFlags {
        zero: Some(result == 0),
        subtract: Some(false),
        half_carry: Some(false),
        carry: Some(util::bit(value, 7) != 0),
    });

    cpu.write_register(r, result)
}

fn srl_r8(cpu: &mut Cpu, r: Register) {
    let value = cpu.read_register(r);
    let result = value >> 1;
    cpu.write_register(r, result);
    cpu.write_flags(WriteFlags {
        zero: Some(result == 0),
        subtract: Some(false),
        half_carry: Some(false),
        carry: Some(util::bit(value, 0) != 0),
    });
}

fn swap_r8(cpu: &mut Cpu, r: Register) {
    let value = cpu.read_register(r);
    let result = value.rotate_right(4);
    cpu.write_register(r, result);
    cpu.write_flags(WriteFlags {
        zero: Some(result == 0),
        subtract: Some(false),
        half_carry: Some(false),
        carry: Some(false),
    });
}

fn xor_r8(cpu: &mut Cpu, register: Register) {
    let a = cpu.read_register(Register::A);
    let r = cpu.read_register(register);
    let result = a ^ r;
    cpu.write_register(Register::A, result);
    cpu.write_flags(WriteFlags {
        zero: Some(result == 0),
        subtract: Some(false),
        half_carry: Some(false),
        carry: Some(false),
    });
}
