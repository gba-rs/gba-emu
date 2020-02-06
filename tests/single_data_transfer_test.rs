extern crate gba_emulator;

#[cfg(test)]
mod tests {
    use gba_emulator::cpu::cpu::{CPU};
    use gba_emulator::memory::{memory_map::MemoryMap};
    use gba_emulator::gba::memory_bus::MemoryBus;

    #[test]
    fn correct_operation_called_byte_immediate_word_aligned() {
        let mut cpu = CPU::new();
        let mut bus = MemoryBus::new();
        let value_to_load = 0x0F77_8866;
        let address_in_memory = 0x0200_0000;
        bus.mem_map.register_memory(0x0200_0000, 0x0203FFFF, &cpu.wram.memory);
        bus.write_u32(address_in_memory, value_to_load);
        cpu.set_register(2, address_in_memory);

        cpu.decode(0x14F20004).unwrap().execute(&mut cpu, &mut bus);

        assert_eq!(cpu.get_register(0), 0x66);
    }

    #[test]
    fn correct_operation_called_byte_immediate_word_plus_1_aligned() {
        let mut cpu = CPU::new();
        let mut bus = MemoryBus::new();
        let value_to_load = 0x0F77_8888;
        let address_in_memory = 0x0200_0001;
        bus.mem_map.register_memory(0x0200_0000, 0x0203FFFF, &cpu.wram.memory);
        bus.write_u32(address_in_memory, value_to_load);
        cpu.set_register(2, address_in_memory);

        cpu.decode(0x14F20004).unwrap().execute(&mut cpu, &mut bus);

        assert_eq!(cpu.get_register(0), 0x88);
    }

    #[test]
    fn transfer_byte_register_offset() {
        let mut cpu = CPU::new();
        let mut bus = MemoryBus::new();
        let value_to_load = 0xF0;
        let address_in_memory = 0x0200_0000;
        let offset = 0x8; 


        bus.mem_map.register_memory(0x0200_0000, 0x0203FFFF, &cpu.wram.memory);
        bus.write_u8(address_in_memory + (offset >> 2), value_to_load);
        cpu.set_register(0, address_in_memory);
        // original offset register is 8 but a log. shift right of 2 is applied in the instruction
        cpu.set_register(8, offset);

        let instr = cpu.decode(0xE7D08128).unwrap();
        // println!("{:?}", instr.asm());
        instr.execute(&mut cpu, &mut bus);

        assert_eq!(cpu.get_register(8), 0xF0);
    }
}