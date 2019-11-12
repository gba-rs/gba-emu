use super::{common::Condition, common::Instruction};
use crate::{operations::arithmetic};
use crate::memory::memory_map::MemoryMap;
use crate::operations::shift::{ShiftType, Shift, apply_shift};
use crate::cpu::{cpu::CPU, program_status_register::ConditionFlags,program_status_register::ProgramStatusRegister};


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
            _=> panic!("Error in data processing opcode")
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
            let shift_register_amount = cpu.get_register(self.operand2.shift.shift_register);
            apply_shift(op2, &self.operand2.shift, cpu);
        }

        return op2;
    }
    
    fn set_flags(&mut self, cpu: &mut CPU, value: u64, op1: u32, op2: u32) -> ConditionFlags {
        let carryout: bool = (value >> 32) != 0;
        let op1_sign: bool = (op1 >> 31) != 0;
        let op2_sign: bool = (op2 >> 31) != 0;
        let value_sign: bool = ((value >> 31) & 0x01) != 0;
        return ConditionFlags {
            negative: (value & (0x1 << 31)) != 0,
            zero: value == 0,
            carry: carryout,
            signed_overflow: (op1_sign == op2_sign) && (op1_sign != value_sign)
        };
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
                    arithmetic::sub(cpu.get_register(self.op1_register), op2);
                cpu.set_register(self.destination_register, value);
                cpu.cpsr.flags = flags;
            },
            OpCodes::RSB => { //rsb
                let (value, flags) =
                    arithmetic::rsb(cpu.get_register(self.op1_register), op2);
                cpu.set_register(self.destination_register, value);
                cpu.cpsr.flags = flags;
            },
            OpCodes::ADD => { //add
//                println!("Adding {:X} + {:X}", cpu.registers[self.op1_register as usize], op2);
                let (value, flags) =
                    arithmetic::add(cpu.get_register(self.op1_register), op2);
                cpu.set_register(self.destination_register, value);
                cpu.cpsr.flags = flags;
            },
            OpCodes::ADC => { //ADC
                let (value, flags) =
                    arithmetic::adc(cpu.get_register(self.op1_register), op2);
                cpu.set_register(self.destination_register, value);
                cpu.cpsr.flags = flags;
            },
            OpCodes::SBC => { //SBC
                let (value, flags) =
                    arithmetic::sbc(cpu.get_register(self.op1_register), op2);
                cpu.set_register(self.destination_register, value);
                cpu.cpsr.flags = flags;
            },
            OpCodes::RSC => { //RSC
                let (value, flags) =
                    arithmetic::rsc(cpu.get_register(self.op1_register), op2);
                cpu.set_register(self.destination_register, value);
                cpu.cpsr.flags = flags;
            },
            OpCodes::TST => { //TST AND
                if !self.set_condition { //MRS CPSR
                    let value = cpu.get_register(self.destination_register);
                    cpu.cpsr = ProgramStatusRegister::from(value);
                }
                else{
                    let op1 = cpu.get_register(self.op1_register);
                    let value = (op1 & op2) as u64;
                    cpu.cpsr.flags = DataProcessing::set_flags(self, cpu, value, op1, op2);
                }
            },
            OpCodes::TEQ => { //TEQ EOR
                if !self.set_condition { //MSR CPSR
                    let negative = if cpu.cpsr.flags.negative {0b1000} else {0};
                    let zero = if cpu.cpsr.flags.zero {0b0100} else {0};
                    let carry = if cpu.cpsr.flags.carry {0b0010} else {0};
                    let overflow = if cpu.cpsr.flags.signed_overflow {0b0001} else {0};
                    let mut value = (negative + zero + carry + overflow) as u32;
                    value = value << 26;
                    let irq = if cpu.cpsr.control_bits.irq_disable {0b10000000} else {0};
                    let fiq = if cpu.cpsr.control_bits.fiq_disable {0b01000000} else {0};
                    let state = if cpu.cpsr.control_bits.state_bit {0b00100000} else {0};
                    value += irq + fiq + state;
                    value += cpu.cpsr.control_bits.mode_bits as u32;
                    cpu.set_register(self.op1_register, value);
                }
                else{
                    let op1 = cpu.get_register(self.op1_register);
                    let value = (op1 ^ op2) as u64;
                    cpu.cpsr.flags = DataProcessing::set_flags(self, cpu, value, op1, op2);
                }
            },
            OpCodes::CMP => { //cmp
                if !self.set_condition { //MRS SPSR
                    println!("Going into an SPR");
                    let value = cpu.get_register(self.destination_register);
                    cpu.set_spsr(ProgramStatusRegister::from(value));
                }
                else {
                    cpu.cpsr.flags = arithmetic::cmp(cpu.get_register(self.op1_register), op2);
                }
            },
            OpCodes::CMN => { //cmn
                //check bit 20 is a 0, if so then it is MSR
                if !self.set_condition { // MSR SPSR
                    let spsr = cpu.get_spsr();
                    let negative = if spsr.flags.negative {0b1000} else {0};
                    let zero = if spsr.flags.zero {0b0100} else {0};
                    let carry = if spsr.flags.carry {0b0010} else {0};
                    let overflow = if spsr.flags.signed_overflow {0b0001} else {0};
                    let mut value = (negative + zero + carry + overflow) as u32;
                    value = value << 26;
                    let irq = if spsr.control_bits.irq_disable {0b10000000} else {0};
                    let fiq = if spsr.control_bits.fiq_disable {0b01000000} else {0};
                    let state = if spsr.control_bits.state_bit {0b00100000} else {0};
                    value += irq + fiq + state;
                    value += spsr.control_bits.mode_bits as u32;
                    cpu.set_register(self.op1_register, value);
                }
                else{
                    cpu.cpsr.flags = arithmetic::cmn(cpu.get_register(self.op1_register), op2);
                }
            },
            OpCodes::MOV => { //mov
                cpu.set_register(self.destination_register, op2);
            },
            OpCodes::BIC => { // bic
                cpu.set_register(self.destination_register,!op2 & self.op1_register as u32);
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