use std::cell::RefCell;
use std::rc::Rc;
use memory_macros::*;
use super::GbaMem;
use serde::{Serialize, Deserialize};

io_register! (
    KeyStatus => 2, 0x4000130,
    button_a: 0, 1,
    button_b: 1, 1,
    button_select: 2, 1,
    button_start: 3, 1,
    dpad_right: 4, 1,
    dpad_left: 5, 1,
    dpad_up: 6, 1,
    dpad_down: 7, 1,
    button_r: 8, 1,
    button_l: 9, 1,
);

io_register! (
    KeyInterruptControl => 2, 0x4000132,
    button_a: 0, 1,
    button_b: 1, 1,
    button_select: 2, 1,
    button_start: 3, 1,
    dpad_right: 4, 1,
    dpad_left: 5, 1,
    dpad_up: 6, 1,
    dpad_down: 7, 1,
    button_r: 8, 1,
    button_l: 9, 1,
    irq_enable_flag: 14, 1,
    irq_condition: 15, 1,
);
