use crate::formats::common::Instruction;
use crate::cpu::{cpu::CPU, program_status_register::ConditionFlags,program_status_register::ProgramStatusRegister};
use crate::memory::memory_map::MemoryMap;

pub struct LoadAddress {
    pub sp_cp: u8, //bit 11, calculates an address by adding a 10 bit constant to pc or sp
    pub destination: u8, 
    pub word8: u8
}

impl From<u32> for LoadAddress {
    fn from(value: u32) -> LoadAddress {
        return LoadAddress {
            sp_cp: ((value & 0x800) >> 11) as u8,
            destination: ((value & 0x700) >> 8) as u8,
            word8: (value & 0xFF) as u8,
        }
    }
}

impl Instruction for LoadAddress {
    fn execute(&mut self, cpu: &mut CPU, _mem_map: &mut MemoryMap) {
        let new_pc = cpu.get_register(self.rn);
        if self.sp_cp == 0 {
            //idk how the this is going to be called so maybe return PC?
        }
        else if self.sp_cp == 1 {
            //
        }
        else {
            panic!();
        }
    }
}