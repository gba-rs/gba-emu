use crate::operations::arm_arithmetic::add;

pub struct CycleClock {
    pub prev_address: u32,
    pub cycles: u32
}

pub const BIOS_START: u32 = 0x0000_0000;
pub const EWRAM_START: u32 = 0x0200_0000;
pub const IWRAM_START: u32 = 0x0300_0000;
pub const IOMEM_START: u32 = 0x0400_0000;
pub const PALRAM_START: u32 = 0x0500_0000;
pub const VRAM_START: u32 = 0x0600_0000;
pub const OAM_START: u32 = 0x0700_0000;
pub const PAKROM_START: u32 = 0x0800_0000;
pub const CARTRAM_START: u32 = 0x0E00_0000;

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
    pub fn update_cycles(&mut self, address: u32, access_size: MemAccessSize) {
        // TODO
        self.prev_address = address;
        match address & 0xFF00_0000 {
            BIOS_START => {}
            EWRAM_START => {
                self.cycles += 2;
            }
            IWRAM_START => {}
            IOMEM_START => {}
            PALRAM_START => {}
            VRAM_START => {}
            OAM_START => {}
            PAKROM_START => {}
            CARTRAM_START => {}
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

        gba.mem_map.read_u8(0x0200_0000);
        gba.mem_map.read_u16(0x0200_0000);
        gba.mem_map.read_u32(0x0200_0000);

        assert_eq!(gba.mem_map.cycle_clock.get_cycles(), 6);
        assert_eq!(gba.mem_map.cycle_clock.get_cycles(), 0);
    }
}

