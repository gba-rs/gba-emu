use crate::{operations::arm_arithmetic};
use crate::cpu::{cpu::CPU, condition::Condition};
use crate::operations::instruction::Instruction;
use crate::memory::memory_bus::MemoryBus;

#[derive(Debug)]
pub struct Multiply {
    pub condition: Condition,     // Cond
    pub accumulate: bool,         // A
    pub set_condition: bool,      // S
    pub destination_register: u8, // Rd
    pub op3_register: u8,         // Rn (ignored unless accumulate bit is set)
    pub op2_register: u8,         // Rs
    pub op1_register: u8          // Rm

}

impl From<u32> for Multiply {
    fn from(value: u32) -> Multiply {
        return Multiply {
            condition: Condition::from((value & 0xF000_0000) >> 28),
            accumulate: ((value & 0x20_0000) >> 21) != 0,
            set_condition: ((value & 0x10_0000) >> 20) != 0,
            destination_register: ((value & 0xF_0000) >> 16) as u8,
            op3_register: ((value & 0xF000) >> 12) as u8,
            op2_register: ((value & 0xF00) >> 8) as u8,
            op1_register: (value & 0xF) as u8,
        }
    }
}

impl Instruction for Multiply {
    fn execute(&self, cpu: &mut CPU, _mem_bus: &mut MemoryBus) -> u32 {
            if self.accumulate { // MLA
                let (value, flags) = arm_arithmetic::mla(
                        cpu.get_register(self.op1_register),
                        cpu.get_register(self.op2_register),
                        cpu.get_register(self.op3_register));
                cpu.set_register(self.destination_register, value);
                if self.set_condition {
                    cpu.cpsr.flags.negative = (value >> 31) != 0;
                    cpu.cpsr.flags.zero = flags.zero;
                    cpu.cpsr.flags.carry = flags.carry;
                }
            }else{ // MUL
                let (value, flags) = arm_arithmetic::mul(
                        cpu.get_register(self.op1_register),
                        cpu.get_register(self.op2_register));
                cpu.set_register(self.destination_register, value);
                if self.set_condition {
                    cpu.cpsr.flags.negative = flags.negative;
                    cpu.cpsr.flags.zero = flags.zero;
                    cpu.cpsr.flags.carry = flags.carry;
                }
            }
        _mem_bus.cycle_clock.get_cycles()
    }

    fn asm(&self) -> String {
        return format!("{:?}", self);
    }
    fn cycles(&self) -> u32 {return 3;}
}

// Unit Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiply_zero() {
        let a: Multiply = Multiply::from(0x00000000);
        assert_eq!(a.condition, Condition::EQ);
        assert_eq!(a.accumulate, false);
        assert_eq!(a.set_condition, false);
        assert_eq!(a.destination_register, 0);
        assert_eq!(a.op3_register, 0);
        assert_eq!(a.op2_register, 0);
        assert_eq!(a.op1_register, 0);
    }

    #[test]
    fn multiply_max() {
        let a: Multiply = Multiply::from(0xFFFFFFFF);
        assert_eq!(a.condition, Condition::Error);
        assert_eq!(a.accumulate, true);
        assert_eq!(a.set_condition, true);
        assert_eq!(a.destination_register, 0b1111);
        assert_eq!(a.op3_register, 0b1111);
        assert_eq!(a.op2_register, 0b1111);
        assert_eq!(a.op1_register, 0b1111);
    }
}