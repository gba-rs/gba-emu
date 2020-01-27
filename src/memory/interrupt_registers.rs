//4000208h - IME - Interrupt Master Enable Register (R/W)
//4000200h - IE - Interrupt Enable Register (R/W)
//4000202h - IF - Interrupt Request Flags / IRQ Acknowledge (R/W, see below)
use std::cell::RefCell;
use std::rc::Rc;
use crate::operations::{bitutils::get_bits_u8};
use memory_macros::*;

#[memory_segment(2)]
#[bit_field(disable, 0, 1)]
pub struct InterruptMasterEnableRegister {
    pub memory: Rc<RefCell<Vec<u8>>>
}

#[memory_segment(2)]
#[bit_field(lcd_v_blank, 0, 1)]
#[bit_field(lcd_h_blank, 1, 1)]
#[bit_field(lcd_v_counter_, 2, 1)]
#[bit_field(timer_zero_overflow, 3, 1)]
#[bit_field(timer_one_overflow, 4, 1)]
#[bit_field(timer_two_overflow, 5, 1)]
#[bit_field(timer_three_overflow, 6, 1)]
#[bit_field(serial_communication, 7, 1)]
#[bit_field(dma_zero, 8, 1)]
#[bit_field(dma_one, 9, 1)]
#[bit_field(dma_two, 10, 1)]
#[bit_field(dma_three, 11, 1)]
#[bit_field(keypad, 12, 1)]
#[bit_field(game_pack, 13, 1)]
pub struct InterruptEnableRegister {
    pub memory: Rc<RefCell<Vec<u8>>>
}

#[memory_segment(2)]
#[bit_field(lcd_v_blank, 0, 1)]
#[bit_field(lcd_h_blank, 1, 1)]
#[bit_field(lcd_v_counter_, 2, 1)]
#[bit_field(timer_zero_overflow, 3, 1)]
#[bit_field(timer_one_overflow, 4, 1)]
#[bit_field(timer_two_overflow, 5, 1)]
#[bit_field(timer_three_overflow, 6, 1)]
#[bit_field(serial_communication, 7, 1)]
#[bit_field(dma_zero, 8, 1)]
#[bit_field(dma_one, 9, 1)]
#[bit_field(dma_two, 10, 1)]
#[bit_field(dma_three, 11, 1)]
#[bit_field(keypad, 12, 1)]
#[bit_field(game_pack, 13, 1)]
pub struct InterruptRequestFlags {
    pub memory: Rc<RefCell<Vec<u8>>>
}