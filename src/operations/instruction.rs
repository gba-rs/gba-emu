use crate::cpu::cpu::CPU;
use crate::memory::memory_map::MemoryMap;

pub trait Instruction {
    fn execute(&self, cpu: &mut CPU, mem_map: &mut MemoryMap);
    fn asm(&self) -> String;
}