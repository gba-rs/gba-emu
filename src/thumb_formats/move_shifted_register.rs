use crate::cpu::cpu::CPU;
use crate::memory::memory_map::MemoryMap;
use crate::operations::instruction::Instruction;
use crate::operations::shift::{Shift, apply_shift};
use crate::operations::shift::ShiftType::{LogicalLeft, LogicalRight, ArithmeticRight};

#[derive(Debug)]
pub struct MoveShifted {
    pub op: u8,
    pub offset: u8,
    pub rs: u8,
    pub rd: u8,
}

impl From<u16> for MoveShifted {
    fn from(value: u16) -> MoveShifted {
        return MoveShifted {
            op: ((value & 0x1800) >> 11) as u8,
            offset: ((value & 0x7C0) >> 6) as u8,
            rs: ((value & 0x38) >> 3) as u8,
            rd: (value & 0x7) as u8,
        };
    }
}

impl Instruction for MoveShifted {
    fn execute(&self, cpu: &mut CPU, mem_map: &mut MemoryMap) {
        match self.op {
            0 => {
                let shift = Shift {
                    shift_amount: self.offset,
                    shift_register: self.rs,
                    shift_type: LogicalLeft,
                    immediate: true,
                };
                let base_value = cpu.get_register(self.rs);
                let shifted_value = apply_shift(base_value, &shift, cpu);
                cpu.set_register(self.rd, shifted_value);
            }
            1 => {
                let shift = Shift {
                    shift_amount: self.offset,
                    shift_register: self.rs,
                    shift_type: LogicalRight,
                    immediate: true,
                };
                let base_value = cpu.get_register(self.rs);
                let shifted_value = apply_shift(base_value, &shift, cpu);
                cpu.set_register(self.rd, shifted_value);
            }
            2 => {
                let shift = Shift {
                    shift_amount: self.offset,
                    shift_register: self.rs,
                    shift_type: ArithmeticRight,
                    immediate: true,
                };
                let base_value = cpu.get_register(self.rs);
                let shifted_value = apply_shift(base_value, &shift, cpu);
                cpu.set_register(self.rd, shifted_value);
            }
            _ => {
                panic!("Move Shifted Register failed to execuse: Invalid OP code")
            }
        }
    }

    fn asm(&self) -> String {
        let shift_code;
        match self.op {
            0 => shift_code = "LSL",
            1 => shift_code = "LSR",
            2 => shift_code = "ASR",
            _ => { panic!("Move Shifted Register Erro: Invalid OP Code")}
        }
        return format!("MOVS r{}, r{}, {}, #0x{:X} ", self.rd, self.rs, shift_code, self.offset);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::work_ram::WorkRam;

    #[test]
    fn test_creation() {
        let move_shifted = MoveShifted::from(0x1FFF);
        let move_shifted_1 = MoveShifted::from(0x15AA);

        assert_eq!(move_shifted.op, 0x3);
        assert_eq!(move_shifted.offset, 0x1F);
        assert_eq!(move_shifted.rs, 0x7);
        assert_eq!(move_shifted.rd, 0x7);

        assert_eq!(move_shifted_1.op, 0x2);
        assert_eq!(move_shifted_1.offset, 0x16);
        assert_eq!(move_shifted_1.rs, 5);
        assert_eq!(move_shifted_1.rd, 2);
    }

    #[test]
    fn test_execute_op0() {
        let mut instruction = MoveShifted::from(0x54);

        let rs = 0x02;
        let rd = 0x04;
        let value = 0x10;

        let mut cpu = CPU::new();
        let mut mem_map = MemoryMap::new();
        cpu.set_register(rs, value);

        instruction.execute(&mut cpu, &mut mem_map);

        assert_eq!(instruction.op, 0);
        assert_eq!(instruction.offset, 1);
        assert_eq!(instruction.rs, 2);
        assert_eq!(instruction.rd, 4);
        assert_eq!(cpu.get_register(rd), value << 1);
    }

    #[test]
    fn test_execute_op1() {
        let mut instruction = MoveShifted::from(0x894);

        let rs = 0x02;
        let rd = 0x04;
        let value = 0x10;

        let mut cpu = CPU::new();
        let mut mem_map = MemoryMap::new();
        cpu.set_register(rs, value);

        instruction.execute(&mut cpu, &mut mem_map);

        assert_eq!(instruction.op, 1);
        assert_eq!(instruction.offset, 2);
        assert_eq!(instruction.rs, 2);
        assert_eq!(instruction.rd, 4);
        assert_eq!(cpu.get_register(rd), value >> 2);
    }

    #[test]
    fn test_execute_op2() {
        let mut instruction = MoveShifted::from(0x1094);

        let rs = 0x02;
        let rd = 0x04;
        let value = 0x100;

        let mut cpu = CPU::new();
        let mut mem_map = MemoryMap::new();
        cpu.set_register(rs, value);

        instruction.execute(&mut cpu, &mut mem_map);

        assert_eq!(instruction.op, 2);
        assert_eq!(instruction.offset, 2);
        assert_eq!(instruction.rs, 2);
        assert_eq!(instruction.rd, 4);
        assert_eq!(cpu.get_register(rd), 0x40);
    }
}