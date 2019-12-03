use gba_emulator::gba::GBA;
use std::fs::File;
use std::io::prelude::*;
use std::env;
use raylib::prelude::*;
use std::ffi::CString;

pub mod gui;

use gui::control_gui::ControlGUI;
use gui::register_gui::RegisterGUI;
use gui::flags_gui::FlagsGUI;
use gui::status_gui::StatusGUI;

pub fn draw_registers(handle: &mut RaylibDrawHandle<RaylibHandle>, gba: &mut GBA) {
    for i in 0..16 {
        handle.draw_text(&format!("R{} = {}", i, gba.cpu.get_register(i)), 20, 40 + (i as i32 * 15), 11, Color::DARKGRAY);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let rom_file = File::open(&args[1]);
    let mut rom = Vec::new();
    let _ = rom_file.unwrap().read_to_end(&mut rom);

    let bios_file = File::open(&args[2]);
    let mut bios = Vec::new();
    let _ = bios_file.unwrap().read_to_end(&mut bios);

    let mut gba: GBA = GBA::new(0x08000000, bios, rom);

    let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, World").build();

    let register_wb = rgui::WindowBox {
        bounds: Rectangle::new(10.0, 10.0, 120.0, 300.0),
        text: CString::new("Registers").unwrap()
    };

    let control = ControlGUI::new(150.0, 10.0);
    let register = RegisterGUI::new(10.0, 10.0);
    let mut flags = FlagsGUI::new(150.0, 130.0);
    let status = StatusGUI::new(300.0, 10.0);

    let exit_program = false;

    while !exit_program && !rl.window_should_close() {
        let mut d: RaylibDrawHandle<RaylibHandle> = rl.begin_drawing(&thread);
        
        d.clear_background(Color::WHITE);
        d.draw_gui(&register_wb);
        
        control.draw(&mut d, &mut gba);
        register.draw(&mut d, &mut gba);
        flags.draw(&mut d, &mut gba);
        status.draw(&mut d, &mut gba);
    }
}
