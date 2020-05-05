use crate::memory::{dma_registers::*, GbaMem, memory_bus::MemoryBus};
use crate::interrupts::interrupts::Interrupts;
use std::cell::RefCell;
use std::rc::Rc;
use std::fmt;

pub struct DMAChannel {
    pub source_address: DMASourceAddress,
    pub destination_address: DMADestinationAddress,
    pub word_count: DMAWordCount,
    pub control: DMAControl,
    pub internal_source_address: u32,
    pub internal_destination_address: u32,
    pub internal_word_count: u32,
    pub id: usize,
    pub previously_disabled: bool
}

impl fmt::Debug for DMAChannel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DMA {}: {:X}, {:X}, {:X}", self.id, self.internal_source_address, self.internal_destination_address, self.internal_word_count)
    }
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
            id: channel,
            previously_disabled: true,
        }
    }

    pub fn register(&mut self, mem: &Rc<RefCell<GbaMem>>) {
        self.source_address.register(mem);
        self.destination_address.register(mem);
        self.word_count.register(mem);
        self.control.register(mem);
    }

    pub fn update_source_address(&mut self) {
        let word_size = if self.control.get_dma_transfer_type() == 0 { 2 } else { 4 };

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
    }

    
    pub fn update_destination_address(&mut self) {
        let word_size = if self.control.get_dma_transfer_type() == 0 { 2 } else { 4 };

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

    fn reload_data(&mut self) {
        self.internal_source_address = self.source_address.get_address();
        self.internal_destination_address = self.destination_address.get_address();
        self.reload_wordcount();
    }

    fn reload_wordcount(&mut self) {
        self.internal_word_count = if self.id != 3 {
            self.word_count.get_word_count() & 0x7FFF
        } else {
            self.word_count.get_word_count()
        } as u32;

        if self.internal_word_count == 0 {
            self.internal_word_count = if self.id != 3 { 0x4000 } else { 0x10000 };    
        }
    }

    pub fn transfer(&mut self, mem_map: &mut MemoryBus, irq_ctl: &mut Interrupts) {        
        match self.control.get_dma_transfer_type() {
            0 => {  // 16
                for _ in 0..self.internal_word_count {
                    let value = mem_map.read_u16(self.internal_source_address & !1);
                    mem_map.write_u16(self.internal_destination_address & !1, value);

                    self.update_source_address();
                    self.update_destination_address();
                }
            },
            1 => { // 32
                for _ in 0..self.internal_word_count {
                    let value = mem_map.read_u32(self.internal_source_address & !3);
                    mem_map.write_u32(self.internal_destination_address & !3, value);

                    self.update_source_address();
                    self.update_destination_address();
                }
            },
            _ => panic!("DMA transfer type error")
        } 

        // trigger IRQ here
        if self.control.get_irq_upon_end_of_wordcount() != 0 {
            irq_ctl.if_interrupt.set_register((irq_ctl.if_interrupt.get_register() as u32) | (0x1 << (8 + self.id)));
        }

        // if we aren't repeating reset the enable bit
        if self.control.get_dma_repeat() == 0 {
            self.control.set_dma_enable(0);
            self.previously_disabled = true;
        } else {
            if self.control.get_destination_address_control() == 3 {
                // reload
                self.internal_destination_address = self.destination_address.get_address();
            }

            self.reload_wordcount();
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
                if self.dma_channels[i].previously_disabled {
                    self.dma_channels[i].reload_data();
                    self.dma_channels[i].previously_disabled = false;
                }

                // self.dma_channels[i].reload_data();
                match self.dma_channels[i].control.get_dma_start_timing() {
                    0 => {
                        // start immedietly
                        self.dma_channels[i].transfer(mem_map, irq_ctl);
                        // log::info!("Doing dma: {:?}", self.dma_channels[i]);
                    },
                    1 => {
                        // start at vblank
                        if self.vblanking {
                            // log::info!("Doing vblank dma: {:?}", self.dma_channels[i]);
                            self.dma_channels[i].transfer(mem_map, irq_ctl);
                            self.vblanking = false;
                        }
                    },
                    2 => {
                        // start at hblank
                        if self.hblanking {
                            // log::info!("Doing hblank dma: {:?}", self.dma_channels[i]);
                            // self.dma_channels[i].transfer(mem_map, irq_ctl);
                            // self.hblanking = false;
                            self.dma_channels[i].control.set_dma_enable(0);
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
            } else if !self.dma_channels[i].previously_disabled {
                self.dma_channels[i].previously_disabled = true;
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