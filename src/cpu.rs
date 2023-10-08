use crate::util;

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
            RegisterWide::AF => (self.a, self.f) = util::u16_to_u8(value),
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
