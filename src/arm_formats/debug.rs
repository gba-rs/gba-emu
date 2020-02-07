use crate::operations::instruction::Instruction;
use crate::cpu::{cpu::CPU};
use log::{debug};
use crate::gba::memory_bus::MemoryBus;

#[derive(Debug)]
pub struct Debug {
    source_register: u8,
    immidiete: bool,
    hex: bool
}

impl From<u32> for Debug {
    fn from(value: u32) -> Debug {
        return Debug {
            source_register: (value & 0x0F) as u8,
            immidiete: ((value & 0x100) >> 8) != 0,
            hex: ((value & 0x200) >> 9) != 0
        }
    }
}

impl Instruction for Debug {
    fn execute(&self, cpu: &mut CPU, mem_bus: &mut MemoryBus) {
        if self.immidiete {
            if self.hex {
                debug!("{:X}", cpu.get_register(self.source_register));
            } else {
                debug!("{}", cpu.get_register(self.source_register));
            }
        } else {
            // TODO implement null terminated strings here
        }
    }
    fn cycles(&self) -> u32 {return 0;}

    fn asm(&self) -> String {
        return format!("{:?}", self);
    }
}