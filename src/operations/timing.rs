use crate::memory::system_control::WaitStateControl;
use std::cell::RefCell;
use std::rc::Rc;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CycleClock {
    pub prev_address: u32,
    pub cycles: u32,
    #[serde(skip)]
    pub wait_state_control: WaitStateControl,
}

pub const BIOS_START: u32 = 0x0000_0000;
pub const EWRAM_START: u32 = 0x0200_0000;
pub const IWRAM_START: u32 = 0x0300_0000;
pub const IOMEM_START: u32 = 0x0400_0000;
pub const PALRAM_START: u32 = 0x0500_0000;
pub const VRAM_START: u32 = 0x0600_0000;
pub const OAM_START: u32 = 0x0700_0000;
pub const GAMEPAK_WS0_START: u32 = 0x0800_0000;
pub const GAMEPAK_WS0_HI: u32 = 0x0900_0000;
pub const GAMEPAK_WS1_START: u32 = 0x0A00_0000;
pub const GAMEPAK_WS1_HI: u32 = 0x0B00_0000;
pub const GAMEPAK_WS2_START: u32 = 0x0C00_0000;
pub const GAMEPAK_WS2_HI: u32 = 0x0D00_0000;

#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum MemAccessSize {
    Mem8,
    Mem16,
    Mem32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum CycleType {
    N,
    S,
}

impl CycleClock {
    pub fn new() -> CycleClock {
        return CycleClock {
            prev_address: 0,
            cycles: 0,
            wait_state_control: WaitStateControl::new(),
        };
    }

    pub fn register(&mut self, mem: &Rc<RefCell<Vec<u8>>>) {
        self.wait_state_control.register(mem);
    }

    pub fn update_cycles(&mut self, address: u32, access_size: MemAccessSize) {
        let nonseq_cycles = [4, 3, 2, 8];
        let ws0_seq_cycles = [2, 1];
        let ws1_seq_cycles = [4, 1];
        let ws2_seq_cycles = [8, 1];
        let access_type = self.is_sequential(address, access_size);
        self.prev_address = address;
        match address & 0xFF00_0000 {
            BIOS_START | IWRAM_START | IOMEM_START => self.cycles += 1,
            EWRAM_START => {
                // Might need to revisit this in relation to wait states
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
            GAMEPAK_WS0_START | GAMEPAK_WS0_HI => {
                match access_type {
                    CycleType::N => {
                        self.cycles += nonseq_cycles[self.wait_state_control.get_wait_state_zero_first_access() as usize];
                        if access_size == MemAccessSize::Mem32 {
                            self.cycles += ws0_seq_cycles[self.wait_state_control.get_wait_state_zero_second_access() as usize];
                        }
                    }
                    CycleType::S => {
                        self.cycles += ws0_seq_cycles[self.wait_state_control.get_wait_state_zero_second_access() as usize];
                        if access_size == MemAccessSize::Mem32 {
                            self.cycles += ws0_seq_cycles[self.wait_state_control.get_wait_state_zero_second_access() as usize];
                        }
                    }
                }
            }
            GAMEPAK_WS1_START | GAMEPAK_WS1_HI => {
                match access_type {
                    CycleType::N => {
                        self.cycles += nonseq_cycles[self.wait_state_control.get_wait_state_one_first_access() as usize];
                        if access_size == MemAccessSize::Mem32 {
                            self.cycles += ws1_seq_cycles[self.wait_state_control.get_wait_state_one_second_access() as usize];
                        }
                    }
                    CycleType::S => {
                        self.cycles += ws1_seq_cycles[self.wait_state_control.get_wait_state_one_second_access() as usize];
                        if access_size == MemAccessSize::Mem32 {
                            self.cycles += ws1_seq_cycles[self.wait_state_control.get_wait_state_one_second_access() as usize];
                        }
                    }
                }
            }
            GAMEPAK_WS2_START | GAMEPAK_WS2_HI => {
                match access_type {
                    CycleType::N => {
                        self.cycles += nonseq_cycles[self.wait_state_control.get_wait_state_two_first_access() as usize];
                        if access_size == MemAccessSize::Mem32 {
                            self.cycles += ws2_seq_cycles[self.wait_state_control.get_wait_state_two_second_access() as usize];
                        }
                    }
                    CycleType::S => {
                        self.cycles += ws2_seq_cycles[self.wait_state_control.get_wait_state_two_second_access() as usize];
                        if access_size == MemAccessSize::Mem32 {
                            self.cycles += ws2_seq_cycles[self.wait_state_control.get_wait_state_two_second_access() as usize];
                        }
                    }
                }
            }
            _ => { }//log::error!("Trying to read unknown address: {:X}", address) }
        }
    }

    pub fn get_cycles(&mut self) -> u32 {
        let temp = self.cycles;
        self.cycles = 0;
        return temp;
    }

    pub fn is_sequential(&self, address: u32, access_size: MemAccessSize) -> CycleType {
        let address_diff;
        match access_size {
            MemAccessSize::Mem8 => address_diff = 1,
            MemAccessSize::Mem16 => address_diff = 2,
            MemAccessSize::Mem32 => address_diff = 4
        }
        if (address as i64 - self.prev_address as i64) == address_diff {
            return CycleType::S;
        }
        return CycleType::N;
    }
}

impl Default for CycleClock {
    fn default() -> Self {
        CycleClock {
            prev_address: 0,
            cycles: 0,
            wait_state_control: WaitStateControl::new(),
        }
    }
}

#[cfg(test)]
mod tests {
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

