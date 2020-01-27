use crate::operations::instruction::Instruction;
use crate::memory::memory_map::MemoryMap;
use crate::operations::{arm_arithmetic};
use crate::cpu::{cpu::CPU, cpu::THUMB_SP};
use std::fmt;

pub struct AddOffsetSP{
    pub sign: bool,
    pub immediate: u32
}

impl From<u16> for AddOffsetSP {
    fn from(value: u16) -> AddOffsetSP {
        return AddOffsetSP {
            sign: ((value & 0x80) >> 7) != 0,
            immediate: ((value & 0x7F) as u32) << 2
        };
    }
}

impl Instruction for AddOffsetSP {
    fn execute(&self, cpu: &mut CPU, _mem_map: &mut MemoryMap) {
        let stack_pointer = cpu.get_register(THUMB_SP);
        if self.sign {
            let value = ((self.immediate as i64) * -1) as u32;
            let (new_sp, _) = arm_arithmetic::add(stack_pointer, value);
            cpu.set_register(THUMB_SP, new_sp);
        } else {
            let (new_sp, _) = arm_arithmetic::add(stack_pointer, self.immediate);
            cpu.set_register(THUMB_SP, new_sp);
        }
    }

    fn asm(&self) -> String{
        return format!("{:?}", self);
    }
    fn cycles(&self) -> u32 {return 1;} // 1s
}

impl fmt::Debug for AddOffsetSP {
    fn fmt( & self, f: & mut fmt::Formatter < '_ > ) -> fmt::Result {
        if self.sign {
            write!(f, "ADD sp, #-0x{:X}", self.immediate)
        } else {
            write!(f, "ADD sp, #0x{:X}", self.immediate)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gba::GBA;
    use crate::cpu::{cpu::InstructionSet, cpu::THUMB_SP};
    use std::borrow::{BorrowMut};

    #[test]
    fn add_positive_test() {
        let mut gba: GBA = GBA::default(); 
        gba.cpu.current_instruction_set = InstructionSet::Thumb;

        gba.cpu.set_register(THUMB_SP, 12);

        // SP = 12
        // #imm = 12
        let decode_result = gba.cpu.decode(0xB003);
        match decode_result {
            Ok(mut instr) => {
                (instr.borrow_mut() as &mut dyn Instruction).execute(&mut gba.cpu, &mut gba.mem_map);
            },
            Err(e) => {
                panic!("{:?}", e);
            }
        }

        assert_eq!(24, gba.cpu.get_register(THUMB_SP));
    }

    #[test]
    fn add_negative_test() {
        let mut gba: GBA = GBA::default(); 
        gba.cpu.current_instruction_set = InstructionSet::Thumb;

        gba.cpu.set_register(THUMB_SP, 12);

        // SP = 12
        // #imm = -12
        let decode_result = gba.cpu.decode(0xB083);
        match decode_result {
            Ok(mut instr) => {
                (instr.borrow_mut() as &mut dyn Instruction).execute(&mut gba.cpu, &mut gba.mem_map);
            },
            Err(e) => {
                panic!("{:?}", e);
            }
        }

        assert_eq!(0, gba.cpu.get_register(THUMB_SP));
    }
}