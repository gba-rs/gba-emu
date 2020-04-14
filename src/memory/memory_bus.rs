use crate::memory::memory_map::MemoryMap;
use crate::operations::timing::{CycleClock, MemAccessSize};
use crate::gamepak::BackupType;

#[derive(Debug, PartialEq)]
pub enum HaltState {
    Running,
    Halt,
    Stop
}

pub struct MemoryBus {
    pub mem_map: MemoryMap,
    pub cycle_clock: CycleClock,
    pub halt_state: HaltState,
    pub backup_type: BackupType,
    pub backed_up: bool
}

impl MemoryBus {
    pub fn new(backup_type: BackupType) -> MemoryBus {
        return MemoryBus {
            mem_map: MemoryMap::new(),
            cycle_clock: CycleClock::new(),
            halt_state: HaltState::Running,
            backup_type: backup_type,
            backed_up: false
        };
    }

    pub fn read_u8(&mut self, address: u32) -> u8 {
        self.cycle_clock.update_cycles(address, MemAccessSize::Mem8);


        if address >= 0x08000000 && address < 0x10000000 {
            // This should really be from the end of rom space to the top of memory
            match self.backup_type {
                BackupType::Sram => {/* don't need to do anything here */},
                BackupType::Eeprom => log::error!("Backup to EEPROM not implemented"),
                BackupType::Flash64K => log::error!("Backup to FLASH64K not implemented"),
                BackupType::Flash128K => {
                    if address == 0x0E000000 {
                        log::info!("Reading FLASH128_ stub");
                        return 0x62;
                    } else if address == 0x0E000001 {
                        log::info!("Reading FLASH128_ stub");
                        return 0x13;
                    }
                },
                BackupType::Error => log::error!("Backup to unknown"),
            }
        }

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

        if address < 0x00003FFF {
            log::error!("Writing to bios: {:X}", address);
            return;
        }

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

        if address < 0x00003FFF {
            log::error!("Writing to bios: {:X}", address);
            return;
        }

        self.mem_map.write_u16(address, value);
    }

    pub fn write_u32(&mut self, address: u32, value: u32) {
        self.cycle_clock.update_cycles(address, MemAccessSize::Mem32);

        if address < 0x00003FFF {
            // panic!("Writing to bios");
            log::error!("Writing to bios: {:X}", address);
            return;
        }

        self.mem_map.write_u32(address, value);
    }
}