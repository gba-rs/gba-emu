use std::cell::RefCell;
use std::rc::Rc;
use log::error;

pub struct Range<T: Ord> {
    pub lower: T,
    pub higher: T
}

impl<T> Range<T> where T: std::cmp::Ord {
    pub fn new(lower: T, upper: T) -> Range<T>  {
        return Range {
            lower: lower,
            higher: upper
        };
    }

    pub fn contains(&self, value: T) -> bool {
        return value <= self.higher && value >= self.lower;
    }
}

pub struct MemoryBlock {
    pub range: Range<u32>,
    pub memory: Rc<RefCell<Vec<u8>>>
}

pub struct MemoryMap {
    pub memory: Rc<RefCell<Vec<u8>>>
}

impl MemoryMap {

    pub fn new() -> MemoryMap {
        return MemoryMap {
            memory_mapping: vec![],
            memory: Rc::new(RefCell::new(vec![0; 0x1000_0000]))
        }
    }

    pub fn write_u8(&mut self, address: u32, value: u8) {
        if address > 0x0FFFFFFF { return }
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


#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::work_ram::WorkRam;
    use crate::memory::mock_memory::MockMemory;
    
    #[test]
    fn test_memory_map_read() {
        let mut map = MemoryMap::new();
        let wram = WorkRam::new(256000, 10);
        map.register_memory(0x02000000, 0x0203FFFF, &wram.memory);
        assert_eq!(map.read_u8(0x02000000), 0x0A);
        assert_eq!(map.read_u16(0x02000000), 0x0A0A);
        assert_eq!(map.read_u32(0x02000000), 0x0A0A0A0A);
    }

    #[test]
    fn test_memory_map_write() {
        let mut map = MemoryMap::new();
        let wram = WorkRam::new(256000, 10);
        map.register_memory(0x02000000, 0x0203FFFF, &wram.memory);
        map.write_u8(0x02000000, 0x30);
        assert_eq!(map.read_u8(0x02000000), 0x30);
        map.write_u16(0x02000000, 0x1234);
        assert_eq!(map.read_u16(0x02000000), 0x1234);
        map.write_u32(0x02000000, 0x12345678);
        assert_eq!(map.read_u32(0x02000000), 0x12345678);
    }

    #[test]
    fn test_memory_map_multiple() {
        let mut map = MemoryMap::new();
        let wram = WorkRam::new(256000, 10);
        let mut mock_mem = MockMemory::new(0xFF);
        map.register_memory(0x02000000, 0x0203FFFF, &wram.memory);
        map.register_memory(0x0, 0x0003FFFF, &mock_mem.memory);

        mock_mem.set_mock_field(100);
        assert_eq!(map.read_u8(0x00000064), 100);
        assert_eq!(mock_mem.get_mock_field(), 100);

        map.write_u8(0x02000000, 0xFF);
        assert_eq!(map.read_u8(0x02000000), 0xFF);

    }
}