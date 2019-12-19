use crate::{arm_formats::common::Condition, arm_formats::common::Instruction};
use crate::memory::memory_map::MemoryMap;
use crate::cpu::{cpu::CPU, cpu::OperatingMode};

#[derive(Debug)]
pub struct SoftwareInterrupt {
    pub comment_immediate: u8
}

impl From<u16> for SoftwareInterrupt {
    fn from(value: u16) -> SoftwareInterrupt{
        return SoftwareInterrupt{
            comment_immediate: (value & 0xFF) as u8
        };
    }
}