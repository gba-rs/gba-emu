use crate::memory::system_control::WaitStateControl;
use crate::memory::GbaMem;
use std::rc::Rc;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CycleClock {
    pub prev_address: u32,
    pub cycles: u32,
    #[serde(skip)]
    pub wait_state_control: WaitStateControl,
    prefetch_next_address: u32,
    prefetch_credit_bytes: u32,
    prefetch_idle_cycle_carry: u32,
}

const PREFETCH_BUFFER_CAPACITY_BYTES: u32 = 16;

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
            prefetch_next_address: 0,
            prefetch_credit_bytes: 0,
            prefetch_idle_cycle_carry: 0,
        };
    }

    fn gamepak_rom_region(address: u32) -> Option<u8> {
        match address & 0xFF00_0000 {
            GAMEPAK_WS0_START | GAMEPAK_WS0_HI => Some(0),
            GAMEPAK_WS1_START | GAMEPAK_WS1_HI => Some(1),
            GAMEPAK_WS2_START | GAMEPAK_WS2_HI => Some(2),
            _ => None,
        }
    }

    fn seq_halfword_cost(&self, region: u8) -> u32 {
        let ws0_seq_cycles = [2, 1];
        let ws1_seq_cycles = [4, 1];
        let ws2_seq_cycles = [8, 1];
        match region {
            0 => ws0_seq_cycles[self.wait_state_control.get_wait_state_zero_second_access() as usize],
            1 => ws1_seq_cycles[self.wait_state_control.get_wait_state_one_second_access() as usize],
            _ => ws2_seq_cycles[self.wait_state_control.get_wait_state_two_second_access() as usize],
        }
    }

    fn access_size_bytes(access_size: MemAccessSize) -> u32 {
        match access_size {
            MemAccessSize::Mem8 => 1,
            MemAccessSize::Mem16 => 2,
            MemAccessSize::Mem32 => 4,
        }
    }

    /// GBATEK: the prefetch buffer only serves opcode fetches from GamePak ROM, never data reads.
    pub fn update_cycles_for_fetch(&mut self, address: u32, access_size: MemAccessSize) {
        let size_bytes = Self::access_size_bytes(access_size);
        let region = Self::gamepak_rom_region(address);

        if let Some(_) = region {
            let buffered_hit = self.wait_state_control.get_gamepak_prefetch_buffer() != 0
                && address == self.prefetch_next_address
                && self.prefetch_credit_bytes >= size_bytes;

            if buffered_hit {
                self.prefetch_credit_bytes -= size_bytes;
                self.prefetch_next_address = address + size_bytes;
                self.prev_address = address;
                return;
            }
        }

        self.update_cycles(address, access_size);
        self.prefetch_next_address = address + size_bytes;
        self.prefetch_credit_bytes = 0;
        self.prefetch_idle_cycle_carry = 0;
    }

    /// Idle bus time (non-ROM accesses) lets the prefetch unit fill ahead of the opcode stream.
    fn grow_prefetch_credit(&mut self, idle_cycles: u32) {
        if self.wait_state_control.get_gamepak_prefetch_buffer() == 0 {
            return;
        }
        let region = match Self::gamepak_rom_region(self.prefetch_next_address) {
            Some(region) => region,
            None => return,
        };
        if self.prefetch_credit_bytes >= PREFETCH_BUFFER_CAPACITY_BYTES {
            return;
        }
        let per_halfword = self.seq_halfword_cost(region).max(1);
        self.prefetch_idle_cycle_carry += idle_cycles;
        let new_halfwords = self.prefetch_idle_cycle_carry / per_halfword;
        self.prefetch_idle_cycle_carry %= per_halfword;
        self.prefetch_credit_bytes = (self.prefetch_credit_bytes + new_halfwords * 2).min(PREFETCH_BUFFER_CAPACITY_BYTES);
    }

    pub fn register(&mut self, mem: &Rc<GbaMem>) {
        self.wait_state_control.register(mem);
    }

    pub fn update_cycles(&mut self, address: u32, access_size: MemAccessSize) {
        let nonseq_cycles = [4, 3, 2, 8];
        let ws0_seq_cycles = [2, 1];
        let ws1_seq_cycles = [4, 1];
        let ws2_seq_cycles = [8, 1];
        let access_type = self.is_sequential(address, access_size);
        self.prev_address = address;
        let cycles_before = self.cycles;
        let is_rom_access = Self::gamepak_rom_region(address).is_some();
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
        if !is_rom_access {
            let charged = self.cycles - cycles_before;
            self.grow_prefetch_credit(charged);
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
            prefetch_next_address: 0,
            prefetch_credit_bytes: 0,
            prefetch_idle_cycle_carry: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::gba::GBA;
    use super::MemAccessSize;

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

    #[test]
    fn sequential_rom_fetch_is_free_after_enough_idle_cycles_when_prefetch_enabled() {
        let mut gba = GBA::default();
        gba.memory_bus.write_u16(0x4000204, 0x4000);
        gba.memory_bus.cycle_clock.get_cycles();

        gba.memory_bus.cycle_clock.update_cycles_for_fetch(0x0800_0000, MemAccessSize::Mem16);
        gba.memory_bus.cycle_clock.get_cycles();

        gba.memory_bus.read_u8(0x0300_0000);
        gba.memory_bus.read_u8(0x0300_0001);
        gba.memory_bus.cycle_clock.get_cycles();

        gba.memory_bus.cycle_clock.update_cycles_for_fetch(0x0800_0002, MemAccessSize::Mem16);
        assert_eq!(gba.memory_bus.cycle_clock.get_cycles(), 0);
    }

    #[test]
    fn sequential_rom_fetch_is_not_free_when_prefetch_disabled() {
        let mut gba = GBA::default();

        gba.memory_bus.cycle_clock.update_cycles_for_fetch(0x0800_0000, MemAccessSize::Mem16);
        gba.memory_bus.cycle_clock.get_cycles();

        gba.memory_bus.read_u8(0x0300_0000);
        gba.memory_bus.read_u8(0x0300_0001);
        gba.memory_bus.cycle_clock.get_cycles();

        gba.memory_bus.cycle_clock.update_cycles_for_fetch(0x0800_0002, MemAccessSize::Mem16);
        assert!(gba.memory_bus.cycle_clock.get_cycles() > 0);
    }

    #[test]
    fn non_sequential_rom_fetch_ignores_buffered_credit() {
        let mut gba = GBA::default();
        gba.memory_bus.write_u16(0x4000204, 0x4000);
        gba.memory_bus.cycle_clock.get_cycles();

        gba.memory_bus.cycle_clock.update_cycles_for_fetch(0x0800_0000, MemAccessSize::Mem16);
        gba.memory_bus.cycle_clock.get_cycles();

        gba.memory_bus.read_u8(0x0300_0000);
        gba.memory_bus.read_u8(0x0300_0001);
        gba.memory_bus.cycle_clock.get_cycles();

        gba.memory_bus.cycle_clock.update_cycles_for_fetch(0x0800_1000, MemAccessSize::Mem16);
        assert!(gba.memory_bus.cycle_clock.get_cycles() > 0);
    }

    #[test]
    fn prefetch_buffer_caps_at_eight_halfwords() {
        let mut gba = GBA::default();
        gba.memory_bus.write_u16(0x4000204, 0x4000);
        gba.memory_bus.cycle_clock.get_cycles();

        gba.memory_bus.cycle_clock.update_cycles_for_fetch(0x0800_0000, MemAccessSize::Mem16);
        gba.memory_bus.cycle_clock.get_cycles();

        for _ in 0..100 {
            gba.memory_bus.read_u8(0x0300_0000);
        }
        gba.memory_bus.cycle_clock.get_cycles();

        let mut free_fetches = 0;
        for i in 1..=9u32 {
            gba.memory_bus.cycle_clock.update_cycles_for_fetch(0x0800_0000 + i * 2, MemAccessSize::Mem16);
            if gba.memory_bus.cycle_clock.get_cycles() == 0 {
                free_fetches += 1;
            }
        }
        assert_eq!(free_fetches, 8);
    }
}
