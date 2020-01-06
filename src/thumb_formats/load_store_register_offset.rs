use crate::operations::load_store::DataType;
use crate::operations::instruction::Instruction;
use crate::cpu::cpu::CPU;
use crate::memory::memory_map::MemoryMap;

struct LoadStoreRegisterOffset {
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
    fn execute(&self, cpu: &mut CPU, mem_map: &mut MemoryMap) {
        let target_address = cpu.get_register(self.rb) + cpu.get_register(self.offset_register);
        if self.load {
            if self.data_type == DataType::Word {
                cpu.set_register(self.rd, mem_map.read_u32(target_address));
            } else {
                cpu.set_register(self.rd, mem_map.read_u8(target_address) as u32);
            }
        } else {
            if self.data_type == DataType::Word {
                mem_map.write_u32(target_address, cpu.get_register(self.rd));
            } else {
                mem_map.write_u8(target_address, cpu.get_register(self.rd) as u8);
            }
        }
    }

    fn asm(&self) -> String {
        let mut op;
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::work_ram::WorkRam;

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
        let mut cpu = CPU::new();
        let mut mem_map = MemoryMap::new();
        let wram = WorkRam::new(256000, 0);
        mem_map.register_memory(0x0000, 0x00FF, &wram.memory);

        let offset_amount = 4;
        let memory_address = 0x04;
        let value_to_load = 0xF0F;

        cpu.set_register(2, offset_amount); // set up offset
        cpu.set_register(format.rb, memory_address);
        mem_map.write_u32(memory_address + offset_amount, value_to_load);
        format.execute(&mut cpu, &mut mem_map);

        assert_eq!(cpu.get_register(format.rd), value_to_load)
    }

    #[test]
    fn test_execute_load_byte() {
        let format = LoadStoreRegisterOffset::from(0x5CB3);
        let mut cpu = CPU::new();
        let mut mem_map = MemoryMap::new();
        let wram = WorkRam::new(256000, 0);
        mem_map.register_memory(0x0000, 0x00FF, &wram.memory);

        let offset_amount = 6;
        let memory_address = 0x04;
        let value_to_load = 0xF0F;

        cpu.set_register(2, offset_amount); // set up offset
        cpu.set_register(format.rb, memory_address);
        mem_map.write_u32(memory_address + offset_amount, value_to_load);
        format.execute(&mut cpu, &mut mem_map);

        assert_eq!(cpu.get_register(format.rd) as u8, value_to_load as u8)
    }

    #[test]
    fn test_execute_store_byte() {
        let format = LoadStoreRegisterOffset::from(0x54B3);
        let mut cpu = CPU::new();
        let mut mem_map = MemoryMap::new();
        let wram = WorkRam::new(256000, 0);
        mem_map.register_memory(0x0000, 0x00FF, &wram.memory);

        let offset_amount = 6;
        let memory_address = 0x04;
        let value_to_store = 0xFF1;

        cpu.set_register(2, offset_amount); // set up offset
        cpu.set_register(format.rb, memory_address);
        cpu.set_register(format.rd, value_to_store);

        format.execute(&mut cpu, &mut mem_map);

        assert_eq!(mem_map.read_u8(memory_address + offset_amount), value_to_store as u8);
    }

    #[test]
    fn test_execute_store_word() {
        let format = LoadStoreRegisterOffset::from(0x50B3);
        let mut cpu = CPU::new();
        let mut mem_map = MemoryMap::new();
        let wram = WorkRam::new(256000, 0);
        mem_map.register_memory(0x0000, 0x00FF, &wram.memory);

        let offset_amount = 6;
        let memory_address = 0x04;
        let value_to_store = 0xFF1;

        cpu.set_register(2, offset_amount); // set up offset
        cpu.set_register(format.rb, memory_address);
        cpu.set_register(format.rd, value_to_store);

        format.execute(&mut cpu, &mut mem_map);
        assert_eq!(mem_map.read_u32(memory_address + offset_amount), value_to_store);
    }
}
