use crate::memory::interrupt_registers::*;
use crate::memory::memory_bus::{MemoryBus, HaltState};
use crate::cpu::cpu;
use crate::cpu::cpu::{OperatingMode, InstructionSet, ARM_LR, THUMB_LR, ARM_PC, THUMB_PC};
use std::collections::VecDeque;

//use crate::cpu::InstructionSet;

pub struct Interrupts {
    pub ime_interrupt: InterruptMasterEnableRegister,
    pub ie_interrupt: InterruptEnableRegister,
    pub if_interrupt: InterruptRequestFlags
}

impl Interrupts {
    pub fn new() -> Interrupts {
        return Interrupts {
            ime_interrupt: InterruptMasterEnableRegister::new(),
            ie_interrupt: InterruptEnableRegister::new(),
            if_interrupt: InterruptRequestFlags::new()
        }
    }
    pub fn enabled(&self) -> bool {
        return self.ime_interrupt.get_register() & 1 == 1;
    }

    pub fn should_service(&self) -> bool {
        return (self.ie_interrupt.get_register() & self.if_interrupt.get_register()) != 0;
    }

    pub fn service(&mut self, cpu: &mut cpu::CPU, mem_bus: &mut MemoryBus){
        if self.enabled() & self.should_service() {
            if !cpu.cpsr.control_bits.irq_disable || mem_bus.halt_state == HaltState::Halt {
                // log::info!("running");
                mem_bus.halt_state = HaltState::Running;

                let old_cpsr = cpu.cpsr;
                cpu.set_operating_mode(OperatingMode::Interrupt);
                cpu.set_instruction_set(InstructionSet::Arm);
                cpu.set_register(ARM_LR, cpu.get_register(ARM_PC) + 4);
                cpu.set_spsr(old_cpsr);
                cpu.cpsr.control_bits.irq_disable = true;
                cpu.set_register(15, 0x18);
            }
        }
    }
}



