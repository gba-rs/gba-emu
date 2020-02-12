use wasm_bindgen::__rt::core::cell::RefCell;
use wasm_bindgen::__rt::std::rc::Rc;
use memory_macros::*;
use super::GbaMem;

#[memory_segment(2, 0x4000100)]
#[bit_field(data, 0, 15)]
pub struct TimerDataRegister0 {
    pub memory: Rc<RefCell<GbaMem>>
}

#[memory_segment(2, 0x4000104)]
#[bit_field(data, 0, 15)]
pub struct TimerDataRegister1 {
    pub memory: Rc<RefCell<GbaMem>>
}

#[memory_segment(2, 0x4000108)]
#[bit_field(data, 0, 15)]
pub struct TimerDataRegister2{
    pub memory: Rc<RefCell<GbaMem>>
}

#[memory_segment(2, 0x400010C)]
#[bit_field(data, 0, 15)]
pub struct TimerDataRegister3{
    pub memory: Rc<RefCell<GbaMem>>
}

#[memory_segment(2, 0x4000102)]
#[bit_field(pre_scalar_selection, 0,2)]
#[bit_field(count_up_enable, 2,1)]
#[bit_field(timer_irq_enable, 6,1)]
#[bit_field(timer_start_stop, 7,1)]
pub struct TimerControlRegister0{
    pub memory: Rc<RefCell<GbaMem>>
}

#[memory_segment(2, 0x4000106)]
#[bit_field(pre_scalar_selection, 0,2)]
#[bit_field(count_up_enable, 2,1)]
#[bit_field(timer_irq_enable, 6,1)]
#[bit_field(timer_start_stop, 7,1)]
pub struct TimerControlRegister1{
    pub memory: Rc<RefCell<GbaMem>>
}

#[memory_segment(2, 0x400010A)]
#[bit_field(pre_scalar_selection, 0,2)]
#[bit_field(count_up_enable, 2,1)]
#[bit_field(timer_irq_enable, 6,1)]
#[bit_field(timer_start_stop, 7,1)]
pub struct TimerControlRegister2{
    pub memory: Rc<RefCell<GbaMem>>
}

#[memory_segment(2, 0x400010E)]
#[bit_field(pre_scalar_selection, 0,2)]
#[bit_field(count_up_enable, 2,1)]
#[bit_field(timer_irq_enable, 6,1)]
#[bit_field(timer_start_stop, 7,1)]
pub struct TimerControlRegister3{
    pub memory: Rc<RefCell<GbaMem>>
}