use crate::operations::instruction::Instruction;
use crate::cpu::cpu::CPU;
use crate::memory::memory_map::MemoryMap;
use crate::cpu::cpu::ThumbInstructionFormat::LoadStoreHalfWord;
use crate::thumb_formats::load_store_halfword::LoadStoreHalfword;
use crate::operations::arm_arithmetic::add;

#[derive(Debug)]
pub struct LoadStoreSignExtended {
    h_flag: bool,
    sign_extended: bool,
    offset_register: u8,
    base_register: u8,
    destination_register: u8,
}

impl From<u16> for LoadStoreSignExtended {
    fn from(value: u16) -> LoadStoreSignExtended {
        return LoadStoreSignExtended {
            h_flag: ((value & 0x800) >> 11) != 0,
            sign_extended: ((value & 0x400) >> 10) != 0,
            offset_register: ((value & 0x1C0) >> 6) as u8,
            base_register: ((value & 0x38) >> 3) as u8,
            destination_register: (value & 0x7) as u8,
        };
    }
}

impl Instruction for LoadStoreSignExtended {
    fn execute(&self, cpu: &mut CPU, mem_map: &mut MemoryMap) {
        let address = cpu.get_register(self.base_register) +
            cpu.get_register(self.offset_register);
        cpu.set_register(self.base_register, address);
        if !self.sign_extended && !self.h_flag {
            // store halfword
            let store_halfword = LoadStoreHalfword {
                load: false,
                immediate_offset: cpu.get_register(self.offset_register) as u8,
                rb: self.base_register,
                rd: self.destination_register,
            };
            store_halfword.execute(cpu, mem_map);
        } else if !self.sign_extended && self.h_flag {
            // load halfword
            cpu.set_register(self.base_register, address);
            cpu.set_register(self.destination_register,
                             (mem_map.read_u16(address) as u32 & 0x0000_FFFF));
        } else if self.sign_extended && !self.h_flag {
            // load sign-extended byte
            let byte_from_memory = mem_map.read_u8(address);
            let value_to_load: u32;
            if (byte_from_memory & (1 << 7)) > 0 {
                value_to_load = byte_from_memory as u32 & 0xFFFF_FFFF;
            } else {
                value_to_load = byte_from_memory as u32 & 0x0000_00FF;
            }
            cpu.set_register(self.destination_register, value_to_load);
        } else {
            // load sign-extended halfword
            let halfword_from_memory = mem_map.read_u16(address);
            let value_to_load: u32;
            if (byte_from_memory & (1 << 15)) > 0 {
                value_to_load = byte_from_memory as u32 & 0xFFFF_FFFF;
            } else {
                value_to_load = byte_from_memory as u32 & 0x0000_FFFF;
            }
            cpu.set_register(self.destination_register, value_to_load);
        }
    }

    fn asm(&self) -> String {
        // TODO
        return format!("TODO");
    }
}
