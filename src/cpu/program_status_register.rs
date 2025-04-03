use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
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

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct ControlBits {
    pub fiq_disable: bool,
    pub irq_disable: bool,
    pub state_bit: bool,
    pub mode_bits: u8
}

impl From<u32> for ControlBits {
    fn from(value: u32) -> ControlBits {
        return ControlBits {
            irq_disable: ((value >> 7) & 0x01) != 0,
            fiq_disable: ((value >> 6) & 0x01) != 0,
            state_bit: ((value >> 5) & 0x01) != 0,
            mode_bits: (value & 0x1F) as u8
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
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

impl From<ProgramStatusRegister> for u32 {
    fn from(reg: ProgramStatusRegister) -> u32 {
        let mut value: u32 = 0;
        value |= (reg.flags.negative as u32) << 31;
        value |= (reg.flags.zero as u32) << 30;
        value |= (reg.flags.carry as u32) << 29;
        value |= (reg.flags.signed_overflow as u32) << 28;
        value |= (reg.control_bits.irq_disable as u32) << 7;
        value |= (reg.control_bits.fiq_disable as u32) << 6;
        value |= (reg.control_bits.state_bit as u32) << 5;
        value |= reg.control_bits.mode_bits as u32;

        return value;
    }
}
