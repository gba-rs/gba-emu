use std::cell::RefCell;
use std::rc::Rc;
use crate::gamepak::BackupType;
use crate::gamepak::flash::Flash;
use crate::memory::eeprom::Eeprom;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::ser::SerializeStruct;
use serde::de::{self, Visitor, MapAccess, SeqAccess};
use std::fmt;
use std::marker::PhantomData;

thread_local! {
    /// The address of the instruction currently being executed, set once per
    /// CPU::fetch() call. Consumed by general_open_bus()'s "PC+8 lookahead"
    /// prefetch model and by the BIOS-protection read path below.
    pub static CURRENT_INSTR_PC: std::cell::Cell<u32> = std::cell::Cell::new(0);
    /// Companion to CURRENT_INSTR_PC — true if the CPU is currently in Thumb
    /// mode, since general_open_bus()'s prefetch model fetches a differently
    /// sized/aligned value depending on instruction set.
    pub static CURRENT_INSTR_IS_THUMB: std::cell::Cell<bool> = std::cell::Cell::new(false);
    /// The last 32-bit opcode fetched while PC was actually inside the
    /// BIOS (0x00000000-0x00003FFF). Real hardware protects BIOS memory
    /// from being read except by code executing from within it — any read
    /// from BIOS address space while PC is elsewhere returns this latched
    /// "last prefetched opcode" instead of the real byte content. Set by
    /// CPU::fetch(), consumed by MemoryMap::read_u8's BIOS branch.
    /// Confirmed against jsmolka/gba-tests' bios.gba (all 4 tests: at
    /// startup, after an SWI, during an IRQ, and after an IRQ each expect a
    /// different specific latched value, matching whichever BIOS routine
    /// most recently ran).
    pub static BIOS_OPCODE_LATCH: std::cell::Cell<u32> = std::cell::Cell::new(0);
}

/// Real BIOS size (16KB) — the actual protected/latched region. GBA's BIOS
/// occupies exactly 0x00000000-0x00003FFF; nothing above that is BIOS.
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
    pub memory: Rc<RefCell<Vec<u8>>>,
    pub halt_state: HaltState,
    pub backup_type: BackupType,
    pub backed_up: bool,
    pub flash: Flash,
    pub eeprom: RefCell<Eeprom>,
    // Direct Sound FIFO A/B (0x040000A0-0x040000A7) — real hardware queues, not flat registers.
    pub fifo_a: std::collections::VecDeque<u8>,
    pub fifo_b: std::collections::VecDeque<u8>,
    // PSG channel Trigger bit is write-only/edge-triggered on real hardware; latched
    // here at write time (bit N = channel N+1) since our registers can't model that.
    pub trigger_flags: u8,
}

impl MemoryMap {

    pub fn new(backup_type: BackupType) -> MemoryMap {
        let mut memory = vec![0u8; 0x1000_00F0];
        // Real SRAM/Flash chips read as 0xFF (all bits set) until something
        // is actually written — that's the electrically "erased" state, not
        // 0x00. Confirmed by jsmolka/gba-tests' save/*.gba suite, whose very
        // first check reads backup memory before writing anything and
        // expects 0xFF. Only the backup-storage window needs this; the rest
        // of the map (WRAM, VRAM, ...) has no meaningful "erased" value and
        // zero-init there is fine.
        let backup_start = SRAM_START as usize;
        let backup_end = backup_start + 0x20000; // covers SRAM/Flash64K/Flash128K
        memory[backup_start..backup_end].fill(0xFF);

        return MemoryMap {
            memory: Rc::new(RefCell::new(memory)),
            halt_state: HaltState::Running,
            backup_type: backup_type,
            backed_up: false,
            flash: Flash::new(),
            eeprom: RefCell::new(Eeprom::new()),
            fifo_a: std::collections::VecDeque::new(),
            fifo_b: std::collections::VecDeque::new(),
            trigger_flags: 0,
        }
    }

    /// Called by DMA right before it streams values into the EEPROM address
    /// window, with the halfword count for that specific transfer — the
    /// EEPROM state machine uses this to tell an opcode+address request
    /// apart from an opcode+address+data request without needing to know
    /// the chip size ahead of time.
    pub fn prepare_eeprom_write(&self, halfword_count: u32) {
        if self.backup_type == BackupType::Eeprom {
            self.eeprom.borrow_mut().prepare_write_transfer(halfword_count);
        }
    }

    /// Called by DMA right before it streams the data read-back out of the
    /// EEPROM address window, with the halfword count for that transfer.
    pub fn prepare_eeprom_read(&self, halfword_count: u32) {
        if self.backup_type == BackupType::Eeprom {
            self.eeprom.borrow_mut().prepare_read_transfer(halfword_count);
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
                }else if address == 0x4000065 || address == 0x400006D || address == 0x4000075 || address == 0x400007D {
                    // Bit 7 of a channel's frequency-control high byte is Trigger.
                    if value & 0x80 != 0 {
                        let channel_bit = match address {
                            0x4000065 => 0x1,
                            0x400006D => 0x2,
                            0x4000075 => 0x4,
                            _ => 0x8,
                        };
                        self.trigger_flags |= channel_bit;
                    }
                    self.memory.borrow_mut()[address as usize] = value;
                }else if (0x4000_00A0..=0x4000_00A3).contains(&address) {
                    if self.fifo_a.len() < 32 {
                        self.fifo_a.push_back(value);
                    }
                    self.memory.borrow_mut()[address as usize] = value;
                }else if (0x4000_00A4..=0x4000_00A7).contains(&address) {
                    if self.fifo_b.len() < 32 {
                        self.fifo_b.push_back(value);
                    }
                    self.memory.borrow_mut()[address as usize] = value;
                }else {
                    self.memory.borrow_mut()[address as usize] = value;
                }

            },
            // Palette RAM, and most of VRAM, only have a 16-bit-wide write
            // bus on real hardware: an 8-bit store duplicates the byte into
            // both halves of the containing halfword rather than writing
            // just that one byte. OAM has no 8-bit bus access at all, so a
            // byte store there is simply dropped. Confirmed against
            // jsmolka/gba-tests' memory/video_strb.asm.
            0x05 => {
                let halfword_addr = ((address & PALETTE_RAM_SIZE) + PALETTE_RAM_START) & !1;
                let mut mem = self.memory.borrow_mut();
                mem[halfword_addr as usize] = value;
                mem[(halfword_addr + 1) as usize] = value;
            },
            0x06 => {
                let mirrored = Self::vram_mirrored_address(address);
                if mirrored - VIDEO_RAM_START >= self.vram_obj_boundary() {
                    // Byte stores to the OBJ tile-data portion of VRAM are
                    // ignored, same as OAM.
                } else {
                    let halfword_addr = mirrored & !1;
                    let mut mem = self.memory.borrow_mut();
                    mem[halfword_addr as usize] = value;
                    mem[(halfword_addr + 1) as usize] = value;
                }
            },
            0x07 => {
                // Byte stores to OAM are ignored entirely on real hardware.
            },
            0x08..=0x0F => {
                match self.backup_type {
                    BackupType::Sram => {
                        // The chip only decodes its own low bits, so it
                        // mirrors throughout the whole 0x0E/0x0F window,
                        // not just at the base address.
                        if upper_byte == 0x0E || upper_byte == 0x0F {
                            self.memory.borrow_mut()[((address & SRAM_SIZE) + SRAM_START) as usize] = value;
                        } else {
                            self.memory.borrow_mut()[Self::rom_mirrored_address(address) as usize] = value;
                        }
                    },
                    BackupType::Eeprom => {
                        // The chip only responds on the 0x0D bank; 0x08-0x0C
                        // (and 0x0E/0x0F) are still plain ROM/mirror space
                        // for an EEPROM cart and must not be diverted into
                        // the bit-serial protocol.
                        if upper_byte == 0x0D {
                            self.eeprom.borrow_mut().write_bit(value as u16);
                        } else {
                            self.memory.borrow_mut()[address as usize] = value;
                        }
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

        if upper_byte == 0x06 {
            // The mirror seams (every 0x8000/0x18000/0x20000 bytes) all sit
            // on addresses divisible by 4, so any CPU-aligned halfword/word
            // access lands entirely within one mirrored segment — no
            // straddle handling needed here, same assumption the other
            // regions below already make.
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
            // Access straddles the region's wraparound point.
            return None;
        }

        Some((offset + start) as usize)
    }

    /// SRAM and Flash chips both only have an 8-bit-wide data bus — a
    /// 16/32-bit CPU access to either one only ever really transfers a
    /// single byte (see the write_u16/write_u32/read_u16/read_u32 call
    /// sites, and `operations::load_store::store`'s alignment bypass).
    #[inline]
    fn has_narrow_backup_bus(&self, upper_byte: u32) -> bool {
        matches!(self.backup_type, BackupType::Sram | BackupType::Flash64K | BackupType::Flash128K)
            && (upper_byte == 0x0E || upper_byte == 0x0F)
    }

    /// The cartridge ROM is mapped into three address windows
    /// (0x08000000-0x09FFFFFF, 0x0A000000-0x0BFFFFFF, 0x0C000000-0x0DFFFFFF)
    /// that only differ in their *default wait-state timing* — all three
    /// read the exact same underlying ROM bytes. `write_block`/`load_rom`
    /// only ever store the ROM at its first window, so the other two need
    /// remapping back onto it. Confirmed against jsmolka/gba-tests'
    /// memory/mirrors.asm ("GamePak mirror 1/2" tests).
    #[inline]
    fn rom_mirrored_address(address: u32) -> u32 {
        (address & ROM_SIZE) + ROM_START
    }

    /// GBA VRAM is 96KB (0x18000) but the chip-select decodes it in 128KB
    /// (0x20000) steps: within each step, the layout is 64KB + 32KB + a
    /// 32KB mirror of that second 32KB block. Confirmed against
    /// jsmolka/gba-tests' memory/mirrors.asm (128K-period and 32K-submirror
    /// tests).
    /// The offset (from VRAM's base) where OBJ (sprite) tile data starts —
    /// everything before it is BG tile/map data. Bitmap modes (3-5) give BG
    /// more room (0x14000) than tile modes (0-2, 0x10000), since bitmap
    /// mode framebuffers are larger. Reads DISPCNT directly out of the
    /// backing array (I/O registers live there like everything else) to
    /// avoid needing a GPU reference just for this one field.
    #[inline]
    fn vram_obj_boundary(&self) -> u32 {
        let bg_mode = self.memory.borrow()[0x04000000] & 0x7;
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
            let mut mem = self.memory.borrow_mut();
            mem[idx] = bytes[0];
            mem[idx + 1] = bytes[1];
            return;
        }

        let upper_byte = address >> 24;
        if self.backup_type == BackupType::Eeprom && upper_byte == 0x0D {
            self.eeprom.borrow_mut().write_bit(value);
            return;
        }

        // SRAM's data bus is only 8 bits wide: a 16-bit store only ever
        // actually writes a single byte, at `address` itself — never
        // address+1. Which byte of `value` lands depends on address&1
        // (the CPU replicates the halfword across its internal 32-bit bus,
        // and whichever byte lane the 8-bit chip is wired to is what
        // reaches it). Confirmed against jsmolka/gba-tests' save/sram.asm
        // ("Store half position" / "Store half just byte").
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
            let mut mem = self.memory.borrow_mut();
            mem[idx..idx + 4].copy_from_slice(&value.to_le_bytes());
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
            let result = u32::from_le_bytes([mem[idx], mem[idx + 1], mem[idx + 2], mem[idx + 3]]);
            drop(mem);
            return result;
        }

        let upper_byte = address >> 24;
        if self.has_narrow_backup_bus(upper_byte) {
            // SRAM/Flash's 8-bit bus means a wider read only ever samples
            // the one real byte at `address`; the CPU/bus then replicates
            // it to fill the requested width, rather than reading
            // neighboring (unrelated) bytes. Confirmed against
            // jsmolka/gba-tests' save/sram.asm and save/flash.asm ("Load
            // word").
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
            let mem = self.memory.borrow();
            let result = u16::from_le_bytes([mem[idx], mem[idx + 1]]);
            drop(mem);
            return result;
        }

        let upper_byte = address >> 24;
        if self.backup_type == BackupType::Eeprom && upper_byte == 0x0D {
            return self.eeprom.borrow_mut().read_bit();
        }

        // See read_u32's branch above — same 8-bit-bus replication.
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
            0x02 => return self.memory.borrow()[((address & ON_BOARD_WRAM_SIZE) + ON_BOARD_WRAM_START) as usize],
            0x03 => return self.memory.borrow()[((address & ON_CHIP_WRAM_SIZE) + ON_CHIP_WRAM_START) as usize],
            0x04 => return self.memory.borrow()[address as usize],
            0x05 => return self.memory.borrow()[((address & PALETTE_RAM_SIZE) + PALETTE_RAM_START) as usize],
            0x06 => return self.memory.borrow()[Self::vram_mirrored_address(address) as usize],
            0x07 => return self.memory.borrow()[((address & OBJECT_ATTRIBUTES_SIZE) + OBJECT_ATTRIBUTES_START) as usize],
            0x08..=0x0F => {
                match self.backup_type {
                    BackupType::Sram => {
                        if upper_byte == 0x0E || upper_byte == 0x0F {
                            return self.memory.borrow()[((address & SRAM_SIZE) + SRAM_START) as usize]
                        } else {
                            return self.memory.borrow()[Self::rom_mirrored_address(address) as usize];
                        }
                    },
                    BackupType::Eeprom => {
                        if upper_byte == 0x0D {
                            return self.eeprom.borrow_mut().read_bit() as u8;
                        } else {
                            return self.memory.borrow()[Self::rom_mirrored_address(address) as usize];
                        }
                    },
                    BackupType::Flash64K | BackupType::Flash128K => {
                        if upper_byte == 0x0E || upper_byte == 0x0F {
                            return self.read_flash(address);
                        } else {
                            return self.memory.borrow()[Self::rom_mirrored_address(address) as usize];
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
                        if upper_byte == 0x0E || upper_byte == 0x0F {
                            // No backup chip is actually present on this
                            // cart — real hardware has nothing responding
                            // on this address range at all (it's on a
                            // separate chip-select from the ROM, so ROM
                            // data doesn't leak through here either), which
                            // reads back as a floating/open bus that
                            // settles high. Confirmed against
                            // jsmolka/gba-tests' save/none.asm, which
                            // expects the same 0xFF "erased" reading as a
                            // real backup chip would give.
                            return self.memory.borrow()[((address & SRAM_SIZE) + SRAM_START) as usize];
                        } else {
                            return self.memory.borrow()[Self::rom_mirrored_address(address) as usize];
                        }
                    },
                }
            }
            0x00 => {
                if address >= BIOS_SIZE {
                    // Unused address space above the real 16KB BIOS image;
                    // nothing lives here on real hardware either, so reads
                    // return general open-bus garbage just like the fully
                    // unmapped regions below.
                    let open_bus = self.general_open_bus();
                    return ((open_bus >> ((address & 3) * 8)) & 0xFF) as u8;
                }
                let current_pc = CURRENT_INSTR_PC.with(|pc| pc.get());
                if current_pc < BIOS_SIZE {
                    // PC is actually executing from inside the BIOS, so
                    // this read is allowed to see the real byte content.
                    return self.memory.borrow()[address as usize];
                }
                // BIOS memory is protected from being read by anything
                // outside the BIOS itself: real hardware returns whatever
                // opcode the BIOS's own prefetch last fetched, not the
                // literal byte at `address`. Confirmed against
                // jsmolka/gba-tests' bios.gba.
                let latch = BIOS_OPCODE_LATCH.with(|l| l.get());
                return ((latch >> ((address & 3) * 8)) & 0xFF) as u8;
            }
            _ => {
                // Genuinely unmapped memory (the 0x01000000-0x01FFFFFF gap,
                // and everything above the 28-bit address space GBA's bus
                // actually decodes). Nothing responds here on real
                // hardware, so what shows up is open-bus garbage — the
                // CPU's own current prefetch value — not a fixed 0. Getting
                // this wrong turns any code that stumbles into this region
                // (e.g. dereferencing a null/garbage pointer) into a
                // guaranteed infinite loop instead of the finite, if
                // glitchy, garbage-data walk real hardware would do.
                let open_bus = self.general_open_bus();
                return ((open_bus >> ((address & 3) * 8)) & 0xFF) as u8;
            }
        }
    }

    /// General open-bus value for reads from unused/unmapped memory —
    /// distinct from BIOS_OPCODE_LATCH above, which models the BIOS chip's
    /// own internal latch (frozen at whatever it last drove while code was
    /// actually executing from BIOS). This instead reflects the CPU's own
    /// prefetch pipeline, which keeps moving regardless of what memory
    /// region PC is currently in — so it's computed live from
    /// CURRENT_INSTR_PC every time, never cached. Mirrors the same
    /// "PC+8 lookahead" trick CPU::fetch() already uses for the BIOS case;
    /// Thumb mode only ever fetches 16-bit halfwords, so the pipeline holds
    /// a duplicated halfword rather than one 32-bit word.
    fn general_open_bus(&self) -> u32 {
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

        // Serialize memory by borrowing the RefCell and using the underlying Vec<u8>
        state.serialize_field("memory", &*self.memory.borrow())?;

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
                let mem_vec = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let halt_state = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let backup_type = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let backed_up = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let flash = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let eeprom = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let fifo_a = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let fifo_b = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let trigger_flags = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;

                let memory = Rc::new(RefCell::new(mem_vec));
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

    #[test]
    fn eeprom_backup_type_does_not_intercept_rom_reads() {
        // Regression guard: only the 0x0D bank is the real EEPROM chip.
        // 0x08-0x0C (and 0x0E/0x0F) are still plain ROM/mirror space for
        // an EEPROM-backed cart, and the CPU fetches actual game code from
        // 0x08000000+ — routing those reads through the bit-serial EEPROM
        // protocol instead of returning the real ROM bytes previously
        // broke booting entirely for every EEPROM-backed game.
        let mut mem = MemoryMap::new(BackupType::Eeprom);
        mem.memory.borrow_mut()[0x0800_0100] = 0x12;
        mem.memory.borrow_mut()[0x0800_0101] = 0x34;
        assert_eq!(mem.read_u8(0x0800_0100), 0x12);
        assert_eq!(mem.read_u16(0x0800_0100), 0x3412);

        mem.write_u8(0x0800_0200, 0x56);
        assert_eq!(mem.memory.borrow()[0x0800_0200], 0x56);
    }

    #[test]
    fn eeprom_backup_type_intercepts_only_the_0d_bank() {
        let mut mem = MemoryMap::new(BackupType::Eeprom);
        // A write request: opcode "10", 6-bit address, 64 data bits, stop bit.
        let bits: Vec<u16> = std::iter::once(1)
            .chain(std::iter::once(0))
            .chain((0..6).map(|_| 0))
            .chain((0..64).map(|i| if i == 63 { 1 } else { 0 }))
            .chain(std::iter::once(0))
            .collect();
        for bit in bits {
            mem.write_u16(0x0D00_0000, bit);
        }
        // A byte written to the EEPROM window must not land in flat memory
        // at that literal address the way an ordinary ROM/mirror write would.
        assert_eq!(mem.memory.borrow()[0x0D00_0000], 0);
    }
}
