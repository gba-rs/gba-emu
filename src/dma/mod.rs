use crate::memory::{dma_registers::*, GbaMem, memory_map::MemoryMap};
use std::cell::RefCell;
use std::rc::Rc;

pub struct DMAChannel {
    pub source_address: DMASourceAddress,
    pub destination_address: DMADestinationAddress,
    pub word_count: DMAWordCount,
    pub control: DMAControl,
    pub internal_source_address: u32,
    pub internal_destination_address: u32,
    pub internal_word_count: u16,
    pub start_transfer: bool,
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
            start_transfer: false,
            id: channel
        }
    }

    pub fn register(&mut self, mem: &Rc<RefCell<GbaMem>>) {
        self.source_address.register(mem);
        self.destination_address.register(mem);
        self.word_count.register(mem);
        self.control.register(mem);
    }

    pub fn transfer(&mut self, mem_map: &mut MemoryMap) {
        
        // if we are doing a fifo just skip it for now. 
        // TODO remove this for sound shit
        if self.control.get_dma_start_timing() == 3 && self.control.get_dma_repeat() == 1 && (self.id == 1 || self.id == 2) {
            self.control.set_dma_enable(0);
            self.start_transfer = false;
            return;
        }
        
        self.internal_source_address = self.source_address.get_address();
        self.internal_destination_address = self.destination_address.get_address();
        self.internal_word_count = self.word_count.get_register();



        // if we aren't repeating reset the enable bit
        if self.control.get_dma_repeat() == 0 {
            self.control.set_dma_enable(0);
            self.start_transfer = false;
        }
    }
}

pub struct DMAController {
    pub dma_channels: [DMAChannel; 4]
}

impl DMAController {
    pub fn register(&mut self, mem: &Rc<RefCell<GbaMem>>) {
        for i in 0..4 {
            self.dma_channels[i].register(mem);
        }
    }
    
    pub fn set_starting(&mut self, val: u8) {
        for i in 0..4 {
            let start_time = self.dma_channels[i].control.get_dma_start_timing();
            self.dma_channels[i].start_transfer = (start_time == val) || start_time == 0;
        }
    }

    pub fn is_active(&self) -> bool {
        let mut result = false;
        for i in 0..4 {
            if self.dma_channels[i].control.get_dma_enable() == 1 && self.dma_channels[i].start_transfer {
                result = true;
            }
        }

        return result;
    }

    pub fn start_transfer(&mut self, mem_map: &mut MemoryMap) {
        for i in 0..4 {
            if self.dma_channels[i].control.get_dma_enable() == 1  && self.dma_channels[i].start_transfer {
                log::debug!("DMA_{}: Source: {:X}, Destination: {:X}, Word Count: {}, Dma Repeat: {}, Start Timing: {}", i, self.dma_channels[i].source_address.get_address(), self.dma_channels[i].destination_address.get_address(), self.dma_channels[i].word_count.get_register(), self.dma_channels[i].control.get_dma_repeat(), self.dma_channels[i].control.get_dma_start_timing());
                self.dma_channels[i].transfer(mem_map);
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
            ]
        }
    }
}