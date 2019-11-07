extern crate gba_emulator;

use gba_emulator::*;

#[cfg(test)]
mod tests {
    use gba_emulator::cpu::cpu::{CPU, InstructionSet, OperatingMode};
    use gba_emulator::formats::common::Instruction;
    use gba_emulator::memory::{work_ram::WorkRam, bios_ram::BiosRam, memory_map::MemoryMap};
    use gba_emulator::formats::halfword_register::{HalfwordImmediateOffset, HalfwordRegisterOffset};

    #[test]
    fn correct_operation_called_halfword_immediate() {
        let mut cpu = CPU::new();
        let mut map = MemoryMap::new();
        let value_to_load = 0x0F0F_8888;
        let address_in_memory = 0x0200_0000;
        map.register_memory(0x0200_0000, 0x0203FFFF, &cpu.wram.memory);
        map.write_u32(address_in_memory, value_to_load);
        cpu.set_register(4, address_in_memory);

        cpu.decode(&mut map, 0x101421BF);

        assert_eq!(cpu.get_register(2), 0x0F0F);
    }

    #[test]
    fn correct_operation_called_halfword_immediate_halfword_aligned() {
        let mut cpu = CPU::new();
        let mut map = MemoryMap::new();
        let value_to_load = 0x0F0F_8888;
        let address_in_memory = 0x0200_0002;
        map.register_memory(0x0200_0000, 0x0203FFFF, &cpu.wram.memory);
        map.write_u32(address_in_memory, value_to_load);
        cpu.set_register(4, address_in_memory);

        cpu.decode(&mut map, 0x101421BF);

        assert_eq!(cpu.get_register(2), 0x8888);
    }

    #[test]
    fn correct_operation_called_halfword_register() {
        let mut cpu = CPU::new();
        let mut map = MemoryMap::new();
        let value_to_load = 0xFFFF_8888;
        let address_in_memory = 0x0200_0002;
        let offset = 2;
        map.register_memory(0x0200_0000, 0x0203FFFF, &cpu.wram.memory);
        map.write_u32(address_in_memory + offset, value_to_load);
        cpu.set_register(4, address_in_memory);
        cpu.set_register(15, offset);

        cpu.decode(&mut map, 0x1194_21BF);

        assert_eq!(cpu.get_register(2), 0xFFFF);
    }

    #[test]
    fn correct_operation_called_halfword_register_halfword_aligned() {
        let mut cpu = CPU::new();
        let mut map = MemoryMap::new();
        let value_to_load = 0xFFFF_8888;
        let address_in_memory = 0x0200_0000;
        let offset = 2;
        map.register_memory(0x0200_0000, 0x0203FFFF, &cpu.wram.memory);
        map.write_u32(address_in_memory + offset, value_to_load);
        cpu.set_register(4, address_in_memory);
        cpu.set_register(15, offset);

        cpu.decode(&mut map, 0x1194_21BF);

        assert_eq!(cpu.get_register(2), 0x8888);
    }
    #[test]
    fn correct_operation_called_halfword_register_halfword_aligned_subtract_offset() {
        let mut cpu = CPU::new();
        let mut map = MemoryMap::new();
        let value_to_load = 0xFFFF_8888;
        let address_in_memory = 0x0200_0004;
        let offset = 2;
        map.register_memory(0x0200_0000, 0x0203FFFF, &cpu.wram.memory);
        map.write_u32(address_in_memory - offset, value_to_load);
        cpu.set_register(4, address_in_memory);
        cpu.set_register(15, offset);

        cpu.decode(&mut map, 0x1114_21BF);

        assert_eq!(cpu.get_register(2), 0x8888);
    }
}