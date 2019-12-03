use raylib::prelude::*;
use std::ffi::CString;
use gba_emulator::gba::GBA;

pub struct ControlGUI {
    pub control_wb: rgui::WindowBox,
    pub step_1_btn: rgui::Button,
    pub step_5_btn: rgui::Button,
    pub step_50_btn: rgui::Button,
    pub step_100_btn: rgui::Button,
}

impl ControlGUI {
    pub fn new(x: f32, y: f32) -> ControlGUI {
        return ControlGUI {
            control_wb: rgui::WindowBox {
                bounds: Rectangle::new(x, y, 120.0, 100.0),
                text: CString::new("Controls").unwrap()
            },
            step_1_btn: rgui::Button {
                bounds: Rectangle::new(x + 10.0, y + 30.0, 20.0, 16.0),
                text: CString::new("1").unwrap()
            },
            step_5_btn: rgui::Button {
                bounds: Rectangle::new(x + 15.0 + 20.0, y + 30.0, 20.0, 16.0),
                text: CString::new("5").unwrap()
            },
            step_50_btn: rgui::Button {
                bounds: Rectangle::new(x + 20.0 + 40.0, y + 30.0, 20.0, 16.0),
                text: CString::new("50").unwrap()
            },
            step_100_btn: rgui::Button {
                bounds: Rectangle::new(x + 25.0 + 60.0, y + 30.0, 20.0, 16.0),
                text: CString::new("100").unwrap()
            },
        };
    }

    pub fn draw(&self, handle: &mut RaylibDrawHandle<RaylibHandle>, gba: &mut GBA) {
        handle.draw_gui(&self.control_wb);

        if let rgui::DrawResult::Bool(b) = handle.draw_gui(&self.step_1_btn) {
            if b { 
                gba.step();
            }
        }

        if let rgui::DrawResult::Bool(b) = handle.draw_gui(&self.step_5_btn) {
            if b { 
                for _ in 0..5{
                    gba.step();
                }
            }
        }

        if let rgui::DrawResult::Bool(b) = handle.draw_gui(&self.step_50_btn) {
            if b { 
                for _ in 0..100{
                    gba.step();
                }
            }
        }

        if let rgui::DrawResult::Bool(b) = handle.draw_gui(&self.step_100_btn) {
            if b { 
                for _ in 0..100{
                    gba.step();
                }
            }
        }
    }
}