use crate::memory::memory_map::MemoryMap;
use crate::operations::timing::{CycleClock, MemAccessSize, CycleType};

pub struct MemoryBus {
    pub mem_map: MemoryMap,
    pub cycle_clock: CycleClock
}

impl MemoryBus {
    pub fn new() -> MemoryBus {
        return MemoryBus {
            mem_map: MemoryMap::new(),
            cycle_clock: CycleClock::new()
        };
    }
    // TODO REMOVE HARDCODED ACCESS TYPE

    pub fn read_u8(&mut self, address: u32) -> u8 {
        self.cycle_clock.update_cycles(address, MemAccessSize::Mem8, CycleType::N);
        self.mem_map.read_u8(address)
    }

    pub fn read_u16(&mut self, address: u32) -> u16 {
        self.cycle_clock.update_cycles(address, MemAccessSize::Mem16, CycleType::N);
        self.mem_map.read_u16(address)
    }

    pub fn read_u32(&mut self, address: u32) -> u32 {
        self.cycle_clock.update_cycles(address, MemAccessSize::Mem32, CycleType::N);
        self.mem_map.read_u32(address)
    }

    pub fn write_u8(&mut self, address: u32, value: u8) {
        self.cycle_clock.update_cycles(address, MemAccessSize::Mem8, CycleType::N);
        self.mem_map.write_u8(address, value);
    }

    pub fn write_u16(&mut self, address: u32, value: u16) {
        self.cycle_clock.update_cycles(address, MemAccessSize::Mem16, CycleType::N);
        self.mem_map.write_u16(address, value);
    }

    pub fn write_u32(&mut self, address: u32, value: u32) {
        self.cycle_clock.update_cycles(address, MemAccessSize::Mem32, CycleType::N);
        self.mem_map.write_u32(address, value);
    }
}