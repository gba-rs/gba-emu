use std::cell::RefCell;
use std::rc::Rc;
use memory_macros::*;
use super::GbaMem;

#[multiple_memory_segment(4, 0x40000B0, 0x40000BC, 0x40000C8, 0x40000D4)]
#[bit_field(address, 0, 28)]
pub struct DMASourceAddress {
    pub memory: Rc<RefCell<GbaMem>>,
    pub index: usize
}

#[multiple_memory_segment(4, 0x40000B4, 0x40000C0, 0x40000CC, 0x40000D8)]
#[bit_field(address, 0, 28)]
pub struct DMADestinationAddress {
    pub memory: Rc<RefCell<GbaMem>>,
    pub index: usize
}

#[multiple_memory_segment(2, 0x40000B8, 0x40000C4, 0x40000D0, 0x40000DC)]
pub struct DMAWordCount {
    pub memory: Rc<RefCell<GbaMem>>,
    pub index: usize
}

#[multiple_memory_segment(2, 0x40000BA, 0x40000C6, 0x40000D2, 0x40000DE)]
#[bit_field(destination_address_control, 5, 2)]
#[bit_field(source_address_control, 7, 2)]
#[bit_field(dma_repeat, 9, 1)]
#[bit_field(dma_transfer_type, 10, 1)]
#[bit_field(gamepack_drq, 11, 1)]
#[bit_field(dma_start_timing, 12, 2)]
#[bit_field(irq_upon_end_of_wordcount, 14, 1)]
#[bit_field(dma_enable, 15, 1)]
pub struct DMAControl {
    pub memory: Rc<RefCell<GbaMem>>,
    pub index: usize
}