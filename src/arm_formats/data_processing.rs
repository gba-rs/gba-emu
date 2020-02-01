use crate::operations::{arm_arithmetic, logical};
use crate::operations::shift::{Shift, apply_shift, BarrelCarryOut};
use crate::operations::instruction::Instruction;
use crate::memory::memory_map::MemoryMap;
use crate::cpu::{cpu::CPU, program_status_register::ConditionFlags, program_status_register::ProgramStatusRegister, condition::Condition};
use log::{debug};
use std::fmt;

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
    pub opcode: OpCodes,
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
            opcode: OpCodes::from(((value & 0x1E0_0000) >> 21) as u8),
            set_condition: ((value & 0x10_0000) >> 20) != 0,
            immediate_operand: ((value & 0x200_0000) >> 25) != 0,
            condition: Condition::from((value & 0xF000_0000) >> 28)
        }
    }
}

impl fmt::Debug for DataProcessing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.set_condition {
            write!(f, "{:?}S{:?}", self.opcode, self.condition)?;
        } else {
            write!(f, "{:?}{:?}", self.opcode, self.condition)?;
        }
        
        write!(f, " r{}, ", self.destination_register)?;
        if !(self.opcode == OpCodes::MOV || self.opcode == OpCodes::MVN) {
            write!(f, "r{}, ", self.op1_register)?;
        }

        if self.operand2.immediate {
            let op2 = (self.operand2.immediate_value as u32).rotate_right((self.operand2.rotate as u32) * 2);
            write!(f, "#{:X}", op2)
        } else {
            write!(f, "r{}, ", self.operand2.rm)?;
            write!(f, "{:?} ", self.operand2.shift)
        }
    }
}

impl DataProcessing {
    pub fn barrel_shifter(&self, cpu: &mut CPU) -> (u32, Option<u32>) {
        let op2: u32;
        let carry_out;
        if self.operand2.immediate {
            op2 = (self.operand2.immediate_value as u32).rotate_right((self.operand2.rotate as u32) * 2);
            carry_out = None;
        } else {
            // let shift_register_amount = cpu.get_register(self.operand2.shift.shift_register);
            let (shifted_value, c) = apply_shift(cpu.get_register(self.operand2.rm), &self.operand2.shift, cpu);
            op2 = shifted_value;
            carry_out = c;
        }

        return (op2, carry_out);
    }
    
    fn set_flags(&self, _cpu: &mut CPU, value: u64, op1: u32, op2: u32) -> ConditionFlags {
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
    fn execute(&self, cpu: &mut CPU, _mem_map: &mut MemoryMap) {
        let (op2, carry_out) = self.barrel_shifter(cpu);
        let mut op1 = cpu.get_register(self.op1_register);
        if self.op1_register == 15 {
            op1 += 4;
        }
        
        let current_v = cpu.cpsr.flags.signed_overflow;
        let mut logical_op = false;
        let mut logical_flags: (bool, bool) = (cpu.cpsr.flags.negative, cpu.cpsr.flags.zero);
        
        match self.opcode {
            OpCodes::AND => { //and
                logical_op = true;
                let (value, flags) = logical::and(op1, op2);
                logical_flags = flags;
                cpu.set_register(self.destination_register, value);
            }
            OpCodes::EOR => { //eor
                logical_op = true;
                let (value, flags) = logical::eor(op1, op2);
                logical_flags = flags;
                cpu.set_register(self.destination_register, value);
            },
            OpCodes::ORR => {
                logical_op = true;
                let (value, flags) = logical::orr(op1, op2);
                logical_flags = flags;
                cpu.set_register(self.destination_register, value);
            },
            OpCodes::SUB  => { //sub
                let (value, flags) =
                    arm_arithmetic::sub(op1, op2);
                cpu.set_register(self.destination_register, value);
                if self.set_condition {
                    cpu.cpsr.flags = flags;
                }
            },
            OpCodes::RSB => { //rsb
                let (value, flags) =
                    arm_arithmetic::rsb(op1, op2);
                cpu.set_register(self.destination_register, value);
                if self.set_condition {
                    cpu.cpsr.flags = flags;
                }
            },
            OpCodes::ADD => { //add
               let (value, flags) =
                    arm_arithmetic::add(op1, op2);
                cpu.set_register(self.destination_register, value);
                if self.set_condition {
                    cpu.cpsr.flags = flags;
                }
            },
            OpCodes::ADC => { //ADC
                let (value, flags) =
                    arm_arithmetic::adc(op1, op2);
                cpu.set_register(self.destination_register, value);
                if self.set_condition {
                    cpu.cpsr.flags = flags;
                }
            },
            OpCodes::SBC => { //SBC
                let (value, flags) =
                    arm_arithmetic::sbc(op1, op2);
                cpu.set_register(self.destination_register, value);
                if self.set_condition {
                    cpu.cpsr.flags = flags;
                }
            },
            OpCodes::RSC => { //RSC
                let (value, flags) =
                    arm_arithmetic::rsc(op1, op2);
                cpu.set_register(self.destination_register, value);
                if self.set_condition {
                    cpu.cpsr.flags = flags;
                }
            },
            OpCodes::TST => { //TST AND
                if !self.set_condition { //MRS CPSR
                    let value = cpu.get_register(self.destination_register);
                    cpu.cpsr = ProgramStatusRegister::from(value);
                }
                else{
                    logical_op = true;
                    let (_, flags) = logical::and(op1, op2);
                    logical_flags = flags;
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
                    logical_op = true;
                    let (_, flags) = logical::eor(op1, op2);
                    logical_flags = flags;
                }
            },
            OpCodes::CMP => { //cmp
                if !self.set_condition { //MRS SPSR
                    debug!("Going into an SPR");
                    let value = cpu.get_register(self.destination_register);
                    cpu.set_spsr(ProgramStatusRegister::from(value));
                }
                else {
                    cpu.cpsr.flags = arm_arithmetic::cmp(op1, op2);
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
                    cpu.cpsr.flags = arm_arithmetic::cmn(op1, op2);
                }
            },
            OpCodes::MOV => { //mov
                logical_op = true;
                logical_flags = logical::check_flags(op2);
                cpu.set_register(self.destination_register, op2);
            },
            OpCodes::BIC => { // bic
                logical_op = true;
                let (value, flags) = logical::bic(op1, op2);
                logical_flags = flags;
                cpu.set_register(self.destination_register, value);
            },
            OpCodes::MVN => { // MVN
                logical_op = true;
                logical_flags = logical::check_flags(op2);
                cpu.set_register(self.destination_register, !op2);
            },
            OpCodes::Error => {
                panic!("Hit an error opcode in data processing");
            }
        }

        if self.set_condition {
            if self.destination_register == 15 {
                cpu.cpsr = cpu.get_spsr();  // Arm docs 4.5.4
            } else {
                cpu.cpsr.flags.signed_overflow = current_v; // Arm docs 4.5.1
            }

            if logical_op {
                match carry_out {
                    Some(new_c_val) => {
                        cpu.cpsr.flags.carry = new_c_val != 0;
                    },
                    None => {}
                }

                let (n, z) = logical_flags;
                cpu.cpsr.flags.negative = n;
                cpu.cpsr.flags.zero = z;
            }
        }
    }

    fn asm(&self) -> String {
        return format!("{:?}", self);
    }
    fn cycles(&self) -> u32 {return 1;}
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
        assert_eq!(a.opcode, OpCodes::MVN);
        assert_eq!(a.condition, Condition::Error);
        assert_eq!(a.immediate_operand, true);
        assert_eq!(a.set_condition, true);
    }

}