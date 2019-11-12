extern crate gba_emulator;

#[cfg(test)]
mod tests {
    use gba_emulator::cpu::cpu::{CPU};
    use gba_emulator::memory::{memory_map::MemoryMap};


    #[test]
    fn correct_operation_called_mul() {
        let mut cpu = CPU::new();
        let mut map = MemoryMap::new();
        map.register_memory(0x02000000, 0x0203FFFF, &cpu.wram.memory);
        cpu.set_register(4, 0); // Rd
        cpu.set_register(3, 1); // Rn
        cpu.set_register(2, 0x00000014); // Rs
        cpu.set_register(1, 0xFFFFFFF6); // Rm
        cpu.decode(&mut map, 0xE01_432_91); // MULS R4, R2, R1 (R4 = 20 * 0xFFFFFFF6u32)
        assert_eq!(cpu.get_register(4), 0xFFFFFF38);
    }

    #[test]
    fn correct_operation_called_mla() {
        let mut cpu = CPU::new();
        let mut map = MemoryMap::new();
        map.register_memory(0x02000000, 0x0203FFFF, &cpu.wram.memory);
        cpu.set_register(4, 0); // Rd
        cpu.set_register(3, 1); // Rn
        cpu.set_register(2, 0x00000014); // Rs
        cpu.set_register(1, 0xFFFFFFF6); // Rm
        cpu.decode(&mut map, 0xE02_432_91); // MLA R4, R2, R1, R3 (R4 = 20 * 0xFFFFFFF6u32 + 1)
        assert_eq!(cpu.get_register(4), 0xFFFFFF39);
    }

    #[test]
    fn correct_operation_called_umull() {
        let mut cpu = CPU::new();
        let mut map = MemoryMap::new();
        map.register_memory(0x02000000, 0x0203FFFF, &cpu.wram.memory);
        cpu.set_register(4, 1); // RdHi
        cpu.set_register(3, 1); // RdLo
        cpu.set_register(2, 0x00000014); // Rs
        cpu.set_register(1, 0xFFFFFFF6); // Rm
        cpu.decode(&mut map, 0xE09_432_91); // UMULLS  R3, R4, R2, R1 (R4,R3 = 20 * 0xFFFFFFF6u32)
        assert_eq!(cpu.get_register(3), 0xFFFFFF38);
        assert_eq!(cpu.get_register(4), 0x00000013);
    }

    #[test]
    fn correct_operation_called_umlal() {
        let mut cpu = CPU::new();
        let mut map = MemoryMap::new();
        map.register_memory(0x02000000, 0x0203FFFF, &cpu.wram.memory);
        cpu.set_register(4, 1); // RdHi
        cpu.set_register(3, 1); // RdLo
        cpu.set_register(2, 0x00000014); // Rs
        cpu.set_register(1, 0xFFFFFFF6); // Rm
        cpu.decode(&mut map, 0xE0A_432_91); // UMLAL  R3, R4, R2, R1 (R4,R3 = 20 * 0xFFFFFFF6u32 + 0x1,0x1)
        assert_eq!(cpu.get_register(3), 0xFFFFFF39);
        assert_eq!(cpu.get_register(4), 0x00000014);
    }

    #[test]
    fn correct_operation_called_smull() {
        let mut cpu = CPU::new();
        let mut map = MemoryMap::new();
        map.register_memory(0x02000000, 0x0203FFFF, &cpu.wram.memory);
        cpu.set_register(4, 1); // RdHi
        cpu.set_register(3, 1); // RdLo
        cpu.set_register(2, 0x00000014); // Rs
        cpu.set_register(1, 0xFFFFFFF6); // Rm
        cpu.decode(&mut map, 0xE0C_432_91); // SMULL  R3, R4, R2, R1 (R4,R3 = 20 * -10i32)
        assert_eq!(cpu.get_register(3), 0xFFFFFF38);
        assert_eq!(cpu.get_register(4), 0xFFFFFFFF);
    }

    #[test]
    fn correct_operation_called_smlal() {
        let mut cpu = CPU::new();
        let mut map = MemoryMap::new();
        map.register_memory(0x02000000, 0x0203FFFF, &cpu.wram.memory);
        cpu.set_register(4, 0); // RdHi
        cpu.set_register(3, 0x000000C8); // RdLo
        cpu.set_register(2, 0x00000014); // Rs
        cpu.set_register(1, 0xFFFFFFF6); // Rm
        cpu.decode(&mut map, 0xE0F_432_91); // SMLALS  R3, R4, R2, R1 (R4,R3 = 20 * -10i32 + 0x0,0xC8 [should add 200 to -200 to equal 0])
        assert_eq!(cpu.get_register(3), 0);
        assert_eq!(cpu.get_register(4), 0);
    }
}