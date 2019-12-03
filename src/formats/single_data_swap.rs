use super::{common::Condition};

#[derive(Debug)]
struct SingleDataSwap {
    pub source_register: u8,
    pub destination_register: u8,
    pub base_register: u8,
    pub byte_word: bool,
    pub condition: Condition
}

impl From<u32> for SingleDataSwap {
    fn from(value: u32) -> SingleDataSwap {
        return SingleDataSwap {
            source_register: (value & 0xF) as u8,
            destination_register: ((value & 0xF000) >> 12) as u8,
            base_register: ((value & 0xF_0000) >> 16) as u8,
            byte_word: (value & 0x40_0000 >> 22) != 0,
            condition: Condition::from((value & 0xF000_0000) >> 28)
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn singledataswap_zero() {
        let a: SingleDataSwap = SingleDataSwap::from(0x00000000);
        assert_eq!(a.destination_register, 0);
        assert_eq!(a.source_register, 0);
        assert_eq!(a.base_register, 0);
        assert_eq!(a.byte_word, false);


    }

    #[test]
    fn singledataswap_max() {
        let a: SingleDataSwap = SingleDataSwap::from(0xFFFFFFFF);
        assert_eq!(a.destination_register, 0b1111);
        assert_eq!(a.condition, Condition::Error);
        assert_eq!(a.byte_word, true);
    }
}
