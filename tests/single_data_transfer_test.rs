extern crate gba_emulator;

#[cfg(test)]
mod tests {
    use gba_emulator::cpu::cpu::{CPU};
    use gba_emulator::memory::{memory_map::MemoryMap};
    use gba_emulator::gba::memory_bus::MemoryBus;
    use gba_emulator::gba::GBA;

    #[test]
    fn correct_operation_called_byte_immediate_word_aligned() {
        let mut gba = GBA::default();
        let value_to_load = 0x0F77_8866;
        let address_in_memory = 0x0200_0000;
        gba.memory_bus.write_u32(address_in_memory, value_to_load);
        gba.cpu.set_register(2, address_in_memory);

        gba.cpu.decode(0x14F20004).unwrap().execute(&mut gba.cpu, &mut gba.memory_bus);

        assert_eq!(gba.cpu.get_register(0), 0x66);
    }

    #[test]
    fn correct_operation_called_byte_immediate_word_plus_1_aligned() {
        let mut gba = GBA::default();
        let value_to_load = 0x0F77_8888;
        let address_in_memory = 0x0200_0001;
        gba.memory_bus.write_u32(address_in_memory, value_to_load);
        gba.cpu.set_register(2, address_in_memory);

        gba.cpu.decode(0x14F20004).unwrap().execute(&mut gba.cpu, &mut gba.memory_bus);

        assert_eq!(gba.cpu.get_register(0), 0x88);
    }

    #[test]
    fn transfer_byte_register_offset() {
        let mut gba = GBA::default();
        let value_to_load = 0xF0;
        let address_in_memory = 0x0200_0000;
        let offset = 0x8; 


        gba.memory_bus.write_u8(address_in_memory + (offset >> 2), value_to_load);
        gba.cpu.set_register(0, address_in_memory);
        // original offset register is 8 but a log. shift right of 2 is applied in the instruction
        gba.cpu.set_register(8, offset);

        let instr = gba.cpu.decode(0xE7D08128).unwrap();
        // println!("{:?}", instr.asm());
        instr.execute(&mut gba.cpu, &mut gba.memory_bus);

        assert_eq!(gba.cpu.get_register(8), 0xF0);
    }
}