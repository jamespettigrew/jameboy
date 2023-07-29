mod cpu;
mod disassembly;
mod memory;
mod opcode;
mod util;

use crate::cpu::{Cpu, RegisterWide};
use crate::disassembly::Instruction;
use crate::memory::{Address, Memory};

use eframe::egui;
use egui::Align;
use egui_extras::{Column, TableBuilder};

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
        let byte = self.memory.read(Address(pc));
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
    let disassembly = disassembly::disassemble(&bootstrap_rom);

    eframe::run_simple_native("jameboy", options, move |ctx, _frame| {
        render(ctx, &mut jameboy, &disassembly);
        if let State::Running = jameboy.state {
            jameboy.step();
        }
    });
}

fn render(ctx: &egui::Context, jameboy: &mut Jameboy, disassembly: &Vec<Instruction>) {
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
            ui.label(format!(
                "PC: {:04x}",
                jameboy.cpu.read_register_wide(RegisterWide::PC)
            ));
            ui.label(format!(
                "SP: {:04x}",
                jameboy.cpu.read_register_wide(RegisterWide::SP)
            ));
            ui.add_space(5.0);

            ui.heading("Flags");
            ui.horizontal(|ui| {
                let mut flags = jameboy.cpu.read_flags();
                ui.add_enabled(false, egui::Checkbox::new(&mut flags.zero, "Z"));
                ui.add_enabled(false, egui::Checkbox::new(&mut flags.subtract, "S"));
                ui.add_enabled(false, egui::Checkbox::new(&mut flags.half_carry, "HC"));
                ui.add_enabled(false, egui::Checkbox::new(&mut flags.carry, "C"));
            });
        });

        egui::Window::new("Program").show(ctx, |ui| {
            let text_height = egui::TextStyle::Body.resolve(ui.style()).size;
            let mut table = TableBuilder::new(ui)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::auto())
                .column(Column::auto());
            let pc = jameboy.cpu.read_register_wide(RegisterWide::PC);

            table = table.scroll_to_row(pc as usize, Some(Align::Center));
            table
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.strong("Address");
                    });
                    header.col(|ui| {
                        ui.strong("Instruction");
                    });
                })
                .body(|body| {
                    body.rows(text_height, disassembly.len(), |row_index, mut row| {
                        let instruction = &disassembly[row_index];
                        row.col(|ui| {
                            if Address(pc) == instruction.address {
                                let max_rect = ui.max_rect();
                                let stripe_rect = max_rect.expand2(0.5 * ui.spacing().item_spacing);

                                ui.painter().rect_filled(
                                    stripe_rect,
                                    0.0,
                                    ui.visuals().extreme_bg_color,
                                );
                            }
                            ui.label(format!("{:04x}", instruction.address));
                        });
                        row.col(|ui| {
                            if Address(pc) == instruction.address {
                                let max_rect = ui.max_rect();
                                let stripe_rect = max_rect.expand2(0.5 * ui.spacing().item_spacing);

                                ui.painter().rect_filled(
                                    stripe_rect,
                                    0.0,
                                    ui.visuals().extreme_bg_color,
                                );
                            }

                            let label = match &instruction.opcode {
                                Some(op) => op.mnemonic.clone(),
                                None => "UNKNOWN".to_string(),
                            };
                            ui.label(label);
                        });
                    });
                });

            ui.add_space(10.0);
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
        memory.write(Address(i as u16), rom[i] as u8);
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
