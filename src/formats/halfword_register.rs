use super::{common::Condition, common::Instruction};
use crate::cpu::cpu::CPU;
use crate::memory::memory_map::MemoryMap;
use crate::operations::arithmatic::add;

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
    pub is_pre_indexed: bool,
    pub up_down_bit: bool,
    pub write_back: bool,
    pub load: bool,
    pub base_register: u8,
    pub destination: u8,
    pub is_signed: bool,
    pub is_halfword: bool,
    pub condition: Condition,
}

impl From<u32> for HalfwordCommon {
    fn from(value: u32) -> HalfwordCommon {
        return HalfwordCommon {
            is_pre_indexed: ((value & 0x0100_0000) >> 24) != 0,
            up_down_bit: ((value & 0x80_0000) >> 23) != 0,
            write_back: ((value & 0x20_0000) >> 21) != 0,
            load: ((value & 0x10_0000) >> 20) != 0,
            base_register: ((value & 0xF_0000) >> 16) as u8,
            destination: ((value & 0xF000) >> 12) as u8,
            is_signed: ((value & 0x40) >> 6) != 0,
            is_halfword: ((value & 0x20) >> 5) != 0,
            condition: Condition::from((value & 0xF000_0000) >> 28),
        };
    }
}

impl Instruction for HalfwordRegisterOffset {
    fn execute(&mut self, cpu: &mut CPU, mem_map: &mut MemoryMap) {
        let base = cpu.registers[self.halfword_common.base_register as usize];
        let offset = cpu.registers[self.offset_register as usize];
        let address_with_offset = apply_offset(base, offset as u8, self.halfword_common.up_down_bit);

        common_execute(&self.halfword_common, cpu, mem_map, base, address_with_offset);
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
        let address_with_offset = apply_offset(base, offset, self.halfword_common.up_down_bit);

        common_execute(&self.halfword_common, cpu, mem_map, base, address_with_offset);
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
    let address;

    // if pre-index, apply offset to the address that is used
    if halfword_common.is_pre_indexed {
        address = address_with_offset;
    } else {
        address = base_address;
    }

    if halfword_common.load {
        let value_from_memory = mem_map.read_u32(address);
        load(halfword_common.is_signed, halfword_common.is_halfword,
             halfword_common.destination, cpu, value_from_memory, address);
    } else {
        let value_to_store = cpu.registers[halfword_common.destination as usize];
        store(halfword_common.is_halfword, value_to_store, address, mem_map);
    }

    // if post-indexed or write back bit is true, update the base register
    if !halfword_common.is_pre_indexed || halfword_common.write_back {
        cpu.registers[halfword_common.base_register as usize] = address_with_offset;
    }
}

/*
* Extracts a byte or a halfword from a value stored in memory and put it into a CPU register
*/
fn load(is_signed: bool, is_halfword: bool, destination: u8, cpu: &mut CPU,
        value_from_memory: u32, address: u32) {
    let mut value_to_load = 0;
    if !is_signed && !is_halfword {
        value_to_load = get_byte_to_load(value_from_memory, address, false);
    } else if is_signed && !is_halfword {
        value_to_load = get_byte_to_load(value_from_memory, address, true);
    } else if !is_signed && is_halfword {
        value_to_load = get_halfword_to_load(value_from_memory, address, false);
    } else {
        value_to_load = get_halfword_to_load(value_from_memory, address, true);
    }

    cpu.registers[destination as usize] = value_to_load;
}

/*
* Formats a value from a register and stores it in a given memory address
*/
fn store(is_halfword: bool, value_to_store: u32, memory_address: u32, mem_map: &mut MemoryMap) {
    let formatted_value;
    if is_halfword {
        formatted_value = format_halfword_to_store(value_to_store as u16);
    } else {
        formatted_value = format_byte_to_store(value_to_store as u8);
    }
    mem_map.write_u32(memory_address, formatted_value);
}

// Repeats a 16-bit halfword over 32-bits
fn format_halfword_to_store(value_to_store: u16) -> u32 {
    // repeat the bottom 16 bits over a 32-bit value
    let repeat = value_to_store & 0x0000_FFFF;
    let top = (repeat as u32) << 16;
    return top | (repeat as u32);
}

// Repeats an 8-bit halfword over 32-bits
fn format_byte_to_store(value_to_store: u8) -> u32 {
    // repeat the bottom 8 bits over a 32-bit value
    let bits_31_24 = (value_to_store as u32) << 24;
    let bits_23_16 = (value_to_store as u32) << 16;
    let bits_15_8 = (value_to_store as u32) << 8;

    return bits_31_24 | bits_23_16 | bits_15_8 | (value_to_store as u32);
}

/*
* Pulls a byte value out of a 32-bit value pulled from memory based on memory alignment
* If word aligned: byte pulled from bits 31-24
* If word + 1 byte aligned: byte pulled from bits 23-16 and so on...

* Returns u32 where bits 7-0 is the value of the byte
* If signed, the top bits 31-8 are the sign beat repeated
* If not signed, the bits 31-8 are 0s
*/
fn get_byte_to_load(base_value: u32, address: u32, signed: bool) -> u32 {
    let mut data: u8 = 0;
    if (address & 0x3) == 0 { // word aligned (multiple of 4)
        data = ((base_value & 0xFF000000) >> 24) as u8;
    } else if (address & 0x3) == 1 { // word + 1 byte aligned (1 more than mult of 4)
        data = ((base_value & 0x00FF0000) >> 16) as u8;
    } else if (address & 0x3) == 2 { // word + 2 byte aligned (2 more than mult of 4)
        data = ((base_value & 0x0000FF00) >> 8) as u8;
    } else { // word + 3 byte aligned (3 more than mult of 4)
        data = (base_value & 0x000000FF) as u8;
    }

    let byte_to_load: u32;

    if !signed || (data & 0x80) == 0 { // if not signed or sign bit is 0
        byte_to_load = data as u32;
    } else {
        byte_to_load = 0xFFFFFF00 | (data as u32);
    }

    return byte_to_load as u32;
}

/*
* Pulls a halfword value out of a 32-bit value pulled from memory based on memory alignment
* If word aligned: halfword pulled from bits 31-16
* If halfword aligned: halfword pulled from bits 15-0

* Returns u32 where bits 7-0 is the value of the byte
* If signed, the top bits 31-16 are the sign beat repeated
* If not signed, the bits 31-16 are 0s
*/
fn get_halfword_to_load(base_value: u32, address: u32, signed: bool) -> u32 {
    let mut data: u16 = 0;
    if (address & 0x3) == 0 { // word aligned
        data = ((base_value & 0xFFFF0000) >> 16) as u16;
    } else if (address & 0x2) == 2 { // halfword aligned
        data = (base_value & 0x0000_FFFF) as u16;
    } else { // byte aligned
        panic!("Halfword is not correctly aligned");
    }

    let halfword: u32;
    if !signed || (data & 0x8000) == 0 { // if not signed or sign bit is 0
        halfword = data as u32;
    } else {
        halfword = 0xFFFF0000 | (data as u32);
    }

    return halfword;
}

fn apply_offset(base_value: u32, offset: u8, add: bool) -> u32 {
    if add {
        return base_value + (offset as u32);
    }
    return base_value - (offset as u32);
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
        assert_eq!(h.is_halfword, false);
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
        assert_eq!(h.is_halfword, true);
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

    #[test]
    fn test_get_halfword_to_load() {
        assert_eq!(get_halfword_to_load(0x8000_0000, 0x1000, true), 0xFFFF_8000);
        assert_eq!(get_halfword_to_load(0x9997_1122, 0x1000, false), 0x0000_9997);
        assert_eq!(get_halfword_to_load(0x9997_1122, 0x1002, false), 0x0000_1122);
        assert_eq!(get_halfword_to_load(0x9997_1122, 0x1002, true), 0x0000_1122);
    }

    #[test]
    #[should_panic(expected = "Halfword is not correctly aligned")]
    fn test_get_halfword_to_load_byte_aligned() {
        get_halfword_to_load(0x9997_1122, 0x1001, true);
    }

    #[test]
    fn test_get_byte_to_load() {
        assert_eq!(get_byte_to_load(0x8000_0000, 0x1000, true), 0xFFFF_FF80);
        assert_eq!(get_byte_to_load(0x0080_0000, 0x1001, true), 0xFFFF_FF80);
        assert_eq!(get_byte_to_load(0x0000_8000, 0x1002, true), 0xFFFF_FF80);
        assert_eq!(get_byte_to_load(0x0000_0080, 0x1003, true), 0xFFFF_FF80);
        assert_eq!(get_byte_to_load(0xFF00_0080, 0x1000, false), 0x0000_00FF);
        assert_eq!(get_byte_to_load(0x00FF_0080, 0x1001, false), 0x0000_00FF);
        assert_eq!(get_byte_to_load(0x0000_FF80, 0x1002, false), 0x0000_00FF);
        assert_eq!(get_byte_to_load(0x0000_FF80, 0x1003, false), 0x0000_0080);
    }

    #[test]
    fn test_apply_offset() {
        assert_eq!(apply_offset(0x0004, 0x0002, true),  0x0006);
        assert_eq!(apply_offset(0x0004, 0x0002, false),  0x0002);
    }

    #[test]
    fn test_format_byte_to_store() {
        assert_eq!(format_byte_to_store(0xF0), 0xF0F0_F0F0);
        assert_eq!(format_byte_to_store(0xFF), 0xFFFF_FFFF);
        assert_eq!(format_byte_to_store(0x00), 0x0000_0000);
    }

    #[test]
    fn test_format_halfword_to_store() {
        assert_eq!(format_halfword_to_store(0xFF00), 0xFF00_FF00);
        assert_eq!(format_halfword_to_store(0xF0F0), 0xF0F0_F0F0);
        assert_eq!(format_halfword_to_store(0), 0x0);
    }
}


