use crate::memory::{dma_registers::*, GbaMem, memory_bus::MemoryBus};
use crate::interrupts::interrupts::Interrupts;
use std::cell::RefCell;
use std::rc::Rc;

pub struct DMAChannel {
    pub source_address: DMASourceAddress,
    pub destination_address: DMADestinationAddress,
    pub word_count: DMAWordCount,
    pub control: DMAControl,
    pub internal_source_address: u32,
    pub internal_destination_address: u32,
    pub internal_word_count: u32,
    pub id: usize
}

impl DMAChannel {
    pub fn new(channel: usize) -> DMAChannel {
        assert!(channel < 4);
        return DMAChannel {
            source_address: DMASourceAddress::new(channel),
            destination_address: DMADestinationAddress::new(channel),
            word_count: DMAWordCount::new(channel),
            control: DMAControl::new(channel),
            internal_source_address: 0,
            internal_destination_address: 0,
            internal_word_count: 0,
            id: channel
        }
    }

    pub fn register(&mut self, mem: &Rc<RefCell<GbaMem>>) {
        self.source_address.register(mem);
        self.destination_address.register(mem);
        self.word_count.register(mem);
        self.control.register(mem);
    }

    pub fn update_addresses(&mut self, word_size: u32) {
        match self.control.get_source_address_control() {
            0 => {
                self.internal_source_address += word_size;
            },
            1 => {
                self.internal_source_address -= word_size;
            },
            2 => {},
            _ => panic!("Invalid source address control")
        }

        match self.control.get_source_address_control() {
            0 | 3 => {
                self.internal_destination_address += word_size;
            },
            1 => {
                self.internal_destination_address -= word_size;
            },
            2 => {},
            _ => panic!("Invalid source address control")
        }
    }

    pub fn transfer(&mut self, mem_map: &mut MemoryBus, irq_ctl: &mut Interrupts) {        
        self.internal_source_address = self.source_address.get_address();
        self.internal_destination_address = self.destination_address.get_address();
        self.internal_word_count = self.word_count.get_word_count();

        // log::info!("Transfer on channel {}: {:X}, {:X}, {:X}", self.id, self.internal_source_address, self.internal_destination_address, self.internal_word_count);

        // check the word size
        // go for up to word count reading and writing

        match self.control.get_dma_transfer_type() {
            0 => {  // 16
                for _ in 0..self.internal_word_count {
                    let value = mem_map.read_u16(self.internal_source_address & !1);
                    mem_map.write_u16(self.internal_destination_address & !1, value);

                    self.update_addresses(2);
                }
            },
            1 => { // 32
                for _ in 0..self.internal_word_count {
                    let value = mem_map.read_u32(self.internal_source_address & !3);
                    mem_map.write_u32(self.internal_destination_address & !3, value);

                    self.update_addresses(4);
                }
            },
            _ => panic!("DMA transfer type error")
        } 

        // trigger IRQ here
        irq_ctl.if_interrupt.set_register((irq_ctl.if_interrupt.get_register() as u32) | (0x1 << (8 + self.id)));

        // if we aren't repeating reset the enable bit
        if self.control.get_dma_repeat() == 0 {
            self.control.set_dma_enable(0);
        } else {
            if self.control.get_destination_address_control() == 3 {
                // reload
                self.internal_destination_address = self.destination_address.get_address();
            }
        }
    }
}

pub struct DMAController {
    pub dma_channels: [DMAChannel; 4],
    pub hblanking: bool,
    pub vblanking: bool
}

impl DMAController {
    pub fn register(&mut self, mem: &Rc<RefCell<GbaMem>>) {
        for i in 0..4 {
            self.dma_channels[i].register(mem);
        }
    }

    pub fn update(&mut self, mem_map: &mut MemoryBus, irq_ctl: &mut Interrupts) {
        for i in 0..4 {
            if self.dma_channels[i].control.get_dma_enable() == 1 {
                match self.dma_channels[i].control.get_dma_start_timing() {
                    0 => {
                        // start immedietly 
                        self.dma_channels[i].transfer(mem_map, irq_ctl);
                    },
                    1 => {
                        // start at vblank
                        if self.vblanking {
                            self.dma_channels[i].transfer(mem_map, irq_ctl);
                        }
                    },
                    2 => {
                        // start at hblank
                        if self.hblanking {
                            self.dma_channels[i].transfer(mem_map, irq_ctl);
                        }
                    },
                    3 => {
                        // special
                        // TODO implement this
                        self.dma_channels[i].control.set_dma_enable(0);
                    },
                    _ => {
                        panic!("DMA Update fucked up")
                    }
                }
            }
        }
    }

    pub fn new() -> DMAController {
        return DMAController {
            dma_channels: [
                DMAChannel::new(0),
                DMAChannel::new(1),
                DMAChannel::new(2),
                DMAChannel::new(3),
            ],
            hblanking: false,
            vblanking: false
        }
    }
}