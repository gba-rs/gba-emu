use super::{common::Condition, common::Instruction};
use crate::memory::memory_map::MemoryMap;
use crate::cpu::cpu::CPU;

pub struct SoftwareInterrupt {
    pub comment_field_arm: u32,
    pub comment_field_thumb: u32,
    pub condition: Condition,
}

impl From<u32> for SoftwareInterrupt {
    fn from(value: u32) -> SoftwareInterrupt {
        return SoftwareInterrupt {
            comment_field_arm: (value & 0xFF_FFFF) << 16,
            comment_field_thumb: value & 0xFF_FFFF,
            condition: Condition::from((value & 0xF000_0000) >> 28)
        };
    }
}

impl Instruction for SoftwareInterrupt {
    fn execute(&mut self, cpu: &mut CPU, mem_map: &mut MemoryMap) {
        // Enter supervisor mode
        // set the pc to 0x08
        // Save the CPSR to spsr_svc
    }
}