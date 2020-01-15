use crate::cpu::cpu::CPU;
use crate::memory::memory_map::MemoryMap;
use crate::operations::instruction::Instruction;
use crate::arm_formats::data_processing::{DataProcessing};
use crate::operations::arm_arithmetic::{adc,sbc,cmp,cmn,mul};
use crate::cpu::program_status_register::ConditionFlags;

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

#[derive(Debug)]
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

impl Instruction for ALU {
    fn execute(&self, cpu: &mut CPU, _mem_map: &mut MemoryMap) {
        match self.opcode {
            OpCodes::AND => {  
                let value = cpu.get_register(self.rd) & cpu.get_register(self.rs);
                cpu.set_register(self.rd, value)
            },
            OpCodes::EOR =>{
                let value = cpu.get_register(self.rd) ^ cpu.get_register(self.rs);
                cpu.set_register(self.rd, value)
            },
            OpCodes::LSL => {
                let value = cpu.get_register(self.rd) << cpu.get_register(self.rs);
                cpu.set_register(self.rd, value)
            },
            OpCodes::LSR => {
                let value = cpu.get_register(self.rd) >> cpu.get_register(self.rs);
                cpu.set_register(self.rd, value)
            },
            OpCodes::ASR =>{
                let value = cpu.get_register(self.rd) as i32 >> cpu.get_register(self.rs) as i32;
                cpu.set_register(self.rd, value as u32);
            },
            OpCodes::ADC => {
                cpu.set_register(self.rd, adc(cpu.get_register(self.rd),cpu.get_register(self.rs)).0);
            },
            OpCodes::SBC => {
                cpu.set_register(self.rd, sbc(cpu.get_register(self.rd),cpu.get_register(self.rs)).0);
            },
            OpCodes::ROR =>{
                cpu.set_register(self.rd, cpu.get_register(self.rd).rotate_right(cpu.get_register(self.rs)));
            },
            OpCodes::TST => {
                let op1 = cpu.get_register(self.rd);
                let op2 = cpu.get_register(self.rs);
                let value = (op1 & op2) as u64;
                let carryout: bool = (value >> 32) != 0;
                let op1_sign: bool = (op1 >> 31) != 0;
                let op2_sign: bool = (op2 >> 31) != 0;
                let value_sign: bool = ((value >> 31) & 0x01) != 0;
                cpu.cpsr.flags = ConditionFlags {
                    negative: (value & (0x1 << 31)) != 0,
                    zero: value == 0,
                    carry: carryout,
                    signed_overflow: (op1_sign == op2_sign) && (op1_sign != value_sign)
                };
                
            },
            OpCodes::NEG=>{
                cpu.set_register(self.rd, ((cpu.get_register(self.rs) as i32) * -1) as u32);
                
            },
            OpCodes::CMP=>{
                cpu.cpsr.flags = cmp(cpu.get_register(self.rd), cpu.get_register(self.rs));
            },
            OpCodes::CMN=>{
                cpu.cpsr.flags = cmn(cpu.get_register(self.rd), cpu.get_register(self.rs));
            },
            OpCodes::ORR=>{
                cpu.set_register(self.rd, cpu.get_register(self.rd) | cpu.get_register(self.rs));
            },
            OpCodes::MUL=>{
                cpu.set_register(self.rd, mul(cpu.get_register(self.rd), cpu.get_register(self.rs)).0);
            },
            OpCodes::BIC=>{
                cpu.set_register(self.rd, cpu.get_register(self.rd) & !cpu.get_register(self.rs));
            },
            OpCodes::MVN=>{
                cpu.set_register(self.rd, !cpu.get_register(self.rs));
            },
            _ => {
                panic!("Error in processing Thumb ALU instruction");
            }
        }
    }
    fn asm(&self) -> String{
        return format!("{:?}", self);
    }
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