use crate::{formats::data_processing};
use crate::{memory::MemoryMap};

struct CPU {   
    pub registers: [u32; 16]
}

impl CPU {
    pub fn decode(&mut self, instruction: u32) {
        let opcode: u16 = (((instruction >> 16) & 0xFF0) | ((instruction >> 4) & 0x0F)) as u16;
        match opcode {
            0x080 => { // ADD lli
                let format: data_processing::DataProcessing = data_processing::DataProcessing::from(instruction);
            },
            _ => {},
            }
    }
    pub fn fetch(&mut self, map: memory::MemoryMap) -> u32 {
        let instruction: u32 = map.readU32(self.registers[15]);
        self.decode(instruction);
        self.registers[15] += 0x100000;
        return 0;
    }

}

// Unit Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access_registers(){
        let cpu = CPU{registers: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]};
        let _empty_registers: [u32; 16] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];

        assert_eq!(_empty_registers, cpu.registers);

    }

}