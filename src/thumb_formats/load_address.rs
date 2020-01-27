use crate::operations::{instruction::Instruction, arm_arithmetic};
use crate::cpu::{cpu::CPU};
use crate::memory::memory_map::MemoryMap;
use crate::cpu::cpu::{THUMB_PC, THUMB_SP};
use std::fmt;

pub struct LoadAddress {
    pub sp_pc: bool, //bit 11, calculates an address by adding a 10 bit constant to pc or sp
    pub destination: u8, 
    pub word8: u16,
}

impl From<u16> for LoadAddress {
    fn from(value: u16) -> LoadAddress {
        return LoadAddress {
            sp_pc: ((value & 0x800) >> 11) != 0,
            destination: ((value & 0x700) >> 8) as u8,
            word8: (value & 0xFF) << 2, //left shift word8 by 2 should be word8 with 2 00's
        }
    }
}

impl fmt::Debug for LoadAddress {
    fn fmt( & self, f: & mut fmt::Formatter < '_ > ) -> fmt::Result {
        if self.sp_pc {
            write!(f, "ADD, r{}, SP, #0x{:x}", self.destination, self.word8)
        } else{
            write!(f, "ADD, r{}, PC, #0x{:x}", self.destination, self.word8)
        }
    }
}

impl Instruction for LoadAddress {
    fn execute(&self, cpu: &mut CPU, _mem_map: &mut MemoryMap) {
        if self.sp_pc {
            let sp = cpu.get_register(THUMB_SP);
            let (new, _) = arm_arithmetic::add(sp, self.word8 as u32);
            cpu.set_register(self.destination, new);
        } else {
            let mut pc = cpu.get_register(THUMB_PC) + 2;    // Fetch handles other + 2
            if pc % 2 != 0 {
                //there is a 1 as the first bit so we need to swap that bit to 0
                pc = pc - 1;
            }
            let (new, _) = arm_arithmetic::add(pc, self.word8 as u32);
            cpu.set_register(self.destination, new);  
        }

        
    }
    fn asm(&self) -> String {
        return format!("{:?}", self);
    }
    fn cycles(&self) -> u32 {return 3;} // 2s + 1n... PC being written

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_non_zero() {
        let b: LoadAddress = LoadAddress::from(0xA800);
        assert_eq!(b.sp_pc, true);
        assert_eq!(b.word8, 0);
    }

    #[test]
    fn load_zero() {
        let b: LoadAddress = LoadAddress::from(0xA000);
        assert_eq!(b.sp_pc, false);
        assert_eq!(b.word8, 0);
    }

    #[test]
    fn pc_set() {
        let mut cpu = CPU::new();
        let mut map = MemoryMap::new();
        let b: LoadAddress = LoadAddress::from(0xA000);
        b.execute(&mut cpu, &mut map);
        assert_eq!(cpu.get_register(0), 2);     // 2 here because we are skipping the fetch
    }
    #[test]
    fn sp_set() {
        let mut cpu = CPU::new();
        let mut map = MemoryMap::new();
        cpu.set_register(0, 100);
        let b: LoadAddress = LoadAddress::from(0xA800);
        b.execute(&mut cpu, &mut map);
        assert_eq!(cpu.get_register(0), 0);
    }

    #[test]
    fn immidiete_test() {
        let mut cpu = CPU::new();
        let mut map = MemoryMap::new();
        cpu.set_register(0, 100);       // Set register to something else
        cpu.set_register(THUMB_SP, 10);

        // immidiete is 10 << 2 which is 40
        let b: LoadAddress = LoadAddress::from(0xA80A);
        b.execute(&mut cpu, &mut map);
        assert_eq!(40 + 10, cpu.get_register(0));
    }

}