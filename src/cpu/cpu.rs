use crate::formats::{data_processing::DataProcessing, common::Instruction};
use crate::memory::{work_ram::WorkRam, memory_map::MemoryMap, memory_map::MemoryBlock };
use std::cell::RefCell;
use std::rc::Rc;

pub struct CPU {   
    pub registers: [u32; 16],
    pub wram: WorkRam
}

impl CPU {

    pub fn new() -> CPU {
        return CPU {
            registers: [0; 16],
            wram: WorkRam::new(0)
        };
    }

    pub fn decode(&mut self, mem_map: &mut MemoryMap, instruction: u32) {
        let opcode: u16 = (((instruction >> 16) & 0xFF0) | ((instruction >> 4) & 0x0F)) as u16;
        println!("Decoding: {:X}", opcode);
        match opcode {
            0x080 | 0x3A0  => { // ADD lli
                let mut format: DataProcessing = DataProcessing::from(instruction);
                format.execute(self, mem_map);
            }
            _ => panic!("Could not decode {:X}", opcode),
        }
    }
    
    pub fn fetch(&mut self, map: &mut MemoryMap) {
        let instruction: u32 = map.read_u32(self.registers[15]);
        self.registers[15] += 4;
        self.decode(map, instruction);
    }
}

// Unit Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access_registers(){
        let testram = WorkRam::new(10);
        let mut cpu = CPU{registers: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0], wram: testram};
        let _empty_registers: [u32; 16] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
        
        assert_eq!(_empty_registers, cpu.registers);
    }

    #[test]
    #[should_panic]
    fn test_decode_unimplemented(){
        let testram = WorkRam::new(10);
        let mut map = MemoryMap::new();
        map.register_memory(0x02000000, 0x0203FFFF, &testram.memory);
        let mut cpu = CPU{registers: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0], wram: testram};
        
        cpu.decode(&mut map, 0xE3000000);
    }

    #[test]
    fn test_decode(){
        let mut map = MemoryMap::new();
        let testram = WorkRam::new(10);
        map.register_memory(0x02000000, 0x0203FFFF, &testram.memory);
        let mut cpu = CPU{registers: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0], wram: testram};
        cpu.decode(&mut map, 0xE0812001);
    }

    #[test]
    fn test_fetch(){
        let testram = WorkRam::new(10);
        let mut cpu = CPU{registers: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0x02000000], wram: testram};
        let mut map = MemoryMap::new();
        let wram = WorkRam::new(10);
        map.register_memory(0x02000000, 0x0203FFFF, &wram.memory);
        map.write_u32(0x02000000, 0xE0812001);
        map.write_u32(0x02000004, 0xE0812001);
        cpu.fetch(&mut map);
        cpu.fetch(&mut map);
    }

}