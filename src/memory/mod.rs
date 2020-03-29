pub mod memory_map;
pub mod lcd_io_registers;
pub mod interrupt_registers;
pub mod key_input_registers;
pub mod system_control;
pub mod memory_bus;

pub type GbaMem = Vec<u8>;