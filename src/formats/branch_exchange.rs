use super::{common::Condition, common::Instruction};
use crate::memory::memory_map::MemoryMap;
use crate::{cpu::cpu::CPU, cpu::cpu::InstructionSet,cpu::cpu::ARM_PC,cpu::cpu::THUMB_PC};
use std::fmt;

pub struct BranchExchange {
    pub condition: Condition,
    pub rn: u8
}

impl From<u32> for BranchExchange {
    fn from(value: u32) -> BranchExchange {
        return BranchExchange {
            condition: Condition::from((value & 0xF000_0000) >> 28),
            rn: (value & 0x0F) as u8
        };
    }
}

impl fmt::Debug for BranchExchange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BEX{:?} r{}", self.condition, self.rn)
    }
}

impl Instruction for BranchExchange {
    fn execute(&mut self, cpu: &mut CPU, _mem_map: &mut MemoryMap) {
        let new_pc = cpu.get_register(self.rn);
        let mode_bit = new_pc & 0x01;
        println!("Mode bit: {:X}", mode_bit);

        if mode_bit == 0 {
            cpu.current_instruction_set = InstructionSet::Arm;
            cpu.set_register(ARM_PC, new_pc);
            // Flush Pipeline
        } else if mode_bit == 1 {
            cpu.current_instruction_set = InstructionSet::Thumb;
            cpu.set_register(THUMB_PC, new_pc);
            // Flush Pipeline
        }
    }
}

// Unit Tests

#[cfg(test)]
mod tests { 
    use super::*;

    #[test]
    fn test_mode(){
        let a: BranchExchange = BranchExchange::from(0xD12F_FF1F); //Final bit is 1
        // assert_eq!(a.mode_bit, 1);
        let b: BranchExchange = BranchExchange::from(0xD12F_FF1E); //Final bit is 0
        // assert_eq!(b.mode_bit, 0);
    }
}
