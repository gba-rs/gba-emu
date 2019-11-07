extern crate gba_emulator;

use gba_emulator::*;

#[cfg(test)]
mod tests {
    use gba_emulator::cpu::cpu::{CPU, InstructionSet, OperatingMode};
    use gba_emulator::formats::block_data_transfer::BlockDataTransfer;
    use gba_emulator::formats::common::Instruction;
    use gba_emulator::memory::{work_ram::WorkRam, bios_ram::BiosRam, memory_map::MemoryMap};

    #[test]
    fn test_stmib() {
        let mut a: BlockDataTransfer = BlockDataTransfer::from(0xE98D001E);
        let mut cpu = CPU::new();
        cpu.set_register(1, 0x1);
        cpu.set_register(2, 0x2);
        cpu.set_register(3, 0x3);
        cpu.set_register(4, 0x4);
        cpu.set_register(13, 0xFF00);

        let mut map = MemoryMap::new();
        let mut work_ram: WorkRam = WorkRam::new(0xFFFF, 0);
        map.register_memory(0, 0xFFFF, &work_ram.memory);
        a.execute(&mut cpu, &mut map);

        assert_eq!(map.read_u32(0xFF00 + 4), 0x1);
        assert_eq!(map.read_u32(0xFF00 + 8), 0x2);
        assert_eq!(map.read_u32(0xFF00 + 12), 0x3);
        assert_eq!(map.read_u32(0xFF00 + 16), 0x4);
    }

    #[test]
    fn test_stmia() {
        let mut a: BlockDataTransfer = BlockDataTransfer::from(0xE88D001E);
        let mut cpu = CPU::new();
        cpu.set_register(1, 0x1);
        cpu.set_register(2, 0x2);
        cpu.set_register(3, 0x3);
        cpu.set_register(4, 0x4);
        cpu.set_register(13, 0xFF00);

        let mut map = MemoryMap::new();
        let mut work_ram: WorkRam = WorkRam::new(0xFFFF, 0);
        map.register_memory(0, 0xFFFF, &work_ram.memory);
        a.execute(&mut cpu, &mut map);

        assert_eq!(map.read_u32(0xFF00), 0x1);
        assert_eq!(map.read_u32(0xFF00 + 4), 0x2);
        assert_eq!(map.read_u32(0xFF00 + 8), 0x3);
        assert_eq!(map.read_u32(0xFF00 + 12), 0x4);
    }

    #[test]
    fn test_stmdb() {
        let mut a: BlockDataTransfer = BlockDataTransfer::from(0xE90D001E);
        let mut cpu = CPU::new();
        cpu.set_register(1, 0x4);
        cpu.set_register(2, 0x5);
        cpu.set_register(3, 0x6);
        cpu.set_register(4, 0x7);
        cpu.set_register(13, 0xFF00);

        let mut map = MemoryMap::new();
        let mut work_ram: WorkRam = WorkRam::new(0xFFFF, 0);
        map.register_memory(0, 0xFFFF, &work_ram.memory);
        a.execute(&mut cpu, &mut map);

        assert_eq!(map.read_u32(0xFF00 - 4), 0x7);
        assert_eq!(map.read_u32(0xFF00 - 8), 0x6);
        assert_eq!(map.read_u32(0xFF00 - 12), 0x5);
        assert_eq!(map.read_u32(0xFF00 - 16), 0x4);
    }

    #[test]
    fn test_stmda() {
        let mut a: BlockDataTransfer = BlockDataTransfer::from(0xE80D001E);
        let mut cpu = CPU::new();
        cpu.set_register(1, 0x4);
        cpu.set_register(2, 0x5);
        cpu.set_register(3, 0x6);
        cpu.set_register(4, 0x7);
        cpu.set_register(13, 0xFF00);

        let mut map = MemoryMap::new();
        let mut work_ram: WorkRam = WorkRam::new(0xFFFF, 0);
        map.register_memory(0, 0xFFFF, &work_ram.memory);
        a.execute(&mut cpu, &mut map);

        assert_eq!(map.read_u32(0xFF00), 0x7);
        assert_eq!(map.read_u32(0xFEFC), 0x6);
        assert_eq!(map.read_u32(0xFEF8), 0x5);
        assert_eq!(map.read_u32(0xFEF4), 0x4);
    }
}