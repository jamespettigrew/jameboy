mod disassembly;
mod opcode;

use std::fs;
use std::io::Read;
use std::path::Path;

fn main() {
    let bootstrap_rom = open_rom(Path::new("./roms/bootstrap.gb"));
    disassemble(bootstrap_rom);
}

fn open_rom(rom_path: &Path) -> ROM {
    let mut rom_file = fs::File::open(rom_path).expect("ROM path should be valid");
    let mut rom = Vec::new();
    rom_file
        .read_to_end(&mut rom)
        .expect("reading ROM into buffer should not fail");

    rom
}

fn disassemble(r: ROM) {
    for instruction in disassembly::disassemble(&r).iter() {
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
