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

#[derive(Clone, Copy)]
pub struct Flags {
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool,
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
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
    pc: u16, // Program counter
    sp: u16, // Stack pointer
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
            zero: self.f & (1 << 7) == 1,
            subtract: self.f & (1 << 6) == 1,
            half_carry: self.f & (1 << 5) == 1,
            carry: self.f & (1 << 4) == 1,
        }
    }

    pub fn write_flags(&mut self, f: WriteFlags) {
        if let Some(z) = f.zero {
            self.f |= if z { 1 } else { 0 } << 7;
        }
        if let Some(s) = f.subtract {
            self.f |= if s { 1 } else { 0 } << 6;
        }
        if let Some(hc) = f.half_carry {
            self.f |= if hc { 1 } else { 0 } << 5;
        }
        if let Some(c) = f.carry {
            self.f |= if c { 1 } else { 0 } << 4;
        }
    }
}
