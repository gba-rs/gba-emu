use super::{common::Condition};

pub struct HalfwordRegisterOffset {
    pub halfword_common: HalfwordCommon,
    pub offset_register: u8,
}

pub struct HalfwordImmediateOffset {
    pub halfword_common: HalfwordCommon,
    pub offset_high_nibble: u8,
    pub offset_low_nibble: u8,
}

/* A struct that represents the common data between halfword register offset
   and halfword immediate offset
*/
struct HalfwordCommon {
    pub pre_post_indexing_bit: bool,
    pub up_down_bit: bool,
    pub write_back: bool,
    pub load_store: bool,
    pub base_register: u8,
    pub destination: u8,
    pub s_bit: bool,
    pub h_bit: bool,
    pub condition: Condition
}

impl From<u32> for HalfwordCommon {
    fn from(value : u32) -> HalfwordCommon {
        return HalfwordCommon {
            pre_post_indexing_bit: ((value & 0x0100_0000) >> 24) != 0,
            up_down_bit: ((value & 0x80_0000) >> 23) != 0,
            write_back: ((value & 0x20_0000) >> 21) !=0,
            load_store: ((value & 0x10_0000) >> 20) !=0,
            base_register: ((value & 0xF_0000) >> 16) as u8,
            destination: ((value & 0xF000) >> 12) as u8,
            s_bit: ((value & 0x20) >> 6) != 0,
            h_bit: ((value & 0x10) >> 5) != 0,
            condition: Condition:: from((value & 0xF000_0000) >> 28),
        }
    }
}

impl from<u32> for HalfwordRegisterOffset {
    fn from(value: u32) -> HalfwordRegisterOffset {
        return HalfwordRegisterOffset {
            halfword_common: HalfwordCommon::from(value),
            offset_register: (value & 0xF) as u8,
        }
    }
}

impl from<u32> for HalfwordImmediateOffset {
    fn from(value: u32) -> HalfwordImmediateOffset {
        return HalfwordImmediateOffset {
            halfword_common: HalfwordCommon::from(HalfwordCommon),
            offset_high_nibble: ((value & 0xF00) >> 8) as u8,
            offset_low_nibble: (value & 0xF) as u8,
        }
    }
}
