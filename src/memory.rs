extern crate derive_more;

use derive_more::LowerHex;
use rand::rngs::SmallRng;
use rand::{RngCore, SeedableRng};

#[derive(LowerHex, PartialEq, Eq, PartialOrd, Ord)]
pub struct Address(pub u16);

pub struct Memory {
    bootstrap_rom: [u8; 0x100],
    ram: [u8; 0x10000],
}

impl Memory {
    pub fn init() -> Self {
        let mut memory = [0; 0x10000];
        // let mut rng = SmallRng::from_entropy();
        // rng.fill_bytes(&mut memory);

        Self {
            bootstrap_rom: [0; 0x100],
            ram: memory,
        }
    }

    pub fn load_bootstrap_rom(&mut self, rom: &[u8]) {
        for i in 0..rom.len() {
            self.bootstrap_rom[i] = rom[i];
        }
    }

    pub fn read(&self, address: Address) -> u8 {
        // TODO: Remove this. It spoofs the joypad input so that Tetris functions boots without
        // a joypad implementation.
        if address.0 == 0xFF00 {
            return 0xFF;
        }

        if address.0 == 0xFF44 {
            // Uncomment the following line if testing with gameboy-doctor
            // return 0x90;
        }

        if self.ram[0xFF50] == 0 && address.0 < 0x100 {
            self.bootstrap_rom[usize::from(address.0)]
        } else {
            self.ram[usize::from(address.0)]
        }
    }

    pub fn read_range(&self, address: Address, count: u8) -> &[u8] {
        let start = usize::from(address.0);

        &self.ram[start..start + count as usize]
    }

    pub fn write(&mut self, address: Address, value: u8) {
        self.ram[usize::from(address.0)] = value;
    }
}
