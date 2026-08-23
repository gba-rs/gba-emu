pub mod memory_map;
pub mod eeprom;
pub mod lcd_io_registers;
pub mod interrupt_registers;
pub mod key_input_registers;
pub mod system_control;
pub mod memory_bus;
pub mod dma_registers;
pub mod timer_registers;
pub mod sound_registers;

// Cell<u8>, not RefCell<Vec<u8>>: every register struct and MemoryMap share
// this buffer via Rc, and Cell gives interior mutability per-byte with none
// of RefCell's runtime borrow-flag bookkeeping — GBA emulation is
// single-threaded, so there's never a real aliasing hazard to check for.
pub type GbaMem = Vec<std::cell::Cell<u8>>;
