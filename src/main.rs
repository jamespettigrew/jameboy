mod cpu;
mod disassembly;
mod memory;
mod opcode;
mod util;

use crate::cpu::{Cpu, RegisterWide};
use crate::memory::Memory;

use eframe::egui;

use std::fs;
use std::io::Read;
use std::path::Path;

type ROM = Vec<u8>;

enum State {
    Paused,
    Running,
}

struct Jameboy {
    cpu: Cpu,
    memory: Memory,
    prefixed: bool,
    state: State,
}

impl Jameboy {
    fn init() -> Self {
        Self {
            cpu: Cpu::init(),
            memory: Memory::init(),
            prefixed: false,
            state: State::Paused,
        }
    }

    fn step(&mut self) {
        let pc = self.cpu.read_register_wide(RegisterWide::PC);
        let byte = self.memory.read(pc);
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
        opcode.execute(&mut self.cpu, &mut self.memory);
        self.cpu
            .write_register_wide(RegisterWide::PC, pc + opcode.size_bytes as u16);
    }
}

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(640.0, 480.0)),
        ..Default::default()
    };

    let bootstrap_rom = open_rom(Path::new("./roms/bootstrap.gb"));
    let mut jameboy = Jameboy::init();
    map_rom_into_memory(&bootstrap_rom, &mut jameboy.memory);

    eframe::run_simple_native("jameboy", options, move |ctx, _frame| {
        render(ctx, &mut jameboy);
        if let State::Running = jameboy.state {
            jameboy.step();
        }
    });
}

fn render(ctx: &egui::Context, jameboy: &mut Jameboy) {
    egui::CentralPanel::default().show(ctx, |ui| {
        egui::Window::new("CPU").show(ctx, |ui| {
            ui.heading("Registers");
            ui.label(format!(
                "AF: {:04x}",
                jameboy.cpu.read_register_wide(RegisterWide::AF)
            ));
            ui.label(format!(
                "BC: {:04x}",
                jameboy.cpu.read_register_wide(RegisterWide::BC)
            ));
            ui.label(format!(
                "DE: {:04x}",
                jameboy.cpu.read_register_wide(RegisterWide::DE)
            ));
            ui.label(format!(
                "HL: {:04x}",
                jameboy.cpu.read_register_wide(RegisterWide::HL)
            ));
        });

        egui::Window::new("Program").show(ctx, |ui| {
            ui.horizontal(|ui| match jameboy.state {
                State::Paused => {
                    if ui.button("⏵").clicked() {
                        jameboy.state = State::Running;
                    }
                    if ui.button("⏭").clicked() {
                        jameboy.step();
                    }
                }
                State::Running => {
                    if ui.button("⏸").clicked() {
                        jameboy.state = State::Paused;
                    }
                    ui.add_enabled(false, egui::Button::new("⏭"));
                }
            });
        });
    });
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
