#![allow(non_camel_case_types)]
use crate::operations::instruction::Instruction;
use crate::operations::{arm_arithmetic};
use crate::memory::memory_map::MemoryMap;
use crate::cpu::{cpu::CPU};
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum OpCodes {
    ADD = 0b00,
    SUB = 0b01,
    ADD_I = 0b10,
    SUB_I = 0b11,
    Error
}

impl From<u8> for OpCodes {
    fn from(value: u8) -> OpCodes {
        match value {
            0b00 => return OpCodes::ADD,
            0b01 => return OpCodes::SUB,
            0b10 => return OpCodes::ADD_I,
            0b11 => return OpCodes::SUB_I,
            _=> panic!("Error in Add Subtract Processing Opcode")
        }
    }
}

pub struct AddSubtract {
    pub op_register: u8,
    pub source_register: u8,
    pub destination_register: u8,
    pub opcode: OpCodes,
    pub immediate: u8
}

impl fmt::Debug for AddSubtract {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.opcode {
            OpCodes::ADD => {
                write!(f, "ADD r{}, r{}, r{}", self.destination_register, self.source_register, self.op_register)
            },
            OpCodes::SUB => {
                write!(f, "SUB r{}, r{}, r{}", self.destination_register, self.source_register, self.op_register)
            },
            OpCodes::ADD_I => {
                write!(f, "ADD r{}, r{}, #0x{:X}", self.destination_register, self.source_register, self.op_register)
            },
            OpCodes::SUB_I => {
                write!(f, "SUB r{}, r{}, #0x{:X}", self.destination_register, self.source_register, self.op_register)
            },
            OpCodes::Error => {
                write!(f, "Error in add subtract")
            }
        }
    }
}

impl From<u16> for AddSubtract {
    fn from(value: u16) -> AddSubtract {
        return AddSubtract{
            op_register: ((value >> 6) & 0x7) as u8,
            source_register: ((value >> 3) & 0x7) as u8,
            destination_register: (value & 0x7) as u8,
            opcode: OpCodes::from(((value >> 9) & 0x3) as u8),
            immediate: ((value >> 10) &0x1) as u8
        }
    }
}

impl Instruction for AddSubtract {
    fn execute(&self, cpu: &mut CPU, _mem_map: &mut MemoryMap) {
        match self.opcode {
            OpCodes::ADD => {
                let (value, flags) = arm_arithmetic::add(cpu.get_register(self.op_register), cpu.get_register(self.source_register));
                cpu.set_register(self.destination_register, value.into());
                cpu.cpsr.flags = flags;
            }
            OpCodes::SUB => {
                let (value, flags) = arm_arithmetic::sub(cpu.get_register(self.source_register), cpu.get_register(self.op_register));
                cpu.set_register(self.destination_register, value.into());
                cpu.cpsr.flags = flags;
            }
            OpCodes::ADD_I => {
                let (value, flags) = arm_arithmetic::add(self.op_register as u32, cpu.get_register(self.source_register));
                cpu.set_register(self.destination_register, value.into());
                cpu.cpsr.flags = flags;
            }
            OpCodes::SUB_I => {
                let (value, flags) = arm_arithmetic::sub(cpu.get_register(self.source_register), self.op_register as u32);
                cpu.set_register(self.destination_register, value.into());
                cpu.cpsr.flags = flags;
            }
            _ => {
                panic!("{:?}", self.opcode);
            }
        }
    }
    
    fn asm(&self) -> String{
        return format!("{:?}", self);
    }
}

// Unit Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_i() {
        let a: AddSubtract = AddSubtract::from(0x1CD1);
        assert_eq!(a.destination_register, 1);
        assert_eq!(a.source_register, 2);
        assert_eq!(a.op_register, 3);
        assert_eq!(a.opcode, OpCodes::ADD_I);
        assert_eq!(a.immediate, 1);
    }

    #[test]
    fn sub_i() {
        let a: AddSubtract = AddSubtract::from(0x1ED1);
        assert_eq!(a.destination_register, 1);
        assert_eq!(a.source_register, 2);
        assert_eq!(a.op_register, 3);
        assert_eq!(a.opcode, OpCodes::SUB_I);
        assert_eq!(a.immediate, 1);
    }

    #[test]
    fn add() {
        let a: AddSubtract = AddSubtract::from(0x18D1);
        assert_eq!(a.destination_register, 1);
        assert_eq!(a.source_register, 2);
        assert_eq!(a.op_register, 3);
        assert_eq!(a.opcode, OpCodes::ADD);
        assert_eq!(a.immediate, 0);
        //196D
    }

    #[test]
    fn sub() {
        let a: AddSubtract = AddSubtract::from(0x1AD1);
        assert_eq!(a.destination_register, 1);
        assert_eq!(a.source_register, 2);
        assert_eq!(a.op_register, 3);
        assert_eq!(a.opcode, OpCodes::SUB);
        assert_eq!(a.immediate, 0);
    }
}