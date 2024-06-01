use crate::memory::{Address, Memory};

const ADDRESS_JOYP_REGISTER: u16 = 0xFF00;

pub fn handle_input(ctx: &egui::Context, memory: &mut Memory) {
    let mut register = 0xFF;
    
    // Right
    if ctx.input(|i| i.key_pressed(egui::Key::D)) {
        register &= 0b1111_1110;
        register &= 0b1110_1111;
    }

    // Left
    if ctx.input(|i| i.key_pressed(egui::Key::A)) {
        register &= 0b1111_1101;
        register &= 0b1110_1111;
    }

    // Up
    if ctx.input(|i| i.key_pressed(egui::Key::W)) {
        register &= 0b1111_1011;
        register &= 0b1110_1111;
    }

    // Down
    if ctx.input(|i| i.key_pressed(egui::Key::S)) {
        register &= 0b1111_0111;
        register &= 0b1110_1111;
    }

    // A
    if ctx.input(|i| i.key_pressed(egui::Key::J)) {
        register &= 0b1111_1110;
        register &= 0b1110_1111;
    }

    // B
    if ctx.input(|i| i.key_pressed(egui::Key::K)) {
        register &= 0b1111_1101;
        register &= 0b1101_1111;
    }

    // Select
    if ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
        register &= 0b1111_1011;
        register &= 0b1101_1111;
    }

    // Start
    if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
        register &= 0b1111_0111;
        register &= 0b1101_1111;
    }

    memory.write(Address(ADDRESS_JOYP_REGISTER), register);
}
