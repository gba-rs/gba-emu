use super::{common::Condition};

pub struct HalfwordRegisterOffset {
    pub pre_post_indexing_bit: bool,
    pub up_down_bit: bool,
    pub write_back: bool,
    pub load_store: bool,
    pub base_register: u8,
    pub destination: u8,
    pub s_bit: bool,
    pub h_bit: bool,
    pub offset_register: u8,
    pub condition: Condition
}

impl From<u32> for HalfwordRegisterOffset {
    fn from(value : u32) -> HalfwordRegisterOffset {
        return HalfwordRegisterOffset {
            pre_post_indexing_bit: ((value & 0x0100_0000) >> 24) != 0,
            up_down_bit: ((value & 0x80_0000) >> 23) != 0,
            write_back: ((value & 0x20_0000) >> 21) !=0,
            load_store: ((value & 0x10_0000) >> 20) !=0,
            base_register: ((value & 0xF_0000) >> 16) as u8,
            destination: ((value & 0xF000) >> 12) as u8,
            s_bit: ((value & 0x20) >> 6) != 0,
            h_bit: ((value & 0x10) >> 5) != 0,
            offset_register: (value & 0xF) as u8,
            condition: Condition:: from((value & 0xF000_0000) >> 28)
        }
    }
}
