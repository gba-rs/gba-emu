use crate::memory::work_ram::WorkRam;
use crate::memory::interrupt_registers::*;



pub fn enable_flags(InterruptEnableRegister ie, u16 flags){
    ie.get_register() |= flags;
}

pub fn set_interrupts(InterruptRequestFlags irq, u16 flags){
    irq.get_register() |= flags;
}

pub fn pending_interruipts() -> bool {
    return true;
}

pub fn service_interrupts(){
    //do something to service the interrupts here maybe?
    
}
//so my general issue is not knowing where to set the memory and have it live
//i dont wanna call directly into into the gba i dont think, and i dont wanna to pass it in