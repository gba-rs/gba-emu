use crate::cpu::{cpu::CPU, cpu::OperatingMode, cpu::ARM_SP, cpu::ARM_PC};
use crate::memory::{memory_map::MemoryMap, game_pack_rom::GamePackRom};


pub struct GBA {
    pub cpu: CPU,
    pub mem_map: MemoryMap,
    pub game_pack_memory: [GamePackRom; 3]
}

impl GBA {
    pub fn new(pc_address: u32, bios: Vec<u8>, rom: Vec<u8>) -> GBA {
        let temp_gamepack = [
            GamePackRom::new(0),
            GamePackRom::new(0),
            GamePackRom::new(0),
        ];

        let mut temp: GBA = GBA {
            cpu: CPU::new(),
            mem_map: MemoryMap::new(),
            game_pack_memory: temp_gamepack
        };

        // setup the PC
        temp.cpu.set_register(ARM_PC, pc_address);

        // setup the SPs'
        temp.cpu.operating_mode = OperatingMode::Interrupt;
        temp.cpu.set_register(ARM_SP, 0x03007FE0);

        temp.cpu.operating_mode = OperatingMode::User;
        temp.cpu.set_register(ARM_SP, 0x03007FE0);

        temp.cpu.operating_mode = OperatingMode::Supervisor;
        temp.cpu.set_register(ARM_SP, 0x03007FE0);

        // setup the memory
        temp.cpu.bios_ram.load(bios);
        temp.game_pack_memory[0].load(rom);
        temp.mem_map.register_memory(0x00000000, 0x00003FFF, &temp.cpu.bios_ram.memory);
        temp.mem_map.register_memory(0x02000000, 0x0203FFFF, &temp.cpu.wram.memory);
        temp.mem_map.register_memory(0x03000000, 0x03007FFF, &temp.cpu.onchip_wram.memory);
        temp.mem_map.register_memory(0x08000000, 0x09FFFFFF, &temp.game_pack_memory[0].memory);
        temp.mem_map.register_memory(0x0A000000, 0x0BFFFFFF, &temp.game_pack_memory[1].memory);
        temp.mem_map.register_memory(0x0C000000, 0x0DFFFFFF, &temp.game_pack_memory[2].memory);

        return temp;
    }

    pub fn run(&mut self) {
        loop {
            self.cpu.fetch(&mut self.mem_map);
        }
    }

    pub fn step(&mut self) {
        self.cpu.fetch(&mut self.mem_map);
    }
}