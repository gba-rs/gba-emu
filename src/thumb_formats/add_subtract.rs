pub struct AddSubtract {
    pub op_register: u8,
    pub source_register: u8,
    pub destination_register: u8
}

impl From<u16> for AddSubtract {
    fn from(value: u16) -> AddSubtract {
        return AddSubtract{
            op_register: ((value >> 6) & 0x7) as u8,
            source_register: ((value >> 3) & 0x7) as u8,
            destination_register: (value & 0x7) as u8,
            opcode: ((value >> 9) & 0x2) 
        }
    }
}