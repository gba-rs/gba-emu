use crate::cpu::cpu::CPU;
use crate::memory::memory_map::MemoryMap;
use crate::operations::instruction::Instruction;

pub struct ALU {
    pub op_code: u8,
    pub rs: u8,
    pub rd: u8
}

impl From<u16> for ALU{
    fn from(value: u16) -> ALU {
        return ALU {
            op_code: ((value >> 6) & 0xF) as u8,
            rs:((value >> 3) & 0x7) as u8,
            rd: (value & 0x7) as u8
        }
    }
}