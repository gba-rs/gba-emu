use crate::cpu::cpu::Cpu;
use crate::memory::memory_map::MemoryMap;


pub struct GBA {
    pub cpu: Cpu,
    pub mem_map: MemoryMap
}

impl GBA {
    pub fn new(pc_address: u32) -> GBA {
        let mut temp: GBA = GBA {
            cpu: Cpu::new(),
            mem_map: MemoryMap::new()
        };

        temp.cpu.registers[15] = pc_address;
        temp.mem_map.register_memory(0x02000000, 0x0203FFFF, &temp.cpu.wram.memory);

        return temp;
    }
}