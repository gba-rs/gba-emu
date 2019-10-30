#[derive(Clone)]
pub struct ConditionFlags {
    pub negative: bool,
    pub zero: bool,
    pub carry: bool,
    pub signed_overflow: bool
}

impl From<u32> for ConditionFlags {
    fn from(value: u32) -> ConditionFlags {
        return ConditionFlags {
            negative: (value >> 31) != 0,
            zero: ((value >> 30) & 0x01) != 0,
            carry: ((value >> 29) & 0x01) != 0,
            signed_overflow: ((value >> 28) & 0x01) != 0,
        }
    }
}

#[derive(Clone)]
pub struct ControlBits {
    pub fiq_disable: bool,
    pub irq_disable: bool,
    pub state_bit: bool,
    pub mode_bits: u8
}

impl From<u32> for ControlBits {
    fn from(value: u32) -> ControlBits {
        return ControlBits {
            fiq_disable: ((value >> 7) & 0x01) != 0,
            irq_disable: ((value >> 6) & 0x01) != 0,
            state_bit: ((value >> 5) & 0x01) != 0,
            mode_bits: (value & 0x1F) as u8
        }
    }
}

#[derive(Clone)]
pub struct ProgramStatusRegister {
    pub flags: ConditionFlags,
    pub reserved: u32,
    pub control_bits: ControlBits
}

impl From<u32> for ProgramStatusRegister {
    fn from(value: u32) -> ProgramStatusRegister {
        return ProgramStatusRegister {
            flags: ConditionFlags::from(value),
            reserved: ((value & 0xFFF_FF00) >> 8),
            control_bits: ControlBits::from(value)
        }
    }
}