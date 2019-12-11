use crate::{arm_formats::common::Condition, arm_formats::common::Instruction};
use crate::operations::{thumb_arithmetic};
use crate::memory::memory_map::MemoryMap;
use crate::cpu::{cpu::CPU};

#[derive(Debug, PartialEq)]
pub enum OpCodes {
    ADD = 0b0000,
    SUB = 0b0001,
    ADD_I = 0b0010,
    SUB_I = 0b0011,
    Error
}

impl From<u8> for OpCodes {
    fn from(value: u8) -> OpCodes {
        match value {
            0b0000 => return OpCodes::ADD,
            0b0001 => return OpCodes::SUB,
            0b0010 => return OpCodes::ADD_I,
            0b0011 => return OpCodes::SUB_I,
            _=> panic!("Error in Add Subtract Processing Opcode")
        }
    }
}

pub struct AddSubtract {
    pub op_register: u8,
    pub source_register: u8,
    pub destination_register: u8,
    pub opcode: u8,
    pub immediate_operand: u8
}

impl From<u16> for AddSubtract {
    fn from(value: u16) -> AddSubtract {
        return AddSubtract{
            op_register: ((value >> 6) & 0x7) as u8,
            source_register: ((value >> 3) & 0x7) as u8,
            destination_register: (value & 0x7) as u8,
            opcode: ((value >> 9) & 0x2) as u8,
            immediate_operand: ((value >> 10) &0x1) as u8
        }
    }
}


impl Instruction for AddSubtract {
    fn execute(&mut self, cpu: &mut CPU, _mem_map: &mut MemoryMap) {
        match OpCodes::from(self.opcode) {
            OpCodes::ADD => {
                let (value, flags) = thumb_arithmetic::add(cpu.get_register(self.op_register), cpu.get_register(self.source_register));
                cpu.set_register(self.destination_register, value);
            }
            OpCodes::SUB => {
                let (value, flags) = thumb_arithmetic::sub(cpu.get_register(self.op_register), cpu.get_register(self.source_register));
                cpu.set_register(self.destination_register, value);
            }
            OpCodes::ADD_I => {
                let (value, flags) = thumb_arithmetic::add(cpu.get_register(self.op_register), cpu.get_register(self.source_register));
                cpu.set_register(self.destination_register, value);
            }
            OpCodes::SUB_I => {
                let (value, flags) = thumb_arithmetic::sub(cpu.get_register(self.op_register), cpu.get_register(self.source_register));
                cpu.set_register(self.destination_register, value);
            }
        }
    }
}

// Unit Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let a: AddSubtract = AddSubtract::from(0x1CD1);
        assert_eq!(a.destination_register, 1);
        assert_eq!(a.op_register, 0);
        
    }
}