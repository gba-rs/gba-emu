use std::error;
use std::fmt;
use super::cpu::InstructionSet;

#[derive(Clone)]
pub struct DecodeError {
    pub instruction: u32,
    pub opcode: u16,
    pub instruction_set: InstructionSet
}

impl fmt::Debug for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error decoding: {:?}, {:X}, {:X}", self.instruction_set, self.opcode, self.instruction)
    }
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error decoding: {:?}, {:X}, {:X}", self.instruction_set, self.opcode, self.instruction)
    }
}

impl error::Error for DecodeError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

