use crate::cpu::cpu::CPU;
use std::fmt;

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
        write!(f, "{:?} ", self.shift_type);
        if self.immediate {
            write!(f, "{:X}", self.shift_amount)
        } else {
            write!(f, "r{}", self.shift_register)
        }
    }
}

#[derive(Copy, Clone)]
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
            LogicalLeft => write!(f, "LSL"),
            LogicalRight => write!(f, "LSR"),
            ArithmeticRight => write!(f, "ASR"),
            RotateRight => write!(f, "ROR"),
            Error => write!(f, "WTF")
        }
    }
}

pub fn apply_shift(base_value: u32, shift: &Shift, cpu: &mut CPU) -> u32 {
    let shifted_value;
    let shift_amount;
    if shift.immediate {
        shift_amount = shift.shift_amount as u32;
    } else {
        shift_amount = cpu.get_register(shift.shift_register) & 0xFF;
    }
    match shift.shift_type {
        ShiftType::LogicalLeft => {
            shifted_value = base_value << (shift_amount as u32);
            // todo: make sure flags aren't a thing
        }
        ShiftType::LogicalRight => {
            shifted_value = base_value >> (shift_amount as u32);
            // todo: make sure flags aren't a thing
        }
        ShiftType::ArithmeticRight => {
            shifted_value = ((base_value as i32) >> shift_amount as i32) as u32;
            // make sure this isn't truncating
        }
        ShiftType::RotateRight => {
            shifted_value = base_value.rotate_right(shift_amount as u32);
        }
        _ => panic!("Shift type fucked up")
    }

    return shifted_value;
}

