use super::{common::Condition, common::Instruction};
use crate::memory::memory_map::MemoryMap;
use crate::{cpu::cpu::CPU, cpu::cpu::InstructionSet,cpu::cpu::ARM_PC,cpu::cpu::THUMB_PC};

pub struct BranchExchange {
    pub operand_register: u8,
    pub condition: Condition,
    pub mode_bit: u8,
    pub rn: u8
}

impl From<u32> for BranchExchange {
    fn from(value: u32) -> BranchExchange {
        return BranchExchange {
            operand_register: (value & 0x0F) as u8,
            condition: Condition::from((value & 0xF000_0000) >> 28),
            mode_bit: (value & 0x01) as u8,
            rn: (value & 0x0E) as u8
        };
    }
}

impl Instruction for BranchExchange {
    fn execute(&mut self, cpu: &mut CPU, _mem_map: &mut MemoryMap) {
        let new_pc = cpu.get_register(self.rn);

        if self.mode_bit == 0 {
            cpu.current_instruction_set = InstructionSet::Arm;
            cpu.set_register(ARM_PC, new_pc);
            // Flush Pipeline
        } else if self.mode_bit == 1 {
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
        assert_eq!(a.mode_bit, 1);
        let b: BranchExchange = BranchExchange::from(0xD12F_FF1E); //Final bit is 0
        assert_eq!(b.mode_bit, 0);
    }
}
