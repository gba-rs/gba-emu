use crate::cpu::cpu::CPU;
use crate::memory::memory_map::MemoryMap;


pub struct GBA {
    pub cpu: CPU,
    pub mem_map: MemoryMap
}

impl GBA {
    pub fn new(pc_address: u32) -> GBA {
        let mut temp: GBA = GBA {
            cpu: CPU::new(),
            mem_map: MemoryMap::new()
        };

        temp.cpu.registers[15] = pc_address;
        temp.mem_map.register_memory(0x00000000, 0x0003FFFF, &temp.cpu.wram.memory);

        return temp;
    }

    pub fn load(&mut self, address: u32, bytes: Vec<u8>) {
        self.mem_map.write_block(address, bytes);
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