use crate::memory::memory_map::MemoryMap;
use crate::operations::timing::{CycleClock, MemAccessSize};
use crate::gamepak::BackupType;

pub struct MemoryBus {
    pub mem_map: MemoryMap,
    pub cycle_clock: CycleClock,
}

impl MemoryBus {
    pub fn new(backup_type: BackupType) -> MemoryBus {
        return MemoryBus {
            mem_map: MemoryMap::new(backup_type),
            cycle_clock: CycleClock::new(),
        };
    }

    pub fn new_stub() -> MemoryBus {
        return MemoryBus::new(BackupType::Error);
    }

    pub fn read_u8(&mut self, address: u32) -> u8 {
        self.cycle_clock.update_cycles(address, MemAccessSize::Mem8);
        self.mem_map.read_u8(address)
    }

    pub fn read_u16(&mut self, address: u32) -> u16 {
        self.cycle_clock.update_cycles(address, MemAccessSize::Mem16);
        self.mem_map.read_u16(address)
    }

    pub fn read_u32(&mut self, address: u32) -> u32 {
        self.cycle_clock.update_cycles(address, MemAccessSize::Mem32);
        self.mem_map.read_u32(address)
    }

    pub fn write_u8(&mut self, address: u32, value: u8) {
        self.cycle_clock.update_cycles(address, MemAccessSize::Mem8);
        self.mem_map.write_u8(address, value);
    }

    pub fn write_u16(&mut self, address: u32, value: u16) {
        self.cycle_clock.update_cycles(address, MemAccessSize::Mem16);

        // if address < 0x00003FFF {
        //     // panic!("Writing to bios: {:X}", address);
        //     return;
        // }

        self.mem_map.write_u16(address, value);
    }

    pub fn write_u32(&mut self, address: u32, value: u32) {
        self.cycle_clock.update_cycles(address, MemAccessSize::Mem32);

        if address < 0x00003FFF {
            // panic!("Writing to bios");
            // panic!("Writing to bios: {:X}", address);
            return;
        }

        self.mem_map.write_u32(address, value);
    }
}