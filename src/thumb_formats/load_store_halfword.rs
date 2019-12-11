use crate::cpu::cpu::CPU;
use crate::memory::memory_map::MemoryMap;
use crate::formats::common::Instruction;

struct LoadStoreHalfword {
    load_store_bit: bool,
    immediate: u8,
    rb: u8,
    rd: u8,
}

impl From<u16> for LoadStoreHalfword {
    fn from(value: u16) -> LoadStoreHalfword {
        return LoadStoreHalfword {
            load_store_bit: ((value & 0x800) >> 11) != 0,
            immediate: ((value & 0x7C0) >> 6) as u8,
            rb: ((value & 0x38) >> 3) as u8,
            rd: (value & 0x7) as u8,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation_0s() {
        let load_store_halfword = LoadStoreHalfword::from(0x8000);

        assert_eq!(load_store_halfword.load_store_bit, false);
        assert_eq!(load_store_halfword.immediate, 0);
        assert_eq!(load_store_halfword.rb, 0);
        assert_eq!(load_store_halfword.rd, 0);
    }

    #[test]
    fn test_creation() {
        let load_store_halfword = LoadStoreHalfword::from(0x8A14);

        assert_eq!(load_store_halfword.load_store_bit, true);
        assert_eq!(load_store_halfword.immediate, 8);
        assert_eq!(load_store_halfword.rb, 2);
        assert_eq!(load_store_halfword.rd, 4);
    }
}