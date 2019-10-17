use super::{common::Condition, common::Instruction};
use crate::cpu::cpu::CPU;
use crate::memory::memory_map::MemoryMap;

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
pub struct HalfwordCommon {
    pub pre_post_indexing_bit: bool,
    pub up_down_bit: bool,
    pub write_back: bool,
    pub load_store: bool,
    pub base_register: u8,
    pub destination: u8,
    pub is_signed: bool,
    pub is_halfword: bool,
    pub condition: Condition,
}

impl From<u32> for HalfwordCommon {
    fn from(value: u32) -> HalfwordCommon {
        return HalfwordCommon {
            pre_post_indexing_bit: ((value & 0x0100_0000) >> 24) != 0,
            up_down_bit: ((value & 0x80_0000) >> 23) != 0,
            write_back: ((value & 0x20_0000) >> 21) != 0,
            load_store: ((value & 0x10_0000) >> 20) != 0,
            base_register: ((value & 0xF_0000) >> 16) as u8,
            destination: ((value & 0xF000) >> 12) as u8,
            is_signed: ((value & 0x40) >> 6) != 0,
            is_halfword: ((value & 0x20) >> 5) != 0,
            condition: Condition::from((value & 0xF000_0000) >> 28),
        };
    }
}

impl From<u32> for HalfwordRegisterOffset {
    fn from(value: u32) -> HalfwordRegisterOffset {
        return HalfwordRegisterOffset {
            halfword_common: HalfwordCommon::from(value),
            offset_register: (value & 0xF) as u8,
        };
    }
}

impl Instruction for HalfwordImmediateOffset {
    fn execute(&mut self, cpu: &mut CPU, mem_map: &mut MemoryMap) {
        let base = cpu.registers[self.halfword_common.base_register as usize];
        let offset = (self.offset_high_nibble << 5) | self.offset_low_nibble;
        let address;

        if self.halfword_common.up_down_bit {
            address = base + offset as u32;
        } else {
            address = base - offset as u32;
        }


        if !self.halfword_common.is_signed && ! self.halfword_common.is_halfword {

        } else if !self.halfword_common.is_signed && self.halfword_common.is_halfword {

        } else if self.halfword_common.is_signed && !self.halfword_common.is_halfword {

        } else if self.halfword_common.is_signed && !self.halfword_common.is_halfword {

        }

        if self.halfword_common.pre_post_indexing_bit  || self.halfword_common.write_back{
            cpu.registers[self.halfword_common.base_register as usize] = address;
        }

    }
}

impl From<u32> for HalfwordImmediateOffset {
    fn from(value: u32) -> HalfwordImmediateOffset {
        return HalfwordImmediateOffset {
            halfword_common: HalfwordCommon::from(value),
            offset_high_nibble: ((value & 0xF00) >> 8) as u8,
            offset_low_nibble: (value & 0xF) as u8,
        };
    }
}

fn LDRH(destination: u8, base_value: u32, cpu: &mut CPU ) {
    let halfword = ((base_value & 0xFFFF) >> 16) as u16;
    cpu.registers[destination as usize] = halfword as u32;
}

fn LDRSH(destination: u8, base_value: u32, cpu: &mut CPU) {
    let halfword = ((base_value & 0xFFFF) >> 16) as u16;
    let sign_bit = ((0x8000 & halfword) >> 15) != 0;
    if sign_bit {
        cpu.registers[destination as usize] = 0xFFFFFFFF & (halfword as u32) ;
    } else {
        cpu.registers[destination as usize] = 0x0000FFFF & (halfword as u32);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_halfword_common_creation_min() {
        let h: HalfwordCommon = HalfwordCommon::from(0 as u32);
        assert_eq!(h.pre_post_indexing_bit, false);
        assert_eq!(h.up_down_bit, false);
        assert_eq!(h.write_back, false);
        assert_eq!(h.load_store, false);
        assert_eq!(h.base_register, 0);
        assert_eq!(h.destination, 0);
        assert_eFFFq!(h.is_signed, false);
        assert_eq!(h.is_halfword, false);
        assert_eq!(h.condition, Condition::EQ);
    }

    #[test]
    fn test_halfword_common_creation_mid() {
        let h: HalfwordCommon = HalfwordCommon::from(0x11237062);
        assert_eq!(h.pre_post_indexing_bit, true);
        assert_eq!(h.up_down_bit, false);
        assert_eq!(h.write_back, true);
        assert_eq!(h.load_store, false);
        assert_eq!(h.base_register, 3);
        assert_eq!(h.destination, 7);
        assert_eq!(h.is_signed, true);
        assert_eq!(h.is_halfword, true);
        assert_eq!(h.condition, Condition::NE);
    }

    #[test]
    fn test_halfword_common_creation_max() {
        let h: HalfwordCommon = HalfwordCommon::from(0xEFFF_FFFF);
        assert_eq!(h.pre_post_indexing_bit, true);
        assert_eq!(h.up_down_bit, true);
        assert_eq!(h.write_back, true);
        assert_eq!(h.load_store, true);
        assert_eq!(h.base_register, 0xF);
        assert_eq!(h.destination, 0xF);
        assert_eq!(h.is_signed, true);
        assert_eq!(h.is_halfword, true);
        assert_eq!(h.condition, Condition::AL);
    }

    #[test]
    fn test_halfword_register_offset_min() {
        let h: HalfwordRegisterOffset = HalfwordRegisterOffset::from(0);
        assert_eq!(h.offset_register, 0);
    }

    #[test]
    fn test_halfword_register_offset_mid() {
        let h: HalfwordRegisterOffset = HalfwordRegisterOffset::from(7);
        assert_eq!(h.offset_register, 7);
    }


    #[test]
    fn test_halfword_register_offset_max() {
        let h: HalfwordRegisterOffset = HalfwordRegisterOffset::from(0xF as u32);
        assert_eq!(h.offset_register, 0xF);
    }

    #[test]
    fn test_halfword_immediate_offset_min() {
        let h: HalfwordImmediateOffset = HalfwordImmediateOffset::from(0);
        assert_eq!(h.offset_high_nibble, 0);
        assert_eq!(h.offset_low_nibble, 0);
    }

    #[test]
    fn test_halfword_immediate_offset_mid() {
        let h: HalfwordImmediateOffset = HalfwordImmediateOffset::from(0x707 as u32);
        assert_eq!(h.offset_high_nibble, 7);
        assert_eq!(h.offset_low_nibble, 7);
    }


    #[test]
    fn test_halfword_immediate_offset_max() {
        let h: HalfwordImmediateOffset = HalfwordImmediateOffset::from(0xF0F as u32);
        assert_eq!(h.offset_high_nibble, 0xF);
        assert_eq!(h.offset_low_nibble, 0xF);
    }
}