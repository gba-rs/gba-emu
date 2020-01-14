use crate::cpu::cpu::CPU;
use crate::memory::memory_map::MemoryMap;
use crate::operations::instruction::Instruction;
use crate::operations::load_store::{store, DataType, apply_offset};

pub struct LoadStoreHalfword {
    pub load: bool,
    pub immediate_offset: u8,
    pub rb: u8,
    pub rd: u8,
}

impl From<u16> for LoadStoreHalfword {
    fn from(value: u16) -> LoadStoreHalfword {
        return LoadStoreHalfword {
            load: ((value & 0x800) >> 11) != 0,
            immediate_offset: ((value & 0x7C0) >> 6) as u8,
            rb: ((value & 0x38) >> 3) as u8,
            rd: (value & 0x7) as u8,
        };
    }
}

impl Instruction for LoadStoreHalfword {
    fn execute(&self, cpu: &mut CPU, mem_map: &mut MemoryMap) {
        let base_register_value = cpu.get_register(self.rb);
        let base_address = apply_offset(base_register_value, self.immediate_offset, true);
        cpu.set_register(self.rb, base_address);
        if self.load {
            let value = mem_map.read_u16(base_address);
            cpu.set_register(self.rd, value as u32);
        } else {
            let value_to_store = cpu.get_register(self.rd);
            mem_map.write_u16(base_address, value_to_store as u16);
        }

    }

    fn asm(&self) -> String {
        let instruction = format!("r{}, [r{}, #0x{:X}]", self.rd, self.rb, self.immediate_offset);
        if self.load {
            return format!("LDRH {}", instruction);
        }
        return format!("STRH {}", instruction);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::{work_ram::WorkRam};

    #[test]
    fn test_creation_0s() {
        let load_store_halfword = LoadStoreHalfword::from(0x8000);

        assert_eq!(load_store_halfword.load, false);
        assert_eq!(load_store_halfword.immediate_offset, 0);
        assert_eq!(load_store_halfword.rb, 0);
        assert_eq!(load_store_halfword.rd, 0);
    }

    #[test]
    fn test_creation() {
        let load_store_halfword = LoadStoreHalfword::from(0x8A14);

        assert_eq!(load_store_halfword.load , true);
        assert_eq!(load_store_halfword.immediate_offset, 8);
        assert_eq!(load_store_halfword.rb, 2);
        assert_eq!(load_store_halfword.rd, 4);
    }

    #[test]
    fn test_load() {
        let load_store_halfword = LoadStoreHalfword::from(0x8C14);

        let mut cpu = CPU::new();
        let mut mem_map = MemoryMap::new();
        let wram = WorkRam::new(256000, 0);
        mem_map.register_memory(0x0000, 0x00FF, &wram.memory);
        let expected_offset = 16;

        cpu.set_register(2, 0x0008);
        mem_map.write_u16(0x0008 + expected_offset, 22);

        load_store_halfword.execute(&mut cpu, &mut mem_map);

        assert_eq!(load_store_halfword.load , true);
        assert_eq!(load_store_halfword.immediate_offset, expected_offset as u8);
        assert_eq!(load_store_halfword.rb, 2);
        assert_eq!(load_store_halfword.rd, 4);

        assert_eq!(cpu.get_register(4), 22);
    }

    #[test]
    fn test_store() {
        let load_store_halfword = LoadStoreHalfword::from(0x8414);

        let mut cpu = CPU::new();
        let mut mem_map = MemoryMap::new();
        let wram = WorkRam::new(256000, 0);
        mem_map.register_memory(0x0000, 0x00FF, &wram.memory);
        let expected_offset = 16;

        cpu.set_register(2, 0x0008);
        cpu.set_register(4, 22);

        load_store_halfword.execute(&mut cpu, &mut mem_map);

        assert_eq!(load_store_halfword.load , false);
        assert_eq!(load_store_halfword.immediate_offset, expected_offset as u8);
        assert_eq!(load_store_halfword.rb, 2);
        assert_eq!(load_store_halfword.rd, 4);

        assert_eq!(mem_map.read_u16(0x0008 + expected_offset), 22);
    }

    #[test]
    fn test_asm() {
        let load_halfword = LoadStoreHalfword::from(0x8C14);
        let store_halfword = LoadStoreHalfword::from(0x8414);

        assert_eq!(load_halfword.asm(), "LDRH r4, [r2, #0x10]");
        assert_eq!(store_halfword.asm(), "STRH r4, [r2, #0x10]");
    }
}