use wasm_bindgen::__rt::core::cell::RefCell;
use wasm_bindgen::__rt::std::rc::Rc;
use memory_macros::*;
use super::GbaMem;
use serde::{Serialize, Deserialize};

io_register! (
    TimerDataRegister => 2, [0x4000100, 0x4000104, 0x4000108, 0x400010C],
    data: 0, 16
);

io_register! (
    TimerControlRegister => 2, [0x4000102, 0x4000106, 0x400010A, 0x400010E],
    pre_scalar_selection: 0,2,
    cascade: 2,1,
    irq_enable: 6,1,
    enable: 7,1,
);

impl TimerDataRegister {
    pub fn get_reload(&self) -> u16 {
        if let Some(mem) = &self.memory {
            let mem_ref = mem.borrow();
            let address = 0x1000_0000 + (TimerDataRegister::SEGMENT_INDICIES[self.index] & 0xF); 
            return (mem_ref[address] as u32 | ((mem_ref[address + 1] as u32) << 8)) as u16;
        } else {
            panic!("IO register was accessed without being registered");
        }
    }

    pub fn write_reload(&mut self, value: u16) {
        if let Some(mem) = &self.memory {
            let mut mem_ref = mem.borrow_mut();
            let address = 0x1000_0000 + (self.index * 2); 

            mem_ref[address] = (value & 0xFF) as u8;
            mem_ref[address + 1] = ((value & 0xFF00) >> 8) as u8;
        } else {
            panic!("IO register was accessed without being registered");
        }
    }
}
