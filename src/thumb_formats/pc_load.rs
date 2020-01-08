use crate::arm_formats::common::Instruction;
use crate::cpu::{cpu::CPU, program_status_register::ConditionFlags,program_status_register::ProgramStatusRegister};
use crate::memory::memory_map::MemoryMap;
use crate::cpu::cpu::{THUMB_PC, THUMB_SP};
use std::fmt;

pub struct LDR {
    pub destination: u8,
    pub word8: u16,
}

impl From<u16> for LDR {
    fn from(value: u16) -> LDR {
        return LDR {
            destination: ((value & 0x700) >> 8) as u8,
            word8: (value & 0xFF) << 2,
        }
    }
}

impl fmt::Debug for LDR {
    fn fmt( & self, f: & mut fmt::Formatter < '_ > ) -> fmt::Result {
            write!(f, "LDR {:?}, {:?}", self.destination, self.word8)
    }
}

impl Instruction for LDR {
    fn execute(&self, cpu: &mut CPU, _mem_map: &mut MemoryMap) {
        let mut result = cpu.get_register(THUMB_PC as u8);
        //add PC and Word8
        result = result + self.word8 as u32;
        cpu.set_register(self.destination, result);
    }
    fn asm(&self) -> String {
        return format!("{:?}", self);
    }
}


// Note: The value of the PC will be 4 bytes greater than the address of this instruction, but bit
//1 of the PC is forced to 0 to ensure it is word aligned.
// This should always be 0, but if there are problems this could be the reason.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::cpu::{THUMB_PC, THUMB_SP};

    #[test]
    fn load_non_zero() {
        let b: LDR = LDR::from(0x8800);
        assert_eq!(b.word8, 0);
    }

    #[test]
    fn load_zero() {
        let b: LDR = LDR::from(0x8802);
        assert_eq!(b.word8, 8);
    }

    #[test]
    fn pc_set() {
        let mut cpu = CPU::new();
        let mut map = MemoryMap::new();
        let b: LDR = LDR::from(0x8808);
        b.execute(&mut cpu, &mut map);
        assert_eq!(cpu.get_register(0), 32);
    }
}
