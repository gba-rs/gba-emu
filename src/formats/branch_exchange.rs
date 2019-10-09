use super::{common::Condition};

struct BranchExchange {
    pub operand_register: u8,
    pub condition: Condition
}

impl From<u32> for BranchExchange {
    fn from(value: u32) -> BranchExchange {
        return BranchExchange {
            operand_register: (value & 0x0F) as u8,
            condition: Condition::from((value & 0xF000_0000) >> 28)
        };
    }
}