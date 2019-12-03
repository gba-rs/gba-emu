use raylib::prelude::*;
use std::ffi::CString;
use gba_emulator::gba::GBA;

pub struct StatusGUI {
    status_wb: rgui::WindowBox,
    x: f32,
    y: f32
}

impl StatusGUI {
    pub fn new(x: f32, y: f32) -> StatusGUI {
        return StatusGUI {
            status_wb: rgui::WindowBox {
                bounds: Rectangle::new(x, y, 120.0, 300.0),
                text: CString::new("Status").unwrap()
            },
            x: x,
            y: y
        }
    }

    pub fn draw(&self, handle: &mut RaylibDrawHandle<RaylibHandle>, gba: &mut GBA) {
        handle.draw_gui(&self.status_wb);
        handle.draw_text(&format!("Arm/Thumb: {:?}", gba.cpu.current_instruction_set), self.x as i32 + 10, self.y as i32 + 30, 11, Color::DARKGRAY);
        handle.draw_text(&format!("OpMode: {:?}", gba.cpu.operating_mode), self.x as i32 + 10, self.y as i32 + 45, 11, Color::DARKGRAY);
        handle.draw_text(&format!("LI: {}", gba.cpu.last_instruction), self.x as i32 + 10, self.y as i32 + 60, 11, Color::DARKGRAY);
        // handle.draw_text(&format!("R{} = {}", i, gba.cpu.get_register(i)), self.x as i32 + 10, self.y as i32 + 30 + (i as i32 * 15), 11, Color::DARKGRAY);
    }
}