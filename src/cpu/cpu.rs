use crate::formats::{data_processing::DataProcessing, common::Instruction};
use crate::memory::{work_ram::WorkRam, memory_map::MemoryMap};

const ARM_FIQ_OFFSET: usize = 8;
const ARM_SVC_OFFSET: usize = 10;
const ARM_ABT_OFFSET: usize = 12;
const ARM_IRQ_OFFSET: usize = 14;
const ARM_UND_OFFSET: usize = 16;

const THUMB_FIQ_OFFSET: usize = 8;
const THUMB_SVC_OFFSET: usize = 10;
const THUMB_ABT_OFFSET: usize = 12;
const THUMB_IRQ_OFFSET: usize = 14;
const THUMB_UND_OFFSET: usize = 16;

const ARM_SP: usize = 13;
const ARM_LR: usize = 14;
const ARM_PC: usize = 15;


pub enum OperatingMode {
    User,
    FastInterrupt,
    Supervisor,
    Abort,
    Interrupt,
    System,
    Undefiend
}

pub enum InstrcutionSet {
    Arm,
    Thumb
}

pub struct CPU {   
    pub registers: [u32; 31],
    pub wram: WorkRam,
    operating_mode: OperatingMode,
    current_instruction_set: InstrcutionSet
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
        self.registers[15] += 4;
        self.decode(map, instruction);
    }

    pub fn get_register(&mut self, reg_num: u8) -> u32 {
        if self.current_instruction_set == InstructionSet::Arm {
            return self.get_register_arm(reg_num);
        } else {
            return self.get_register_thumb(reg_num);
        }
    }

    fn get_register_arm(&mut self, reg_num: u8) -> u32 {
        if reg_num > 15 { panic!("Arm Register out of range: {}", reg_num); }
        match self.operating_mode {
            OperatingMode::User | OperatingMode::System => {
                return self.registers[reg_num as usize];
            },
            OperatingMode::FastInterrupt => {
                if reg_num >= 8 || reg_num <= 14 {
                    return self.registers[(reg_num + ARM_FIQ_OFFSET) as usize];
                } else {
                    return self.registers[reg_num as usize];
                }
            },
            OperatingMode::Supervisor => {
                if reg_num == 13 || reg_num == 14 {
                    return self.registers[(reg_num + ARM_SVC_OFFSET) as usize];
                } else {
                    return self.registers[reg_num as usize];
                }
            },
            OperatingMode::Abort => {
                if reg_num == 13 || reg_num == 14 {
                    return self.registers[(reg_num + ARM_ABT_OFFSET) as usize];
                } else {
                    return self.registers[reg_num as usize];
                }
            },
            OperatingMode::Interrupt => {
                if reg_num == 13 || reg_num == 14 {
                    return self.registers[(reg_num + ARM_IRQ_OFFSET) as usize];
                } else {
                    return self.registers[reg_num as usize];
                }
            },
            OperatingMode::Undefiend => {
                if reg_num == 13 || reg_num == 14 {
                    return self.registers[(reg_num + ARM_UND_OFFSET) as usize];
                } else {
                    return self.registers[reg_num as usize];
                }
            }
        }
    }

    fn get_register_thumb(&mut self, reg_num: u8) -> {
        if reg_num > 10 { panic!("Thumb Register out of range: {}", reg_num); }
        if reg_num < 8 {
            return self.registers[reg_num as usize];
        } else if reg_num == 10 {
            return self.registers[ARM_PC];
        }

        match self.operating_mode {
            OperatingMode::User | OperatingMode::System => {
                if reg_num == 8 {
                    return self.registers[ARM_SP];
                } else {
                    return self.registers[ARM_LR];
                }
            },
            OperatingMode::FastInterrupt => {
                return self.registers[reg_num + THUMB_FIQ_OFFSET];
            },
            OperatingMode::Supervisor => {
                return self.registers[reg_num + THUMB_SVC_OFFSET];
            },
            OperatingMode::Abort => {
                return self.registers[reg_num + THUMB_ABT_OFFSET];
            },
            OperatingMode::Interrupt => {
                return self.registers[reg_num + THUMB_IRQ_OFFSET];
            },
            OperatingMode::Undefiend => {
                return self.registers[reg_num + THUMB_UND_OFFSET];
            }
        }
    }

    pub fn set_operating_mode(&mut self, mode: OperatingMode) {
        // do stuff that needs to be done when changing modes
        self.operating_mode = mode;
    }

    pub fn get_operating_mode(&mut self) -> OperatingMode {
        return self.operating_mode;
    }
}

// Unit Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access_registers(){
        let testram = WorkRam::new(10);
        let mut cpu = CPU{registers: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0], wram: testram, operating_mode: OperatingMode::User};
        let _empty_registers: [u32; 16] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
        
        assert_eq!(_empty_registers, cpu.registers);
    }

    #[test]
    #[should_panic]
    fn test_decode_unimplemented(){
        let testram = WorkRam::new(10);
        let mut map = MemoryMap::new();
        map.register_memory(0x02000000, 0x0203FFFF, &testram.memory);
        let mut cpu = CPU{registers: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0], wram: testram, operating_mode: OperatingMode::User};
        
        cpu.decode(&mut map, 0xE3000000);
    }

    #[test]
    fn test_decode(){
        let mut map = MemoryMap::new();
        let testram = WorkRam::new(10);
        map.register_memory(0x02000000, 0x0203FFFF, &testram.memory);
        let mut cpu = CPU{registers: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0], wram: testram, operating_mode: OperatingMode::User};
        cpu.decode(&mut map, 0xE0812001);
    }

    #[test]
    fn test_fetch(){
        let testram = WorkRam::new(10);
        let mut cpu = CPU{registers: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0x02000000], wram: testram, operating_mode: OperatingMode::User};
        let mut map = MemoryMap::new();
        let wram = WorkRam::new(10);
        map.register_memory(0x02000000, 0x0203FFFF, &wram.memory);
        map.write_u32(0x02000000, 0xE0812001);
        map.write_u32(0x02000004, 0xE0812001);
        cpu.fetch(&mut map);
        cpu.fetch(&mut map);
    }

}