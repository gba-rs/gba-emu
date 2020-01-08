use super::common::Instruction;
use crate::memory::memory_map::MemoryMap;
use crate::cpu::{cpu::CPU};
use log::{debug};

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
    fn execute(&self, cpu: &mut CPU, _mem_map: &mut MemoryMap) {
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

    fn asm(&self) -> String {
        return format!("{:?}", self);
    }
}