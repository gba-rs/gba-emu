use crate::formats::{data_processing::DataProcessing, common::Instruction};
use crate::memory::{work_ram::WorkRam};

pub struct Cpu {   
    pub registers: [u32; 16],
    pub wram: WorkRam
}

impl Cpu {

    pub fn new() -> Cpu {
        return Cpu {
            registers: [0; 16],
            wram: WorkRam::new(0)
        };
    }

    pub fn decode(&mut self, instruction: u32) {
        let opcode: u16 = (((instruction >> 16) & 0xFF0) | ((instruction >> 4) & 0x0F)) as u16;
        match opcode {
            0x080  => { // ADD lli
                    let format: DataProcessing = DataProcessing::from(instruction);
                    format.execute();
                },
                _ => {},
            }
    }
    pub fn fetch() -> u32 {
        return 0;
    }

}


