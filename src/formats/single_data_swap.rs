use super::{common::Condition};

struct SingleDataSwap {
    pub offset: u16,
    pub destination_register: u8,
    pub op1_register: u8,
    pub load_store: bool,
    pub write_back: bool,
    pub byte_word: bool,
    pub up_down: bool,
    pub pre_post_indexing: bool,
    pub immediate_operand: bool,
    pub condition: Condition
}

impl From<u32> for SingleDataSwap {
    fn from(value: u32) -> SingleDataSwap {
        return SingleDataSwap {
            offset: (value & 0xF) as u16,
            destination_register: ((value & 0xF000) >> 12) as u8,
            op1_register: ((value & 0xF_0000) >> 16) as u8,
            load_store: ((value & 0x10_0000) >> 20) != 0, // 0 = store, 1 = load
            write_back: ((value & 0x1E0_0000) >> 21) != 0,
            byte_word: ((value & 0x100_0000) >> 22) != 0, // TODO: check the 2nd part of the bitwise statement, don't really know what im looking at here tbh
            up_down: ((value & 0x100_0000) >> 23) != 0, // TODO: check the 2nd part of the bitwise statement, don't really know what im looking at here tbh
            pre_post_indexing: ((value & 0x100_0000) >> 24) != 0, // TODO: check the 2nd part of the bitwise statement, don't really know what im looking at here tbh
            immediate_operand: ((value & 0x200_0000) >> 25) != 0, //offset is an immediate value if = 0
            condition: Condition::from((value & 0xF000_0000) >> 28)
        }
    }
}