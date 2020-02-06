use crate::cpu::cpu::CPU;
use crate::memory::memory_map::MemoryMap;
use log::{debug};
use crate::operations::bitutils::sign_extend_u32;
use crate::operations::arm_arithmetic;

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
pub fn load(is_signed: bool, data_type: DataType, destination: u8, cpu: &mut CPU, address: u32, mem_map: &mut MemoryMap) {
    let value: u32;
    match data_type {
        DataType::Byte => {
            if is_signed {
                value = sign_extend_u32(mem_map.read_u8(address) as u32, 7);
            } else {
                value = mem_map.read_u8(address) as u32;
            }
        },
        DataType::Halfword => {
            if is_signed {
                value = (((sign_extend_u32(mem_map.read_u16(address - (address % 2)) as u32, 15)) as i32) >> (address % 2) * 8) as u32;
            } else {
                value = (mem_map.read_u16(address - (address % 2)) as u32).rotate_right((address % 2) * 8);
            }
        },
        DataType::Word => {
            value = mem_map.read_u32(address - (address % 4)).rotate_right((address % 4) * 8);
        },
        _ => panic!("Invalid data type")
    }

    cpu.set_register(destination, value);
}

/*
* Formats a value from a register and stores it in a given memory address
*/
pub fn store(data_type: DataType, value_to_store: u32, memory_address: u32, mem_map: &mut MemoryMap) {
    match data_type {
        DataType::Word => {
            // Force word alignment 
            mem_map.write_u32(memory_address - (memory_address % 4), value_to_store);
        }
        DataType::Halfword => {
            // Force halfword alignment 
            mem_map.write_u16(memory_address - (memory_address % 2), value_to_store as u16);
        }
        DataType::Byte => {
            mem_map.write_u8(memory_address, value_to_store as u8);
        }
        _ => panic!("Trying to store invalid data type.")
    }
}

pub fn apply_offset(base_value: u32, offset: u32, add: bool, sign_bit_index: u8) -> u32 {
    let adjusted_offset;
    if sign_bit_index > 0 {
        adjusted_offset = sign_extend_u32(offset, sign_bit_index);
    } else {
        adjusted_offset = offset;
    }
    
    if add {
        return arm_arithmetic::add(base_value, adjusted_offset).0;
    }

    return arm_arithmetic::sub(base_value, adjusted_offset).0;
}


/**
* Common and generic structure that can be used to execute data transfer commands
*/
#[derive(Debug)]
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

    if transfer_info.load {
        load(transfer_info.is_signed, transfer_info.data_type, transfer_info.destination, cpu, address, mem_map);

        if transfer_info.destination != transfer_info.base_register {
            if !transfer_info.is_pre_indexed || transfer_info.write_back {
                cpu.set_register(transfer_info.base_register, address_with_offset);
            }
        }
    
    } else {
        let mut value_to_store = cpu.get_register(transfer_info.destination);
        if transfer_info.destination == 15 {
            value_to_store += 8;
        }

        store(transfer_info.data_type, value_to_store, address, mem_map);
        
        if !transfer_info.is_pre_indexed || transfer_info.write_back {
            cpu.set_register(transfer_info.base_register, address_with_offset);
        }
    }    
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::{work_ram::WorkRam};

    #[test]
    fn test_apply_offset() {
        assert_eq!(apply_offset(0x0004, 0x0002, true, 0), 0x0006);
        assert_eq!(apply_offset(0x0004, 0x0002, false, 0), 0x0002);
    }

    #[test]
    fn test_load_signed_byte_word_aligned() {
    }

    #[test]
    fn test_load_unsigned_byte_word_aligned() {
    }

    #[test]
    fn test_load_unsigned_byte_word_plus_1_aligned() {
    }

    #[test]
    fn test_load_unsigned_byte_word_plus_2_aligned() {
    }

    #[test]
    fn test_load_unsigned_byte_word_plus_3_aligned() {
    }

    #[test]
    fn test_load_unsigned_halfword_word_aligned() {
    }


    #[test]
    fn test_load_unsigned_halfword_word_plus_2_aligned() {
    }

    #[test]
    fn test_load_signed_halfword_word_aligned() {
    }

    #[test]
    fn test_load_signed_halfword_word_aligned_positive() {
    }

    #[test]
    fn test_store_halfword() {
        let memory_address = 0x04;
        let value_to_store = 0x8080;
        let mut mem_map = store_set_up();
        store(DataType::Halfword, value_to_store, memory_address, &mut mem_map);

        assert_eq!(0x8080, mem_map.read_u16(memory_address));
    }

    #[test]
    fn test_store_byte() {
        let memory_address = 0x04;
        let value_to_store = 0x80;
        let mut mem_map = store_set_up();
        store(DataType::Byte, value_to_store, memory_address, &mut mem_map);

        assert_eq!(0x80, mem_map.read_u8(memory_address));
    }

    #[test]
    fn test_store_halfword_improper_alignment() {}

    fn load_set_up(_: u8, value_from_memory: u32, memory_address: u32) -> CPU {
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