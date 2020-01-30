use wasm_bindgen::__rt::core::cell::RefCell;
use wasm_bindgen::__rt::std::rc::Rc;
use memory_macros::*;

// TODO remaining bit fields
#[memory_segment(2)]
#[bit_field(sram_wait_control, 0, 2)]
#[bit_field(wait_state_zero_first_access, 2, 2)]
pub struct WaitStateControl {
    pub memory: Rc<RefCell<Vec<u8>>>
}