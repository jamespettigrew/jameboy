mod cpu;
mod disassembly;
mod joypad;
mod memory;
mod opcode;
mod ppu;
mod util;

use crate::cpu::{Cpu, Register, RegisterWide, WriteFlags};
use crate::disassembly::Instruction;
use crate::memory::{Address, Memory};
use crate::ppu::Ppu;

use eframe::egui;
use egui::{Align, ColorImage};
use egui_extras::{Column, TableBuilder, TableRow};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

type ROM = Vec<u8>;

enum State {
    Paused,
    Running,
}

struct Debugger {
    run_to_pc: Option<u16>,
}

struct Jameboy {
    cpu: Cpu,
    debugger: Debugger,
    memory: Memory,
    ppu: Ppu,
    state: State,
}

impl Jameboy {
    fn init() -> Self {
        Self {
            cpu: Cpu::init(),
            debugger: Debugger {
                run_to_pc: None,
            },
            memory: Memory::init(),
            ppu: Ppu::init(),
            state: State::Paused,
        }
    }

    fn step(&mut self) {
        if let Some(run_to_pc) = self.debugger.run_to_pc {
            if run_to_pc == self.cpu.pc {
                self.state = State::Paused;
                self.debugger.run_to_pc = None;
                return;
            }
        }

        self.memory.step();
        self.cpu.step(&mut self.memory);

        // 4 PPU dots per M-cycle
        for _ in 0..4 {
            self.ppu.step(&mut self.memory);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        if &args[1] == &String::from("--doctor") {
            let rom_path = Path::new(&args[2]);
            let _ = doctor(rom_path);
            return;
        }
    }

    let options = eframe::NativeOptions {
        vsync: false,
        ..Default::default()
    };

    let mut jameboy = Jameboy::init();
    let bootstrap_rom = open_rom(Path::new("./roms/bootstrap.gb"));
    jameboy.memory.load_bootstrap_rom(&bootstrap_rom);

    let rom = open_rom(Path::new("./roms/tetris.gb"));
    map_rom_into_memory(&rom, &mut jameboy.memory);
    let disassembly = disassembly::disassemble(&rom);

    let goal_render_ms = 128_u128;
    eframe::run_simple_native("jameboy", options, move |ctx, _frame| {
        ctx.request_repaint();
        render(ctx, &mut jameboy, &disassembly);

        let last_render = std::time::Instant::now();
        while std::time::Instant::now()
            .duration_since(last_render)
            .as_millis()
            < goal_render_ms
        {
            if let State::Running = jameboy.state {
                joypad::handle_input(ctx, &mut jameboy.memory);
                jameboy.step();
            }
        }
    });
}

fn doctor(rom_path: &Path) -> std::io::Result<()> {
    let mut file = File::create("doctor.out")?;

    let mut jameboy = Jameboy::init();
    jameboy.cpu.write_register(Register::A, 0x01);
    jameboy.cpu.write_register(Register::B, 0x00);
    jameboy.cpu.write_register(Register::C, 0x13);
    jameboy.cpu.write_register(Register::E, 0xD8);
    jameboy.cpu.write_register(Register::H, 0x01);
    jameboy.cpu.write_register(Register::L, 0x4D);
    jameboy.cpu.write_flags(WriteFlags {
        zero: Some(true),
        subtract: None,
        half_carry: Some(true),
        carry: Some(true),
    });
    jameboy.cpu.write_register_wide(RegisterWide::SP, 0xFFFE);
    jameboy.cpu.write_register_wide(RegisterWide::PC, 0x0100);

    let rom = open_rom(rom_path);
    for instruction in disassembly::disassemble(&rom).iter() {
        print!("{:04x}    ", instruction.address);
        match &instruction.opcode {
            Some(opcode) => {
                print!("{}", opcode.mnemonic);
            }
            None => print!("UNKNOWN"),
        }
        print!("\n");
    }

    map_rom_into_memory(&rom, &mut jameboy.memory);
    // Unmap boot rom
    jameboy.memory.write(Address(0xFF50), 1);
    jameboy.state = State::Running;

    while let State::Running = jameboy.state {
        let cpu = &jameboy.cpu;
        let memory = &jameboy.memory;
        let pc = cpu.pc;
        let aa = memory.read(Address(pc));
        let bb = memory.read(Address(pc + 1));
        let cc = memory.read(Address(pc + 2));
        let dd = memory.read(Address(pc + 3));

        // Don't log when we hit CB/prefix opcode or we break gameboy-doctor
        if !jameboy.cpu.prefixed {
            let log = format!("A:{:02X} F:{:02X} B:{:02X} C:{:02X} D:{:02X} E:{:02X} H:{:02X} L:{:02X} SP:{:04X} PC:{:04X} PCMEM:{:02X},{:02X},{:02X},{:02X}\n",
                cpu.a, cpu.f, cpu.b, cpu.c, cpu.d, cpu.e, cpu.h, cpu.l, cpu.sp, cpu.pc, aa, bb, cc, dd);
            file.write_all(&log.into_bytes())?;
        }

        jameboy.step();
    }

    Ok(())
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

            ui.heading("IME");
            ui.horizontal(|ui| {
                let mut ime = jameboy.cpu.ime;
                ui.add_enabled(false, egui::Checkbox::new(&mut ime, "IME"));
            });
        });

        egui::Window::new("Disassembly").show(ctx, |ui| {
            let text_height = egui::TextStyle::Body.resolve(ui.style()).size;
            let table = TableBuilder::new(ui)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::auto())
                .column(Column::auto())
                .resizable(true);

            let pc = jameboy.cpu.read_register_wide(RegisterWide::PC);

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
                    body.rows(text_height, disassembly.len(), |mut row| {
                        let row_index = row.index();
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

                        if row.response().interact(egui::Sense::click()).double_clicked() {
                            jameboy.state = State::Running;
                            jameboy.debugger.run_to_pc = Some(instruction.address.0);
                        }
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

        egui::Window::new("Memory").show(ctx, |ui| {
            let text_height = egui::TextStyle::Body.resolve(ui.style()).size;
            let table = TableBuilder::new(ui)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .columns(Column::auto(), 1 + 16); // 1 for offset, 16 for bytes

            table
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.strong("Offset");
                    });
                    for i in 0..16 {
                        header.col(|ui| {
                            ui.strong(format!("{:02X}", i));
                        });
                    }
                })
                .body(|body| {
                    body.rows(text_height, 0x10000 / 16, |mut row| {
                        let row_index = row.index();
                        row.col(|ui| {
                            ui.label(format!("{:04X}", row_index * 16));
                        });
                        for i in 0..16 {
                            row.col(|ui| {
                                let address = Address((row_index * 16 + i) as u16);
                                let value = jameboy.memory.read(address);
                                ui.label(format!("{:02X}", value));
                            });
                        }
                    });
                });
        });

        egui::Window::new("Tiles").show(ctx, |ui| {
            let image = &jameboy.ppu.get_tile_buffer(&jameboy.memory);
            let image = &image::imageops::resize(
                image,
                image.width() * 4,
                image.height() * 4,
                image::imageops::FilterType::Nearest,
            );
            let size = (image.width() as usize, image.height() as usize);
            let image = ColorImage::from_gray(size.into(), image);
            let texture = ctx.load_texture("LCD", image, egui::TextureOptions::default());

            ui.image(&texture);
        });

        let image = &jameboy.ppu.image_buffer;
        let image = &image::imageops::resize(image, image.width() * 3, image.height() * 3, image::imageops::FilterType::Nearest);
        let size = (image.width() as usize, image.height() as usize);
        let image = ColorImage::from_gray(size.into(), image);
        let texture = ctx.load_texture("LCD", image, egui::TextureOptions::default());

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Display");
            ui.image(&texture);
        });
    });
}

fn map_rom_into_memory(rom: &ROM, memory: &mut Memory) {
    for i in 0..rom.len() {
        memory.write(Address(i as u16), rom[i] as u8);
    }
}

fn open_rom(rom_path: &Path) -> ROM {
    let mut rom_file = File::open(rom_path).expect("ROM path should be valid");
    let mut rom = Vec::new();
    rom_file
        .read_to_end(&mut rom)
        .expect("reading ROM into buffer should not fail");

    rom
}
