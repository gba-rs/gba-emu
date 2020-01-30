use std::cell::RefCell;
use std::rc::Rc;
use crate::operations::timing::CycleClock;
use crate::operations::timing::*;
use crate::operations::timing::MemAccessSize::{Mem32, Mem16, Mem8};
use crate::memory::system_control::WaitStateControl;

pub struct Range<T: Ord> {
    pub lower: T,
    pub higher: T,
}

impl<T> Range<T> where T: std::cmp::Ord {
    pub fn new(lower: T, upper: T) -> Range<T> {
        return Range {
            lower: lower,
            higher: upper,
        };
    }

    pub fn contains(&self, value: T) -> bool {
        return value <= self.higher && value >= self.lower;
    }
}

pub struct MemoryBlock {
    pub range: Range<u32>,
    pub memory: Rc<RefCell<Vec<u8>>>,
}

pub struct MemoryMap {
    pub memory_mapping: Vec<MemoryBlock>,
    pub(crate) cycle_clock: CycleClock,
}

impl MemoryMap {
    pub fn new() -> MemoryMap {
        return MemoryMap {
            memory_mapping: vec![],
            cycle_clock: CycleClock {
                prev_address: 0,
                cycles: 0,
                wait_state_control: WaitStateControl::new()
            },
        };
    }

    pub fn write_u8(&mut self, address: u32, value: u8) {
        let (lower, _, mem) = self.get_memory(address);
        let index: u32 = address - lower;
        mem.borrow_mut()[index as usize] = value;
    }

    pub fn write_u16(&mut self, address: u32, value: u16) {
        let (lower, _, mem) = self.get_memory(address);
        let index: u32 = address - lower;
        let mut memory = mem.borrow_mut();
        memory[(index as usize) + 1] = ((value & 0xFF00) >> 8) as u8;
        memory[index as usize] = (value & 0xFF) as u8;
    }

    pub fn write_u32(&mut self, address: u32, value: u32) {
        let (lower, _, mem) = self.get_memory(address);
        let index: u32 = address - lower;
        let mut memory = mem.borrow_mut();
        memory[(index as usize) + 3] = ((value & 0xFF000000) >> 24) as u8;
        memory[(index as usize) + 2] = ((value & 0xFF0000) >> 16) as u8;
        memory[(index as usize) + 1] = ((value & 0xFF00) >> 8) as u8;
        memory[index as usize] = (value & 0xFF) as u8;
    }

    pub fn write_block(&mut self, address: u32, block: Vec<u8>) {
        let (lower, _, mem) = self.get_memory(address);
        let index: u32 = address - lower;
        let mut memory = mem.borrow_mut();

        let mut offset: usize = 0;
        for byte in block {
            memory[(index as usize) + offset] = byte;
            offset += 1;
        }
    }

    pub fn read_block(&mut self, address: u32, bytes: u32) -> Vec<u8> {
        let mut temp: Vec<u8> = vec![];
        for i in address..(address + bytes) {
            temp.push(self.read_u8(i));
        }
        return temp;
    }

    pub fn read_u32(&mut self, address: u32) -> u32 {
        self.cycle_clock.update_cycles(address, Mem32);
        let (lower, _, mem) = self.get_memory(address);
        let index: u32 = address - lower;
        let mut result: u32 = 0;
        let memory = mem.borrow_mut();
        for i in 0..4 {
            result |= (memory[(index + i) as usize] as u32) << (i * 8);
        }
        return result;
    }

    pub fn read_u16(&mut self, address: u32) -> u16 {
        self.cycle_clock.update_cycles(address, Mem16);
        let (lower, _, mem) = self.get_memory(address);
        let index: u32 = address - lower;
        let memory = mem.borrow_mut();
        let result: u16 = ((memory[(index + 1) as usize] as u16) << 8) | (memory[index as usize] as u16);
        return result;
    }

    pub fn read_u8(&mut self, address: u32) -> u8 {
        self.cycle_clock.update_cycles(address, Mem8);
        let (lower, _, mem) = self.get_memory(address);
        let index: u32 = address - lower;
        return mem.borrow_mut()[index as usize];
    }

    pub fn register_memory(&mut self, lower: u32, upper: u32, mem: &Rc<RefCell<Vec<u8>>>) {
        self.memory_mapping.push(MemoryBlock {
            range: Range::new(lower, upper),
            memory: mem.clone(),
        });
    }

    fn get_memory(&self, address: u32) -> (u32, u32, &RefCell<Vec<u8>>) {
        for mem_block in self.memory_mapping.iter() {
            if mem_block.range.contains(address) {
                return (mem_block.range.lower, mem_block.range.higher, &mem_block.memory);
            }
        }

        panic!("Not implemented: {:X}", address);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::work_ram::WorkRam;
    use crate::memory::mock_memory::MockMemory;
    use crate::gba::GBA;

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

    #[test]
    #[should_panic]
    fn test_memory_map_out_of_range() {
        let mut map = MemoryMap::new();
        map.read_u8(0xFFFFFFFF);
    }
}