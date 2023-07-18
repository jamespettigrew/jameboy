mod cpu;
mod disassembly;
mod memory;
mod opcode;
mod util;

use crate::cpu::{Cpu, RegisterWide};
use crate::memory::Memory;

use std::fs;
use std::io::Read;
use std::path::Path;

type ROM = Vec<u8>;

fn main() {
    let bootstrap_rom = open_rom(Path::new("./roms/bootstrap.gb"));
    disassemble(&bootstrap_rom);

    let cpu = &mut Cpu::init();
    let memory = &mut Memory::init();
    map_rom_into_memory(&bootstrap_rom, memory);

    let mut executing = true;
    let mut prefixed = false;
    while executing {
        println!("{:?}", cpu);
        let pc = cpu.read_register_wide(RegisterWide::PC);
        let byte = memory.read(pc);
        let opcode = if prefixed {
            prefixed = false;
            opcode::decode_prefixed(byte)
        } else {
            opcode::decode(byte)
        };

        if opcode.is_none() {
            executing = false;
            continue;
        }

        let opcode = opcode.unwrap();
        opcode.execute(cpu, memory);
        cpu.write_register_wide(RegisterWide::PC, pc + opcode.size_bytes as u16);
    }
}

fn map_rom_into_memory(rom: &ROM, memory: &mut Memory) {
    for i in 0..rom.len() {
        memory.write(i as u16, rom[i] as u8);
    }
}

fn open_rom(rom_path: &Path) -> ROM {
    let mut rom_file = fs::File::open(rom_path).expect("ROM path should be valid");
    let mut rom = Vec::new();
    rom_file
        .read_to_end(&mut rom)
        .expect("reading ROM into buffer should not fail");

    rom
}

fn disassemble(r: &ROM) {
    for instruction in disassembly::disassemble(r).iter() {
        print!("{:04x}    ", instruction.address);
        match &instruction.opcode {
            Some(opcode) => {
                print!("{}", opcode.mnemonic);
            }
            None => print!("UNKNOWN"),
        }
        print!("\n");
    }
}
