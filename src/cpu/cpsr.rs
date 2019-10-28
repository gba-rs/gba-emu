

pub struct CurrentProgramStatusRegister {
    sign_flag: bool,
    zero_flag: bool,
    carry_flag: bool,
    overflow_flag: bool,
    sticky_overflow: bool,
    irq_disabled: bool,
    fiq_disabled: bool,
    state_bit: bool,
}

impl From<u32> for CurrentProgramStatusRegister {
    pub fn from(value: u32) -> CurrentProgramStatusRegister {
        return CurrentProgramStatusRegister {
            sign_flag: value >> 31,
            zero_flag: (value >> 30) & 0x01,
            carry_flag: (value >> 29) & 0x01,
            overflow_flag: (value >> 28) & 0x01,
            sticky_overflow: (value >> 27) & 0x01,
            irq_disabled: (value >> 7) & 0x01,
            fiq_disabled: (value >> 6) & 0x01,
            state_bit: (value >> 5) & 0x01,
        }
    }
}