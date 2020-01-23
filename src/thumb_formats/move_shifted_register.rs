use crate::cpu::cpu::CPU;
use crate::memory::memory_map::MemoryMap;
use crate::operations::instruction::Instruction;
use crate::operations::shift::{Shift, ShiftType, apply_shift, BarrelCarryOut};
use crate::operations::shift::ShiftType::{LogicalLeft, LogicalRight, ArithmeticRight};

#[derive(Debug)]
pub struct MoveShifted {
    pub shift: Shift,
    pub rs: u8,
    pub rd: u8,
}

impl From<u16> for MoveShifted {
    fn from(value: u16) -> MoveShifted {
        return MoveShifted {
            shift: Shift {
                shift_type: ShiftType::from(((value & 0x1800) >> 11) as u32),
                shift_amount: ((value & 0x7C0) >> 6) as u8,
                shift_register: 0,
                immediate: true
            },
            rs: ((value & 0x38) >> 3) as u8,
            rd: (value & 0x7) as u8,
        };
    }
}

impl Instruction for MoveShifted {
    fn execute(&self, cpu: &mut CPU, _mem_map: &mut MemoryMap) {
        let base_value = cpu.get_register(self.rs);
        let (shifted_value, carry_out) = apply_shift(base_value, &self.shift, cpu);
        cpu.set_register(self.rd, shifted_value);

        // flags
        match carry_out {
            BarrelCarryOut::NewValue(val) => {
                cpu.cpsr.flags.carry = val != 0;
            },
            BarrelCarryOut::OldValue => {}
        }

        cpu.cpsr.flags.negative = if (shifted_value as i32) < 0 { true } else { false };
        cpu.cpsr.flags.zero = if shifted_value == 0 { true } else { false };

    }

    fn asm(&self) -> String {
        return format!("MOVS r{}, r{}, {:?}, #0x{:X} ", self.rd, self.rs, self.shift, self.shift.shift_amount);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::work_ram::WorkRam;

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

        assert_eq!(instruction.shift.shift_type, ShiftType::LogicalLeft);
        assert_eq!(instruction.shift.shift_amount, 1);
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

        assert_eq!(instruction.shift.shift_type, ShiftType::LogicalRight);
        assert_eq!(instruction.shift.shift_amount, 2);
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

        assert_eq!(instruction.shift.shift_type, ShiftType::ArithmeticRight);
        assert_eq!(instruction.shift.shift_amount , 2);
        assert_eq!(instruction.rs, 2);
        assert_eq!(instruction.rd, 4);
        assert_eq!(cpu.get_register(rd), 0x40);
    }
}