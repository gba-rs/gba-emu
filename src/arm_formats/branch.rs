use super::{common::Condition, common::Instruction};
use crate::memory::memory_map::MemoryMap;
use crate::operations::arm_arithmetic::add;
use crate::{cpu::cpu::CPU, cpu::cpu::InstructionSet,cpu::cpu::ARM_PC,cpu::cpu::THUMB_PC};
use crate::cpu::cpu::ARM_LR;
use std::fmt;

pub struct Branch {
    pub condition: Condition,
    pub link: bool,
    pub offset: u32,
}

impl From<u32> for Branch {
    fn from(value: u32) -> Branch {
        return Branch {
            offset: (value & 0xFF_FFFF),
            link: ((value & 0x100_0000) >> 24) != 0,
            condition: Condition::from((value & 0xF000_0000) >> 28)
        }
    }
}

impl fmt::Debug for Branch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "B");
        if self.link {
            write!(f, "L");
        }

        write!(f, "{:?}", self.condition);

        let mut offset = (self.offset << 2) as u32;

        if ((offset >> 25) & 0x1) != 0 {
            offset = offset | 0xFC00_0000;
        }

        let (value, _) = add(offset, 8);

        write!(f, " #{:X}", value)
    }
}

impl Instruction for Branch {
    fn execute(&self, cpu: &mut CPU, _mem_map: &mut MemoryMap) {
        let current_pc = if cpu.current_instruction_set == InstructionSet::Arm { ARM_PC } else { THUMB_PC };
        let current_pc_value = cpu.get_register(current_pc) + 4; // because pipeline bullshit
        let mut offset = (self.offset << 2) as u32;

        if ((offset >> 25) & 0x1) != 0 {
            offset = offset | 0xFC00_0000;
        }

        // Setting the link register
        if self.link {
            // The current PC is 8 ahead but we want to get the next instruction so we subtract 4
            cpu.set_register(ARM_LR, current_pc_value - 4); 
        }

        // Adding the offset to the PC
        let (value, _) = add(current_pc_value, offset);

        cpu.set_register(current_pc, value);
    }

    fn asm(&self) -> String {
        return format!("{:?}", self);
    }
}