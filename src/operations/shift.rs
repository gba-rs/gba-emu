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
            immediate: ((value & 0x10) >> 5) != 0,
        };
    }
}

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

pub fn apply_shift(shift_type: ShiftType, shift_amount: u32, base_value: u32) -> u32{
    let shifted_value;

    match shift_type {
        ShiftType::LogicalLeft => {
            shifted_value = base_value << shift_amount;
            // todo: make sure flags aren't a thing
        }
        ShiftType::LogicalRight => {
            shifted_value = base_value >> shift_amount;
            // todo: make sure flags aren't a thing
        }
        ShiftType::ArithmeticRight => {
            shifted_value = ((base_value as i32) >> shift_amount as i32) as u32;
            // make sure this isn't truncating
        }
        ShiftType::RotateRight => {
            shifted_value = base_value.rotate_right(shift_amount);
        }
        _ => panic!("Shift type fucked up")
    }

    return shifted_value;
}
