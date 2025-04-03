//4000208h - IME - Interrupt Master Enable Register (R/W)
//4000200h - IE - Interrupt Enable Register (R/W)
//4000202h - IF - Interrupt Request Flags / IRQ Acknowledge (R/W, see below)
use std::cell::RefCell;
use std::rc::Rc;
use memory_macros::*;
use super::GbaMem;
use serde::{Serialize, Deserialize};

io_register! (
    InterruptMasterEnableRegister => 4, 0x4000208,
    disable: 0, 1
);

io_register! (
    InterruptEnableRegister => 2, 0x4000200,
    lcd_v_blank: 0, 1,
    lcd_h_blank: 1, 1,
    lcd_v_counter_: 2, 1,
    timer_zero_overflow: 3, 1,
    timer_one_overflow: 4, 1,
    timer_two_overflow: 5, 1,
    timer_three_overflow: 6, 1,
    serial_communication: 7, 1,
    dma_zero: 8, 1,
    dma_one: 9, 1,
    dma_two: 10, 1,
    dma_three: 11, 1,
    keypad: 12, 1,
    game_pack: 13, 1,
);

io_register! (
    InterruptRequestFlags => 2, 0x4000202,
    lcd_v_blank: 0, 1,
    lcd_h_blank: 1, 1,
    lcd_v_counter: 2, 1,
    timer_zero_overflow: 3, 1,
    timer_one_overflow: 4, 1,
    timer_two_overflow: 5, 1,
    timer_three_overflow: 6, 1,
    serial_communication: 7, 1,
    dma_zero: 8, 1,
    dma_one: 9, 1,
    dma_two: 10, 1,
    dma_three: 11, 1,
    keypad: 12, 1,
    game_pack: 13, 1,
);
