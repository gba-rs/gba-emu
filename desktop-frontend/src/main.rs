use gba_emulator::gba::GBA;
use std::fs::File;
use std::io::prelude::*;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let file = File::open(&args[1]);
    let mut rom = Vec::new();
    file.unwrap().read_to_end(&mut rom);

    let mut gba: GBA = GBA::new(0x02000000);
    gba.load(0x02000000, rom);
    gba.run();
}
