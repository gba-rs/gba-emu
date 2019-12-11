use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct DecodeError {
    pub instruction: u32,
    pub opcode: u16
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error decoding: {:X}, {:X}", self.opcode, self.instruction)
    }
}

impl error::Error for DecodeError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

