use super::{common::Condition, common::Instruction};
use crate::memory::memory_map::MemoryMap;
use crate::cpu::{cpu::CPU, cpu::OperatingMode};

pub struct BlockDataTransfer {
    pub register_list: Vec<u8>,
    pub base_register: u8,
    pub load: bool,
    pub write_back: bool,
    pub psr_force_user: bool,
    pub up: bool,
    pub pre_indexing: bool,
    pub condition: Condition
}

impl From<u32> for BlockDataTransfer {
    fn from(value: u32) -> BlockDataTransfer {
        let mut temp_reg_list: Vec<u8> = vec![];
        for i in 0..16 {
            if ((value >> i) & 0x01) != 0 {
                temp_reg_list.push(i as u8);
            }
        }

        return BlockDataTransfer {
            register_list: temp_reg_list,
            base_register: ((value >> 16) & 0x0F) as u8,
            load: ((value >> 20) & 0x01) != 0,
            write_back: ((value >> 21) & 0x01) != 0,
            psr_force_user: ((value >> 22) & 0x01) != 0,
            up: ((value >> 23) & 0x01) != 0,
            pre_indexing: ((value >> 24) & 0x01) != 0,
            condition: Condition::from((value & 0xF000_0000) >> 28)
        };
    }
}

impl Instruction for BlockDataTransfer {
    fn execute(&mut self, cpu: &mut CPU, mem_map: &mut MemoryMap) {
        if self.load {
            self.load_data(cpu, mem_map);
        } else {
            self.save_data(cpu, mem_map);
        }
    }
}

impl BlockDataTransfer {
    fn load_data(&mut self, cpu: &mut CPU, mem_map: &mut MemoryMap) {
        let mut current_address: i64 = cpu.get_register(self.base_register) as i64;
        current_address = self.get_start_address(current_address);
        let mut current_operating_mode = cpu.operating_mode;

        // Handle the psr
        if self.psr_force_user {
            if self.register_list.contains(&15) {
                cpu.cpsr = cpu.get_spsr();
            } else {
                // bank transfer
                current_operating_mode = OperatingMode::User;
                self.write_back = false;
            }
        }

        for reg_num in self.register_list.iter() {
            cpu.set_register_override(*reg_num, current_operating_mode, mem_map.read_u32(current_address as u32));
            current_address += 4;
        }

        if self.write_back && !self.register_list.contains(&self.base_register) {
            // TODO figure out write back
        }
    }

    fn save_data(&mut self, cpu: &mut CPU, mem_map: &mut MemoryMap) {
        let mut current_address: i64 = cpu.get_register(self.base_register) as i64;
        current_address = self.get_start_address(current_address);
        let mut current_operating_mode = cpu.operating_mode;


        // Handle the psr
        if self.psr_force_user {
            // bank transfer
            current_operating_mode = OperatingMode::User;
            self.write_back = false;
        }

        for reg_num in self.register_list.iter() {
            // todo figure out write back with base in reg list
            mem_map.write_u32(current_address as u32, cpu.get_register_override(*reg_num, current_operating_mode));
            current_address += 4;
        }

        if self.write_back {
            // todo figure out normal write back stuff
        }
    }

    fn get_start_address(&mut self, current_address: i64) -> i64 {
        if self.pre_indexing && self.up {
            return current_address + 4
        } else if !self.pre_indexing && self.up {
            return current_address
        } else if self.pre_indexing && !self.up {
            return current_address - (4 * ((self.register_list.len() as i64) - 1)) - 4
        } else if !self.pre_indexing && !self.up {
            return current_address - (4 * ((self.register_list.len() as i64) - 1))
        } else {
            panic!("get_start_address_offset: How did we even get here pre: {}, up: {}", self.pre_indexing, self.up);
        }
    }
}