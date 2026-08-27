use crate::operations::instruction::Instruction;
use crate::operations::{arm_arithmetic, bitutils::sign_extend_u32};
use crate::cpu::{cpu::CPU, cpu::THUMB_PC, cpu::THUMB_LR};
use std::fmt;
use crate::memory::memory_bus::MemoryBus;

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
    fn execute(&self, cpu: &mut CPU, _mem_bus: &mut MemoryBus) -> u32 {
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
        _mem_bus.cycle_clock.get_cycles()
    }

    fn asm(&self) -> String{
        return format!("{:?}", self);
    }
    fn cycles(&self) -> u32 {
        // BL is fetched as two Thumb halves: H=0 sets LR (1S), H=1 performs the actual
        // branch (2S+1N=3) for a combined 4 cycles across both halves.
        if self.offset_bit { 3 } else { 1 }
    }
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

    #[test]
    fn branch_long_negative_offset() {
        let mut gba: GBA = GBA::default(); 
        gba.cpu.set_instruction_set(InstructionSet::Thumb);

        // Offset: 11111111111 11111110110 0 = -20
        //          upper 11    lower 11
        // Upper half instruction   0xF7FF
        match gba.cpu.decode(0xF7FF) {
            Ok(instr) => {
                instr.execute(&mut gba.cpu, &mut gba.memory_bus);
            },
            Err(e) => {
                panic!("{:?}", e);
            }
        }

        // Lower half instruction   0xFFF6
        match gba.cpu.decode(0xFFF6) {
            Ok(instr) => {
                instr.execute(&mut gba.cpu, &mut gba.memory_bus);
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
        gba.cpu.set_instruction_set(InstructionSet::Thumb);

        // Offset: 00000000000 00000001010 0 = 20
        //          upper 11    lower 11
        // Upper half instruction   0xF0000
        match gba.cpu.decode(0xF000) {
            Ok(instr) => {
                instr.execute(&mut gba.cpu, &mut gba.memory_bus);
            },
            Err(e) => {
                panic!("Error: {:?}", e);
            }
        }

        // Lower half instruction   0xF80A
        match gba.cpu.decode(0xF80A) {
            Ok(instr) => {
                instr.execute(&mut gba.cpu, &mut gba.memory_bus);
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