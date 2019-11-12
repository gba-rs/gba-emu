use gba_emulator::gba::GBA;
use std::fs::File;
use std::io::prelude::*;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let rom_file = File::open(&args[1]);
    let mut rom = Vec::new();
    rom_file.unwrap().read_to_end(&mut rom);

    let bios_file = File::open(&args[2]);
    let mut bios = Vec::new();
    bios_file.unwrap().read_to_end(&mut bios);

    let step_count = args[3].parse().unwrap();

    let mut gba: GBA = GBA::new(0x08000000, bios, rom);
    // let mut gba: GBA = GBA::new(0x00000000, bios, rom);

    for i in 0..step_count {
        gba.step();
        for i in 0..16 {
            // println!("R{} = {:X}", i, gba.cpu.get_register(i));
        }
    }
}
