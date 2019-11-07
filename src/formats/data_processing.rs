use super::{common::Condition, common::Instruction};
use crate::{operations::arithmatic};
use crate::memory::memory_map::MemoryMap;
use crate::cpu::cpu::CPU;
use crate::operations::shift::{ShiftType, Shift, apply_shift};


#[derive(Debug, PartialEq)]
pub enum OpCodes {
    AND = 0b0000,
    EOR = 0b0001,
    SUB = 0b0010,
    RSB = 0b0011,
    ADD = 0b0100,
    ADC = 0b0101,
    SBC = 0b0110,
    RSC = 0b0111,
    TST = 0b1000,
    TEQ = 0b1001,
    CMP = 0b1010,
    CMN = 0b1011,
    ORR = 0b1100,
    MOV = 0b1101,
    BIC = 0b1110,
    MVN = 0b1111,
    Error
}

impl From<u8> for OpCodes {
    fn from(value: u8) -> OpCodes {
        match value {
            0b0000 => return OpCodes::AND,
            0b0001 => return OpCodes::EOR,
            0b0010 => return OpCodes::SUB,
            0b0011 => return OpCodes::RSB,
            0b0100 => return OpCodes::ADD,
            0b0101 => return OpCodes::ADC,
            0b0110 => return OpCodes::SBC,
            0b0111 => return OpCodes::RSC,
            0b1000 => return OpCodes::TST,
            0b1001 => return OpCodes::TEQ,
            0b1010 => return OpCodes::CMP,
            0b1011 => return OpCodes::CMN,
            0b1100 => return OpCodes::ORR,
            0b1101 => return OpCodes::MOV,
            0b1110 => return OpCodes::BIC,
            0b1111 => return OpCodes::MVN,
            _ => return panic!("ahh")
        }
    }
}


pub struct DataProcessing {
    pub op1_register: u8,
    pub destination_register: u8,
    pub operand2: DataProcessingOperand,
    pub opcode: u8,
    pub set_condition: bool,
    pub immediate_operand: bool,
    pub condition: Condition
}

impl From<u32> for DataProcessing {
    fn from(value: u32) -> DataProcessing {
        return DataProcessing {
            op1_register: ((value & 0xF_0000) >> 16) as u8,
            destination_register: ((value & 0xF000) >> 12) as u8,
            operand2: DataProcessingOperand::from(value),
            opcode: ((value & 0x1E0_0000) >> 21) as u8,
            set_condition: ((value & 0x10_0000) >> 20) != 0,
            immediate_operand: ((value & 0x200_0000) >> 25) != 0,
            condition: Condition::from((value & 0xF000_0000) >> 28)
        }
    }
}

impl DataProcessing {
    pub fn barrel_shifter(&mut self, cpu: &mut CPU) -> u32 {
        let mut op2: u32;

        if self.operand2.immediate {
            op2 = (self.operand2.immediate_value as u32).rotate_right((self.operand2.rotate as u32) * 2);
        } else {
            op2 = cpu.get_register(self.operand2.rm);
            let shift_amount: u32;
            if self.operand2.shift.immediate {
                shift_amount = self.operand2.shift.shift_amount as u32;
            } else {
                shift_amount = cpu.get_register(self.operand2.shift.shift_register);
            }
            // TODO apply shift here
        }

        return op2;
    }
}

pub struct DataProcessingOperand {
    pub shift: Shift,
    pub rm: u8,
    pub rotate: u8,
    pub immediate_value: u8,
    pub immediate: bool
}

impl From<u32> for DataProcessingOperand {
    fn from(value: u32) -> DataProcessingOperand {
        return DataProcessingOperand {
            shift: Shift::from(value),
            rm: (value & 0xF) as u8,
            rotate: ((value & 0xF00) >> 8) as u8,
            immediate_value: (value & 0xFF) as u8,
            immediate: ((value & 0x200_0000) >> 25) != 0
        }
    }
}

impl Instruction for DataProcessing {
    fn execute(&mut self, cpu: &mut CPU, mem_map: &mut MemoryMap) {
        let op2 = self.barrel_shifter(cpu);
        match OpCodes::from(self.opcode) {
            OpCodes::AND => { //and
                let value = cpu.get_register(self.op1_register) & op2;
                cpu.set_register(self.destination_register, value);
            }
            OpCodes::EOR => { //eor
                let value = cpu.get_register(self.op1_register) ^ op2;
                cpu.set_register(self.destination_register, value);
            }
            OpCodes::SUB  => { //sub
                let (value, flags) =
                    arithmatic::sub(cpu.get_register(self.op1_register), op2);
                cpu.set_register(self.destination_register, value);
            },
            OpCodes::RSB => { //rsb
                let (value, flags) =
                    arithmatic::rsb(cpu.get_register(self.op1_register), op2);
                cpu.set_register(self.destination_register, value);
            },
            OpCodes::ADD => { //add
//                println!("Adding {:X} + {:X}", cpu.registers[self.op1_register as usize], op2);
                let (value, flags) =
                    arithmatic::add(cpu.get_register(self.op1_register), op2);
                cpu.set_register(self.destination_register, value);
            },
            OpCodes::ADC => { //ADC
                let (value, flags) =
                    arithmatic::adc(cpu.get_register(self.op1_register), op2);
                cpu.set_register(self.destination_register, value);
            },
            OpCodes::SBC => { //SBC
                let (value, flags) =
                    arithmatic::sbc(cpu.get_register(self.op1_register), op2);
                cpu.set_register(self.destination_register, value);
            },
            OpCodes::RSC => { //RSC
                let (value, flags) =
                    arithmatic::rsc(cpu.get_register(self.op1_register), op2);
                cpu.set_register(self.destination_register, value);
            },
            OpCodes::TST => { //TST
                //todo: when flags are in set it equal to flags return
            },
            OpCodes::TEQ => { //TEQ
                //todo: when flags are in set it equal to flags return
            },
            OpCodes::CMP => { //cmp

                //todo: when flags are in set it equal to flags return
            },
            OpCodes::CMN => { //cmn
                //todo: when flags are in set it equal to flags return
            },
            OpCodes::MOV => { //mov
                cpu.set_register(self.destination_register, op2);
            },
            OpCodes::BIC => { // bic
                cpu.set_register(self.destination_register,(!op2 & self.op1_register as u32));
            },
            OpCodes::MVN => { // MVN
                cpu.set_register(self.destination_register,!op2);
            },
            _ => {
                panic!("{:X}", self.opcode);
            }
        }
    }
}

// Unit Tests

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn dataprocessing_zero() {
        let a: DataProcessing = DataProcessing::from(0x00000000);
        assert_eq!(a.destination_register, 0);
        assert_eq!(a.op1_register, 0);
    }

    #[test]
    fn dataprocessing_max() {
        let a: DataProcessing = DataProcessing::from(0xFFFFFFFF);
        assert_eq!(a.destination_register, 0b1111);
        assert_eq!(a.op1_register, 0b1111);
        assert_eq!(a.opcode, 0b1111);
        assert_eq!(a.condition, Condition::Error);
        assert_eq!(a.immediate_operand, true);
        assert_eq!(a.set_condition, true);
    }

}