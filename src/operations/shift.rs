use crate::cpu::cpu::CPU;
use std::{fmt, option::Option};

pub struct Shift {
    pub shift_type: ShiftType,
    pub shift_amount: u8,
    pub shift_register: u8,
    pub immediate: bool,
}

impl From<u32> for Shift {
    fn from(value: u32) -> Shift {
        return Shift {
            shift_type: ShiftType::from((value & 0x60) >> 5),
            shift_amount: ((value & 0xF80) >> 7) as u8,
            shift_register: ((value & 0xF00) >> 8) as u8,
            immediate: ((value & 0x10) >> 4) == 0,
        };
    }
}

impl fmt::Debug for Shift {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} ", self.shift_type)?;
        if self.immediate {
            write!(f, "#0x{:X}", self.shift_amount)
        } else {
            write!(f, "r{}", self.shift_register)
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum ShiftType {
    LogicalLeft = 0b00,
    LogicalRight = 0b01,
    ArithmeticRight = 0b10,
    RotateRight = 0b11,
    Error,
}

impl From<u32> for ShiftType {
    fn from(value: u32) -> ShiftType {
        match value {
            0b00 => ShiftType::LogicalLeft,
            0b01 => ShiftType::LogicalRight,
            0b10 => ShiftType::ArithmeticRight,
            0b11 => ShiftType::RotateRight,
            _ => ShiftType::Error
        }
    }
}

impl fmt::Debug for ShiftType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShiftType::LogicalLeft => write!(f, "LSL"),
            ShiftType::LogicalRight => write!(f, "LSR"),
            ShiftType::ArithmeticRight => write!(f, "ASR"),
            ShiftType::RotateRight => write!(f, "ROR"),
            ShiftType::Error => write!(f, "WTF")
        }
    }
}

pub enum BarrelCarryOut {
    NewValue(u32),
    OldValue
}

/// Returns val, carryout
pub fn apply_shift(base_value: u32, shift: &Shift, cpu: &mut CPU) -> (u32, Option<u32>) {
    let shift_amount;
    if shift.immediate {
        return apply_shift_imm(base_value, &shift.shift_type, shift.shift_amount as u32, cpu)
    } else {
        if shift.shift_register == 15 {
            panic!("Can't have RS of 15");
        }
        shift_amount = cpu.get_register(shift.shift_register) & 0xFF;
        return apply_shift_reg(base_value, &shift.shift_type, shift_amount as u32, cpu);
    }
}

fn apply_shift_imm(base_value: u32, shift_type: &ShiftType, shift_amount: u32, cpu: &mut CPU) -> (u32, Option<u32>) {
    let mut shifted_value;
    let carry_out;
    match shift_type {
        ShiftType::LogicalLeft => {
            if shift_amount == 0 {
                carry_out = None;
                shifted_value = base_value << (shift_amount as u32);
            } else {
                shifted_value = base_value << ((shift_amount as i32) - 1);
                carry_out = Some(shifted_value >> 31);
                shifted_value = shifted_value << 1;
            }
        }
        ShiftType::LogicalRight => {
            if shift_amount == 0 {
                shifted_value = 0;
                carry_out = Some(base_value >> 31);
            } else {
                shifted_value = base_value >> ((shift_amount as u32) - 1);
                carry_out = Some(shifted_value & 0x1);
                shifted_value = shifted_value >> 1;
            }
        }
        ShiftType::ArithmeticRight => {
            if shift_amount == 0 {
                let bit = base_value >> 31;
                if bit == 0 {
                    shifted_value = 0;
                    carry_out = Some(0);
                } else {
                    shifted_value = 0xFFFFFFFF;
                    carry_out = Some(1);                    
                }
            } else {
                shifted_value = ((base_value as i32) >> (shift_amount - 1) as u32) as u32;
                carry_out = Some(shifted_value & 0x1);
                shifted_value = ((shifted_value as i32) >> 1 as u32) as u32;
            }
        }
        ShiftType::RotateRight => {
            if shift_amount == 0 {
                let c_in: u32 = (cpu.cpsr.flags.carry as u32) << 31;
                carry_out = Some(base_value & 0x1);
                shifted_value = base_value >> 1;
                shifted_value |= c_in;
            } else {
                shifted_value = base_value.rotate_right(shift_amount as u32);
                carry_out = Some(shifted_value >> 31);
            }
        }
        _ => panic!("Shifts fucked up")
    }

    return (shifted_value, carry_out);
}

fn apply_shift_reg(base_value: u32, shift_type: &ShiftType, shift_amount: u32, cpu: &mut CPU) -> (u32, Option<u32>) {
    if shift_amount > 0 && shift_amount < 32 {
        return apply_shift_imm(base_value, shift_type, shift_amount, cpu);
    }

    if shift_amount == 0 {
        return (base_value, None);
    }

    match shift_type {
        ShiftType::LogicalLeft => {
            if shift_amount == 32 {
                return (0, Some(base_value & 0x1));
            } else {
                return (0, Some(0));
            }
        }
        ShiftType::LogicalRight => {
            if shift_amount == 32 {
                return (0, Some(base_value >> 31));
            } else {
                return (0, Some(0));
            }
        }
        ShiftType::ArithmeticRight => {
            let bit = base_value >> 31;
            if bit == 0 {
                return (0, Some(0));
            } else {
                return (0xFFFFFFFF, Some(1));
            }
        }
        ShiftType::RotateRight => {
            let new_shift_amount = shift_amount % 32;

            if new_shift_amount == 0 {
                return (base_value, Some(base_value >> 31));
            } else {
                return apply_shift_imm(base_value, shift_type, new_shift_amount, cpu);
            }
        }
        _ => panic!("Shifts fucked up")
    }
}

