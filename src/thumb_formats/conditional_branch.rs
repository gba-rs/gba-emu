use crate::operations::instruction::Instruction;
use crate::memory::memory_map::MemoryMap;
use crate::operations::{thumb_arithmetic};
use crate::cpu::{cpu::CPU, condition::Condition};
use std::fmt;

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

impl Instruction for ConditionalBranch {
    fn execute(&self, cpu: &mut CPU, _mem_map: &mut MemoryMap) {

    }

    fn asm(&self) -> String{
        return format!("{:?}", self);
    }
}

impl fmt::Debug for ConditionalBranch {
    fn fmt( & self, f: & mut fmt::Formatter < '_ > ) -> fmt::Result {
        Ok(())
    }
}