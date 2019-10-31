use super::{common::Condition, common::Instruction};
use crate::memory::memory_map::MemoryMap;
use crate::cpu::{cpu::CPU, cpu::InstructionSet, cpu::OperatingMode, cpu::ARM_PC, cpu::ARM_LR};

pub struct SoftwareInterrupt {
    pub comment_field_arm: u32,
    pub comment_field_thumb: u32,
    pub condition: Condition,
}

impl From<u32> for SoftwareInterrupt {
    fn from(value: u32) -> SoftwareInterrupt {
        return SoftwareInterrupt {
            comment_field_arm: (value & 0xFF_0000) >> 16,
            comment_field_thumb: value & 0xFF_0000,
            condition: Condition::from((value & 0xF000_0000) >> 28)
        };
    }
}

impl Instruction for SoftwareInterrupt {
    fn execute(&mut self, cpu: &mut CPU, _: &mut MemoryMap) {
        cpu.current_instruction_set = InstructionSet::Arm;
        cpu.operating_mode = OperatingMode::Supervisor;
        cpu.set_spsr(cpu.cpsr);
        let current_pc = cpu.get_register(ARM_PC);
        cpu.set_register(ARM_LR, current_pc + 4); // set LR to the next instruction        
        cpu.set_register(ARM_PC, 0x08);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::{cpu::CPU};
    
    #[test]
    fn test_execute() {

    }
}