use std::cell::RefCell;
use std::rc::Rc;
use memory_macros::*;
use crate::operations::bitutils::*;

#[memory_segment(2)]
#[bit_field(button_a, 0, 1)]
#[bit_field(button_b, 1, 1)]
#[bit_field(button_select, 2, 1)]
#[bit_field(button_start, 3, 1)]
#[bit_field(dpad_right, 4, 1)]
#[bit_field(dpad_left, 5, 1)]
#[bit_field(dpad_up, 6, 1)]
#[bit_field(dpad_down, 7, 1)]
#[bit_field(button_r, 8, 1)]
#[bit_field(button_l, 9, 1)]
pub struct KeyStatus {
    pub memory: Rc<RefCell<Vec<u8>>>
}

#[memory_segment(2)]
#[bit_field(button_a, 0, 1)]
#[bit_field(button_b, 1, 1)]
#[bit_field(button_select, 2, 1)]
#[bit_field(button_start, 3, 1)]
#[bit_field(dpad_right, 4, 1)]
#[bit_field(dpad_left, 5, 1)]
#[bit_field(dpad_up, 6, 1)]
#[bit_field(dpad_down, 7, 1)]
#[bit_field(button_r, 8, 1)]
#[bit_field(button_l, 9, 1)]
#[bit_field(irq_enable_flag, 14, 1)]
#[bit_field(irq_condition, 15, 1)]
pub struct KeyInterruptControl {
    pub memory: Rc<RefCell<Vec<u8>>>
}