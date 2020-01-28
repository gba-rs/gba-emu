use crate::memory::memory_map::MemoryMap;
use crate::cpu::{cpu::CPU, condition::Condition};
use crate::operations::instruction::Instruction;
use std::fmt;

pub struct SingleDataSwap {
    pub source_register: u8,
    pub destination_register: u8,
    pub base_register: u8,
    pub word: bool,
    pub condition: Condition
}

impl From<u32> for SingleDataSwap {
    fn from(value: u32) -> SingleDataSwap {
        return SingleDataSwap {
            source_register: (value & 0xF) as u8,
            destination_register: ((value & 0xF000) >> 12) as u8,
            base_register: ((value & 0xF_0000) >> 16) as u8,
            word: (value & 0x40_0000 >> 22) != 0,
            condition: Condition::from((value & 0xF000_0000) >> 28)
        };
    }
}

impl fmt::Debug for SingleDataSwap {
    fn fmt( & self, f: & mut fmt::Formatter < '_ > ) -> fmt::Result {
        if self.word {
            write!(f, "SWP{:?} r{}, r{}, [r{}]", self.condition, self.destination_register, self.source_register, self.base_register)
        } else {
            write!(f, "SWPB{:?} r{}, r{}, [r{}]", self.condition, self.destination_register, self.source_register, self.base_register)
        }
    }
}

impl Instruction for SingleDataSwap {
    fn execute(&self, cpu: &mut CPU, mem_map: &mut MemoryMap) {
        // mem read then mem write
        let swap_address = cpu.get_register(self.base_register);
        let source_value = cpu.get_register(self.source_register);
        if self.word {
            let swap_address_contents = mem_map.read_u32(swap_address);
            mem_map.write_u32(swap_address, source_value);
            cpu.set_register(self.destination_register, swap_address_contents);
        } else {
            let swap_address_contents = mem_map.read_u8(swap_address);
            mem_map.write_u8(swap_address, source_value as u8);
            cpu.set_register(self.destination_register, swap_address_contents as u32);
        }
        
    }

    fn asm(&self) -> String {
        return format!("{:?}", self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn singledataswap_zero() {
        let a: SingleDataSwap = SingleDataSwap::from(0x00000000);
        assert_eq!(a.destination_register, 0);
        assert_eq!(a.source_register, 0);
        assert_eq!(a.base_register, 0);
        assert_eq!(a.word, false);


    }

    #[test]
    fn singledataswap_max() {
        let a: SingleDataSwap = SingleDataSwap::from(0xFFFFFFFF);
        assert_eq!(a.destination_register, 0b1111);
        assert_eq!(a.condition, Condition::Error);
        assert_eq!(a.word, true);
    }
}
