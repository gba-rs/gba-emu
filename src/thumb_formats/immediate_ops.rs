use crate::cpu::cpu::CPU;
use crate::memory::memory_map::MemoryMap;
use crate::operations::instruction::Instruction;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum OpCodes {
    MOV = 0,
    CMP = 1,
    ADD  = 2,
    SUB = 3,
}

impl From<u8> for OpCodes {
    fn from(value: u8) -> OpCodes {
        match value {
            0b00 => return OpCodes::MOV,
            0b01 => return OpCodes::CMP,
            0b10 => return OpCodes::ADD,
            0b11 => return OpCodes::SUB,
            _=> panic!("Error in 8-bit immediate instruction processing opcode")
        }
    }
}

pub struct ImmediateOp {
    pub op: OpCodes,
    pub destination_register: u8,
    pub immediate: u8
}

impl From<u16> for ImmediateOp {
    fn from(value: u16) -> ImmediateOp {
        return ImmediateOp {
            op: OpCodes::from(((value & 0x1800) >> 11) as u8),
            destination_register: ((value & 0x0700) >> 8) as u8,
            immediate: (value & 0x00FF) as u8
        };
    }
}

impl Instruction for ImmediateOp {
    fn execute(&self, cpu: &mut CPU, mem_map: &mut MemoryMap) {
        // Unimplemented
    }
    fn asm(&self) -> String {
        return format!("{:?} r{}, #0x{:X}", self.op, self.destination_register, self.immediate);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation_add() {
        let format = ImmediateOp::from(0x3400);
        assert_eq!(format.op, OpCodes::ADD);
        assert_eq!(format.destination_register, 4);
        assert_eq!(format.immediate, 0);
    }

    #[test]
    fn test_creation_mov() {
        let format = ImmediateOp::from(0x2754);
        assert_eq!(format.op, OpCodes::MOV);
        assert_eq!(format.destination_register, 7);
        assert_eq!(format.immediate, 84);
    }

    #[test]
    fn test_creation_sub() {
        let format = ImmediateOp::from(0x3F7F);
        assert_eq!(format.op, OpCodes::SUB);
        assert_eq!(format.destination_register, 7);
        assert_eq!(format.immediate, 127);
    }

    #[test]
    fn test_creation_cmp() {
        let format = ImmediateOp::from(0x2AFF);
        assert_eq!(format.op, OpCodes::CMP);
        assert_eq!(format.destination_register, 2);
        assert_eq!(format.immediate, 255);
    }
}