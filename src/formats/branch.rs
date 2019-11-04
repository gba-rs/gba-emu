use super::{common::Condition, common::Instruction};
use crate::memory::memory_map::MemoryMap;
use crate::{cpu::cpu::CPU, cpu::cpu::InstructionSet,cpu::cpu::ARM_PC,cpu::cpu::THUMB_PC, cpu::cpu::REG_MAP};
use wasm_bindgen::__rt::core::intrinsics::offset;

struct Branch {
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
        //todo: check if I have to do this
        //. This is shifted left
        //two bits, sign extended to 32 bits, and added to the PC.
        cpu.set_register(pc_contents as u8, pc_contents + self.offset as u32);
        //Should I use our add method here or just default add
    }
}
