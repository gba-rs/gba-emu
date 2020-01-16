use crate::memory::memory_map::MemoryMap;
use crate::cpu::{cpu::CPU, cpu::InstructionSet, cpu::ARM_PC, cpu::THUMB_PC, condition::Condition};
use log::{info, debug};
use crate::operations::instruction::Instruction;
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
    fn execute(&self, cpu: &mut CPU, _mem_map: &mut MemoryMap) {
        let new_pc = cpu.get_register(self.rn);
        let mode_bit = new_pc & 0x01;
        debug!("Mode bit: {:X}", mode_bit);

        if mode_bit == 0 {
            cpu.current_instruction_set = InstructionSet::Arm;
            cpu.set_register(ARM_PC, new_pc);
            // Flush Pipeline
        } else if mode_bit == 1 {
            cpu.current_instruction_set = InstructionSet::Thumb;
            cpu.set_register(THUMB_PC, new_pc - 1);
            // Flush Pipeline
        }
    }

    fn asm(&self) -> String {
        return format!("{:?}", self);
    }
}

// Unit Tests

#[cfg(test)]
mod tests { 
    use super::*;
    use crate::cpu::cpu::CPU;
    use crate::memory::memory_map::MemoryMap;
    use crate::cpu::cpu::InstructionSet;

    #[test]
    fn test_mode(){
        let a: BranchExchange = BranchExchange::from(0xD12F_FF1F); //Final bit is 1
        let mut cpu = CPU::new();
        let mut map = MemoryMap::new();
        let current_pc = if cpu.current_instruction_set == InstructionSet::Arm { ARM_PC } else { THUMB_PC };
        cpu.set_register(current_pc, 0);
        a.execute(&mut cpu,&mut map);
        assert_eq!(cpu.current_instruction_set, InstructionSet::Arm);
        cpu.set_register(current_pc, 1);
        a.execute(&mut cpu,&mut map);
        assert_eq!(cpu.current_instruction_set, InstructionSet::Thumb);
    }
}
