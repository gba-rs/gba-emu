use crate::cpu::cpu::CPU;
//use crate::memory::memory_map::MemoryMap;
use crate::memory::memory_bus::MemoryBus;
use crate::dma::DMAController;
use crate::interrupts::interrupts::Interrupts;

pub trait Instruction {
    fn execute(&self, cpu: &mut CPU, mem_bus: &mut MemoryBus) -> u32;
    fn asm(&self) -> String;
    fn cycles(&self) -> u32;

    // GBATEK: DMA can grab the bus between a multi-register LDM/STM's individual
    // register transfers. Only those override this; everyone else keeps the atomic execute().
    fn execute_with_dma(&self, cpu: &mut CPU, mem_bus: &mut MemoryBus, _dma: &mut DMAController, _irq: &mut Interrupts) -> u32 {
        self.execute(cpu, mem_bus)
    }
}