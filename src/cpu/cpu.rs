use crate::formats::{data_processing::DataProcessing, common::Instruction};
use crate::memory::{work_ram::WorkRam, memory_map::MemoryMap};

const ARM_PC: u8 = 15;
const THUMB_PC: u8 = 10;

const REG_MAP: [[[u8; 16]; 7]; 2] = [
    // arm
    [
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],     // System
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],     // User
        [0, 1, 2, 3, 4, 5, 6, 7, 16, 17, 18 , 19, 20, 21, 22, 15],  // FIQ
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 23, 24, 15],     // Supervisor
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 25, 26, 15],     // Abort
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 27, 28, 15],     // IRQ
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 29, 30, 15]      // Undefiend
    ],
    // thumb
    [
        [0, 1, 2, 3, 4, 5, 6, 7, 13, 14, 15, 0, 0, 0, 0, 0],        // System
        [0, 1, 2, 3, 4, 5, 6, 7, 13, 14, 15, 0, 0, 0, 0, 0],        // User
        [0, 1, 2, 3, 4, 5, 6, 7, 21, 22, 15, 0, 0, 0, 0, 0],        // FIQ
        [0, 1, 2, 3, 4, 5, 6, 7, 23, 24, 15, 0, 0, 0, 0, 0],        // Supervisor
        [0, 1, 2, 3, 4, 5, 6, 7, 25, 26, 15, 0, 0, 0, 0, 0],        // Abort
        [0, 1, 2, 3, 4, 5, 6, 7, 27, 28, 15, 0, 0, 0, 0, 0],        // IRQ
        [0, 1, 2, 3, 4, 5, 6, 7, 29, 30, 15, 0, 0, 0, 0, 0]         // Undefiend
    ]
];

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum OperatingMode {
    System = 0,
    User = 1,
    FastInterrupt = 2,
    Supervisor = 3,
    Abort = 4,
    Interrupt = 5,
    Undefiend = 6
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum InstrcutionSet {
    Arm,
    Thumb
}

pub struct CPU {   
    registers: [u32; 31],
    pub wram: WorkRam,
    pub operating_mode: OperatingMode,
    pub current_instruction_set: InstrcutionSet
}

impl CPU {
    pub fn new() -> CPU {
        return CPU {
            registers: [0; 31],
            wram: WorkRam::new(0),
            operating_mode: OperatingMode::User,
            current_instruction_set: InstrcutionSet::Arm
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
        let current_pc = if self.current_instruction_set == InstrcutionSet::Arm { ARM_PC } else { THUMB_PC };
        let pc_contents = self.get_register(current_pc); 
        self.set_register(current_pc, pc_contents + 4);
        self.decode(map, instruction);
    }

    pub fn get_register(&mut self, reg_num: u8) -> u32 {
        if self.current_instruction_set == InstrcutionSet::Thumb {
            if reg_num > 10 {
                panic!("Attempting to get register out of range for Thumb: {}", reg_num);
            }
        } else {
            if reg_num > 15 {
                panic!("Attempting to get register out of range for Arm: {}", reg_num);
            }
        }
        return self.registers[REG_MAP[self.current_instruction_set as usize][self.operating_mode as usize][reg_num as usize] as usize];
    }

    pub fn set_register(&mut self, reg_num: u8, value: u32) {
        if self.current_instruction_set == InstrcutionSet::Thumb {
            if reg_num > 10 {
                panic!("Attempting to set register out of range for Thumb: {}", reg_num);
            }
        } else {
            if reg_num > 15 {
                panic!("Attempting to set register out of range for Arm: {}", reg_num);
            }
        }
        self.registers[REG_MAP[self.current_instruction_set as usize][self.operating_mode as usize][reg_num as usize] as usize] = value;
    }
}

// Unit Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access_registers(){
        let testram = WorkRam::new(10);
        let mut cpu = CPU{registers: [0; 31], wram: testram, operating_mode: OperatingMode::User, current_instruction_set: InstrcutionSet::Arm};
        let _empty_registers: [u32; 31] = [0; 31];
        
        assert_eq!(_empty_registers, cpu.registers);
    }

    #[test]
    #[should_panic]
    fn test_decode_unimplemented(){
        let testram = WorkRam::new(10);
        let mut map = MemoryMap::new();
        map.register_memory(0x02000000, 0x0203FFFF, &testram.memory);
        let mut cpu = CPU{registers: [0; 31], wram: testram, operating_mode: OperatingMode::User, current_instruction_set: InstrcutionSet::Arm};
        
        cpu.decode(&mut map, 0xE3000000);
    }

    #[test]
    fn test_decode(){
        let mut map = MemoryMap::new();
        let testram = WorkRam::new(10);
        map.register_memory(0x02000000, 0x0203FFFF, &testram.memory);
        let mut cpu = CPU{registers: [0; 31], wram: testram, operating_mode: OperatingMode::User, current_instruction_set: InstrcutionSet::Arm};
        cpu.decode(&mut map, 0xE0812001);
    }

    #[test]
    fn test_fetch(){
        let testram = WorkRam::new(10);
        let mut cpu = CPU{registers: [0; 31], wram: testram, operating_mode: OperatingMode::User, current_instruction_set: InstrcutionSet::Arm};
        cpu.set_register(15, 0x02000000);
        let mut map = MemoryMap::new();
        let wram = WorkRam::new(10);
        map.register_memory(0x02000000, 0x0203FFFF, &wram.memory);
        map.write_u32(0x02000000, 0xE0812001);
        map.write_u32(0x02000004, 0xE0812001);
        cpu.fetch(&mut map);
        cpu.fetch(&mut map);
    }

}