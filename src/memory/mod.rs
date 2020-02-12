pub mod work_ram;
pub mod memory_map;
pub mod mock_memory;
pub mod bios_ram;
pub mod game_pack_rom;
pub mod io_registers;
pub mod lcd_io_registers;
pub mod interrupt_registers;
pub mod key_input_registers;
pub mod system_control;
pub mod timer_registers;

pub type GbaMem = Vec<u8>;