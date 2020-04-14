use std::cell::RefCell;
use std::rc::Rc;
use log::error;

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
        if address > 0x0FFF_FFFF { return }
        
        if address <= 0x03007FFF && address >= 0x03007F00 {
            // mirror memory
            self.memory.borrow_mut()[((address & 0xFF) + 0x03FFFF00) as usize] = value;
        }

        if address == 0x4000202 || address == 0x4000203 {
            let new_val = self.read_u8(address) & !value;
            self.memory.borrow_mut()[address as usize] = new_val;
            return;
        }

        if address == 0x4000100 || address == 0x4000101 ||
           address == 0x4000104 || address == 0x4000105 ||
           address == 0x4000108 || address == 0x4000109 ||
           address == 0x400010C || address == 0x400010D {
            let index: usize = (address & 0xF) as usize;
            self.memory.borrow_mut()[0x1000_0000usize + index] = value;
        }

        self.memory.borrow_mut()[address as usize] = value;
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
        for byte in block {
            self.write_u8(address + offset, *byte);
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
        if address > 0x0FFFFFFF { return 0; }
        return self.memory.borrow()[address as usize];
    }
}
