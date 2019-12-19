use crate::cpu::cpu::CPU;
use crate::memory::memory_map::MemoryMap;
use crate::arm_formats::common::Instruction;

pub struct ConditionalBranch {
    pub condition: u8, 
    pub signed_offset: u8
}

impl From<u16> for ConditionalBranch {
    fn from(value: u16) -> ConditionalBranch {
        return ConditionalBranch {
            condition: ((value >> 8) & 0xF) as u8,
            signed_offset: (value & 0xFF) as u8
        };
    }
}