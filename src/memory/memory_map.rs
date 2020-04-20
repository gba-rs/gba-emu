use std::cell::RefCell;
use std::rc::Rc;
use log::error;

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


pub struct MemoryMap {
    pub memory: Rc<RefCell<Vec<u8>>>
}

impl MemoryMap {

    pub fn new() -> MemoryMap {
        return MemoryMap {
            memory: Rc::new(RefCell::new(vec![0; 0x1000_00F0]))
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
                } else {
                    self.memory.borrow_mut()[address as usize] = value;
                }

            },
            0x05 => self.memory.borrow_mut()[((address & PALETTE_RAM_SIZE) + PALETTE_RAM_START) as usize] = value,
            0x06 => self.memory.borrow_mut()[address as usize] = value,
            0x07 => self.memory.borrow_mut()[((address & OBJECT_ATTRIBUTES_SIZE) + OBJECT_ATTRIBUTES_START) as usize] = value,
            0x08..=0xD => self.memory.borrow_mut()[((address & ROM_SIZE) + ROM_START) as usize] = value,
            0x0E | 0x0F => self.memory.borrow_mut()[((address & SRAM_SIZE) + SRAM_START) as usize] = value,
            _ => {}
        }


    }

    pub fn write_u16(&mut self, address: u32, value: u16) {
        self.write_u8(address + 1, ((value & 0xFF00) >> 8) as u8);
        self.write_u8(address, (value & 0xFF) as u8);
    }

    pub fn write_u32(&mut self, address: u32, value: u32) {
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

    pub fn read_u32(&self, address: u32) -> u32 {
        let mut result: u32 = 0;
        for i in 0..4 {
            result |= (self.read_u8(address + i) as u32) <<  (i * 8);
        }
        return result;
    }

    pub fn read_u16(&self, address: u32) -> u16 {
        let result: u16 = ((self.read_u8(address + 1) as u16) << 8) | (self.read_u8(address) as u16);
        return result;
    }

    pub fn read_u8(&self, address: u32) -> u8 {
        let upper_byte = address >> 24;

        match upper_byte {
            0x02 => return self.memory.borrow_mut()[((address & ON_BOARD_WRAM_SIZE) + ON_BOARD_WRAM_START) as usize],
            0x03 => return self.memory.borrow_mut()[((address & ON_CHIP_WRAM_SIZE) + ON_CHIP_WRAM_START) as usize],
            0x04 => return self.memory.borrow()[address as usize],
            0x05 => return self.memory.borrow_mut()[((address & PALETTE_RAM_SIZE) + PALETTE_RAM_START) as usize],
            0x06 => return self.memory.borrow()[address as usize],
            0x07 => return self.memory.borrow_mut()[((address & OBJECT_ATTRIBUTES_SIZE) + OBJECT_ATTRIBUTES_START) as usize],
            0x08..=0xD => return self.memory.borrow_mut()[((address & ROM_SIZE) + ROM_START) as usize],
            0x0E | 0x0F => return self.memory.borrow_mut()[((address & SRAM_SIZE) + SRAM_START) as usize],
            _ => {
                if address > 0x0FFFFFFF { return 0; }

                return self.memory.borrow()[address as usize];
            }
        }
    }
}
