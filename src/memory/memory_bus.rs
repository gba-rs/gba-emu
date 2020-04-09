use crate::memory::memory_map::MemoryMap;
use crate::operations::timing::{CycleClock, MemAccessSize};

#[derive(Debug, PartialEq)]
pub enum HaltState {
    Running,
    Halt,
    Stop
}

#[derive(Debug, PartialEq)]
pub enum BackupType {
    Sram,
    Eeprom,
    Flash64K,
    Flash128K,
    Error
}

pub struct MemoryBus {
    pub mem_map: MemoryMap,
    pub cycle_clock: CycleClock,
    pub halt_state: HaltState,
    pub backup_type: BackupType
}

impl MemoryBus {
    pub fn new() -> MemoryBus {
        return MemoryBus {
            mem_map: MemoryMap::new(),
            cycle_clock: CycleClock::new(),
            halt_state: HaltState::Running,
            backup_type: BackupType::Error
        };
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

        if address == 0x4000301 {
            if value == 0 {
                self.halt_state = HaltState::Halt;
            } else {
                self.halt_state = HaltState::Stop
            }

            return;
        }

        self.mem_map.write_u8(address, value);
    }

    pub fn write_u16(&mut self, address: u32, value: u16) {
        self.cycle_clock.update_cycles(address, MemAccessSize::Mem16);
        self.mem_map.write_u16(address, value);
    }

    pub fn write_u32(&mut self, address: u32, value: u32) {
        self.cycle_clock.update_cycles(address, MemAccessSize::Mem32);
        self.mem_map.write_u32(address, value);
    }
}