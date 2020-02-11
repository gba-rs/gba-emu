use crate::operations::instruction::Instruction;
use crate::cpu::cpu::{CPU, THUMB_PC};
use crate::operations::load_store::{DataTransfer, DataType, data_transfer_execute};
use crate::gba::memory_bus::MemoryBus;

pub struct LoadStoreRegisterOffset {
    load: bool,
    data_type: DataType,
    offset_register: u8,
    rb: u8,
    rd: u8,
}

impl From<u16> for LoadStoreRegisterOffset {
    fn from(value: u16) -> LoadStoreRegisterOffset {
        let data_type: DataType;

        if (value & 0x400) == 0 {
            data_type = DataType::Word
        } else {
            data_type = DataType::Byte
        }
        return LoadStoreRegisterOffset {
            load: ((value & 0x800) >> 11) != 0,
            data_type,
            offset_register: ((value & 0x1C0) >> 6) as u8,
            rb: ((value & 0x38) >> 3) as u8,
            rd: (value & 0x7) as u8,
        };
    }
}

impl Instruction for LoadStoreRegisterOffset {
    fn execute(&self, cpu: &mut CPU, mem_bus: &mut MemoryBus) -> u32 {

        let transfer_info = DataTransfer {
            is_pre_indexed: true,
            write_back: false,
            load: self.load,
            is_signed: false,
            data_type: self.data_type,
            base_register: self.rb,
            destination: self.rd,
        };

        let target_address = cpu.get_register(self.rb) + cpu.get_register(self.offset_register);
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
        let op;
        let mut b = "";
        if self.load {
            op = "LD";
        } else {
            op = "STR"
        }

        if self.data_type == DataType::Byte {
            b = "B";
        }

        return format!("{}{} r{}, [r{}, r{}]", op, b, self.rd, self.rb, self.offset_register );
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
        let format = LoadStoreRegisterOffset::from(0x5000);

        assert_eq!(format.load, false);
        assert_eq!(format.data_type, DataType::Word);
        assert_eq!(format.offset_register, 0);
        assert_eq!(format.rb, 0);
        assert_eq!(format.rd, 0);
    }

    #[test]
    fn test_creation() {
        let format = LoadStoreRegisterOffset::from(0x58B3);

        assert_eq!(format.load, true);
        assert_eq!(format.data_type, DataType::Word);
        assert_eq!(format.offset_register, 2);
        assert_eq!(format.rb, 6);
        assert_eq!(format.rd, 3);
    }

    #[test]
    fn test_creation_byte() {
        let format = LoadStoreRegisterOffset::from(0x54B3);

        assert_eq!(format.load, false);
        assert_eq!(format.data_type, DataType::Byte);
        assert_eq!(format.offset_register, 2);
        assert_eq!(format.rb, 6);
        assert_eq!(format.rd, 3);
    }

    #[test]
    fn test_execute_load_word() {
        let format = LoadStoreRegisterOffset::from(0x58B3);
        let mut gba = GBA::default();
        let offset_amount = 4;
        let memory_address = 0x04;
        let value_to_load = 0xF0F;

        gba.cpu.set_register(2, offset_amount); // set up offset
        gba.cpu.set_register(format.rb, memory_address);
        gba.memory_bus.write_u32(memory_address + offset_amount, value_to_load);
        format.execute(&mut gba.cpu, &mut gba.memory_bus);

        assert_eq!(gba.cpu.get_register(format.rd), value_to_load)
    }

    #[test]
    fn test_execute_load_byte() {
        let format = LoadStoreRegisterOffset::from(0x5CB3);
        let mut gba = GBA::default();
        let offset_amount = 6;
        let memory_address = 0x04;
        let value_to_load = 0xF0F;

        gba.cpu.set_register(2, offset_amount); // set up offset
        gba.cpu.set_register(format.rb, memory_address);
        gba.memory_bus.write_u32(memory_address + offset_amount, value_to_load);
        format.execute(&mut gba.cpu, &mut gba.memory_bus);

        assert_eq!(gba.cpu.get_register(format.rd) as u8, value_to_load as u8)
    }

    #[test]
    fn test_execute_store_byte() {
        let format = LoadStoreRegisterOffset::from(0x54B3);
        let mut gba = GBA::default();

        let offset_amount = 6;
        let memory_address = 0x04;
        let value_to_store = 0xFF1;

        gba.cpu.set_register(2, offset_amount); // set up offset
        gba.cpu.set_register(format.rb, memory_address);
        gba.cpu.set_register(format.rd, value_to_store);

        format.execute(&mut gba.cpu, &mut gba.memory_bus);

        assert_eq!(gba.memory_bus.read_u8(memory_address + offset_amount), value_to_store as u8);
    }

    #[test]
    fn test_execute_store_word() {
        let format = LoadStoreRegisterOffset::from(0x50B3);
        let mut gba = GBA::default();

        let offset_amount = 4;
        let memory_address = 0x04;
        let value_to_store = 0xFF1;

        gba.cpu.set_register(2, offset_amount); // set up offset
        gba.cpu.set_register(format.rb, memory_address);
        gba.cpu.set_register(format.rd, value_to_store);

        format.execute(&mut gba.cpu, &mut gba.memory_bus);
        assert_eq!(gba.memory_bus.read_u32(memory_address + offset_amount), value_to_store);
    }
}
