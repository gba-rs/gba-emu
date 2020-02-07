extern crate gba_emulator;

#[cfg(test)]
mod tests {
    use gba_emulator::gba::GBA;


    #[test]
    fn correct_operation_called_mul() {
        let mut gba = GBA::default();
        gba.cpu.set_register(4, 0); // Rd
        gba.cpu.set_register(3, 1); // Rn
        gba.cpu.set_register(2, 0x00000014); // Rs
        gba.cpu.set_register(1, 0xFFFFFFF6); // Rm
        gba.cpu.decode(0xE01_432_91).unwrap().execute(&mut gba.cpu, &mut gba.memory_bus); // MULS R4, R2, R1 (R4 = 20 * 0xFFFFFFF6u32)
        assert_eq!(gba.cpu.get_register(4), 0xFFFFFF38);
    }

    #[test]
    fn correct_operation_called_mla() {
        let mut gba = GBA::default();
        gba.cpu.set_register(4, 0); // Rd
        gba.cpu.set_register(3, 1); // Rn
        gba.cpu.set_register(2, 0x00000014); // Rs
        gba.cpu.set_register(1, 0xFFFFFFF6); // Rm
        gba.cpu.decode(0xE02_432_91).unwrap().execute(&mut gba.cpu, &mut gba.memory_bus); // MLA R4, R2, R1, R3 (R4 = 20 * 0xFFFFFFF6u32 + 1)
        assert_eq!(gba.cpu.get_register(4), 0xFFFFFF39);
    }

    #[test]
    fn correct_operation_called_umull() {
        let mut gba = GBA::default();
        gba.cpu.set_register(4, 1); // RdHi
        gba.cpu.set_register(3, 1); // RdLo
        gba.cpu.set_register(2, 0x00000014); // Rs
        gba.cpu.set_register(1, 0xFFFFFFF6); // Rm
        gba.cpu.decode(0xE09_432_91).unwrap().execute(&mut gba.cpu, &mut gba.memory_bus); // UMULLS  R3, R4, R2, R1 (R4,R3 = 20 * 0xFFFFFFF6u32)
        assert_eq!(gba.cpu.get_register(3), 0xFFFFFF38);
        assert_eq!(gba.cpu.get_register(4), 0x00000013);
    }

    #[test]
    fn correct_operation_called_umlal() {
        let mut gba = GBA::default();
        gba.cpu.set_register(4, 1); // RdHi
        gba.cpu.set_register(3, 1); // RdLo
        gba.cpu.set_register(2, 0x00000014); // Rs
        gba.cpu.set_register(1, 0xFFFFFFF6); // Rm
        gba.cpu.decode(0xE0A_432_91).unwrap().execute(&mut gba.cpu, &mut gba.memory_bus); // UMLAL  R3, R4, R2, R1 (R4,R3 = 20 * 0xFFFFFFF6u32 + 0x1,0x1)
        assert_eq!(gba.cpu.get_register(3), 0xFFFFFF39);
        assert_eq!(gba.cpu.get_register(4), 0x00000014);
    }

    #[test]
    fn correct_operation_called_smull() {
        let mut gba = GBA::default();
        gba.cpu.set_register(4, 1); // RdHi
        gba.cpu.set_register(3, 1); // RdLo
        gba.cpu.set_register(2, 0x00000014); // Rs
        gba.cpu.set_register(1, 0xFFFFFFF6); // Rm
        gba.cpu.decode(0xE0C_432_91).unwrap().execute(&mut gba.cpu, &mut gba.memory_bus); // SMULL  R3, R4, R2, R1 (R4,R3 = 20 * -10i32)
        assert_eq!(gba.cpu.get_register(3), 0xFFFFFF38);
        assert_eq!(gba.cpu.get_register(4), 0xFFFFFFFF);
    }

    #[test]
    fn correct_operation_called_smlal() {
        let mut gba = GBA::default();
        gba.cpu.set_register(4, 0); // RdHi
        gba.cpu.set_register(3, 0x000000C8); // RdLo
        gba.cpu.set_register(2, 0x00000014); // Rs
        gba.cpu.set_register(1, 0xFFFFFFF6); // Rm
        gba.cpu.decode(0xE0F_432_91).unwrap().execute(&mut gba.cpu, &mut gba.memory_bus); // SMLALS  R3, R4, R2, R1 (R4,R3 = 20 * -10i32 + 0x0,0xC8 [should add 200 to -200 to equal 0])
        assert_eq!(gba.cpu.get_register(3), 0);
        assert_eq!(gba.cpu.get_register(4), 0);
    }
}