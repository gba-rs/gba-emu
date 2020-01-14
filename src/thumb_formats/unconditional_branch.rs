use crate::operations::instruction::Instruction;
use crate::memory::memory_map::MemoryMap;
use crate::operations::{arm_arithmetic, bitutils::sign_extend_u32};
use crate::cpu::{cpu::CPU, condition::Condition, cpu::THUMB_PC};
use std::fmt;
use log::{debug};

pub struct UnconditionalBranch {
    pub offset: u32
}

impl From<u16> for UnconditionalBranch {
    fn from(value: u16) -> UnconditionalBranch {
        return UnconditionalBranch {
            offset: sign_extend_u32(((value & 0x7FF) << 1) as u32, 10)
        };
    }
}

impl Instruction for UnconditionalBranch {
    fn execute(&self, cpu: &mut CPU, _mem_map: &mut MemoryMap) {
        // execute
        let (signed_offset, _) = arm_arithmetic::add(self.offset, 4);
        let (new_pc, _) = arm_arithmetic::add(cpu.get_register(THUMB_PC), signed_offset);
        cpu.set_register(THUMB_PC, new_pc);
    }

    fn asm(&self) -> String{
        return format!("{:?}", self);
    }
}

impl fmt::Debug for UnconditionalBranch {
    fn fmt( & self, f: & mut fmt::Formatter < '_ > ) -> fmt::Result {
        write!(f, "B #0x{:X}", self.offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gba::GBA;
    use crate::cpu::{cpu::InstructionSet, cpu::THUMB_PC};
    use std::borrow::{BorrowMut};

    #[test]
    fn branch_unconditional_negative_offset() {
        let mut gba: GBA = GBA::default(); 
        gba.cpu.current_instruction_set = InstructionSet::Thumb;

        let decode_result = gba.cpu.decode(0xE7F6);
        match decode_result {
            Ok(mut instr) => {
                (instr.borrow_mut() as &mut dyn Instruction).execute(&mut gba.cpu, &mut gba.mem_map);
            },
            Err(e) => {
                panic!("{:?}", e);
            }
        }

        assert_eq!(0x08000000 + 4 - 20, gba.cpu.get_register(THUMB_PC));
    }

    #[test]
    fn branch_unconditional_positive_offset() {
        let mut gba: GBA = GBA::default(); 
        gba.cpu.current_instruction_set = InstructionSet::Thumb;

        let decode_result = gba.cpu.decode(0xE00A);
        match decode_result {
            Ok(mut instr) => {
                (instr.borrow_mut() as &mut dyn Instruction).execute(&mut gba.cpu, &mut gba.mem_map);
            },
            Err(e) => {
                panic!("{:?}", e);
            }
        }

        assert_eq!(0x08000000 + 4 + 20, gba.cpu.get_register(THUMB_PC));
    }
}