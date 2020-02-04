use crate::memory::interrupt_registers::*;
use crate::cpu::{program_status_register::ProgramStatusRegister, cpu};

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
        if should_service != 0x0 {
            cpu.set_register(15, 0x18);
        }
    }
}



