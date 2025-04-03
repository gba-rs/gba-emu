use std::cell::RefCell;
use std::rc::Rc;
use memory_macros::*;
use super::GbaMem;
use serde::{Serialize, Deserialize};

io_register! (
    DMASourceAddress => 4, [0x40000B0, 0x40000BC, 0x40000C8, 0x40000D4],
    address: 0, 28
);

io_register! (
    DMADestinationAddress => 4, [0x40000B4, 0x40000C0, 0x40000CC, 0x40000D8],
    address: 0, 28
);

io_register! (
    DMAWordCount => 2, [0x40000B8, 0x40000C4, 0x40000D0, 0x40000DC],
    word_count: 0, 16
);

io_register! (
    DMAControl => 2, [0x40000BA, 0x40000C6, 0x40000D2, 0x40000DE],
    destination_address_control: 5, 2,
    source_address_control: 7, 2,
    dma_repeat: 9, 1,
    dma_transfer_type: 10, 1,
    gamepack_drq: 11, 1,
    dma_start_timing: 12, 2,
    irq_upon_end_of_wordcount: 14, 1,
    dma_enable: 15, 1,
);
