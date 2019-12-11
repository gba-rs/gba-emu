use super::{common::Condition, common::Instruction};
use crate::cpu::cpu::CPU;
use crate::memory::memory_map::MemoryMap;
use crate::operations::load_store::*;

#[derive(Debug)]
pub struct HalfwordRegisterOffset {
    pub halfword_common: HalfwordCommon,
    pub offset_register: u8,
}

#[derive(Debug)]
pub struct HalfwordImmediateOffset {
    pub halfword_common: HalfwordCommon,
    pub offset_high_nibble: u8,
    pub offset_low_nibble: u8,
}

/* A struct that represents the common data between halfword register offset
   and halfword immediate offset
*/
#[derive(Debug)]
pub struct HalfwordCommon {
    pub is_pre_indexed: bool,
    pub up_down_bit: bool,
    pub write_back: bool,
    pub load: bool,
    pub base_register: u8,
    pub destination: u8,
    pub is_signed: bool,
    pub condition: Condition,
    pub data_type: DataType,
}

impl From<u32> for HalfwordCommon {
    fn from(value: u32) -> HalfwordCommon {
        let is_halfword = ((value & 0x20) >> 5) != 0;
        let data_type;
        if is_halfword {
            data_type = DataType::Halfword;
        } else {
            data_type = DataType::Byte;
        }

        return HalfwordCommon {
            is_pre_indexed: ((value & 0x0100_0000) >> 24) != 0,
            up_down_bit: ((value & 0x80_0000) >> 23) != 0,
            write_back: ((value & 0x20_0000) >> 21) != 0,
            load: ((value & 0x10_0000) >> 20) != 0,
            base_register: ((value & 0xF_0000) >> 16) as u8,
            destination: ((value & 0xF000) >> 12) as u8,
            is_signed: ((value & 0x40) >> 6) != 0,
            condition: Condition::from((value & 0xF000_0000) >> 28),
            data_type,
        };
    }
}

impl Instruction for HalfwordRegisterOffset {
    fn execute(&self, cpu: &mut CPU, mem_map: &mut MemoryMap) {
        let base = cpu.get_register(self.halfword_common.base_register);
        let offset = cpu.get_register(self.offset_register);
        let address_with_offset = apply_offset(base, offset as u8, self.halfword_common.up_down_bit);

        common_execute(&self.halfword_common, cpu, mem_map, base, address_with_offset);
    }

    fn asm(&self) -> String {
        return format!("{:?}", self);
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
    fn execute(&self, cpu: &mut CPU, mem_map: &mut MemoryMap) {
        let base = cpu.get_register(self.halfword_common.base_register);
        let offset = (self.offset_high_nibble << 5) | self.offset_low_nibble;
        let address_with_offset = apply_offset(base, offset, self.halfword_common.up_down_bit);

        common_execute(&self.halfword_common, cpu, mem_map, base, address_with_offset);
    }

    fn asm(&self) -> String {
        return format!("{:?}", self);
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

/*
* Handles the actual execution of the loading or storing operation. Either loads a value from memory
* into a register or loads a value from a register into a memory location.
*/
fn common_execute(halfword_common: &HalfwordCommon, cpu: &mut CPU, mem_map: &mut MemoryMap,
                  base_address: u32, address_with_offset: u32) {
    let transfer_info = DataTransfer {
        is_pre_indexed: halfword_common.is_pre_indexed,
        write_back: halfword_common.write_back,
        load: halfword_common.load,
        is_signed: halfword_common.is_signed,
        data_type: halfword_common.data_type,
        base_register: halfword_common.base_register,
        destination: halfword_common.destination,
    };

    data_transfer_execute(transfer_info, base_address, address_with_offset, cpu, mem_map);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_halfword_common_creation_min() {
        let h: HalfwordCommon = HalfwordCommon::from(0 as u32);
        assert_eq!(h.is_pre_indexed, false);
        assert_eq!(h.up_down_bit, false);
        assert_eq!(h.write_back, false);
        assert_eq!(h.load, false);
        assert_eq!(h.base_register, 0);
        assert_eq!(h.destination, 0);
        assert_eq!(h.is_signed, false);
        assert_eq!(h.data_type, DataType::Byte);
        assert_eq!(h.condition, Condition::EQ);
    }

    #[test]
    fn test_halfword_common_creation_mid() {
        let h: HalfwordCommon = HalfwordCommon::from(0x11237062);
        assert_eq!(h.is_pre_indexed, true);
        assert_eq!(h.up_down_bit, false);
        assert_eq!(h.write_back, true);
        assert_eq!(h.load, false);
        assert_eq!(h.base_register, 3);
        assert_eq!(h.destination, 7);
        assert_eq!(h.is_signed, true);
        assert_eq!(h.data_type, DataType::Halfword);
        assert_eq!(h.condition, Condition::NE);
    }

    #[test]
    fn test_halfword_common_creation_max() {
        let h: HalfwordCommon = HalfwordCommon::from(0xEFFF_FFFF);
        assert_eq!(h.is_pre_indexed, true);
        assert_eq!(h.up_down_bit, true);
        assert_eq!(h.write_back, true);
        assert_eq!(h.load, true);
        assert_eq!(h.base_register, 0xF);
        assert_eq!(h.destination, 0xF);
        assert_eq!(h.is_signed, true);
        assert_eq!(h.data_type, DataType::Halfword);
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


