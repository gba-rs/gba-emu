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