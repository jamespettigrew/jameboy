use crate::memory::{Address, Memory};

const ADDRESS_JOYP_REGISTER: u16 = 0xFF00;

const KEY_RIGHT: egui::Key = egui::Key::D;
const KEY_LEFT: egui::Key = egui::Key::A;
const KEY_UP: egui::Key = egui::Key::W;
const KEY_DOWN: egui::Key = egui::Key::S;
const KEY_A: egui::Key = egui::Key::J;
const KEY_B: egui::Key = egui::Key::K;
const KEY_SELECT: egui::Key = egui::Key::Enter;
const KEY_START: egui::Key = egui::Key::Escape;

pub fn handle_input(ctx: &egui::Context, memory: &mut Memory) {
    let mut register = 0xFF;
    
    ctx.input(|i| {
        if i.key_pressed(KEY_RIGHT) {
            register &= 0b1111_1110;
            register &= 0b1110_1111;
        }

        if i.key_pressed(KEY_LEFT) {
            register &= 0b1111_1101;
            register &= 0b1110_1111;
        }

        if i.key_pressed(KEY_UP) {
            register &= 0b1111_1011;
            register &= 0b1110_1111;
        }

        if i.key_pressed(KEY_DOWN) {
            register &= 0b1111_0111;
            register &= 0b1110_1111;
        }

        if i.key_pressed(KEY_A) {
            register &= 0b1111_1110;
            register &= 0b1110_1111;
        }

        if i.key_pressed(KEY_B) {
            register &= 0b1111_1101;
            register &= 0b1101_1111;
        }

        if i.key_pressed(KEY_SELECT) {
            register &= 0b1111_1011;
            register &= 0b1101_1111;
        }

        if i.key_pressed(KEY_START) {
            register &= 0b1111_0111;
            register &= 0b1101_1111;
        }
    });

    memory.write(Address(ADDRESS_JOYP_REGISTER), register);
}
