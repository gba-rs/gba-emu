use super::{common::Condition, common::ShiftType, common::Shift, common::Instruction};
use crate::{operations::arithmatic};
use crate::memory::memory_map::MemoryMap;
use crate::cpu::cpu::CPU;

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
            op2 = cpu.registers[self.operand2.rm as usize];
            let shift_amount: u32;
            if self.operand2.shift.immediate {
                shift_amount = self.operand2.shift.shift_amount as u32;
            } else {
                shift_amount = cpu.registers[self.operand2.shift.shift_register as usize];
            }

            match self.operand2.shift.shift_type {
                ShiftType::LogicalLeft => {
                    op2 = op2 << shift_amount;
                    // todo: make sure flags aren't a thing
                },
                ShiftType::LogicalRight => {
                    op2 = op2 >> shift_amount;
                    // todo: make sure flags aren't a thing
                },
                ShiftType::ArithmeticRight => {
                    op2 = ((op2 as i32) >> shift_amount) as u32;
                    // make sure this isn't truncating
                },
                ShiftType::RotateRight => {
                    op2 = op2.rotate_right(shift_amount);
                },
                _ => panic!("Shift type fucked up")
            }
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
        //self.destination_register = op2 as u8;
        match self.opcode {
            0b0100 => {
                println!("Adding {:X} + {:X}", cpu.registers[self.op1_register as usize], op2);
                let (value, flags) =
                    arithmatic::add(cpu.registers[self.op1_register as usize], op2);
                cpu.registers[self.destination_register as usize] = value;
            },
            0b1101 => {
                println!("Moving {:X} = {:X}", self.destination_register, op2);
                cpu.registers[self.destination_register as usize] = op2;
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