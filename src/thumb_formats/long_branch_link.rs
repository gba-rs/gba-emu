use crate::operations::instruction::Instruction;
use crate::memory::memory_map::MemoryMap;
use crate::operations::{arm_arithmetic, bitutils::sign_extend_u32};
use crate::cpu::{cpu::CPU, cpu::THUMB_PC, cpu::THUMB_LR};
use std::fmt;

pub struct BL {
    pub offset_bit: bool,
    pub offset: u32,
}

impl From<u16> for BL {
    fn from(value: u16) -> BL {
        return BL {
            offset_bit: ((value & 0x800) >> 11) != 0,
            offset: (value & 0x7FF) as u32,
        }
    }
}

impl Instruction for BL {
    fn execute(&self, cpu: &mut CPU, _mem_map: &mut MemoryMap) {
        if self.offset_bit {
            // H = 1
            // Bottom half of the 23 bit offset (bits 11-1)
            let offset: u32 = self.offset << 1;
            let pc: u32 = cpu.get_register(THUMB_PC);
            let (lr, _) = arm_arithmetic::add(cpu.get_register(THUMB_LR), offset);
            let (final_lr, _) = arm_arithmetic::add(lr, 2);
            cpu.set_register(THUMB_PC, final_lr);
            cpu.set_register(THUMB_LR, pc + 1); // need to set first bit
        } else {
            // H = 0
            // Top half of the 23 bit offset (bits 23-12)
            let offset: u32 = sign_extend_u32(self.offset << 12, 22);
            let pc: u32 = cpu.get_register(THUMB_PC);
            let (lr, _) = arm_arithmetic::add(pc, offset);
            cpu.set_register(THUMB_LR, lr);
        }
    }

    fn asm(&self) -> String{
        return format!("{:?}", self);
    }
    fn cycles(&self) -> u32 {return 4;} // for more info look at 10.3 in dwedit.org/files/arm7tdmi.pdf
}

impl fmt::Debug for BL {
    fn fmt( & self, f: & mut fmt::Formatter < '_ > ) -> fmt::Result {
        write!(f, "BL #0x{:X}", self.offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gba::GBA;
    use crate::cpu::{cpu::InstructionSet, cpu::THUMB_PC};
    use std::borrow::{BorrowMut};

    #[test]
    fn branch_long_negative_offset() {
        let mut gba: GBA = GBA::default(); 
        gba.cpu.current_instruction_set = InstructionSet::Thumb;

        // Offset: 11111111111 11111110110 0 = -20
        //          upper 11    lower 11
        // Upper half instruction   0xF7FF
        match gba.cpu.decode(0xF7FF) {
            Ok(mut instr) => {
                (instr.borrow_mut() as &mut dyn Instruction).execute(&mut gba.cpu, &mut gba.memory_bus.mem_map);
            },
            Err(e) => {
                panic!("{:?}", e);
            }
        }

        // Lower half instruction   0xFFF6
        match gba.cpu.decode(0xFFF6) {
            Ok(mut instr) => {
                (instr.borrow_mut() as &mut dyn Instruction).execute(&mut gba.cpu, &mut gba.memory_bus.mem_map);
            },
            Err(e) => {
                panic!("{:?}", e);
            }
        }

        // PC should be offset by -20
        assert_eq!(0x08000000 + 2 - 20, gba.cpu.get_register(THUMB_PC));

        // LR should be PC + 2
        assert_eq!(0x08000000 + 1, gba.cpu.get_register(THUMB_LR));
    }

    #[test]
    fn branch_long_positive_offset() {
        let mut gba: GBA = GBA::default(); 
        gba.cpu.current_instruction_set = InstructionSet::Thumb;

        // Offset: 00000000000 00000001010 0 = 20
        //          upper 11    lower 11
        // Upper half instruction   0xF0000
        match gba.cpu.decode(0xF000) {
            Ok(mut instr) => {
                (instr.borrow_mut() as &mut dyn Instruction).execute(&mut gba.cpu, &mut gba.memory_bus.mem_map);
            },
            Err(e) => {
                panic!("Error: {:?}", e);
            }
        }

        // Lower half instruction   0xF80A
        match gba.cpu.decode(0xF80A) {
            Ok(mut instr) => {
                (instr.borrow_mut() as &mut dyn Instruction).execute(&mut gba.cpu, &mut gba.memory_bus.mem_map);
            },
            Err(e) => {
                panic!("Error: {:?}", e);
            }
        }

        // PC should be offset by -20
        assert_eq!(0x08000000 + 2 + 20, gba.cpu.get_register(THUMB_PC));

        // LR should be PC + 4
        assert_eq!(0x08000000 + 1, gba.cpu.get_register(THUMB_LR));
    }
}