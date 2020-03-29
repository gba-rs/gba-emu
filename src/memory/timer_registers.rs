use wasm_bindgen::__rt::core::cell::RefCell;
use wasm_bindgen::__rt::std::rc::Rc;
use memory_macros::*;
use super::GbaMem;

io_register! (
    TimerDataRegister => 2, [0x4000100, 0x4000104, 0x4000108, 0x400010C],
    data: 0, 15
);

io_register! (
    TimerControlRegister => 2, [0x4000102, 0x4000106, 0x400010A, 0x400010E],
    pre_scalar_selection: 0,2,
    count_up_enable: 2,1,
    timer_irq_enable: 6,1,
    timer_start_stop: 7,1,
);