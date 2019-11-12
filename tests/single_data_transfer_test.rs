extern crate gba_emulator;

use gba_emulator::*;

#[cfg(test)]
mod tests {
    use gba_emulator::cpu::cpu::{CPU, InstructionSet, OperatingMode};
    use gba_emulator::formats::common::Instruction;
    use gba_emulator::memory::{work_ram::WorkRam, bios_ram::BiosRam, memory_map::MemoryMap};
    use gba_emulator::formats::single_data_transfer;

    #[test]
    fn correct_operation_called_byte_immediate_word_aligned() {
        let mut cpu = CPU::new();
        let mut map = MemoryMap::new();
        let value_to_load = 0x0F77_8866;
        let address_in_memory = 0x0200_0000;
        map.register_memory(0x0200_0000, 0x0203FFFF, &cpu.wram.memory);
        map.write_u32(address_in_memory, value_to_load);
        cpu.set_register(2, address_in_memory);

        cpu.decode(&mut map, 0x14F20004);

        assert_eq!(cpu.get_register(0), 0x0F);
    }

    #[test]
    fn correct_operation_called_byte_immediate_word_plus_1_aligned() {
        let mut cpu = CPU::new();
        let mut map = MemoryMap::new();
        let value_to_load = 0x0F77_8888;
        let address_in_memory = 0x0200_0001;
        map.register_memory(0x0200_0000, 0x0203FFFF, &cpu.wram.memory);
        map.write_u32(address_in_memory, value_to_load);
        cpu.set_register(2, address_in_memory);

        cpu.decode(&mut map, 0x14F20004);

        assert_eq!(cpu.get_register(0), 0x77);
    }

    #[test]
    fn transfer_byte_register_offset() {
        let mut cpu = CPU::new();
        let mut map = MemoryMap::new();
        let value_to_load = 0x0F77_8888;
        let address_in_memory = 0x0200_0000;
        let offset = 0x4;


        map.register_memory(0x0200_0000, 0x0203FFFF, &cpu.wram.memory);
        map.write_u32(address_in_memory + offset, value_to_load);
        cpu.set_register(0, address_in_memory);
        // original offset register is 8 but a log. shift right of 2 is applied in the instruction
        cpu.set_register(2, offset);

        cpu.decode(&mut map,   0x17D08128);

        assert_eq!(cpu.get_register(8), 0x0F);
    }
}