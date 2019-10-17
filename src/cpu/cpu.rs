use crate::formats::{data_processing::DataProcessing, common::Instruction};
use crate::memory::{work_ram::WorkRam};
use crate::memory::{memory_map::MemoryMap};

pub struct cpu {
    registers: [u32; 16],
    wram: WorkRam
}

impl cpu {

    pub fn new() -> cpu {
        return cpu {
            registers: [0; 16],
            wram: WorkRam::new(0)
        };
    }

    pub fn decode(&mut self, mem_map: &mut MemoryMap, instruction: u32) {
        let opcode: u16 = (((instruction >> 16) & 0xFF0) | ((instruction >> 4) & 0x0F)) as u16;
        match opcode {
            0x080  => { // ADD lli
                let format: DataProcessing = DataProcessing::from(instruction);
                format.execute(self, mem_map);
                },
            0x1A0 => { //mov lli
                let format: DataProcessing = DataProcessing::from(instruction);
                format.execute(self, mem_map);
            }
                _ => {},
            }
    }
    pub fn fetch() -> u32 {
        return 0;
    }

}


