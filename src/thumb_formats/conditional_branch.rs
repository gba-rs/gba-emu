use crate::operations::instruction::Instruction;
use crate::memory::memory_map::MemoryMap;
use crate::operations::{arm_arithmetic, bitutils::sign_extend_u32};
use crate::cpu::{cpu::CPU, condition::Condition, cpu::THUMB_PC};
use std::fmt;
use crate::gba::memory_bus::MemoryBus;

pub struct ConditionalBranch {
    pub condition: Condition, 
    pub signed_offset: u32
}

impl From<u16> for ConditionalBranch {
    fn from(value: u16) -> ConditionalBranch {
        return ConditionalBranch {
            condition: Condition::from((value as u32 >> 8) & 0xF),
            signed_offset: sign_extend_u32(((value & 0xFF) << 1) as u32, 8)
        };
    }
}

impl Instruction for ConditionalBranch {
    fn execute(&self, cpu: &mut CPU, _mem_bus: &mut MemoryBus) {
        if cpu.check_condition(&self.condition) {
            // execute
            let (signed_offset, _) = arm_arithmetic::add(self.signed_offset, 2);    // Fetch adds the other +2
            let (new_pc, _) = arm_arithmetic::add(cpu.get_register(THUMB_PC), signed_offset);
            cpu.set_register(THUMB_PC, new_pc);
        }
    }

    fn asm(&self) -> String{
        return format!("{:?}", self);
    }
    fn cycles(&self) -> u32 {return 3;} // 2S + 1N

}

impl fmt::Debug for ConditionalBranch {
    fn fmt( & self, f: & mut fmt::Formatter < '_ > ) -> fmt::Result {
        write!(f, "B{:?} #0x{:X}", self.condition, self.signed_offset)
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use crate::gba::GBA;
    use crate::cpu::{cpu::InstructionSet, cpu::THUMB_PC};
    use std::borrow::{BorrowMut};

    #[test]
    fn branch_conditional_negative_offset() {
        let mut gba: GBA = GBA::default(); 
        gba.cpu.current_instruction_set = InstructionSet::Thumb;
        gba.cpu.cpsr.flags.zero = true;

        let decode_result = gba.cpu.decode(0xD0F6);
        match decode_result {
            Ok(mut instr) => {
                (instr.borrow_mut() as &mut dyn Instruction).execute(&mut gba.cpu, &mut gba.memory_bus);
            },
            Err(e) => {
                panic!("{:?}", e);
            }
        }

        assert_eq!(0x08000000 + 2 - 20, gba.cpu.get_register(THUMB_PC));
    }

    #[test]
    fn branch_conditional_positive_offset() {
        let mut gba: GBA = GBA::default(); 
        gba.cpu.current_instruction_set = InstructionSet::Thumb;
        gba.cpu.cpsr.flags.zero = true;

        let decode_result = gba.cpu.decode(0xD00A);
        match decode_result {
            Ok(mut instr) => {
                (instr.borrow_mut() as &mut dyn Instruction).execute(&mut gba.cpu, &mut gba.memory_bus);
            },
            Err(e) => {
                panic!("{:?}", e);
            }
        }

        assert_eq!(0x08000000 + 2 + 20, gba.cpu.get_register(THUMB_PC));
    }

    #[test]
    fn branch_conditional_false_condition() {
        let mut gba: GBA = GBA::default(); 
        gba.cpu.current_instruction_set = InstructionSet::Thumb;
        gba.cpu.cpsr.flags.zero = false;

        let decode_result = gba.cpu.decode(0xD00A);
        match decode_result {
            Ok(mut instr) => {
                (instr.borrow_mut() as &mut dyn Instruction).execute(&mut gba.cpu, &mut gba.memory_bus);
            },
            Err(e) => {
                panic!("{:?}", e);
            }
        }

        assert_eq!(0x08000000, gba.cpu.get_register(THUMB_PC));
    }
}