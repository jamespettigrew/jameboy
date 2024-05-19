use crate::util;
use crate::memory::{ Address, Memory };
use crate::opcode;
use crate::util::{ bit, u16_to_u8, set_bits };

const ADDRESS_INTERRUPT_FLAG_REGISTER: u16 = 0xFF0F;
const ADDRESS_INTERRUPT_ENABLE_REGISTER: u16 = 0xFFFF;
const ADDRESS_VBLANK_INTERRUPT: u16 = 0x40;
const ADDRESS_LCD_INTERRUPT: u16 = 0x48;
const ADDRESS_TIMER_INTERRUPT: u16 = 0x50;
const ADDRESS_SERIAL_INTERRUPT: u16 = 0x58;
const ADDRESS_JOYPAD_INTERRUPT: u16 = 0x60;

#[derive(Clone, Copy)]
pub enum Register {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Clone, Copy)]
pub enum RegisterWide {
    AF,
    BC,
    DE,
    HL,
    PC,
    SP,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Flags {
    pub zero: bool,
    pub subtract: bool,
    pub half_carry: bool,
    pub carry: bool,
}

#[derive(Clone, Copy, Default)]
pub struct WriteFlags {
    pub zero: Option<bool>,
    pub subtract: Option<bool>,
    pub half_carry: Option<bool>,
    pub carry: Option<bool>,
}

#[derive(Debug)]
pub struct Cpu {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
    pub pc: u16, // Program counter
    pub sp: u16, // Stack pointer
    pub ime: bool,
    steps_since_request_ime_enable: Option<u8>, // IME enable should be delayed by one instruction after EI
    pub prefixed: bool,
}

impl Cpu {
    pub fn init() -> Self {
        Self {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0,
            pc: 0,
            sp: 0,
            ime: true,
            steps_since_request_ime_enable: None,
            prefixed: false,
        }
    }

    pub fn read_register(&self, r: Register) -> u8 {
        match r {
            Register::A => self.a,
            Register::B => self.b,
            Register::C => self.c,
            Register::D => self.d,
            Register::E => self.e,
            Register::H => self.h,
            Register::L => self.l,
        }
    }

    pub fn read_register_wide(&self, r: RegisterWide) -> u16 {
        match r {
            RegisterWide::AF => util::u8_to_u16(self.a, self.f),
            RegisterWide::BC => util::u8_to_u16(self.b, self.c),
            RegisterWide::DE => util::u8_to_u16(self.d, self.e),
            RegisterWide::HL => util::u8_to_u16(self.h, self.l),
            RegisterWide::PC => self.pc,
            RegisterWide::SP => self.sp,
        }
    }

    pub fn write_register(&mut self, r: Register, value: u8) {
        match r {
            Register::A => self.a = value,
            Register::B => self.b = value,
            Register::C => self.c = value,
            Register::D => self.d = value,
            Register::E => self.e = value,
            Register::H => self.h = value,
            Register::L => self.l = value,
        };
    }

    pub fn write_register_wide(&mut self, r: RegisterWide, value: u16) {
        match r {
            RegisterWide::AF => {
                let (msb, lsb) = util::u16_to_u8(value);
                self.a = msb;
                self.f = lsb & 0b1111_0000; // Can only write to upper nibble of F register
            }
            RegisterWide::BC => (self.b, self.c) = util::u16_to_u8(value),
            RegisterWide::DE => (self.d, self.e) = util::u16_to_u8(value),
            RegisterWide::HL => (self.h, self.l) = util::u16_to_u8(value),
            RegisterWide::PC => self.pc = value,
            RegisterWide::SP => self.sp = value,
        };
    }

    pub fn read_flags(&self) -> Flags {
        Flags {
            zero: self.f & (1 << 7) != 0,
            subtract: self.f & (1 << 6) != 0,
            half_carry: self.f & (1 << 5) != 0,
            carry: self.f & (1 << 4) != 0,
        }
    }

    pub fn request_ime_enable(&mut self) {
        self.steps_since_request_ime_enable = Some(0);
    }

    pub fn request_ime_disable(&mut self) {
        self.ime = false;
        self.steps_since_request_ime_enable = None;
    }

    pub fn step(&mut self, memory: &mut Memory) {
        if self.handled_interrupts(memory) {
            self.check_interrupts_enabled();
            return;
        }

        let pc = self.read_register_wide(RegisterWide::PC);
        let byte = memory.read(Address(pc));
        let opcode = if self.prefixed {
            self.prefixed = false;
            opcode::decode_prefixed(byte)
        } else {
            opcode::decode(byte)
        };

        if opcode.is_none() {
            return;
        }

        let opcode = opcode.unwrap();
        if opcode.mnemonic == "PREFIX" {
            self.prefixed = true;
        }
        self.pc += opcode.size_bytes as u16;
        opcode.execute(self, memory);
        self.check_interrupts_enabled();
    }

    fn check_interrupts_enabled(&mut self) {
        match self.steps_since_request_ime_enable {
            Some(1) => {
                self.ime = true;
                self.steps_since_request_ime_enable = None
            },
            Some(0) => {
                self.steps_since_request_ime_enable = Some(1);
            },
            _ => {}
        }
    }

    fn handled_interrupts(&mut self, memory: &mut Memory) -> bool {
        if !self.ime {
            return false;
        }

        let ie_register = memory.read(Address(ADDRESS_INTERRUPT_ENABLE_REGISTER));
        let if_register = memory.read(Address(ADDRESS_INTERRUPT_FLAG_REGISTER));

        let (bit_to_reset, interrupt_handler_address) = if bit(ie_register, 0) & bit(if_register, 0) == 1 {
            (0, ADDRESS_VBLANK_INTERRUPT)
        } else if bit(ie_register, 1) & bit(if_register, 1) == 1 {
            (1, ADDRESS_LCD_INTERRUPT)
        } else if bit(ie_register, 2) & bit(if_register, 2) == 1 {
            (2, ADDRESS_TIMER_INTERRUPT)
        } else if bit(ie_register, 3) & bit(if_register, 3) == 1 {
            (3, ADDRESS_SERIAL_INTERRUPT)
        } else if bit(ie_register, 4) & bit(if_register, 4) == 1 {
            (4, ADDRESS_JOYPAD_INTERRUPT)
        } else {
            return false;
        };

        // When an interrupt is executed, the corresponding bit in the IF register becomes automatically reset 
        // by the CPU, and the IME flag becomes cleared.
        self.ime = false;
        memory.write(Address(ADDRESS_INTERRUPT_FLAG_REGISTER), set_bits(if_register, 0, 1 << bit_to_reset));

        let pc = self.read_register_wide(RegisterWide::PC);
        let sp = self.read_register_wide(RegisterWide::SP);
        let new_sp = sp - 2;
        let (msb, lsb) = u16_to_u8(pc);
        memory.write(Address(new_sp), lsb);
        memory.write(Address(new_sp + 1), msb);
        self.write_register_wide(RegisterWide::SP, new_sp);
        self.write_register_wide(RegisterWide::PC, interrupt_handler_address);

        return true;
    }

    pub fn write_flags(&mut self, f: WriteFlags) {
        match f.zero {
            Some(true) => self.f |= 1 << 7,
            Some(false) => self.f &= !(1 << 7),
            None => (),
        };

        match f.subtract {
            Some(true) => self.f |= 1 << 6,
            Some(false) => self.f &= !(1 << 6),
            None => (),
        };

        match f.half_carry {
            Some(true) => self.f |= 1 << 5,
            Some(false) => self.f &= !(1 << 5),
            None => (),
        };

        match f.carry {
            Some(true) => self.f |= 1 << 4,
            Some(false) => self.f &= !(1 << 4),
            None => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_flags() {
        let mut cpu = Cpu::init();
        assert_eq!(
            Flags {
                zero: false,
                subtract: false,
                half_carry: false,
                carry: false,
            },
            cpu.read_flags()
        );

        cpu.f = 0b11110000;
        assert_eq!(
            Flags {
                zero: true,
                subtract: true,
                half_carry: true,
                carry: true,
            },
            cpu.read_flags()
        );

        cpu.f = 0b10000000;
        assert_eq!(
            Flags {
                zero: true,
                subtract: false,
                half_carry: false,
                carry: false,
            },
            cpu.read_flags()
        );

        cpu.f = 0b01000000;
        assert_eq!(
            Flags {
                zero: false,
                subtract: true,
                half_carry: false,
                carry: false,
            },
            cpu.read_flags()
        );
        cpu.f = 0b00100000;
        assert_eq!(
            Flags {
                zero: false,
                subtract: false,
                half_carry: true,
                carry: false,
            },
            cpu.read_flags()
        );

        cpu.f = 0b00010000;
        assert_eq!(
            Flags {
                zero: false,
                subtract: false,
                half_carry: false,
                carry: true,
            },
            cpu.read_flags()
        );
    }

    #[test]
    fn test_write_flags() {
        let mut cpu = Cpu::init();
        assert_eq!(0b00000000, cpu.f);

        cpu.write_flags(WriteFlags {
            zero: Some(true),
            ..Default::default()
        });
        assert_eq!(0b10000000, cpu.f);

        cpu.write_flags(WriteFlags {
            subtract: Some(true),
            ..Default::default()
        });
        assert_eq!(0b11000000, cpu.f);

        cpu.write_flags(WriteFlags {
            half_carry: Some(true),
            ..Default::default()
        });
        assert_eq!(0b11100000, cpu.f);

        cpu.write_flags(WriteFlags {
            carry: Some(true),
            ..Default::default()
        });
        assert_eq!(0b11110000, cpu.f);

        cpu.write_flags(WriteFlags {
            ..Default::default()
        });
        assert_eq!(0b11110000, cpu.f);

        cpu.write_flags(WriteFlags {
            zero: Some(false),
            subtract: Some(false),
            half_carry: Some(false),
            carry: Some(false),
        });
        assert_eq!(0b00000000, cpu.f);
    }
}
