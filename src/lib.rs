mod utils;
pub mod arm_formats;
pub mod operations;
pub mod cpu;
pub mod memory;
pub mod gba;
pub mod thumb_formats;
pub mod gpu;
pub mod interrupts;
pub mod timers;
pub mod dma;
pub mod gamepak;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
