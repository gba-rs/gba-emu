use wasm_bindgen::__rt::core::cell::RefCell;
use wasm_bindgen::__rt::std::rc::Rc;
use memory_macros::*;
use super::GbaMem;

#[multiple_memory_segment(2, 0x4000100, 0x4000104, 0x4000108, 0x400010C)]
#[bit_field(data, 0, 15)]
pub struct TimerDataRegister {
    pub memory: Rc<RefCell<GbaMem>>,
    pub index: usize
}

#[multiple_memory_segment(2, 0x4000102, 0x4000106, 0x400010A, 0x400010E)]
#[bit_field(pre_scalar_selection, 0,2)]
#[bit_field(count_up_enable, 2,1)]
#[bit_field(timer_irq_enable, 6,1)]
#[bit_field(timer_start_stop, 7,1)]
pub struct TimerControlRegister{
    pub memory: Rc<RefCell<GbaMem>>,
    pub index: usize
}