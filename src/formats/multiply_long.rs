use super::{common::Condition, common::Instruction};
use crate::{operations::arithmatic};
use crate::memory::memory_map::MemoryMap;
use crate::cpu::cpu::CPU;

pub struct MultiplyLong {
    pub condition: Condition,        // Cond
    pub unsigned: bool,              // U
    pub accumulate: bool,            // A
    pub set_condition: bool,         // S
    pub destination_register_hi: u8, // RdHi (Upper 32 bits of value to add when accumulate bit is set)
    pub destination_register_lo: u8, // RdLo (Lower 32 bits of value to add when accumulate bit is set)
    pub op2_register: u8,            // Rs
    pub op1_register: u8             // Rm

}

impl From<u32> for MultiplyLong {
    fn from(value: u32) -> MultiplyLong {
        return MultiplyLong {
            condition: Condition::from((value & 0xF000_0000) >> 28),
            unsigned: ((value & 0x40_0000) >> 22) == 0, // (0 = unsigned, 1 = signed)
            accumulate: ((value & 0x20_0000) >> 21) != 0,
            set_condition: ((value & 0x10_0000) >> 20) != 0,
            destination_register_hi: ((value & 0xF_0000) >> 16) as u8,
            destination_register_lo: ((value & 0xF000) >> 12) as u8,
            op2_register: ((value & 0xF00) >> 8) as u8,
            op1_register: (value & 0xF) as u8,
        }
    }
}

impl Instruction for MultiplyLong {
    fn execute(&mut self, cpu: &mut CPU, mem_map: &mut MemoryMap) {
        if self.accumulate { // MLA
            let (value, flags) = arithmatic::mlal(
                cpu.get_register(self.op1_register),
                cpu.get_register(self.op2_register),
                cpu.get_register(self.op3_register));
            cpu.set_register(self.destination_register, value);
        }else{ // MUL
            let (value, flags) = arithmatic::mull(
                cpu.get_register(self.op1_register),
                cpu.get_register(self.op2_register));
            cpu.set_register(self.destination_register, value);
        }
    }
}

// Unit Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiply_long_zero() {
        let a: MultiplyLong = MultiplyLong::from(0x00000000);
        assert_eq!(a.condition, Condition::EQ);
        assert_eq!(a.unsigned, true);
        assert_eq!(a.accumulate, false);
        assert_eq!(a.set_condition, false);
        assert_eq!(a.destination_register_hi, 0);
        assert_eq!(a.destination_register_lo, 0);
        assert_eq!(a.op2_register, 0);
        assert_eq!(a.op1_register, 0);
    }

    #[test]
    fn multiply_long_max() {
        let a: MultiplyLong = MultiplyLong::from(0xFFFFFFFF);
        assert_eq!(a.condition, Condition::Error);
        assert_eq!(a.unsigned, false);
        assert_eq!(a.accumulate, true);
        assert_eq!(a.set_condition, true);
        assert_eq!(a.destination_register_hi, 0b1111);
        assert_eq!(a.destination_register_lo, 0b1111);
        assert_eq!(a.op2_register, 0b1111);
        assert_eq!(a.op1_register, 0b1111);
    }
}