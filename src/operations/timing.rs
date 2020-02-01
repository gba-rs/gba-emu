use crate::operations::arm_arithmetic::add;
use crate::memory::system_control::WaitStateControl;

pub struct CycleClock {
    pub prev_address: u32,
    pub cycles: u32,
    pub wait_state_control: WaitStateControl
}

pub const BIOS_START: u32 = 0x0000_0000;
pub const EWRAM_START: u32 = 0x0200_0000;
pub const IWRAM_START: u32 = 0x0300_0000;
pub const IOMEM_START: u32 = 0x0400_0000;
pub const PALRAM_START: u32 = 0x0500_0000;
pub const VRAM_START: u32 = 0x0600_0000;
pub const OAM_START: u32 = 0x0700_0000;
pub const PAKROM_START: u32 = 0x0800_0000;
pub const GAMEPAK_SRAM_START: u32 = 0x0E00_0000;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum MemAccessSize {
    Mem8,
    Mem16,
    Mem32,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum CycleType {
    N,
    S,
}

impl CycleClock {
    pub fn new() -> CycleClock {
        return CycleClock{
            prev_address: 0,
            cycles: 0,
            wait_state_control: WaitStateControl::new()
        };
    }

    pub fn update_cycles(&mut self, address: u32, access_size: MemAccessSize) {
        // TODO
        let nonseq_cycles = [4, 3, 2, 8];
        let seq_cycles = [2, 1];

        self.prev_address = address;
        match address & 0xFF00_0000 {
            BIOS_START | IWRAM_START | IOMEM_START => self.cycles += 1,
            EWRAM_START => {
                // TODO Default waitstate settings, see System Control chapter of GBATEK
                match access_size {
                    MemAccessSize::Mem8 | MemAccessSize::Mem16 => self.cycles += 3,
                    MemAccessSize::Mem32 => self.cycles += 6
                }
            }
            PALRAM_START | VRAM_START => {
                // TODO Plus 1 cycle if GBA accesses video memory at the same time.
                match access_size {
                    MemAccessSize::Mem8 | MemAccessSize::Mem16 => self.cycles += 1,
                    MemAccessSize::Mem32 => self.cycles += 2
                }
            }
            OAM_START => {
                // TODO Plus 1 cycle if GBA accesses video memory at the same time.
                self.cycles += 1;
            }
            PAKROM_START => {
                // TODO Default waitstate settings, see System Control chapter separate timings for sequential, and non-sequential accesses
                match access_size {
                    MemAccessSize::Mem8 | MemAccessSize::Mem16 => self.cycles += 5,
                    MemAccessSize::Mem32 => {
                        // below is an example of accessing the wait state control. not meant to actually do anything yet
                        self.cycles += nonseq_cycles[self.wait_state_control.get_wait_state_zero_first_access() as usize];
                        self.cycles += 8
                    }
                }
            }
            GAMEPAK_SRAM_START => {
                self.cycles += 5;
            }
            _ => { panic!("Trying to read unknown address") }
        }
    }

    pub fn get_cycles(&mut self) -> u32 {
        let temp = self.cycles;
        self.cycles = 0;
        return temp;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::work_ram::WorkRam;
    use crate::memory::memory_map::MemoryMap;
    use crate::gba::GBA;

    #[test]
    fn test_placeholder() {
        let mut gba: GBA = GBA::default();

        gba.memory_bus.read_u8(0x0200_0000); // 3

        gba.memory_bus.read_u16(0x0200_0000); // 3
        gba.memory_bus.read_u32(0x0200_0000); // 6
        gba.memory_bus.read_u32(0x0300_0000); // 1
        assert_eq!(gba.memory_bus.cycle_clock.get_cycles(), 13);
        assert_eq!(gba.memory_bus.cycle_clock.get_cycles(), 0);
    }
}

