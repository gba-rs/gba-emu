use crate::cpu::cpu::CPU;
//use crate::memory::memory_map::MemoryMap;
use crate::gba::memory_bus::MemoryBus;

pub trait Instruction {
    fn execute(&self, cpu: &mut CPU, mem_bus: &mut MemoryBus) -> u32;
    fn asm(&self) -> String;
    fn cycles(&self) -> u32;
}