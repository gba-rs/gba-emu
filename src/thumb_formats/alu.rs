use crate::cpu::cpu::CPU;
use crate::operations::instruction::Instruction;
use crate::operations::{arm_arithmetic, logical};
use crate::operations::shift::{Shift, ShiftType, apply_shift};
use crate::cpu::program_status_register::ConditionFlags;
use std::fmt;
use crate::gba::memory_bus::MemoryBus;

#[derive(Debug, PartialEq)]
pub enum OpCodes {
    AND = 0b0000,
    EOR = 0b0001,
    LSL = 0b0010,
    LSR = 0b0011,
    ASR = 0b0100,
    ADC = 0b0101,
    SBC = 0b0110,
    ROR = 0b0111,
    TST = 0b1000,
    NEG = 0b1001,
    CMP = 0b1010,
    CMN = 0b1011,
    ORR = 0b1100,
    MUL = 0b1101,
    BIC = 0b1110,
    MVN = 0b1111,
    Error
}

impl From<u8> for OpCodes {
    fn from(value: u8) -> OpCodes {
        match value {
            0b0000 => return OpCodes::AND,
            0b0001 => return OpCodes::EOR,
            0b0010 => return OpCodes::LSL,
            0b0011 => return OpCodes::LSR,
            0b0100 => return OpCodes::ASR,
            0b0101 => return OpCodes::ADC,
            0b0110 => return OpCodes::SBC,
            0b0111 => return OpCodes::ROR,
            0b1000 => return OpCodes::TST,
            0b1001 => return OpCodes::NEG,
            0b1010 => return OpCodes::CMP,
            0b1011 => return OpCodes::CMN,
            0b1100 => return OpCodes::ORR,
            0b1101 => return OpCodes::MUL,
            0b1110 => return OpCodes::BIC,
            0b1111 => return OpCodes::MVN,
            _=> panic!("Error in ALU Processing Opcode")
        }
    }
}

pub struct ALU {
    pub opcode: OpCodes,
    pub rs: u8,
    pub rd: u8
}

impl From<u16> for ALU{
    fn from(value: u16) -> ALU {
        return ALU {
            opcode: OpCodes::from(((value >> 6) & 0xF) as u8),
            rs:((value >> 3) & 0x7) as u8,
            rd: (value & 0x7) as u8
        }
    }
}

impl fmt::Debug for ALU {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} r{}, r{}", self.opcode, self.rd, self.rs)
    }
}

impl Instruction for ALU {
    fn execute(&self, cpu: &mut CPU, _mem_bus: &mut MemoryBus) -> u32 {
        let op1 = cpu.get_register(self.rd);
        let op2 = cpu.get_register(self.rs);

        match self.opcode {
            OpCodes::AND => {  
                let (value, (n, z)) = logical::and(op1, op2);
                cpu.set_register(self.rd, value);
                cpu.cpsr.flags.negative = n;
                cpu.cpsr.flags.zero = z;
            },
            OpCodes::EOR =>{
                let (value, (n, z)) = logical::eor(op1, op2);
                cpu.set_register(self.rd, value);
                cpu.cpsr.flags.negative = n;
                cpu.cpsr.flags.zero = z;
            },
            OpCodes::LSL => {
                let shift = Shift {
                    shift_type: ShiftType::LogicalLeft,
                    shift_amount: 0,
                    shift_register: self.rs,
                    immediate: false
                };

                let (value, carry_out) = apply_shift(op1, &shift, cpu);
                let (n, z) = logical::check_flags(value);

                cpu.cpsr.flags.negative = n;
                cpu.cpsr.flags.zero = z;
                match carry_out {
                    Some(new_c_val) => {
                        cpu.cpsr.flags.carry = new_c_val != 0;
                    },
                    None => {}
                }

                cpu.set_register(self.rd, value);
            },
            OpCodes::LSR => {
                let shift = Shift {
                    shift_type: ShiftType::LogicalRight,
                    shift_amount: 0,
                    shift_register: self.rs,
                    immediate: false
                };

                let (value, carry_out) = apply_shift(op1, &shift, cpu);
                let (n, z) = logical::check_flags(value);

                cpu.cpsr.flags.negative = n;
                cpu.cpsr.flags.zero = z;
                match carry_out {
                    Some(new_c_val) => {
                        cpu.cpsr.flags.carry = new_c_val != 0;
                    },
                    None => {}
                }

                cpu.set_register(self.rd, value);
            },
            OpCodes::ASR =>{
                let shift = Shift {
                    shift_type: ShiftType::ArithmeticRight,
                    shift_amount: 0,
                    shift_register: self.rs,
                    immediate: false
                };

                let (value, carry_out) = apply_shift(op1, &shift, cpu);
                let (n, z) = logical::check_flags(value);

                cpu.cpsr.flags.negative = n;
                cpu.cpsr.flags.zero = z;
                match carry_out {
                    Some(new_c_val) => {
                        cpu.cpsr.flags.carry = new_c_val != 0;
                    },
                    None => {}
                }

                cpu.set_register(self.rd, value as u32);
            },
            OpCodes::ADC => {
                let (value, flags) = arm_arithmetic::adc(cpu.get_register(self.rd),  cpu.get_register(self.rs), cpu.cpsr.flags.carry);
                cpu.set_register(self.rd, value);
                cpu.cpsr.flags = flags;
            },
            OpCodes::SBC => {
                let (value, flags) = arm_arithmetic::sbc(cpu.get_register(self.rd),cpu.get_register(self.rs), cpu.cpsr.flags.carry);
                cpu.set_register(self.rd, value);
                cpu.cpsr.flags = flags;
            },
            OpCodes::ROR =>{
                cpu.set_register(self.rd, cpu.get_register(self.rd).rotate_right(cpu.get_register(self.rs)));
                cpu.cpsr.flags = set_flags(self.rd, self.rs, cpu);
            },
            OpCodes::TST => {
                let (_, (n, z)) = logical::and(op1, op2);
                cpu.cpsr.flags.negative = n;
                cpu.cpsr.flags.zero = z;
            },
            OpCodes::NEG=>{
                let (value, flags) = arm_arithmetic::rsb(op2, 0);
                cpu.set_register(self.rd, value);
                cpu.cpsr.flags = flags;
            },
            OpCodes::CMP=>{
                cpu.cpsr.flags = arm_arithmetic::cmp(op1, op2);
            },
            OpCodes::CMN=>{
                cpu.cpsr.flags = arm_arithmetic::cmn(op1, op2);
            },
            OpCodes::ORR=>{
                let (value, (n, z)) = logical::orr(op1, op2);
                cpu.set_register(self.rd, value);
                cpu.cpsr.flags.negative = n;
                cpu.cpsr.flags.zero = z;
            },
            OpCodes::MUL=>{
                let (value, flags) = arm_arithmetic::mul(op1, op2);
                cpu.set_register(self.rd, value);
                cpu.cpsr.flags = flags;
            },
            OpCodes::BIC=>{
                let (value, (n, z)) = logical::bic(op1, op2);
                cpu.set_register(self.rd, value);
                cpu.cpsr.flags.negative = n;
                cpu.cpsr.flags.zero = z;
            },
            OpCodes::MVN=>{
                cpu.set_register(self.rd, !op2);
                let (n, z) = logical::check_flags(!op2);
                cpu.cpsr.flags.negative = n;
                cpu.cpsr.flags.zero = z;
            },
            _ => {
                panic!("Error in processing Thumb ALU instruction");
            }
        }
        _mem_bus.cycle_clock.get_cycles()
    }
    
    fn asm(&self) -> String{
        return format!("{:?}", self);
    }
    fn cycles(&self) -> u32 {return 1;} // 1s

}

fn set_flags(rd: u8, rs: u8, cpu: &mut CPU) -> ConditionFlags{
    let op1 = cpu.get_register(rd);
    let op2 = cpu.get_register(rs);
    let value = (op1 & op2) as u64;
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
//Unit Tests

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_and(){
        let a: ALU = ALU::from(0x400A);
        assert_eq!(a.rs, 1);
        assert_eq!(a.rd, 2);
        assert_eq!(a.opcode, OpCodes::AND);
    }
    #[test]
    fn test_eor(){
        let a: ALU = ALU::from(0x404A);
        assert_eq!(a.rs, 1);
        assert_eq!(a.rd, 2);
        assert_eq!(a.opcode, OpCodes::EOR);
    }
    #[test]
    fn test_lsl(){
        let a: ALU = ALU::from(0x408A);
        assert_eq!(a.rs, 1);
        assert_eq!(a.rd, 2);
        assert_eq!(a.opcode, OpCodes::LSL);
    }
    #[test]
    fn test_lsr(){
        let a: ALU = ALU::from(0x40CA);
        assert_eq!(a.rs, 1);
        assert_eq!(a.rd, 2);
        assert_eq!(a.opcode, OpCodes::LSR);
    }
    #[test]
    fn test_asr(){
        let a: ALU = ALU::from(0x410A);
        assert_eq!(a.rs, 1);
        assert_eq!(a.rd, 2);
        assert_eq!(a.opcode, OpCodes::ASR);
    }
    #[test]
    fn test_adc(){
        let a: ALU = ALU::from(0x414A);
        assert_eq!(a.rs, 1);
        assert_eq!(a.rd, 2);
        assert_eq!(a.opcode, OpCodes::ADC);
    }
    #[test]
    fn test_ror(){
        let a: ALU = ALU::from(0x41CA);
        assert_eq!(a.rs, 1);
        assert_eq!(a.rd, 2);
        assert_eq!(a.opcode, OpCodes::ROR);
    }
    #[test]
    fn test_tst(){
        let a: ALU = ALU::from(0x420A);
        assert_eq!(a.rs, 1);
        assert_eq!(a.rd, 2);
        assert_eq!(a.opcode, OpCodes::TST);
    }
    #[test]
    fn test_neg(){
        let a: ALU = ALU::from(0x424A);
        assert_eq!(a.rs, 1);
        assert_eq!(a.rd, 2);
        assert_eq!(a.opcode, OpCodes::NEG);
    }
    #[test]
    fn test_cmp(){
        let a: ALU = ALU::from(0x428A);
        assert_eq!(a.rs, 1);
        assert_eq!(a.rd, 2);
        assert_eq!(a.opcode, OpCodes::CMP);
    }
    #[test]
    fn test_cmn(){
        let a: ALU = ALU::from(0x42CA);
        assert_eq!(a.rs, 1);
        assert_eq!(a.rd, 2);
        assert_eq!(a.opcode, OpCodes::CMN);
    }
    #[test]
    fn test_orr(){
        let a: ALU = ALU::from(0x430A);
        assert_eq!(a.rs, 1);
        assert_eq!(a.rd, 2);
        assert_eq!(a.opcode, OpCodes::ORR);
    }
    #[test]
    fn test_mul(){
        let a: ALU = ALU::from(0x434A);
        assert_eq!(a.rs, 1);
        assert_eq!(a.rd, 2);
        assert_eq!(a.opcode, OpCodes::MUL);
    }
    #[test]
    fn test_bic(){
        let a: ALU = ALU::from(0x438A);
        assert_eq!(a.rs, 1);
        assert_eq!(a.rd, 2);
        assert_eq!(a.opcode, OpCodes::BIC);
    }
    #[test]
    fn test_mvn(){
        let a: ALU = ALU::from(0x43CA);
        assert_eq!(a.rs, 1);
        assert_eq!(a.rd, 2);
        assert_eq!(a.opcode, OpCodes::MVN);
    }
}