use raylib::prelude::*;
use std::ffi::CString;
use gba_emulator::gba::GBA;
use gba_emulator::cpu::cpu::InstructionSet;

pub struct RegisterGUI {
    register_wb: rgui::WindowBox,
    x: f32,
    y: f32
}

impl RegisterGUI {
    pub fn new(x: f32, y: f32) -> RegisterGUI {
        return RegisterGUI {
            register_wb: rgui::WindowBox {
                bounds: Rectangle::new(x, y, 120.0, 300.0),
                text: CString::new("Registers").unwrap()
            },
            x: x,
            y: y
        }
    }

    pub fn draw(&self, handle: &mut RaylibDrawHandle<RaylibHandle>, gba: &mut GBA) {
        handle.draw_gui(&self.register_wb);
        let range = if gba.cpu.current_instruction_set == InstructionSet::Arm { 16 } else { 10 };

        for i in 0..range {
            handle.draw_text(&format!("R{} = {}", i, gba.cpu.get_register(i)), self.x as i32 + 10, self.y as i32 + 30 + (i as i32 * 15), 11, Color::DARKGRAY);
        }
    }
}