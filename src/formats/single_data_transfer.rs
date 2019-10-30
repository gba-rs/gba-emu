use super::{common::Condition, common::Shift};

struct SingleDataTransfer {
    pub offset: SingleDataTransferOperand,
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

impl From<u32> for SingleDataTransfer {
    fn from(value: u32) -> SingleDataTransfer {
        return SingleDataTransfer {
            offset: SingleDataTransferOperand::from(value),
            destination_register: ((value & 0xF000) >> 12) as u8,
            op1_register: ((value & 0xF_0000) >> 16) as u8,
            load_store: ((value & 0x10_0000) >> 20) != 0,
            write_back: ((value & 0x20_0000) >> 21) != 0,
            byte_word: ((value & 0x40_0000) >> 22) != 0,
            up_down: ((value & 0x80_0000) >> 23) != 0,
            pre_post_indexing: ((value & 0x100_0000) >> 24) != 0,
            immediate_operand: ((value & 0x200_0000) >> 25) != 0, //offset is an immediate value if = 0
            condition: Condition::from((value & 0xF000_0000) >> 28)
        }
    }
}


pub struct SingleDataTransferOperand {
    pub shift: Shift,
    pub rm: u8,
    pub immediate_value: u8,
    pub immediate: bool
}

impl From<u32> for SingleDataTransferOperand {
    fn from(value: u32) -> SingleDataTransferOperand {
        return SingleDataTransferOperand {
            shift: Shift::from(value),
            rm: (value & 0xF) as u8,
            immediate_value: (value & 0xFF) as u8,
            immediate: ((value & 0x200_0000) >> 25) != 0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn singledatatransfer_zero() {
        let a: SingleDataTransfer = SingleDataTransfer::from(0x00000000);
        assert_eq!(a.destination_register, 0);
        assert_eq!(a.op1_register, 0);
        assert_eq!(a.pre_post_indexing, false);
        assert_eq!(a.up_down, false);
        assert_eq!(a.byte_word, false);
        assert_eq!(a.load_store, false);
    }

    #[test]
    fn singledatatransfer_max() {
        let a: SingleDataTransfer = SingleDataTransfer::from(0xFFFFFFFF);
        assert_eq!(a.destination_register, 0b1111);
        assert_eq!(a.op1_register, 0b1111);
        assert_eq!(a.condition, Condition::Error);
        assert_eq!(a.immediate_operand, true);
        assert_eq!(a.pre_post_indexing, true);
        assert_eq!(a.up_down, true);
        assert_eq!(a.byte_word, true);
        assert_eq!(a.load_store, true);
    }
}