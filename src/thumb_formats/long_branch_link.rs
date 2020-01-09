use crate::operations::instruction::Instruction;
use std::fmt;

pub struct BL {
    pub link: u8,
    pub offset: u16,
}

impl From<u16> for BL {
    fn from(value: u16) -> BL {
        return BL {
            link:(value & 0x800) as u8,
            offset:((value & 0x7FF) >> 11),
        }
    }
}