use crate::cpu::cpu::CPU;
use crate::operations::instruction::Instruction;
use crate::operations::shift::{Shift, ShiftType, apply_shift};
use crate::operations::logical;
use crate::memory::memory_bus::MemoryBus;

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
    fn execute(&self, cpu: &mut CPU, _mem_bus: &mut MemoryBus) -> u32{
        let base_value = cpu.get_register(self.rs);
        let (shifted_value, carry_out) = apply_shift(base_value, &self.shift, cpu);
        cpu.set_register(self.rd, shifted_value);

        // flags
        match carry_out {
            Some(val) => {
                cpu.cpsr.flags.carry = val != 0;
            },
            None => {}
        }

        let (n, z) = logical::check_flags(shifted_value);
        cpu.cpsr.flags.negative = n;
        cpu.cpsr.flags.zero = z;
        _mem_bus.cycle_clock.get_cycles()
    }

    fn asm(&self) -> String {
        return format!("MOVS r{}, r{}, {:?}", self.rd, self.rs, self.shift);
    }
    fn cycles(&self) -> u32 {return 1;} // 1s

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_op0() {
        let instruction = MoveShifted::from(0x54);

        let rs = 0x02;
        let rd = 0x04;
        let value = 0x10;

        let mut cpu = CPU::new();
        let mut mem_bus = MemoryBus::new();
        cpu.set_register(rs, value);

        instruction.execute(&mut cpu, &mut mem_bus);

        assert_eq!(instruction.shift.shift_type, ShiftType::LogicalLeft);
        assert_eq!(instruction.shift.shift_amount, 1);
        assert_eq!(instruction.rs, 2);
        assert_eq!(instruction.rd, 4);
        assert_eq!(cpu.get_register(rd), value << 1);
    }

    #[test]
    fn test_execute_op1() {
        let instruction = MoveShifted::from(0x894);

        let rs = 0x02;
        let rd = 0x04;
        let value = 0x10;

        let mut cpu = CPU::new();
        let mut mem_bus = MemoryBus::new();
        cpu.set_register(rs, value);

        instruction.execute(&mut cpu, &mut mem_bus);

        assert_eq!(instruction.shift.shift_type, ShiftType::LogicalRight);
        assert_eq!(instruction.shift.shift_amount, 2);
        assert_eq!(instruction.rs, 2);
        assert_eq!(instruction.rd, 4);
        assert_eq!(cpu.get_register(rd), value >> 2);
    }

    #[test]
    fn test_execute_op2() {
        let instruction = MoveShifted::from(0x1094);

        let rs = 0x02;
        let rd = 0x04;
        let value = 0x100;

        let mut cpu = CPU::new();
        let mut mem_bus = MemoryBus::new();
        cpu.set_register(rs, value);

        instruction.execute(&mut cpu, &mut mem_bus);

        assert_eq!(instruction.shift.shift_type, ShiftType::ArithmeticRight);
        assert_eq!(instruction.shift.shift_amount , 2);
        assert_eq!(instruction.rs, 2);
        assert_eq!(instruction.rd, 4);
        assert_eq!(cpu.get_register(rd), 0x40);
    }
}