use super::{common::Condition, common::Instruction};
use crate::memory::memory_map::MemoryMap;
use crate::operations::arithmatic::add;
use crate::{cpu::cpu::CPU, cpu::cpu::InstructionSet,cpu::cpu::ARM_PC,cpu::cpu::THUMB_PC, cpu::cpu::REG_MAP};
use crate::cpu::cpu::ARM_LR;

pub struct Branch {
    pub condition: Condition,
    pub link: bool,
    pub offset: u32,
}

impl From<u32> for Branch {
    fn from(value: u32) -> Branch {
        return Branch {
            offset: (value & 0xF),
            link: ((value & 0x100_0000) >> 24) != 0,
            condition: Condition::from((value & 0xF000_0000) >> 28)
        }
    }
}

impl Instruction for Branch {
    fn execute(&mut self, cpu: &mut CPU, mem_map: &mut MemoryMap) {
        let current_pc = if cpu.current_instruction_set == InstructionSet::Arm { ARM_PC } else { THUMB_PC };
        let current_pc_value = cpu.get_register(current_pc);
        println!("reg: {:X}", cpu.get_register(current_pc));
        let mut offset = (self.offset >> 2) as u32;
        println!("offset before {:X}", offset);

        if ((offset >> 23) & 0x1) != 0 {
            offset = offset | 0xFF00_0000;
        }
        println!("offset: {:X}", offset);

        // Setting the link register
        if self.link {
            cpu.set_register(ARM_LR, current_pc_value); // TODO is this plus 4?
        }

        // Adding the offset to the PC
        println!("Before current PC us changed: {:X}", cpu.get_register(current_pc));
        let (value, flags) = add(current_pc_value, offset);
        cpu.set_register(current_pc, value);
        println!("After current PC is set to Value: {:X}", cpu.get_register(current_pc as u8));

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mode(){
        let a: Branch = Branch::from(0xD12F_FF1F); // ‭11010001 0010 1111 1111 1111 0001 1111‬
    }
}