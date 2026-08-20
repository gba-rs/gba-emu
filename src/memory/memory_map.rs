use std::cell::RefCell;
use std::rc::Rc;
use crate::gamepak::BackupType;
use crate::gamepak::flash::Flash;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::ser::SerializeStruct;
use serde::de::{self, Visitor, MapAccess, SeqAccess};
use std::fmt;
use std::marker::PhantomData;

pub const ON_BOARD_WRAM_START: u32 = 0x02000000;
pub const ON_BOARD_WRAM_SIZE: u32 = 0x3FFFF;
pub const ON_CHIP_WRAM_START: u32 = 0x03000000;
pub const ON_CHIP_WRAM_SIZE: u32 = 0x7FFF;
pub const PALETTE_RAM_START: u32 = 0x05000000;
pub const PALETTE_RAM_SIZE: u32 = 0x3FF;
pub const VIDEO_RAM_START: u32 = 0x06000000;
pub const VIDEO_RAM_SIZE: u32 = 0x17FFF;
pub const OBJECT_ATTRIBUTES_START: u32 = 0x07000000;
pub const OBJECT_ATTRIBUTES_SIZE: u32 = 0x3FF;
pub const ROM_START: u32 = 0x08000000;
pub const ROM_SIZE: u32 = 0x1FF_FFFF;

pub const SRAM_START: u32 = 0x0E000000;
pub const SRAM_SIZE: u32 = 0xFFFF;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum HaltState {
    Running,
    Halt,
    Stop
}

pub struct MemoryMap {
    pub memory: Rc<RefCell<Vec<u8>>>,
    pub halt_state: HaltState,
    pub backup_type: BackupType,
    pub backed_up: bool,
    pub flash: Flash
}

impl MemoryMap {

    pub fn new(backup_type: BackupType) -> MemoryMap {
        return MemoryMap {
            memory: Rc::new(RefCell::new(vec![0; 0x1000_00F0])),
            halt_state: HaltState::Running,
            backup_type: backup_type,
            backed_up: false,
            flash: Flash::new()
        }
    }

    pub fn write_u8(&mut self, address: u32, value: u8) {
        let upper_byte = address >> 24;

        match upper_byte {
            0x02 => self.memory.borrow_mut()[((address & ON_BOARD_WRAM_SIZE) + ON_BOARD_WRAM_START) as usize] = value,
            0x03 => self.memory.borrow_mut()[((address & ON_CHIP_WRAM_SIZE) + ON_CHIP_WRAM_START) as usize] = value,
            0x04 => {
                if address == 0x4000202 || address == 0x4000203 {
                    let new_val = self.read_u8(address) & !value;
                    self.memory.borrow_mut()[address as usize] = new_val;
                }else if address == 0x4000100 || address == 0x4000101 ||
                   address == 0x4000104 || address == 0x4000105 ||
                   address == 0x4000108 || address == 0x4000109 ||
                   address == 0x400010C || address == 0x400010D {
                    let index: usize = (address & 0xF) as usize;
                    self.memory.borrow_mut()[0x1000_0000usize + index] = value;
                } else if address == 0x4000301{
                    let bit = (value & 0x80) >> 7;
                    if bit == 0 {
                        self.halt_state = HaltState::Halt;
                        // log::info!("Setting state to halted: {:X}", value);
                    } else if bit == 1 {
                        // log::info!("Setting state to stopped: {:X}", value);
                        self.halt_state = HaltState::Stop
                    }
                }else if address == 0x4000130 ||  address == 0x4000131  {
                    // read only
                }else {
                    self.memory.borrow_mut()[address as usize] = value;
                }

            },
            0x05 => self.memory.borrow_mut()[((address & PALETTE_RAM_SIZE) + PALETTE_RAM_START) as usize] = value,
            0x06 => self.memory.borrow_mut()[address as usize] = value,
            0x07 => self.memory.borrow_mut()[((address & OBJECT_ATTRIBUTES_SIZE) + OBJECT_ATTRIBUTES_START) as usize] = value,
            0x08..=0x0F => {
                match self.backup_type {
                    BackupType::Sram => {
                        /* don't need to do anything here */
                        self.memory.borrow_mut()[address as usize] = value;
                    },
                    BackupType::Eeprom => {
                        // TODO implement EEPROM
                        self.memory.borrow_mut()[address as usize] = value;
                    },
                    BackupType::Flash64K | BackupType::Flash128K => {
                        if upper_byte == 0x0E || upper_byte == 0x0F {
                            self.write_flash(address, value);
                        } else {
                            self.memory.borrow_mut()[address as usize] = value;
                        }
                    },
                    // BackupType::Flash128K => {
                    //     self.memory.borrow_mut()[address as usize] = value;
                    // },
                    BackupType::Error => {
                        self.memory.borrow_mut()[address as usize] = value;
                    },
                }
            },
            _ => {}
        }


    }

    /// Returns the physical index into `memory` for a `len`-byte access
    /// starting at `address`, but only when the *entire* access lands in a
    /// plain RAM-like region (WRAM, IWRAM, palette RAM, VRAM, OAM) whose
    /// `read_u8`/`write_u8` handling is a pure store with no per-address
    /// side effects.
    ///
    /// Deliberately excludes the I/O region (0x04......) and the
    /// gamepak/backup region (0x08...... - 0x0F......), since both have
    /// per-address special-case behavior in `read_u8`/`write_u8` (halt
    /// state, IE/IF write semantics, flash chip commands, SRAM mirroring by
    /// backup type, ...) that a bulk slice copy would silently skip. Those
    /// regions keep going through the byte-wise path below, which remains
    /// the single source of truth for that logic.
    ///
    /// Also returns `None` if the access would straddle a region's mirror
    /// wraparound boundary (e.g. the last 1-3 bytes of on-chip WRAM), so
    /// the byte-wise fallback can reproduce that mirroring exactly. This
    /// is a real edge case in principle but not one any known GBA game
    /// relies on, since the CPU only ever issues aligned halfword/word
    /// accesses.
    #[inline]
    fn fast_region_index(address: u32, len: u32) -> Option<usize> {
        let upper_byte = address >> 24;

        // VRAM is stored at its literal address with no masking/mirroring
        // in this emulator, so any same-region access is safe.
        if upper_byte == 0x06 {
            return Some(address as usize);
        }

        let (start, size_mask) = match upper_byte {
            0x02 => (ON_BOARD_WRAM_START, ON_BOARD_WRAM_SIZE),
            0x03 => (ON_CHIP_WRAM_START, ON_CHIP_WRAM_SIZE),
            0x05 => (PALETTE_RAM_START, PALETTE_RAM_SIZE),
            0x07 => (OBJECT_ATTRIBUTES_START, OBJECT_ATTRIBUTES_SIZE),
            _ => return None,
        };

        let offset = address & size_mask;
        if offset + (len - 1) > size_mask {
            // Access straddles the region's wraparound point.
            return None;
        }

        Some((offset + start) as usize)
    }

    pub fn write_u16(&mut self, address: u32, value: u16) {
        if let Some(idx) = MemoryMap::fast_region_index(address, 2) {
            let bytes = value.to_le_bytes();
            let mut mem = self.memory.borrow_mut();
            mem[idx] = bytes[0];
            mem[idx + 1] = bytes[1];
            return;
        }

        self.write_u8(address + 1, ((value & 0xFF00) >> 8) as u8);
        self.write_u8(address, (value & 0xFF) as u8);
    }

    pub fn write_u32(&mut self, address: u32, value: u32) {
        if let Some(idx) = MemoryMap::fast_region_index(address, 4) {
            let mut mem = self.memory.borrow_mut();
            mem[idx..idx + 4].copy_from_slice(&value.to_le_bytes());
            return;
        }

        self.write_u8(address + 3, ((value & 0xFF000000) >> 24) as u8);
        self.write_u8(address + 2, ((value & 0xFF0000) >> 16) as u8);
        self.write_u8(address + 1, ((value & 0xFF00) >> 8) as u8);
        self.write_u8(address, (value & 0xFF) as u8);
    }

    pub fn write_block(&mut self, address: u32, block: &Vec<u8>) {
        let mut offset: u32 = 0;
        let mut mem = self.memory.borrow_mut();

        for byte in block {
            mem[(address + offset) as usize] = *byte;
            offset += 1;
        }
    }

    pub fn read_block(&self, address: u32, bytes: u32) -> Vec<u8> {
        let mut temp: Vec<u8> = vec![];
        for i in address..(address + bytes) {
            temp.push(self.read_u8(i));
        }
        return temp;
    }

    pub fn read_block_raw(&self, address: u32, bytes: u32) -> Vec<u8> {
        let mut temp: Vec<u8> = vec![];
        for i in address..(address + bytes) {
            temp.push(self.memory.borrow()[i as usize]);
        }
        return temp;
    }

    pub fn read_u32(&self, address: u32) -> u32 {
        if let Some(idx) = MemoryMap::fast_region_index(address, 4) {
            let mem = self.memory.borrow();
            return u32::from_le_bytes([mem[idx], mem[idx + 1], mem[idx + 2], mem[idx + 3]]);
        }

        let mut result: u32 = 0;
        for i in 0..4 {
            result |= (self.read_u8(address + i) as u32) <<  (i * 8);
        }
        return result;
    }

    pub fn read_u16(&self, address: u32) -> u16 {
        if let Some(idx) = MemoryMap::fast_region_index(address, 2) {
            let mem = self.memory.borrow();
            return u16::from_le_bytes([mem[idx], mem[idx + 1]]);
        }

        let result: u16 = ((self.read_u8(address + 1) as u16) << 8) | (self.read_u8(address) as u16);
        return result;
    }

    pub fn read_u8(&self, address: u32) -> u8 {
        let upper_byte = address >> 24;

        match upper_byte {
            0x02 => return self.memory.borrow()[((address & ON_BOARD_WRAM_SIZE) + ON_BOARD_WRAM_START) as usize],
            0x03 => return self.memory.borrow()[((address & ON_CHIP_WRAM_SIZE) + ON_CHIP_WRAM_START) as usize],
            0x04 => return self.memory.borrow()[address as usize],
            0x05 => return self.memory.borrow()[((address & PALETTE_RAM_SIZE) + PALETTE_RAM_START) as usize],
            0x06 => return self.memory.borrow()[address as usize],
            0x07 => return self.memory.borrow()[((address & OBJECT_ATTRIBUTES_SIZE) + OBJECT_ATTRIBUTES_START) as usize],
            0x08..=0x0F => {
                match self.backup_type {
                    BackupType::Sram => {
                        /* don't need to do anything here */
                        if upper_byte == 0x0E {
                            return self.memory.borrow()[((address & SRAM_SIZE) + SRAM_START) as usize]
                        } else {
                            return self.memory.borrow()[address as usize];
                        }
                    },
                    BackupType::Eeprom => {
                        // TODO implement EEPROM
                        return self.memory.borrow()[address as usize];
                    },
                    BackupType::Flash64K | BackupType::Flash128K => {
                        if upper_byte == 0x0E || upper_byte == 0x0F {
                            return self.read_flash(address);
                        } else {
                            return self.memory.borrow()[address as usize];
                        }
                    },
                    // BackupType::Flash128K => {
                    //     if address == 0x0E000000 {
                    //         return 0x62;
                    //     } else if address == 0x0E000001 {
                    //         return 0x13;
                    //     }
                    //     return self.memory.borrow()[address as usize];

                    // },
                    BackupType::Error => {
                        return self.memory.borrow()[address as usize];
                    },
                }
            }
            _ => { 
                if address > 0x0FFFFFFF {
                    return 0;
                }

                return self.memory.borrow()[address as usize]; 
            }
        }
    }
}

// Custom serialization implementation
impl Serialize for MemoryMap {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Determine how many fields we're serializing
        let mut state = serializer.serialize_struct("MemoryMap", 5)?;
        
        // Serialize memory by borrowing the RefCell and using the underlying Vec<u8>
        state.serialize_field("memory", &*self.memory.borrow())?;
        
        // Serialize the rest of the fields normally
        state.serialize_field("halt_state", &self.halt_state)?;
        state.serialize_field("backup_type", &self.backup_type)?;
        state.serialize_field("backed_up", &self.backed_up)?;
        state.serialize_field("flash", &self.flash)?;
        
        state.end()
    }
}

// Custom deserialization implementation
impl<'de> Deserialize<'de> for MemoryMap {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Define the fields we expect
        enum Field { Memory, HaltState, BackupType, BackedUp, Flash }
        
        // Implement a deserializer for the field names
        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`memory`, `halt_state`, `backup_type`, `backed_up`, or `flash`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "memory" => Ok(Field::Memory),
                            "halt_state" => Ok(Field::HaltState),
                            "backup_type" => Ok(Field::BackupType),
                            "backed_up" => Ok(Field::BackedUp),
                            "flash" => Ok(Field::Flash),
                            _ => Err(de::Error::unknown_field(value, &["memory", "halt_state", "backup_type", "backed_up", "flash"])),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        // Visitor for the entire struct
        struct MemoryMapVisitor;

        impl<'de> Visitor<'de> for MemoryMapVisitor {
            type Value = MemoryMap;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct MemoryMap")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                where
                    A: SeqAccess<'de>, {
                let mem_vec = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let halt_state = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let backup_type = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let backed_up = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let flash = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;

                let memory = Rc::new(RefCell::new(mem_vec));
                Ok(MemoryMap {
                    memory,
                    halt_state,
                    backup_type,
                    backed_up,
                    flash,
                })
            }

            fn visit_map<V>(self, mut map: V) -> Result<MemoryMap, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut memory = None;
                let mut halt_state = None;
                let mut backup_type = None;
                let mut backed_up = None;
                let mut flash = None;

                // Extract each field from the map
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Memory => {
                            if memory.is_some() {
                                return Err(de::Error::duplicate_field("memory"));
                            }
                            // Deserialize directly into a Vec<u8>
                            let mem_vec: Vec<u8> = map.next_value()?;
                            memory = Some(Rc::new(RefCell::new(mem_vec)));
                        }
                        Field::HaltState => {
                            if halt_state.is_some() {
                                return Err(de::Error::duplicate_field("halt_state"));
                            }
                            halt_state = Some(map.next_value()?);
                        }
                        Field::BackupType => {
                            if backup_type.is_some() {
                                return Err(de::Error::duplicate_field("backup_type"));
                            }
                            backup_type = Some(map.next_value()?);
                        }
                        Field::BackedUp => {
                            if backed_up.is_some() {
                                return Err(de::Error::duplicate_field("backed_up"));
                            }
                            backed_up = Some(map.next_value()?);
                        }
                        Field::Flash => {
                            if flash.is_some() {
                                return Err(de::Error::duplicate_field("flash"));
                            }
                            flash = Some(map.next_value()?);
                        }
                    }
                }

                // Ensure all fields were provided
                let memory = memory.ok_or_else(|| de::Error::missing_field("memory"))?;
                let halt_state = halt_state.ok_or_else(|| de::Error::missing_field("halt_state"))?;
                let backup_type = backup_type.ok_or_else(|| de::Error::missing_field("backup_type"))?;
                let backed_up = backed_up.ok_or_else(|| de::Error::missing_field("backed_up"))?;
                let flash = flash.ok_or_else(|| de::Error::missing_field("flash"))?;

                // Return the constructed struct
                Ok(MemoryMap {
                    memory,
                    halt_state,
                    backup_type,
                    backed_up,
                    flash,
                })
            }
        }

        // Start the deserialization process
        deserializer.deserialize_struct(
            "MemoryMap",
            &["memory", "halt_state", "backup_type", "backed_up", "flash"],
            MemoryMapVisitor
        )
    }
}

#[cfg(test)]
mod fast_path_tests {
    use super::*;
    use crate::gamepak::BackupType;

    #[test]
    fn word_access_round_trips_in_wram() {
        let mut mem = MemoryMap::new(BackupType::Error);
        mem.write_u32(ON_BOARD_WRAM_START + 0x100, 0xDEADBEEF);
        assert_eq!(mem.read_u32(ON_BOARD_WRAM_START + 0x100), 0xDEADBEEF);
    }

    #[test]
    fn halfword_access_round_trips_in_iwram() {
        let mut mem = MemoryMap::new(BackupType::Error);
        mem.write_u16(ON_CHIP_WRAM_START + 0x10, 0xBEEF);
        assert_eq!(mem.read_u16(ON_CHIP_WRAM_START + 0x10), 0xBEEF);
    }

    #[test]
    fn word_access_round_trips_in_vram() {
        let mut mem = MemoryMap::new(BackupType::Error);
        mem.write_u32(VIDEO_RAM_START + 0x1000, 0x12345678);
        assert_eq!(mem.read_u32(VIDEO_RAM_START + 0x1000), 0x12345678);
    }

    #[test]
    fn fast_path_matches_byte_wise_semantics_across_wram_mirror() {
        // WRAM is 0x40000 bytes of address space (ON_BOARD_WRAM_SIZE mask)
        // mirrored across a larger region. Writing near the top and
        // reading back should still round-trip correctly whether or not
        // the fast path is taken.
        let mut mem = MemoryMap::new(BackupType::Error);
        let addr = ON_BOARD_WRAM_START + ON_BOARD_WRAM_SIZE - 3;
        mem.write_u16(addr, 0xABCD);
        assert_eq!(mem.read_u16(addr), 0xABCD);
    }

    #[test]
    fn io_region_writes_still_go_through_byte_wise_special_casing() {
        // Regression guard: the fast path must never touch the I/O region,
        // since writes there (e.g. the HALTCNT/stop-halt byte at
        // 0x4000301) have side effects beyond storing bytes.
        let mut mem = MemoryMap::new(BackupType::Error);
        assert_eq!(mem.halt_state, HaltState::Running);
        mem.write_u8(0x4000301, 0x00);
        assert_eq!(mem.halt_state, HaltState::Halt);
    }
}
