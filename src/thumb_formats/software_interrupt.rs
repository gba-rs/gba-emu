use crate::operations::instruction::Instruction;
use crate::cpu::{cpu::CPU,  cpu::InstructionSet, cpu::OperatingMode, cpu::THUMB_PC, cpu::THUMB_LR};
use std::fmt;
use crate::gba::memory_bus::MemoryBus;

pub struct ThumbSoftwareInterrupt {
    pub comment_immediate: u8
}

impl From<u16> for ThumbSoftwareInterrupt {
    fn from(value: u16) -> ThumbSoftwareInterrupt{
        return ThumbSoftwareInterrupt{
            comment_immediate: (value & 0xFF) as u8
        };
    }
}

impl Instruction for ThumbSoftwareInterrupt {
    fn execute(&self, cpu: &mut CPU, _mem_bus: &mut MemoryBus) {
        cpu.set_spsr(cpu.cpsr);
        let current_pc = cpu.get_register(THUMB_PC);
        cpu.set_register(THUMB_LR, current_pc + 2); // set LR to the next instruction (fetch does the other +2)      
        cpu.set_register(THUMB_PC, 0x08);
        cpu.current_instruction_set = InstructionSet::Arm;
        cpu.operating_mode = OperatingMode::Supervisor;
    }

    fn asm(&self) -> String{
        return format!("{:?}", self);
    }
    fn cycles(&self) -> u32 {return 2;} // Coprocessor data operations take 1S + bI incremental cycles to execute, where b is the number of cycles spent in the coprocessor busy-wait loop.

}

impl fmt::Debug for ThumbSoftwareInterrupt {
    fn fmt( & self, f: & mut fmt::Formatter < '_ > ) -> fmt::Result {
        write!(f, "SWI {}", self.comment_immediate)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gba::GBA;
    use crate::cpu::{cpu::InstructionSet, cpu::THUMB_PC, cpu::ARM_PC, cpu::ARM_LR};
    use std::borrow::{BorrowMut};

    #[test]
    fn swi_test() {
        let mut gba: GBA = GBA::default(); 
        gba.cpu.current_instruction_set = InstructionSet::Thumb;
        gba.cpu.operating_mode = OperatingMode::Supervisor;

        gba.cpu.set_register(THUMB_PC, 24);

        let decode_result = gba.cpu.decode(0xDF00);
        match decode_result {
            Ok(mut instr) => {
                (instr.borrow_mut() as &mut dyn Instruction).execute(&mut gba.cpu, &mut gba.memory_bus);
            },
            Err(e) => {
                panic!("{:?}", e);
            }
        }

        assert_eq!(InstructionSet::Arm, gba.cpu.current_instruction_set);
        assert_eq!(OperatingMode::Supervisor, gba.cpu.operating_mode);
        assert_eq!(0x8, gba.cpu.get_register(ARM_PC));
        assert_eq!(26, gba.cpu.get_register(ARM_LR));
    }
}
