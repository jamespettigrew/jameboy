extern crate derive_more;

use derive_more::LowerHex;
use rand::rngs::SmallRng;
use rand::{RngCore, SeedableRng};

use crate::util::u8_to_u16;

const ADDRESS_DMA: u16 = 0xFF46;

#[derive(LowerHex, PartialEq, Eq, PartialOrd, Ord)]
pub struct Address(pub u16);

enum DmaState {
    Inactive,
    Active { src_addr: u16, cycles: u8, last_transferred_byte: u8 },
}

pub struct Memory {
    bootstrap_rom: [u8; 0x100],
    dma_state: DmaState,
    ram: [u8; 0x10000],
}

impl Memory {
    pub fn init() -> Self {
        let mut memory = [0; 0x10000];
        // let mut rng = SmallRng::from_entropy();
        // rng.fill_bytes(&mut memory);

        Self {
            bootstrap_rom: [0; 0x100],
            dma_state: DmaState::Inactive,
            ram: memory,
        }
    }

    pub fn load_bootstrap_rom(&mut self, rom: &[u8]) {
        for i in 0..rom.len() {
            self.bootstrap_rom[i] = rom[i];
        }
    }

    pub fn read(&self, address: Address) -> u8 {
        if let DmaState::Active { last_transferred_byte, .. } = self.dma_state {
            return last_transferred_byte;
        }

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

    pub fn step(&mut self) {
        if let DmaState::Active { src_addr, cycles, .. } = self.dma_state {
            let dst_address = u8_to_u16(0xFE, cycles);
            let byte_to_transfer = self.ram[(src_addr + cycles as u16) as usize];
            self.ram[dst_address as usize] = byte_to_transfer;

            self.dma_state = match cycles {
                0..=159 => DmaState::Active { src_addr, cycles: cycles + 1, last_transferred_byte: byte_to_transfer },
                _ => DmaState::Inactive,
            };
        }
    }

    pub fn write(&mut self, address: Address, value: u8) {
        if let DmaState::Active { .. } = self.dma_state {
            return;
        }

        self.ram[usize::from(address.0)] = value;

        if address.0 == ADDRESS_DMA {
            let src_addr = (value as u16) * 100; // Register value is source address / 100
            self.dma_state = DmaState::Active { src_addr, cycles: 0, last_transferred_byte: 0 };
        }
    }
}
