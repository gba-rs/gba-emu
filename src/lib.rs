mod utils;
pub mod formats;
pub mod operations;
pub mod cpu;
pub mod memory;

use wasm_bindgen::prelude::*;
use crate::formats::{data_processing::DataProcessing};


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// #[wasm_bindgen]
// extern {
//     fn alert(s: &str);
// }

// #[wasm_bindgen]
// pub fn greet() {
//     alert("Hello, gba-emulator!");
// }

pub fn decode(instruction: u32) {
    let opcode: u16 = (((instruction >> 16) & 0xFF0) | ((instruction >> 4) & 0x0F)) as u16;
    match opcode {
        0x080 => { // ADD lli
            let format: DataProcessing = DataProcessing::from(instruction);

        },
        _ => {},
    }
}