extern crate derive_more;
use derive_more::LowerHex;

#[derive(LowerHex, PartialEq, Eq, PartialOrd, Ord)]
pub struct Address(pub u16);

pub struct Memory([u8; 0xFFFF]);

impl Memory {
    pub fn init() -> Self {
        Self([0; 0xFFFF])
    }

    pub fn read(&self, address: Address) -> u8 {
        self.0[usize::from(address.0)]
    }

    pub fn write(&mut self, address: Address, value: u8) {
        self.0[usize::from(address.0)] = value;
    }
}
