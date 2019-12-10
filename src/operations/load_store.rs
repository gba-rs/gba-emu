use crate::cpu::cpu::CPU;
use crate::memory::memory_map::MemoryMap;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum DataType {
    Word = 0b00,
    Halfword = 0b01,
    Byte = 0b10,
    Error,
}

impl From<u32> for DataType {
    fn from(value: u32) -> DataType {
        match value {
            0b00 => DataType::Word,
            0b01 => DataType::Halfword,
            0b10 => DataType::Byte,
            _ => DataType::Error
        }
    }
}

/*
* Extracts a byte or a halfword from a value stored in memory and put it into a CPU register
*/
pub fn load(is_signed: bool, data_type: DataType, destination: u8, cpu: &mut CPU,
            value_from_memory: u32, address: u32) {
    let mut value_to_load = 0;
    if !is_signed && data_type == DataType::Byte {
        value_to_load = get_byte_to_load(value_from_memory, address, false);
    } else if is_signed && data_type == DataType::Byte {
        value_to_load = get_byte_to_load(value_from_memory, address, true);
    } else if !is_signed && data_type == DataType::Halfword {
        value_to_load = get_halfword_to_load(value_from_memory, address, false);
    } else if is_signed && data_type == DataType::Halfword {
        value_to_load = get_halfword_to_load(value_from_memory, address, true);
    }

    cpu.set_register(destination, value_to_load);
}

/*
* Formats a value from a register and stores it in a given memory address
*/
pub fn store(data_type: DataType, value_to_store: u32, memory_address: u32, mem_map: &mut MemoryMap) {
    if (data_type == DataType::Halfword) && !is_halfword_aligned(memory_address) && !is_word_aligned(memory_address) {
        panic!("Attempting to store halfword in a memory location that is not word aligned or halfword aligned!");
    }

    let formatted_value;
    match data_type {
        DataType::Word => {
            formatted_value = value_to_store;
        }
        DataType::Halfword => {
            formatted_value = format_halfword_to_store(value_to_store as u16);
        }
        DataType::Byte => {
            formatted_value = format_byte_to_store(value_to_store as u8);
        }
        _ => panic!("Trying to store invalid data type.")
    }
    mem_map.write_u32(memory_address, formatted_value);
}

pub fn apply_offset(base_value: u32, offset: u8, add: bool) -> u32 {
    if add {
        return base_value + (offset as u32);
    }
    let val = base_value - (offset as u32);
    return val;
}

pub fn is_word_aligned(memory_address: u32) -> bool {
    return (memory_address & 0x3) == 0; // mult of 4s
}

pub fn is_word_plus_1_aligned(memory_address: u32) -> bool {
    return (memory_address & 0x2) == 0; // 1 more than mult. of 4
}

pub fn is_halfword_aligned(memory_address: u32) -> bool {
    return (memory_address & 0x1) == 0; // 2 more than mult. of 4
}

pub fn load_to_register(memory_address: u32, register: u8) {}

/*
* Pulls a halfword value out of a 32-bit value pulled from memory based on memory alignment
* If word aligned: halfword pulled from bits 31-16
* If halfword aligned: halfword pulled from bits 15-0

* Returns u32 where bits 7-0 is the value of the byte
* If signed, the top bits 31-16 are the sign beat repeated
* If not signed, the bits 31-16 are 0s
*/
pub fn get_halfword_to_load(base_value: u32, address: u32, signed: bool) -> u32 {
    let data: u16;
    if is_word_aligned(address) {
        data = ((base_value & 0xFFFF0000) >> 16) as u16;
    } else if is_halfword_aligned(address) {
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

/*
* Pulls a byte value out of a 32-bit value pulled from memory based on memory alignment
* If word aligned: byte pulled from bits 31-24
* If word + 1 byte aligned: byte pulled from bits 23-16 and so on...

* Returns u32 where bits 7-0 is the value of the byte
* If signed, the top bits 31-8 are the sign beat repeated
* If not signed, the bits 31-8 are 0s
*/
pub fn get_byte_to_load(base_value: u32, address: u32, signed: bool) -> u32 {
    println!("Base value: {:X}", base_value);
    println!("Is word aligned: {}", is_word_aligned(address));
    let data: u8;
    // if is_word_aligned(address) { //0011
    //     println!("word aligned");
    //     data = ((base_value & 0xFF000000) >> 24) as u8;
    // } else if is_word_plus_1_aligned(address) { //0010
    //     println!("word plus 1 aligned");
    //     data = ((base_value & 0x00FF0000) >> 16) as u8;
    // } else if is_halfword_aligned(address) {    //0001
    //     println!("halfword aligned");
    //     data = ((base_value & 0x0000FF00) >> 8) as u8;
    // } else { // word + 3 byte aligned (3 more than mult of 4)
    //     println!("Else");
    //     data = (base_value & 0x000000FF) as u8;
    // }
    data = (base_value & 0x000000FF) as u8;

    println!("data: {:X}", data);

    let byte_to_load: u32;

    if !signed || (data & 0x80) == 0 { // if not signed or sign bit is 0
        byte_to_load = data as u32;
    } else {
        byte_to_load = 0xFFFFFF00 | (data as u32);
    }

    println!("Byte to load: {:X}", byte_to_load);
    return byte_to_load as u32;
}

// Repeats a 16-bit halfword over 32-bits
pub fn format_halfword_to_store(value_to_store: u16) -> u32 {
    // repeat the bottom 16 bits over a 32-bit value
    let repeat = value_to_store & 0x0000_FFFF;
    let top = (repeat as u32) << 16;
    return top | (repeat as u32);
}

// Repeats an 8-bit byte over 32-bits
pub fn format_byte_to_store(value_to_store: u8) -> u32 {
    // repeat the bottom 8 bits over a 32-bit value
    let bits_31_24 = (value_to_store as u32) << 24;
    let bits_23_16 = (value_to_store as u32) << 16;
    let bits_15_8 = (value_to_store as u32) << 8;

    return bits_31_24 | bits_23_16 | bits_15_8 | (value_to_store as u32);
}

/**
* Common and generic structure that can be used to execute data transfer commands
*/
pub struct DataTransfer {
    pub is_pre_indexed: bool,
    pub write_back: bool,
    pub load: bool,
    pub is_signed: bool,
    pub data_type: DataType,
    pub base_register: u8,
    pub destination: u8,
}

/**
* Handles loading and storing of a defined DataTransfer structure
*/
pub fn data_transfer_execute(transfer_info: DataTransfer, base_address: u32, address_with_offset: u32,
                             cpu: &mut CPU, mem_map: &mut MemoryMap) {
    let address;
    // if pre-index, apply offset to the address that is used
    if transfer_info.is_pre_indexed {
        address = address_with_offset;
    } else {
        address = base_address;
    }

    println!("Address: {:X}", address);

    if transfer_info.load {
        let value_from_memory = mem_map.read_u32(address);
        load(transfer_info.is_signed, transfer_info.data_type,
             transfer_info.destination, cpu, value_from_memory, address);
    } else {
        let value_to_store = cpu.get_register(transfer_info.destination);
        store(transfer_info.data_type, value_to_store, address, mem_map);
    }

    // if post-indexed or write back bit is true, update the base register
    if !transfer_info.is_pre_indexed || transfer_info.write_back {
        cpu.set_register(transfer_info.base_register, address_with_offset);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::{work_ram::WorkRam};

    #[test]
    fn test_apply_offset() {
        assert_eq!(apply_offset(0x0004, 0x0002, true), 0x0006);
        assert_eq!(apply_offset(0x0004, 0x0002, false), 0x0002);
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

    #[test]
    fn test_load_signed_byte_word_aligned() {
        let value_from_memory = 0x8000_0000;
        let memory_address = 0x0004;
        let mut cpu = load_set_up(0, value_from_memory, memory_address);

        load(true, DataType::Byte, 0, &mut cpu, value_from_memory, memory_address);

        assert_eq!(cpu.get_register(0), 0xFFFF_FF80);
    }

    #[test]
    fn test_load_unsigned_byte_word_aligned() {
        let value_from_memory = 0x8000_0000;
        let memory_address = 0x0004;
        let mut cpu = load_set_up(0, value_from_memory, memory_address);

        load(false, DataType::Byte, 0, &mut cpu, value_from_memory, memory_address);

        assert_eq!(cpu.get_register(0), 0x80);
    }

    #[test]
    fn test_load_unsigned_byte_word_plus_1_aligned() {
        let value_from_memory = 0x0080_0000;
        let memory_address = 0x0005;
        let mut cpu = load_set_up(0, value_from_memory, memory_address);

        load(false, DataType::Byte, 0, &mut cpu, value_from_memory, memory_address);

        assert_eq!(cpu.get_register(0), 0x80);
    }

    #[test]
    fn test_load_unsigned_byte_word_plus_2_aligned() {
        let value_from_memory = 0x0000_8000;
        let memory_address = 0x0006;
        let mut cpu = load_set_up(0, value_from_memory, memory_address);

        load(false, DataType::Byte, 0, &mut cpu, value_from_memory, memory_address);

        assert_eq!(cpu.get_register(0), 0x80);
    }

    #[test]
    fn test_load_unsigned_byte_word_plus_3_aligned() {
        let value_from_memory = 0x0000_0080;
        let memory_address = 0x0007;
        let mut cpu = load_set_up(0, value_from_memory, memory_address);
        load(false, DataType::Byte, 0, &mut cpu, value_from_memory, memory_address);

        assert_eq!(cpu.get_register(0), 0x80);
    }

    #[test]
    fn test_load_unsigned_halfword_word_aligned() {
        let value_from_memory = 0x8080_0000;
        let memory_address = 0x0004;
        let mut cpu = load_set_up(0, value_from_memory, memory_address);
        load(false, DataType::Halfword, 0, &mut cpu, value_from_memory, memory_address);

        assert_eq!(cpu.get_register(0), 0x8080);
    }


    #[test]
    fn test_load_unsigned_halfword_word_plus_2_aligned() {
        let value_from_memory = 0x0000_8080;
        let memory_address = 0x0006;
        let mut cpu = load_set_up(0, value_from_memory, memory_address);
        load(false, DataType::Halfword, 0, &mut cpu, value_from_memory, memory_address);

        assert_eq!(cpu.get_register(0), 0x8080);
    }

    #[test]
    fn test_load_signed_halfword_word_aligned() {
        let value_from_memory = 0x8080_0000;
        let memory_address = 0x0004;
        let mut cpu = load_set_up(0, value_from_memory, memory_address);
        load(true, DataType::Halfword, 0, &mut cpu, value_from_memory, memory_address);

        assert_eq!(cpu.get_register(0), 0xFFFF_8080);
    }

    #[test]
    fn test_load_signed_halfword_word_aligned_positive() {
        let value_from_memory = 0x7080_0000;
        let memory_address = 0x0004;
        let mut cpu = load_set_up(0, value_from_memory, memory_address);
        load(true, DataType::Halfword, 0, &mut cpu, value_from_memory, memory_address);

        assert_eq!(cpu.get_register(0), 0x0000_7080);
    }

    #[test]
    fn test_store_halfword() {
        let memory_address = 0x04;
        let value_to_store = 0x8080;
        let mut mem_map = store_set_up();
        store(DataType::Halfword, value_to_store, memory_address, &mut mem_map);

        assert_eq!(0x8080_8080, mem_map.read_u32(memory_address));
    }

    #[test]
    fn test_store_byte() {
        let memory_address = 0x04;
        let value_to_store = 0x80;
        let mut mem_map = store_set_up();
        store(DataType::Byte, value_to_store, memory_address, &mut mem_map);

        assert_eq!(0x8080_8080, mem_map.read_u32(memory_address));
    }

    #[test]
    #[should_panic]
    fn test_store_halfword_improper_alignment() {
        let memory_address = 0x05;
        let value_to_store = 0x80;
        let mut mem_map = store_set_up();

        store(DataType::Halfword, value_to_store, memory_address, &mut mem_map);
    }

    fn load_set_up(destination: u8, value_from_memory: u32, memory_address: u32) -> CPU {
        let mut cpu = CPU::new();
        let mut mem_map = MemoryMap::new();
        let wram = WorkRam::new(256000, 0);
        mem_map.register_memory(0x0000, 0x00FF, &wram.memory);
        mem_map.write_u32(memory_address, value_from_memory);

        cpu.set_register(0x002, memory_address);

        return cpu;
    }

    fn store_set_up() -> MemoryMap {
        let mut mem_map = MemoryMap::new();
        let wram = WorkRam::new(256000, 0);
        mem_map.register_memory(0x0000, 0x00FF, &wram.memory);
        return mem_map;
    }
}