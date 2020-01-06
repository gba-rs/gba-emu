use crate::cpu::cpu::CPU;
use crate::memory::memory_map::MemoryMap;
use crate::operations::instruction::Instruction;

#[derive(Debug)]
pub struct MultipleLoadStore {
    pub opcode: u8,
    pub rb: u8,
    pub register_list: Vec<u8>
}

impl From<u16> for MultipleLoadStore {
    fn from(value: u16) -> MultipleLoadStore {
        let mut temp_reg_list: Vec<u8> = vec![];
        for i in 0..7 {
            if ((value >> i) & 0x01) != 0{
                temp_reg_list.push(i as u8);
            }
        }
        return MultipleLoadStore {
            register_list: temp_reg_list,
            rb: ((value >> 8) & 0x7) as u8,
            opcode: ((value >> 11) & 0x1) as u8
        };
    }
}