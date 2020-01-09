use crate::operations::instruction::Instruction;
use crate::cpu::{cpu::CPU, program_status_register::ConditionFlags,program_status_register::ProgramStatusRegister};
use crate::memory::memory_map::MemoryMap;
use crate::cpu::cpu::{THUMB_PC, THUMB_SP};
use std::fmt;

pub struct LoadAddress {
    pub sp_pc: u8, //bit 11, calculates an address by adding a 10 bit constant to pc or sp
    pub destination: u8, 
    pub word8: u16,
}

impl From<u16> for LoadAddress {
    fn from(value: u16) -> LoadAddress {
        return LoadAddress {
            sp_pc: ((value & 0x800) >> 11) as u8,
            destination: ((value & 0x700) >> 8) as u8,
            word8: (value & 0xFF) << 2, //left shift word8 by 2 should be word8 with 2 00's
        }
    }
}

impl fmt::Debug for LoadAddress {
    fn fmt( & self, f: & mut fmt::Formatter < '_ > ) -> fmt::Result {
        if self.sp_pc == 0 {
                write!(f, "Load PC {:?}", self.word8)
        } else{
            write!(f, "{:?}", self.word8)
        }
    }
}

impl Instruction for LoadAddress {
    fn execute(&self, cpu: &mut CPU, _mem_map: &mut MemoryMap) {
        if self.sp_pc == 0 {
            let mut pc = cpu.get_register(THUMB_PC) + 4;
            if pc & (1 << 0) != 0 {
                //there is a 1 as the first bit so we need to swap that bit to 0
                pc = pc - 1;
            }
            let new = pc + self.word8 as u32;
            cpu.set_register(THUMB_PC, new);
        } else if self.sp_pc == 1 {
            let sp = cpu.get_register(THUMB_SP);
            let new = sp + self.word8 as u32;
            cpu.set_register(THUMB_SP, new);
        }
    }
    fn asm(&self) -> String {
        return format!("{:?}", self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::cpu::{THUMB_PC, THUMB_SP};

    #[test]
    fn load_non_zero() {
        let b: LoadAddress = LoadAddress::from(0xA800);
        assert_eq!(b.sp_pc, 1);
        assert_eq!(b.word8, 0);
    }

    #[test]
    fn load_zero() {
        let b: LoadAddress = LoadAddress::from(0xA000);
        assert_eq!(b.sp_pc, 0);
        assert_eq!(b.word8, 0);
    }

    #[test]
    fn pc_set() {
        let mut cpu = CPU::new();
        let mut map = MemoryMap::new();
        let b: LoadAddress = LoadAddress::from(0xA000);
        b.execute(&mut cpu, &mut map);
        assert_eq!(cpu.get_register(THUMB_PC), 4);
    }
    #[test]
    fn sp_set() {
        let mut cpu = CPU::new();
        let mut map = MemoryMap::new();
        let b: LoadAddress = LoadAddress::from(0xA000);
        b.execute(&mut cpu, &mut map);
        assert_eq!(cpu.get_register(THUMB_SP), 0);
    }

}