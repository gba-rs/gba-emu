use super::{common::Condition};

struct BlockDataTransfer {
    pub register_list: [bool; 16],
    pub base_register: u8,
    pub load_store_bit: bool,
    pub write_back_bit: bool,
    pub psr_force_user_bit: bool,
    pub up_down_bit: bool,
    pub pre_post_indexing_bit: bool,
    pub condition: Condition
}

impl From<u32> for BlockDataTransfer {
    fn from(value: u32) -> BlockDataTransfer {
        let mut temp_reg_list: [bool; 16] = [false; 16];
        for i in 0..16 {
            temp_reg_list[i] = ((value >> i) & 0x01) != 0;
        }

        return BlockDataTransfer {
            register_list: temp_reg_list,
            base_register: ((value >> 16) & 0x0F) as u8,
            load_store_bit: ((value >> 20) & 0x01) != 0,
            write_back_bit: ((value >> 21) & 0x01) != 0,
            psr_force_user_bit: ((value >> 22) & 0x01) != 0,
            up_down_bit: ((value >> 23) & 0x01) != 0,
            pre_post_indexing_bit: ((value >> 24) & 0x01) != 0,
            condition: Condition::from((value & 0xF000_0000) >> 28)
        }
    }
}