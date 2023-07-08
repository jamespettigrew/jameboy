mod opcode;

use std::fs;
use std::io::Read;
use std::path::Path;

fn main() {
    let mut bootstrap_rom_file =
        fs::File::open(Path::new("./roms/bootstrap.gb")).expect("Can't open bootstrap ROM");
    let mut bootstrap_rom = Vec::new();
    bootstrap_rom_file
        .read_to_end(&mut bootstrap_rom)
        .expect("Error reading bootstrap ROM");
    let program_length = bootstrap_rom.len();
    let mut pc = 0;
    let mut prefixed = false;
    while pc < program_length {
        print!("{:04x}", pc);
        let byte = bootstrap_rom[pc];
        let opcode = if prefixed {
            prefixed = false;
            opcode::decode_prefixed(byte)
        } else {
            opcode::decode(byte)
        };

        match opcode {
            Some(i) => {
                print!("    {}", i.mnemonic);
                pc += i.size_bytes as usize;
                if i.mnemonic == "PREFIX" {
                    prefixed = true;
                }
            }
            None => {
                print!("    UNKNOWN({:04x})", &bootstrap_rom[pc]);
                pc += 1;
            }
        }
        print!("\n");
    }
}
