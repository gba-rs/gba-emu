extern crate gba_emulator;

#[cfg(test)]
mod tests {
    use gba_emulator::cpu::cpu::{CPU};
    use gba_emulator::arm_formats::block_data_transfer::BlockDataTransfer;
    use gba_emulator::operations::instruction::Instruction;
    use gba_emulator::memory::memory_bus::MemoryBus;

    #[test]
    fn test_stmib() {
        let a: BlockDataTransfer = BlockDataTransfer::from(0xE98D001E);
        let mut cpu = CPU::new();
        cpu.set_register(1, 0x1);
        cpu.set_register(2, 0x2);
        cpu.set_register(3, 0x3);
        cpu.set_register(4, 0x4);
        cpu.set_register(13, 0x05000000);

        let mut bus = MemoryBus::new_stub();
        a.execute(&mut cpu, &mut bus);

        assert_eq!(bus.read_u32(0x05000000 + 4), 0x1);
        assert_eq!(bus.read_u32(0x05000000 + 8), 0x2);
        assert_eq!(bus.read_u32(0x05000000 + 12), 0x3);
        assert_eq!(bus.read_u32(0x05000000 + 16), 0x4);
    }

    #[test]
    fn test_stmia() {
        let a: BlockDataTransfer = BlockDataTransfer::from(0xE88D001E);
        let mut cpu = CPU::new();
        cpu.set_register(1, 0x1);
        cpu.set_register(2, 0x2);
        cpu.set_register(3, 0x3);
        cpu.set_register(4, 0x4);
        cpu.set_register(13, 0x05000000);

        let mut bus = MemoryBus::new_stub();
        a.execute(&mut cpu, &mut bus);

        assert_eq!(bus.read_u32(0x05000000), 0x1);
        assert_eq!(bus.read_u32(0x05000000 + 4), 0x2);
        assert_eq!(bus.read_u32(0x05000000 + 8), 0x3);
        assert_eq!(bus.read_u32(0x05000000 + 12), 0x4);
    }

    #[test]
    fn test_stmdb() {
        let a: BlockDataTransfer = BlockDataTransfer::from(0xE90D001E);
        let mut cpu = CPU::new();
        cpu.set_register(1, 0x4);
        cpu.set_register(2, 0x5);
        cpu.set_register(3, 0x6);
        cpu.set_register(4, 0x7);
        cpu.set_register(13, 0x050000FF);

        let mut bus = MemoryBus::new_stub();
        a.execute(&mut cpu, &mut bus);

        assert_eq!(bus.read_u32(0x050000FF - 4), 0x7);
        assert_eq!(bus.read_u32(0x050000FF - 8), 0x6);
        assert_eq!(bus.read_u32(0x050000FF - 12), 0x5);
        assert_eq!(bus.read_u32(0x050000FF - 16), 0x4);
    }

    #[test]
    fn test_stmda() {
        let a: BlockDataTransfer = BlockDataTransfer::from(0xE80D001E);
        let mut cpu = CPU::new();
        cpu.set_register(1, 0x4);
        cpu.set_register(2, 0x5);
        cpu.set_register(3, 0x6);
        cpu.set_register(4, 0x7);
        cpu.set_register(13, 0x0500FF00);

        let mut bus = MemoryBus::new_stub();
        a.execute(&mut cpu, &mut bus);

        assert_eq!(bus.read_u32(0x0500FF00), 0x7);
        assert_eq!(bus.read_u32(0x0500FEFC), 0x6);
        assert_eq!(bus.read_u32(0x0500FEF8), 0x5);
        assert_eq!(bus.read_u32(0x0500FEF4), 0x4);
    }

    #[test]
    fn test_ldmib() {
        let a: BlockDataTransfer = BlockDataTransfer::from(0xE99D001E);
        let mut cpu = CPU::new();

        cpu.set_register(13, 0x0500FF00);

        let mut bus = MemoryBus::new_stub();

        bus.write_u32(0x0500FF04, 1);
        bus.write_u32(0x0500FF08, 2);
        bus.write_u32(0x0500FF0C, 3);
        bus.write_u32(0x0500FF10, 4);

        a.execute(&mut cpu, &mut bus);

        assert_eq!(cpu.get_register(1), 1);
        assert_eq!(cpu.get_register(2), 2);
        assert_eq!(cpu.get_register(3), 3);
        assert_eq!(cpu.get_register(4), 4);
    }
    
    #[test]
    fn test_ldmia() {
        let a: BlockDataTransfer = BlockDataTransfer::from(0xE89D001E);
        let mut cpu = CPU::new();

        cpu.set_register(13, 0x0500FF00);

        let mut bus = MemoryBus::new_stub();

        bus.write_u32(0x0500FF00, 1);
        bus.write_u32(0x0500FF04, 2);
        bus.write_u32(0x0500FF08, 3);
        bus.write_u32(0x0500FF0C, 4);

        a.execute(&mut cpu, &mut bus);

        assert_eq!(cpu.get_register(1), 1);
        assert_eq!(cpu.get_register(2), 2);
        assert_eq!(cpu.get_register(3), 3);
        assert_eq!(cpu.get_register(4), 4);
    }

    #[test]
    fn test_ldmdb() {
        let a: BlockDataTransfer = BlockDataTransfer::from(0xE91D001E);
        let mut cpu = CPU::new();

        cpu.set_register(13, 0x0500FF00);

        let mut bus = MemoryBus::new_stub();

        bus.write_u32(0x0500FEF0, 1);
        bus.write_u32(0x0500FEF4, 2);
        bus.write_u32(0x0500FEF8, 3);
        bus.write_u32(0x0500FEFC, 4);

        a.execute(&mut cpu, &mut bus);

        assert_eq!(cpu.get_register(1), 1);
        assert_eq!(cpu.get_register(2), 2);
        assert_eq!(cpu.get_register(3), 3);
        assert_eq!(cpu.get_register(4), 4);
    }

    #[test]
    fn test_ldmda() {
        let a: BlockDataTransfer = BlockDataTransfer::from(0xE81D001E);
        let mut cpu = CPU::new();

        cpu.set_register(13, 0x0500FF00);

        let mut bus = MemoryBus::new_stub();

        bus.write_u32(0x0500FEF0, 1);
        bus.write_u32(0x0500FEF4, 2);
        bus.write_u32(0x0500FEF8, 3);
        bus.write_u32(0x0500FEFC, 4);

        a.execute(&mut cpu, &mut bus);

        assert_eq!(cpu.get_register(1), 2);
        assert_eq!(cpu.get_register(2), 3);
        assert_eq!(cpu.get_register(3), 4);
        assert_eq!(cpu.get_register(4), 0);
    }
}