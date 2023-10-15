extern crate derive_more;
use derive_more::LowerHex;
use rand::rngs::SmallRng;
use rand::{RngCore, SeedableRng};

#[derive(LowerHex, PartialEq, Eq, PartialOrd, Ord)]
pub struct Address(pub u16);

pub struct Memory([u8; 0xFFFF]);

impl Memory {
    pub fn init() -> Self {
        let mut memory = [0; 0xFFFF];
        let mut rng = SmallRng::from_entropy();
        rng.fill_bytes(&mut memory);
        Self(memory)
    }

    pub fn read(&self, address: Address) -> u8 {
        self.0[usize::from(address.0)]
    }

    pub fn read_range(&self, address: Address, count: u8) -> &[u8] {
        let start = usize::from(address.0);

        &self.0[start..start + count as usize]
    }

    pub fn write(&mut self, address: Address, value: u8) {
        self.0[usize::from(address.0)] = value;
    }
}
