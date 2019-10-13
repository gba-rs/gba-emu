pub mod common;

use common::*;
use std::cell::RefCell;

struct MemoryMap {
    work_ram: RefCell<Vec<u8>>
}

impl MemoryMap {

    pub fn new(init_val: u8) -> MemoryMap {
        return MemoryMap {
            work_ram: RefCell::new(vec![init_val; 0x3_E800])
        }
    }

    pub fn writeU8(&mut self, address: u32, value: u8) {
        let (lower, upper, mem) = self.getMemory(address);
        let index: u32 = address - lower;
        mem.borrow_mut()[index as usize] = value;
    }

    pub fn writeU16(&mut self, address: u32, value: u16) {
        let (lower, upper, mem) = self.getMemory(address);
        let index: u32 = address - lower;
        let mut memory = mem.borrow_mut();
        memory[index as usize] = (value & 0xFF) as u8;
        memory[(index as usize) + 1] = ((value & 0xFF00) >> 8) as u8;
    }

    pub fn writeU32(&mut self, address: u32, value: u32) {
        let (lower, upper, mem) = self.getMemory(address);
        let index: u32 = address - lower;
        let mut memory = mem.borrow_mut();
        memory[index as usize] = (value & 0xFF) as u8;
        memory[(index as usize) + 1] = ((value & 0xFF00) >> 8) as u8;
        memory[(index as usize) + 2] = ((value & 0xFF0000) >> 16) as u8;
        memory[(index as usize) + 3] = ((value & 0xFF000000) >> 24) as u8;
    }

    pub fn readU32(&mut self, address: u32) -> u32 {
        let (lower, upper, mem) = self.getMemory(address);
        let index: u32 = address - lower;
        let mut result: u32 = 0;
        let mut memory = mem.borrow_mut();
        for i in 0..4 {
            result |= (memory[(index + i) as usize] as u32) << (i * 8);
        }
        return result;
    }

    pub fn readU16(&mut self, address: u32) -> u16 {
        let (lower, upper, mem) = self.getMemory(address);
        let index: u32 = address - lower;
        let mut memory = mem.borrow_mut();
        let mut result: u16 = ((memory[(index + 1) as usize] as u16) << 8) | (memory[index as usize] as u16);
        return result;
    }

    pub fn readU8(&mut self, address: u32) -> u8 {
        let (lower, upper, mem) = self.getMemory(address);
        let index: u32 = address - lower;
        return mem.borrow_mut()[index as usize];
    }

    fn getMemory(&mut self, address: u32) -> (u32, u32, &RefCell<Vec<u8>>) {
        match address {
            0x02000000...0x0203FFFF => {
                return (0x02000000, 0x0203FFFF, &self.work_ram);
            },
            _ => panic!("Out of memory range")
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_memory_map_read() {
        let mut map = MemoryMap::new(10);
        assert_eq!(map.readU8(0x02000000), 0x0A);
        assert_eq!(map.readU16(0x02000000), 0x0A0A);
        assert_eq!(map.readU32(0x02000000), 0x0A0A0A0A);
    }

    #[test]
    fn test_memory_map_write() {
        let mut map = MemoryMap::new(10);
        map.writeU8(0x02000000, 0x30);
        assert_eq!(map.readU8(0x02000000), 0x30);
        map.writeU16(0x02000000, 0x1234);
        assert_eq!(map.readU16(0x02000000), 0x1234);
        map.writeU32(0x02000000, 0x12345678);
        assert_eq!(map.readU32(0x02000000), 0x12345678);
    }

    #[test]
    #[should_panic]
    fn test_memory_map_out_of_range() {
        let mut map = MemoryMap::new(10);
        map.readU8(0xFFFFFFFF);
    }
}