use crate::formats::common::Instruction;
use crate::cpu::{cpu::CPU, program_status_register::ConditionFlags,program_status_register::ProgramStatusRegister};
use crate::memory::memory_map::MemoryMap;
use crate::cpu::cpu::{THUMB_PC, THUMB_SP};
use std::fmt;

pub struct LoadAddress {
    pub sp_cp: u8, //bit 11, calculates an address by adding a 10 bit constant to pc or sp
    pub destination: u8, 
    pub word8: u32,
}

impl From<u32> for LoadAddress {
    fn from(value: u32) -> LoadAddress {
        return LoadAddress {
            sp_cp: ((value & 0x800) >> 11) as u8,
            destination: ((value & 0x700) >> 8) as u8,
            word8: (value & 0xFF) << 2, //left shift word8 by 2 should be word8 with 2 00's
        }
    }
}

impl Instruction for LoadAddress {
    fn execute(&mut self, cpu: &mut CPU, _mem_map: &mut MemoryMap) {
        if self.sp_cp == 0 {
            let pc = cpu.get_register(THUMB_PC) + 4;
            //todo: set the first bit in pc to 0
            if pc & (1 << 0) != 0 {
                //there is a 1 as the first bit
                
            }
            let new = pc + self.word8 as u32;
            cpu.set_register(THUMB_PC, new);
        }
        else if self.sp_cp == 1 {
            let sp = cpu.get_register(THUMB_SP);
            let new = sp + self.word8 as u32;
            cpu.set_register(THUMB_SP, new);
        }
    }

//    fn asm(&self) -> String {
//        return format!("{:?:", self};
//    }

}

impl fmt::Debug for LoadAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.sp_cp == 0 {
            write!(f, "{:?}", self.sp_cp);
        }
        write!(f, "{:?}", self.word8)
    }
}

