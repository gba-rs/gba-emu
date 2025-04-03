use wasm_bindgen::__rt::core::cell::RefCell;
use wasm_bindgen::__rt::std::rc::Rc;
use memory_macros::*;
use super::GbaMem;
use serde::{Serialize, Deserialize};

io_register! (
    WaitStateControl => 2, 0x4000204,
    sram_wait_control: 0, 2,
    wait_state_zero_first_access: 2, 2,
    wait_state_zero_second_access: 4, 1,
    wait_state_one_first_access: 5, 2,
    wait_state_one_second_access: 7, 1,
    wait_state_two_first_access: 8, 2,
    wait_state_two_second_access: 10, 1,
    phi_terminal_output: 11, 2,
    gamepak_prefetch_buffer: 14, 1,
    gamepak_type_flag: 15, 1,
);
