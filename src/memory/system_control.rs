use wasm_bindgen::__rt::core::cell::RefCell;
use wasm_bindgen::__rt::std::rc::Rc;
use memory_macros::*;

#[memory_segment(2)]
#[bit_field(sram_wait_control, 0, 2)]
#[bit_field(wait_state_zero_first_access, 2, 2)]
#[bit_field(wait_state_zero_second_access, 4, 1)]
#[bit_field(wait_state_one_first_access, 5, 2)]
#[bit_field(wait_state_one_second_access, 7, 1)]
#[bit_field(wait_state_two_first_access, 8, 2)]
#[bit_field(wait_state_two_second_access, 10, 1)]
#[bit_field(phi_terminal_output, 11, 2)]
#[bit_field(gamepak_prefetch_buffer, 14, 1)]
#[bit_field(gamepak_type_flag, 15, 1)]
pub struct WaitStateControl {
    pub memory: Rc<RefCell<Vec<u8>>>
}