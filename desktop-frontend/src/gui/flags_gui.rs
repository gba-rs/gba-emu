use raylib::prelude::*;
use std::ffi::CString;
use gba_emulator::gba::GBA;

pub struct FlagsGUI {
    flags_wb: rgui::WindowBox,
    x: f32,
    y: f32
}

impl FlagsGUI {
    pub fn new(x: f32, y: f32) -> FlagsGUI {
        return FlagsGUI {
            flags_wb: rgui::WindowBox {
                bounds: Rectangle::new(x, y, 120.0, 200.0),
                text: CString::new("Flags").unwrap()
            },
            x: x,
            y: y
        }
    }

    pub fn draw(&mut self, handle: &mut RaylibDrawHandle<RaylibHandle>, gba: &mut GBA) {
        handle.draw_gui(&self.flags_wb);
        handle.draw_text("CPSR", self.x as i32 + 10, self.y as i32 + 30, 15, Color::BLACK);
        handle.draw_text(&format!("Carry = {}", gba.cpu.cpsr.flags.carry), self.x as i32 + 10, self.y as i32 + 45, 11, Color::DARKGRAY);
        handle.draw_text(&format!("Negative = {}", gba.cpu.cpsr.flags.negative), self.x as i32 + 10, self.y as i32 + 60, 11, Color::DARKGRAY);
        handle.draw_text(&format!("S Overflow = {}", gba.cpu.cpsr.flags.signed_overflow), self.x as i32 + 10, self.y as i32 + 75, 11, Color::DARKGRAY);
        handle.draw_text(&format!("Zero = {}", gba.cpu.cpsr.flags.zero), self.x as i32 + 10, self.y as i32 + 90, 11, Color::DARKGRAY);
    }
}