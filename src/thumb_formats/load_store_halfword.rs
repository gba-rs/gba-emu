use crate::cpu::cpu::{CPU, THUMB_PC};
use crate::operations::instruction::Instruction;
use crate::operations::load_store::{DataTransfer, DataType, data_transfer_execute};
use crate::memory::memory_bus::MemoryBus;

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
            immediate_offset: (((value & 0x7C0) >> 6) << 1) as u8,
            rb: ((value & 0x38) >> 3) as u8,
            rd: (value & 0x7) as u8,
        };
    }
}

impl Instruction for LoadStoreHalfword {
    fn execute(&self, cpu: &mut CPU, mem_bus: &mut MemoryBus) -> u32 {
        let transfer_info = DataTransfer {
            is_pre_indexed: true,
            write_back: false,
            load: self.load,
            is_signed: false,
            data_type: DataType::Halfword,
            base_register: self.rb,
            destination: self.rd,
        };

        let target_address = cpu.get_register(self.rb) + self.immediate_offset as u32;
        let base;
        if self.rb == THUMB_PC {
            base = cpu.get_register(self.rb) + 2;
        } else {
            base = cpu.get_register(self.rb);
        }

        data_transfer_execute(transfer_info, base, target_address, cpu, mem_bus);
        mem_bus.cycle_clock.get_cycles()
    }

    fn asm(&self) -> String {
        let instruction = format!("r{}, [r{}, #0x{:X}]", self.rd, self.rb, self.immediate_offset);
        if self.load {
            return format!("LDRH {}", instruction);
        }
        return format!("STRH {}", instruction);
    }
    fn cycles(&self) -> u32 {return 3;} // 1s + 1n + 1l
    // unless pc then its 5 2s + 2n + 1l but that isn't known till later.

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gba::GBA;

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
        assert_eq!(load_store_halfword.immediate_offset, 0x10);
        assert_eq!(load_store_halfword.rb, 2);
        assert_eq!(load_store_halfword.rd, 4);
    }

    #[test]
    fn test_load() {
        let load_store_halfword = LoadStoreHalfword::from(0x8C14);
        let mut gba = GBA::default();

        let expected_offset = 32;

        gba.cpu.set_register(2, 0x08000000);
        gba.memory_bus.write_u16(0x08000000 + expected_offset, 22);

        load_store_halfword.execute(&mut gba.cpu, &mut gba.memory_bus);

        assert_eq!(load_store_halfword.load , true);
        assert_eq!(load_store_halfword.immediate_offset, expected_offset as u8);
        assert_eq!(load_store_halfword.rb, 2);
        assert_eq!(load_store_halfword.rd, 4);

        assert_eq!(gba.cpu.get_register(4), 22);
    }

    #[test]
    fn test_store() {
        let load_store_halfword = LoadStoreHalfword::from(0x8414);
        let mut gba = GBA::default();

        let expected_offset = 32;

        gba.cpu.set_register(2, 0x08000000);
        gba.cpu.set_register(4, 22);

        load_store_halfword.execute(&mut gba.cpu, &mut gba.memory_bus);

        assert_eq!(load_store_halfword.load , false);
        assert_eq!(load_store_halfword.immediate_offset, expected_offset as u8);
        assert_eq!(load_store_halfword.rb, 2);
        assert_eq!(load_store_halfword.rd, 4);

        assert_eq!(gba.memory_bus.read_u16(0x08000000 + expected_offset), 22);
    }
}