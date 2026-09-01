use std::cell::{Cell, RefCell};
use std::rc::Rc;
use crate::gamepak::BackupType;
use crate::gamepak::flash::Flash;
use crate::memory::eeprom::Eeprom;
use crate::memory::GbaMem;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::ser::SerializeStruct;
use serde::de::{self, Visitor, MapAccess, SeqAccess};
use std::fmt;
use std::marker::PhantomData;

thread_local! {
    pub static CURRENT_INSTR_PC: std::cell::Cell<u32> = std::cell::Cell::new(0);
    pub static CURRENT_INSTR_IS_THUMB: std::cell::Cell<bool> = std::cell::Cell::new(false);
    pub static DMA_BUS_OVERRIDE: std::cell::Cell<Option<u32>> = std::cell::Cell::new(None);
}

pub const BIOS_SIZE: u32 = 0x4000;

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
    pub memory: Rc<GbaMem>,
    pub halt_state: HaltState,
    pub backup_type: BackupType,
    pub backed_up: bool,
    pub flash: Flash,
    pub eeprom: RefCell<Eeprom>,
    pub fifo_a: std::collections::VecDeque<u8>,
    pub fifo_b: std::collections::VecDeque<u8>,
    pub trigger_flags: u8,
    rom_size: u32,
    rom_undersize_mirror_mask: Option<u32>,
    wave_ram_banks: [[u8; 16]; 2],
}

impl MemoryMap {

    pub fn new(backup_type: BackupType) -> MemoryMap {
        let mut memory: GbaMem = vec![Cell::new(0u8); 0x1000_00F0];
        let backup_start = SRAM_START as usize;
        let backup_end = backup_start + 0x20000;
        memory[backup_start..backup_end].fill(Cell::new(0xFF));

        return MemoryMap {
            memory: Rc::new(memory),
            halt_state: HaltState::Running,
            backup_type: backup_type,
            backed_up: false,
            flash: Flash::new(),
            eeprom: RefCell::new(Eeprom::new()),
            fifo_a: std::collections::VecDeque::new(),
            fifo_b: std::collections::VecDeque::new(),
            trigger_flags: 0,
            rom_size: ROM_SIZE + 1,
            rom_undersize_mirror_mask: None,
            wave_ram_banks: [[0; 16]; 2],
        }
    }

    pub fn read_wave_ram_byte(&self, bank: u8, offset: u32) -> u8 {
        self.wave_ram_banks[(bank & 1) as usize][(offset & 0xF) as usize]
    }

    fn write_wave_ram_byte(&mut self, bank: u8, offset: u32, value: u8) {
        self.wave_ram_banks[(bank & 1) as usize][(offset & 0xF) as usize] = value;
    }

    fn cpu_visible_wave_ram_bank(&self) -> u8 {
        let sound3cnt_l = self.memory[0x0400_0070usize].get();
        ((sound3cnt_l >> 6) & 1) ^ 1
    }

    pub fn prepare_eeprom_write(&self, halfword_count: u32) {
        if self.backup_type == BackupType::Eeprom {
            self.eeprom.borrow_mut().prepare_write_transfer(halfword_count);
        }
    }

    pub fn prepare_eeprom_read(&self, halfword_count: u32) {
        if self.backup_type == BackupType::Eeprom {
            self.eeprom.borrow_mut().prepare_read_transfer(halfword_count);
        }
    }

    pub fn write_u8(&mut self, address: u32, value: u8) {
        let upper_byte = address >> 24;

        match upper_byte {
            0x02 => self.memory[((address & ON_BOARD_WRAM_SIZE) + ON_BOARD_WRAM_START) as usize].set(value),
            0x03 => self.memory[((address & ON_CHIP_WRAM_SIZE) + ON_CHIP_WRAM_START) as usize].set(value),
            0x04 => {
                if (0x4000060..=0x4000081).contains(&address) && !self.sound_master_enabled() {
                    return;
                }
                if address == 0x4000202 || address == 0x4000203 {
                    let new_val = self.read_u8(address) & !value;
                    self.memory[address as usize].set(new_val);
                }else if address == 0x4000100 || address == 0x4000101 ||
                   address == 0x4000104 || address == 0x4000105 ||
                   address == 0x4000108 || address == 0x4000109 ||
                   address == 0x400010C || address == 0x400010D {
                    let index: usize = (address & 0xF) as usize;
                    self.memory[0x1000_0000usize + index].set(value);
                } else if address == 0x4000301{
                    let bit = (value & 0x80) >> 7;
                    if bit == 0 {
                        self.halt_state = HaltState::Halt;
                    } else if bit == 1 {
                        self.halt_state = HaltState::Stop
                    }
                }else if address == 0x4000130 ||  address == 0x4000131  {
                    // read only
                }else if address == 0x4000084 {
                    self.memory[address as usize].set(value);
                    if value & 0x80 == 0 {
                        for cleared in 0x4000060u32..=0x4000081 {
                            self.memory[cleared as usize].set(0);
                        }
                    }
                }else if address == 0x4000065 || address == 0x400006D || address == 0x4000075 || address == 0x400007D {
                    if value & 0x80 != 0 {
                        let channel_bit = match address {
                            0x4000065 => 0x1,
                            0x400006D => 0x2,
                            0x4000075 => 0x4,
                            _ => 0x8,
                        };
                        self.trigger_flags |= channel_bit;
                    }
                    self.memory[address as usize].set(value);
                }else if (0x0400_0090..=0x0400_009F).contains(&address) {
                    let bank = self.cpu_visible_wave_ram_bank();
                    self.write_wave_ram_byte(bank, address - 0x0400_0090, value);
                }else if (0x4000_00A0..=0x4000_00A3).contains(&address) {
                    if self.fifo_a.len() < 32 {
                        self.fifo_a.push_back(value);
                    }
                    self.memory[address as usize].set(value);
                }else if (0x4000_00A4..=0x4000_00A7).contains(&address) {
                    if self.fifo_b.len() < 32 {
                        self.fifo_b.push_back(value);
                    }
                    self.memory[address as usize].set(value);
                }else {
                    self.memory[address as usize].set(value);
                }

            },
            0x05 => {
                let halfword_addr = ((address & PALETTE_RAM_SIZE) + PALETTE_RAM_START) & !1;
                self.memory[halfword_addr as usize].set(value);
                self.memory[(halfword_addr + 1) as usize].set(value);
            },
            0x06 => {
                let mirrored = Self::vram_mirrored_address(address);
                if mirrored - VIDEO_RAM_START >= self.vram_obj_boundary() {
                } else {
                    let halfword_addr = mirrored & !1;
                    self.memory[halfword_addr as usize].set(value);
                    self.memory[(halfword_addr + 1) as usize].set(value);
                }
            },
            0x07 => {
            },
            0x08..=0x0F => {
                match self.backup_type {
                    BackupType::Sram => {
                        if upper_byte == 0x0E || upper_byte == 0x0F {
                            self.memory[((address & SRAM_SIZE) + SRAM_START) as usize].set(value);
                        } else {
                            self.memory[self.rom_mirrored_address(address) as usize].set(value);
                        }
                    },
                    BackupType::Eeprom => {
                        if upper_byte == 0x0D {
                            self.eeprom.borrow_mut().write_bit(value as u16);
                        } else {
                            self.memory[address as usize].set(value);
                        }
                    },
                    BackupType::Flash64K | BackupType::Flash128K => {
                        if upper_byte == 0x0E || upper_byte == 0x0F {
                            self.write_flash(address, value);
                        } else {
                            self.memory[address as usize].set(value);
                        }
                    },
                    BackupType::Error => {
                        self.memory[address as usize].set(value);
                    },
                }
            },
            _ => {}
        }


    }

    #[inline]
    fn fast_region_index(address: u32, len: u32) -> Option<usize> {
        let upper_byte = address >> 24;

        if upper_byte == 0x06 {
            return Some(Self::vram_mirrored_address(address) as usize);
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
            return None;
        }

        Some((offset + start) as usize)
    }

    #[inline]
    fn has_narrow_backup_bus(&self, upper_byte: u32) -> bool {
        matches!(self.backup_type, BackupType::Sram | BackupType::Flash64K | BackupType::Flash128K)
            && (upper_byte == 0x0E || upper_byte == 0x0F)
    }

    #[inline]
    fn rom_mirrored_address(&self, address: u32) -> u32 {
        let offset = address & ROM_SIZE;
        if offset >= self.rom_size {
            if let Some(mask) = self.rom_undersize_mirror_mask {
                let mirrored = offset & mask;
                if mirrored < self.rom_size {
                    return mirrored + ROM_START;
                }
            }
        }
        offset + ROM_START
    }

    pub fn configure_rom(&mut self, rom_len: usize, game_code: &str) {
        self.rom_size = rom_len as u32;
        if game_code.starts_with('F') && rom_len > 0 {
            self.rom_undersize_mirror_mask = Some((rom_len as u32).next_power_of_two() - 1);
        } else {
            self.rom_undersize_mirror_mask = None;
        }
    }

    #[inline]
    fn sound_master_enabled(&self) -> bool {
        self.memory[0x4000084].get() & 0x80 != 0
    }

    #[inline]
    fn vram_obj_boundary(&self) -> u32 {
        let bg_mode = self.memory[0x04000000].get() & 0x7;
        if bg_mode >= 3 { 0x14000 } else { 0x10000 }
    }

    #[inline]
    fn vram_mirrored_address(address: u32) -> u32 {
        let block_offset = address & 0x1FFFF;
        let block_offset = if block_offset >= 0x18000 { block_offset - 0x8000 } else { block_offset };
        VIDEO_RAM_START + block_offset
    }

    pub fn write_u16(&mut self, address: u32, value: u16) {
        if let Some(idx) = MemoryMap::fast_region_index(address, 2) {
            let bytes = value.to_le_bytes();
            self.memory[idx].set(bytes[0]);
            self.memory[idx + 1].set(bytes[1]);
            return;
        }

        let upper_byte = address >> 24;
        if self.backup_type == BackupType::Eeprom && upper_byte == 0x0D {
            self.eeprom.borrow_mut().write_bit(value);
            return;
        }

        if self.has_narrow_backup_bus(upper_byte) {
            let byte = if address & 1 == 0 { (value & 0xFF) as u8 } else { ((value >> 8) & 0xFF) as u8 };
            self.write_u8(address, byte);
            return;
        }

        self.write_u8(address + 1, ((value & 0xFF00) >> 8) as u8);
        self.write_u8(address, (value & 0xFF) as u8);
    }

    pub fn write_u32(&mut self, address: u32, value: u32) {
        if let Some(idx) = MemoryMap::fast_region_index(address, 4) {
            let bytes = value.to_le_bytes();
            for i in 0..4 {
                self.memory[idx + i].set(bytes[i]);
            }
            return;
        }

        let upper_byte = address >> 24;
        if self.has_narrow_backup_bus(upper_byte) {
            let lane = address & 3;
            let byte = ((value >> (lane * 8)) & 0xFF) as u8;
            self.write_u8(address, byte);
            return;
        }

        self.write_u8(address + 3, ((value & 0xFF000000) >> 24) as u8);
        self.write_u8(address + 2, ((value & 0xFF0000) >> 16) as u8);
        self.write_u8(address + 1, ((value & 0xFF00) >> 8) as u8);
        self.write_u8(address, (value & 0xFF) as u8);
    }

    pub fn write_block(&mut self, address: u32, block: &Vec<u8>) {
        let mut offset: u32 = 0;

        for byte in block {
            self.memory[(address + offset) as usize].set(*byte);
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
            temp.push(self.memory[i as usize].get());
        }
        return temp;
    }

    pub fn read_u32(&self, address: u32) -> u32 {
        if let Some(idx) = MemoryMap::fast_region_index(address, 4) {
            let result = u32::from_le_bytes([
                self.memory[idx].get(), self.memory[idx + 1].get(),
                self.memory[idx + 2].get(), self.memory[idx + 3].get(),
            ]);
            return result;
        }

        let upper_byte = address >> 24;
        if self.has_narrow_backup_bus(upper_byte) {
            let byte = self.read_u8(address) as u32;
            return byte * 0x0101_0101;
        }

        let mut result: u32 = 0;
        for i in 0..4 {
            result |= (self.read_u8(address + i) as u32) <<  (i * 8);
        }
        return result;
    }

    pub fn read_u16(&self, address: u32) -> u16 {
        if let Some(idx) = MemoryMap::fast_region_index(address, 2) {
            let result = u16::from_le_bytes([self.memory[idx].get(), self.memory[idx + 1].get()]);
            return result;
        }

        let upper_byte = address >> 24;
        if self.backup_type == BackupType::Eeprom && upper_byte == 0x0D {
            return self.eeprom.borrow_mut().read_bit();
        }

        if self.has_narrow_backup_bus(upper_byte) {
            let byte = self.read_u8(address) as u16;
            return byte | (byte << 8);
        }

        let result: u16 = ((self.read_u8(address + 1) as u16) << 8) | (self.read_u8(address) as u16);
        return result;
    }

    pub fn read_u8(&self, address: u32) -> u8 {
        let upper_byte = address >> 24;

        match upper_byte {
            0x02 => return self.memory[((address & ON_BOARD_WRAM_SIZE) + ON_BOARD_WRAM_START) as usize].get(),
            0x03 => return self.memory[((address & ON_CHIP_WRAM_SIZE) + ON_CHIP_WRAM_START) as usize].get(),
            0x04 => {
                if (0x0400_0090..=0x0400_009F).contains(&address) {
                    let bank = self.cpu_visible_wave_ram_bank();
                    return self.read_wave_ram_byte(bank, address - 0x0400_0090);
                }
                if address == 0x4000082 {
                    return self.memory[address as usize].get() & 0x0F;
                }
                if address == 0x4000083 {
                    return self.memory[address as usize].get() & 0x77;
                }
                if address >= 0x0400_0410 {
                    let open_bus = self.general_open_bus();
                    return ((open_bus >> ((address & 3) * 8)) & 0xFF) as u8;
                }
                return self.memory[address as usize].get();
            },
            0x05 => return self.memory[((address & PALETTE_RAM_SIZE) + PALETTE_RAM_START) as usize].get(),
            0x06 => return self.memory[Self::vram_mirrored_address(address) as usize].get(),
            0x07 => return self.memory[((address & OBJECT_ATTRIBUTES_SIZE) + OBJECT_ATTRIBUTES_START) as usize].get(),
            0x08..=0x0F => {
                match self.backup_type {
                    BackupType::Sram => {
                        if upper_byte == 0x0E || upper_byte == 0x0F {
                            return self.memory[((address & SRAM_SIZE) + SRAM_START) as usize].get()
                        } else {
                            return self.memory[self.rom_mirrored_address(address) as usize].get();
                        }
                    },
                    BackupType::Eeprom => {
                        if upper_byte == 0x0D {
                            return self.eeprom.borrow_mut().read_bit() as u8;
                        } else {
                            return self.memory[self.rom_mirrored_address(address) as usize].get();
                        }
                    },
                    BackupType::Flash64K | BackupType::Flash128K => {
                        if upper_byte == 0x0E || upper_byte == 0x0F {
                            return self.read_flash(address);
                        } else {
                            return self.memory[self.rom_mirrored_address(address) as usize].get();
                        }
                    },
                    BackupType::Error => {
                        if upper_byte == 0x0E || upper_byte == 0x0F {
                            return self.memory[((address & SRAM_SIZE) + SRAM_START) as usize].get();
                        } else {
                            return self.memory[self.rom_mirrored_address(address) as usize].get();
                        }
                    },
                }
            }
            0x00 => {
                if address >= BIOS_SIZE {
                    let open_bus = self.general_open_bus();
                    return ((open_bus >> ((address & 3) * 8)) & 0xFF) as u8;
                }
                let current_pc = CURRENT_INSTR_PC.with(|pc| pc.get());
                if current_pc < BIOS_SIZE {
                    return self.memory[address as usize].get();
                }
                // Real hardware returns its last-latched BIOS opcode here (verified against
                // jsmolka's bios.gba test, which expects a specific nonzero value after a
                // Halt/IntrWait+IRQ sequence). Some commercial games (e.g. Hello Kitty
                // Collection's heap allocator) dereference a null "next" pointer in the same
                // BIOS-protected range and rely on this read coming back zero to terminate a
                // list walk gracefully. The two requirements are mutually exclusive -- the
                // latch is a single 32-bit value selected only by address&3, so there is no
                // way to distinguish these callers by address. Compatibility with real games
                // wins here at the cost of failing that one synthetic BIOS quirk check.
                0
            }
            _ => {
                let open_bus = self.general_open_bus();
                return ((open_bus >> ((address & 3) * 8)) & 0xFF) as u8;
            }
        }
    }

    fn general_open_bus(&self) -> u32 {
        if let Some(value) = DMA_BUS_OVERRIDE.with(|d| d.get()) {
            return value;
        }
        let pc = CURRENT_INSTR_PC.with(|p| p.get());
        if CURRENT_INSTR_IS_THUMB.with(|t| t.get()) {
            let half = self.read_u16(pc.wrapping_add(4)) as u32;
            (half << 16) | half
        } else {
            self.read_u32(pc.wrapping_add(8))
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
        let mut state = serializer.serialize_struct("MemoryMap", 9)?;

        state.serialize_field("memory", &*self.memory)?;

        // Serialize the rest of the fields normally
        state.serialize_field("halt_state", &self.halt_state)?;
        state.serialize_field("backup_type", &self.backup_type)?;
        state.serialize_field("backed_up", &self.backed_up)?;
        state.serialize_field("flash", &self.flash)?;
        state.serialize_field("eeprom", &*self.eeprom.borrow())?;
        state.serialize_field("fifo_a", &self.fifo_a)?;
        state.serialize_field("fifo_b", &self.fifo_b)?;
        state.serialize_field("trigger_flags", &self.trigger_flags)?;

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
        enum Field { Memory, HaltState, BackupType, BackedUp, Flash, Eeprom, FifoA, FifoB, TriggerFlags }

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
                        formatter.write_str("`memory`, `halt_state`, `backup_type`, `backed_up`, `flash`, `eeprom`, `fifo_a`, `fifo_b`, or `trigger_flags`")
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
                            "eeprom" => Ok(Field::Eeprom),
                            "fifo_a" => Ok(Field::FifoA),
                            "fifo_b" => Ok(Field::FifoB),
                            "trigger_flags" => Ok(Field::TriggerFlags),
                            _ => Err(de::Error::unknown_field(value, &["memory", "halt_state", "backup_type", "backed_up", "flash", "eeprom", "fifo_a", "fifo_b", "trigger_flags"])),
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
                let mem_vec: GbaMem = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let halt_state = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let backup_type = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let backed_up = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let flash = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let eeprom = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let fifo_a = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let fifo_b = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let trigger_flags = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;

                let memory = Rc::new(mem_vec);
                Ok(MemoryMap {
                    memory,
                    halt_state,
                    backup_type,
                    backed_up,
                    flash,
                    eeprom: RefCell::new(eeprom),
                    fifo_a,
                    fifo_b,
                    trigger_flags,
                    rom_size: ROM_SIZE + 1,
                    rom_undersize_mirror_mask: None,
                    wave_ram_banks: [[0; 16]; 2],
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
                let mut eeprom = None;
                let mut fifo_a = None;
                let mut fifo_b = None;
                let mut trigger_flags = None;

                // Extract each field from the map
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Memory => {
                            if memory.is_some() {
                                return Err(de::Error::duplicate_field("memory"));
                            }
                            let mem_vec: GbaMem = map.next_value()?;
                            memory = Some(Rc::new(mem_vec));
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
                        Field::Eeprom => {
                            if eeprom.is_some() {
                                return Err(de::Error::duplicate_field("eeprom"));
                            }
                            eeprom = Some(map.next_value()?);
                        }
                        Field::FifoA => {
                            if fifo_a.is_some() {
                                return Err(de::Error::duplicate_field("fifo_a"));
                            }
                            fifo_a = Some(map.next_value()?);
                        }
                        Field::FifoB => {
                            if fifo_b.is_some() {
                                return Err(de::Error::duplicate_field("fifo_b"));
                            }
                            fifo_b = Some(map.next_value()?);
                        }
                        Field::TriggerFlags => {
                            if trigger_flags.is_some() {
                                return Err(de::Error::duplicate_field("trigger_flags"));
                            }
                            trigger_flags = Some(map.next_value()?);
                        }
                    }
                }

                // Ensure all fields were provided
                let memory = memory.ok_or_else(|| de::Error::missing_field("memory"))?;
                let halt_state = halt_state.ok_or_else(|| de::Error::missing_field("halt_state"))?;
                let backup_type = backup_type.ok_or_else(|| de::Error::missing_field("backup_type"))?;
                let backed_up = backed_up.ok_or_else(|| de::Error::missing_field("backed_up"))?;
                let flash = flash.ok_or_else(|| de::Error::missing_field("flash"))?;
                let eeprom: Eeprom = eeprom.ok_or_else(|| de::Error::missing_field("eeprom"))?;
                let fifo_a = fifo_a.ok_or_else(|| de::Error::missing_field("fifo_a"))?;
                let fifo_b = fifo_b.ok_or_else(|| de::Error::missing_field("fifo_b"))?;
                let trigger_flags = trigger_flags.ok_or_else(|| de::Error::missing_field("trigger_flags"))?;

                // Return the constructed struct
                Ok(MemoryMap {
                    memory,
                    halt_state,
                    backup_type,
                    backed_up,
                    flash,
                    eeprom: RefCell::new(eeprom),
                    fifo_a,
                    fifo_b,
                    trigger_flags,
                    rom_size: ROM_SIZE + 1,
                    rom_undersize_mirror_mask: None,
                    wave_ram_banks: [[0; 16]; 2],
                })
            }
        }

        // Start the deserialization process
        deserializer.deserialize_struct(
            "MemoryMap",
            &["memory", "halt_state", "backup_type", "backed_up", "flash", "eeprom", "fifo_a", "fifo_b", "trigger_flags"],
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
        let mut mem = MemoryMap::new(BackupType::Error);
        let addr = ON_BOARD_WRAM_START + ON_BOARD_WRAM_SIZE - 3;
        mem.write_u16(addr, 0xABCD);
        assert_eq!(mem.read_u16(addr), 0xABCD);
    }

    #[test]
    fn io_region_writes_still_go_through_byte_wise_special_casing() {
        let mut mem = MemoryMap::new(BackupType::Error);
        assert_eq!(mem.halt_state, HaltState::Running);
        mem.write_u8(0x4000301, 0x00);
        assert_eq!(mem.halt_state, HaltState::Halt);
    }

    #[test]
    fn eeprom_backup_type_does_not_intercept_rom_reads() {
        let mut mem = MemoryMap::new(BackupType::Eeprom);
        mem.memory[0x0800_0100].set(0x12);
        mem.memory[0x0800_0101].set(0x34);
        assert_eq!(mem.read_u8(0x0800_0100), 0x12);
        assert_eq!(mem.read_u16(0x0800_0100), 0x3412);

        mem.write_u8(0x0800_0200, 0x56);
        assert_eq!(mem.memory[0x0800_0200].get(), 0x56);
    }

    #[test]
    fn eeprom_backup_type_intercepts_only_the_0d_bank() {
        let mut mem = MemoryMap::new(BackupType::Eeprom);
        let bits: Vec<u16> = std::iter::once(1)
            .chain(std::iter::once(0))
            .chain((0..6).map(|_| 0))
            .chain((0..64).map(|i| if i == 63 { 1 } else { 0 }))
            .chain(std::iter::once(0))
            .collect();
        for bit in bits {
            mem.write_u16(0x0D00_0000, bit);
        }
        assert_eq!(mem.memory[0x0D00_0000].get(), 0);
    }
}

#[cfg(test)]
mod sound_master_enable_tests {
    use super::*;
    use crate::gamepak::BackupType;

    #[test]
    fn psg_registers_ignore_writes_while_master_disabled() {
        let mut mem = MemoryMap::new(BackupType::Error);
        mem.write_u16(0x4000060, 0xFFFF);
        mem.write_u16(0x4000080, 0xFFFF);
        assert_eq!(mem.read_u16(0x4000060), 0);
        assert_eq!(mem.read_u16(0x4000080), 0);
    }

    #[test]
    fn sound_cnt_h_stays_writable_while_master_disabled() {
        let mut mem = MemoryMap::new(BackupType::Error);
        mem.write_u16(0x4000082, 0x770F);
        assert_eq!(mem.read_u16(0x4000082), 0x770F);
    }

    #[test]
    fn psg_registers_accept_writes_once_master_enabled() {
        let mut mem = MemoryMap::new(BackupType::Error);
        mem.write_u8(0x4000084, 0x80);
        mem.write_u16(0x4000060, 0xFFFF);
        assert_eq!(mem.read_u16(0x4000060), 0xFFFF);
    }

    #[test]
    fn disabling_master_clears_psg_registers_immediately() {
        let mut mem = MemoryMap::new(BackupType::Error);
        mem.write_u8(0x4000084, 0x80);
        mem.write_u16(0x4000060, 0xFFFF);
        mem.write_u8(0x4000084, 0x00);
        assert_eq!(mem.read_u16(0x4000060), 0);
    }
}

#[cfg(test)]
mod io_open_bus_tests {
    use super::*;
    use crate::gamepak::BackupType;

    #[test]
    fn unused_io_gap_reads_as_open_bus_not_zero() {
        let mem = MemoryMap::new(BackupType::Error);
        CURRENT_INSTR_PC.with(|pc| pc.set(0x0800_0000));
        CURRENT_INSTR_IS_THUMB.with(|t| t.set(false));
        mem.memory[0x0800_0008].set(0x12);
        mem.memory[0x0800_0009].set(0x34);
        mem.memory[0x0800_000A].set(0x56);
        mem.memory[0x0800_000B].set(0x78);
        assert_eq!(mem.read_u32(0x0400_0FF0), 0x7856_3412);
    }

    #[test]
    fn known_registers_below_the_gap_still_read_their_own_value() {
        let mut mem = MemoryMap::new(BackupType::Error);
        mem.write_u16(0x4000200, 0x1234);
        assert_eq!(mem.read_u16(0x4000200), 0x1234);
    }

    #[test]
    fn dma_bus_override_takes_priority_until_cleared() {
        let mem = MemoryMap::new(BackupType::Error);
        CURRENT_INSTR_PC.with(|pc| pc.set(0x0800_0000));
        CURRENT_INSTR_IS_THUMB.with(|t| t.set(false));
        mem.memory[0x0800_0008].set(0xAA);

        DMA_BUS_OVERRIDE.with(|d| d.set(Some(0xDEAD_BEEF)));
        assert_eq!(mem.read_u32(0x0400_0FF0), 0xDEAD_BEEF);

        DMA_BUS_OVERRIDE.with(|d| d.set(None));
        assert_eq!(mem.read_u32(0x0400_0FF0), 0x0000_00AA);
    }
}
