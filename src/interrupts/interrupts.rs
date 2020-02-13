use crate::memory::interrupt_registers::*;
use crate::cpu::cpu;
use crate::cpu::cpu::{OperatingMode, InstructionSet, ARM_LR, THUMB_LR, ARM_PC, THUMB_PC};
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
    pub fn enabled(&mut self) -> bool {
        return self.ime_interrupt.get_register() & 1 == 1;
    }
    pub fn service(&mut self, cpu: &mut cpu::CPU){
        let should_service = self.ie_interrupt.get_register() & self.if_interrupt.get_register();
        // log::debug!("Should service: {}", should_service);
        if should_service != 0x0 {
            // log::debug!("Serving");
            cpu.set_operating_mode(OperatingMode::Interrupt);
            if cpu.get_instruction_set() == InstructionSet::Arm {cpu.set_register(ARM_LR, cpu.get_register(ARM_PC) + 4)} else {cpu.set_register(THUMB_LR, cpu.get_register(THUMB_PC) + 2)};
            cpu.set_instruction_set(InstructionSet::Arm);
            cpu.set_spsr(cpu.cpsr);
            cpu.cpsr.control_bits.irq_disable = true;
            cpu.set_register(15, 0x18);
        }
    }
}



