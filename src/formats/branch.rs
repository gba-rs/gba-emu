use super::{common::Condition, common::Instruction};
use crate::memory::memory_map::MemoryMap;
use crate::operations::arithmatic::add;
use crate::{cpu::cpu::CPU, cpu::cpu::InstructionSet,cpu::cpu::ARM_PC,cpu::cpu::THUMB_PC, cpu::cpu::REG_MAP};

pub struct Branch {
    pub condition: Condition,
    pub link: bool,
    pub offset: u8, //could be u18
}

impl From<u32> for Branch {
    fn from(value: u32) -> Branch {
        return Branch {
            offset: (value & 0xF) as u8,
            link: ((value & 0x100_0000) >> 24) != 0,
            condition: Condition::from((value & 0xF000_0000) >> 28)
        }
    }
}

impl Instruction for Branch {
    fn execute(&mut self, cpu: &mut CPU, mem_map: &mut MemoryMap) {
        let current_pc = if cpu.current_instruction_set == InstructionSet::Arm { ARM_PC } else { THUMB_PC };
        let pc_contents = cpu.get_register(current_pc);
        let pc_i32 = pc_contents as i32;
        println!("pc_i32: {:X}, pc_contents: {:X}", pc_i32, pc_contents);
        let (value, flags) = add(pc_i32 as u32, self.offset as u32); //or
//        let (value, flags) = add(pc_contents as u32, self.offset as u32);
        cpu.set_register(pc_contents as u8, value);
        //Should I use our add method here or just default add
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mode(){
        let a: Branch = Branch::from(0xD12F_FF1F); //Final bit is 1 ‭11010001 0010 1111 1111 1111 0001 1111‬
//        let a: Branch = Branch::from(0xD12F_FF1E); //Final bit is 1 ‭110100010010111111111111000111110
    }
}