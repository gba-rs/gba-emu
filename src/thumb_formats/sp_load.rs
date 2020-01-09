use crate::operations::instruction::Instruction;
use std::fmt;

pub struct STR {
    pub link: u8,
    pub destination: u8,
    pub word8: u16,
}

impl From<u16> for STR {
    fn from(value: u16) -> STR {
        return STR {
            link:(value & 0x800) as u8,
            destination: ((value & 0x700) >> 8) as u8,
            word8: (value & 0xFF)

        }
    }
}