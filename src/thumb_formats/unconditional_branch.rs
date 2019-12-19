use crate::cpu::cpu::CPU;
use crate::memory::memory_map::MemoryMap;
use crate::arm_formats::common::Instruction;

#[derive(Debug)]
pub struct UnconditionalBranch {
    pub offset: u16
}

impl From<u16> for UnconditionalBranch {
    fn from(value: u16) -> UnconditionalBranch {
        return UnconditionalBranch {
            offset: (value & 0x7FF) as u16,
        };
    }
}