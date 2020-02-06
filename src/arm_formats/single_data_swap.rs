use crate::memory::memory_map::MemoryMap;
use crate::cpu::{cpu::CPU, condition::Condition};
use crate::operations::instruction::Instruction;
use std::fmt;
use crate::gba::memory_bus::MemoryBus;

pub struct SingleDataSwap {
    pub source_register: u8,
    pub destination_register: u8,
    pub base_register: u8,
    pub byte: bool,
    pub condition: Condition
}

impl From<u32> for SingleDataSwap {
    fn from(value: u32) -> SingleDataSwap {
        return SingleDataSwap {
            source_register: (value & 0xF) as u8,
            destination_register: ((value & 0xF000) >> 12) as u8,
            base_register: ((value & 0xF_0000) >> 16) as u8,
            byte: ((value & 0x40_0000) >> 22) != 0,
            condition: Condition::from((value & 0xF000_0000) >> 28)
        };
    }
}

impl fmt::Debug for SingleDataSwap {
    fn fmt( & self, f: & mut fmt::Formatter < '_ > ) -> fmt::Result {
        if self.byte {
            write!(f, "SWPB{:?} r{}, r{}, [r{}]", self.condition, self.destination_register, self.source_register, self.base_register)
        } else {
            write!(f, "SWP{:?} r{}, r{}, [r{}]", self.condition, self.destination_register, self.source_register, self.base_register)
        }
    }
}

impl Instruction for SingleDataSwap {
    fn execute(&self, cpu: &mut CPU, mem_bus: &mut MemoryBus) {
        // mem read then mem write
        let swap_address = cpu.get_register(self.base_register);
        let source_value = cpu.get_register(self.source_register);
        if self.byte {
            let swap_address_contents = mem_map.read_u8(swap_address);
            mem_bus.write_u8(swap_address, source_value as u8);
            cpu.set_register(self.destination_register, swap_address_contents as u32);
        } else {
            let swap_address_contents = mem_map.read_u32(swap_address - (swap_address % 4)).rotate_right((swap_address % 4) * 8);
            mem_bus.write_u32(swap_address - (swap_address % 4), source_value);
            cpu.set_register(self.destination_register, swap_address_contents);
        }
        
    }

    fn asm(&self) -> String {
        return format!("{:?}", self);
    }

    fn cycles(&self) -> u32 {return 2;}
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
        assert_eq!(a.byte, false);


    }

    #[test]
    fn singledataswap_max() {
        let a: SingleDataSwap = SingleDataSwap::from(0xFFFFFFFF);
        assert_eq!(a.destination_register, 0b1111);
        assert_eq!(a.condition, Condition::Error);
        assert_eq!(a.byte, true);
    }
}
