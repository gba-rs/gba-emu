use crate::memory::{dma_registers::*, GbaMem, memory_bus::MemoryBus};
use crate::interrupts::interrupts::Interrupts;
use crate::memory::sound_registers::SoundControlHigh;
use std::rc::Rc;
use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
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
        write!(f, "DMA {}: {:X}, {:X}, {:X}, word size: {:X}, source address control: {:X}", self.id, self.internal_source_address, self.internal_destination_address, self.internal_word_count, self.control.get_dma_transfer_type(), self.control.get_source_address_control())
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

    pub fn register(&mut self, mem: &Rc<GbaMem>) {
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

        match self.control.get_destination_address_control() {
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
        self.internal_source_address = if self.id == 0 {
            self.source_address.get_address() & 0x7FF_FFFF
        } else {
            self.source_address.get_address() & 0xFFF_FFFF
        };
        self.internal_destination_address = if self.id == 3 {
            self.destination_address.get_address() & 0xFFF_FFFF
        } else {
            self.destination_address.get_address() & 0x7FF_FFFF
        };
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
                let dest_region = self.internal_destination_address >> 24;
                let src_region = self.internal_source_address >> 24;
                if dest_region == 0x0D {
                    mem_map.prepare_eeprom_write(self.internal_word_count);
                } else if src_region == 0x0D {
                    mem_map.prepare_eeprom_read(self.internal_word_count);
                }

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

    pub fn refill_sound_fifo(&mut self, mem_map: &mut MemoryBus, is_fifo_a: bool) {
        let value = mem_map.read_u32(self.internal_source_address & !3);
        let fifo = if is_fifo_a { &mut mem_map.mem_map.fifo_a } else { &mut mem_map.mem_map.fifo_b };
        for byte in value.to_le_bytes() {
            if fifo.len() < 32 {
                fifo.push_back(byte);
            }
        }

        match self.control.get_source_address_control() {
            0 => self.internal_source_address += 4,
            1 => self.internal_source_address -= 4,
            2 => {},
            _ => panic!("Invalid source address control")
        }
    }
}

const FIFO_A_ADDRESS: u32 = 0x0400_00A0;
const FIFO_B_ADDRESS: u32 = 0x0400_00A4;

#[derive(Serialize, Deserialize)]
pub struct DMAController {
    pub dma_channels: [DMAChannel; 4],
    pub hblanking: bool,
    pub vblanking: bool,
    sound_control_high: SoundControlHigh,
}

impl DMAController {
    pub fn register(&mut self, mem: &Rc<GbaMem>) {
        for i in 0..4 {
            self.dma_channels[i].register(mem);
        }
        self.sound_control_high.register(mem);
    }

    pub fn update(&mut self, mem_map: &mut MemoryBus, irq_ctl: &mut Interrupts, timer_overflows: [usize; 4]) {
        for i in 0..4 {
            if self.dma_channels[i].control.get_dma_enable() == 1 {
                if self.dma_channels[i].previously_disabled {
                    self.dma_channels[i].reload_data();
                    self.dma_channels[i].previously_disabled = false;
                }

                match self.dma_channels[i].control.get_dma_start_timing() {
                    0 => {
                        // start immedietly
                        self.dma_channels[i].transfer(mem_map, irq_ctl);
                    },
                    1 => {
                        // start at vblank
                        if self.vblanking {
                            self.dma_channels[i].transfer(mem_map, irq_ctl);
                            self.vblanking = false;
                        }
                    },
                    2 => {
                        // start at hblank
                        if self.hblanking {
                            self.dma_channels[i].transfer(mem_map, irq_ctl);
                            self.hblanking = false;
                        }
                        // self.dma_channels[i].control.set_dma_enable(0);
                    },
                    3 => {
                        let destination = self.dma_channels[i].destination_address.get_address();
                        let is_fifo_a = (i == 1 || i == 2) && destination == FIFO_A_ADDRESS;
                        let is_fifo_b = (i == 1 || i == 2) && destination == FIFO_B_ADDRESS;
                        if is_fifo_a || is_fifo_b {
                            let timer = if is_fifo_a {
                                self.sound_control_high.get_dma_sound_a_timer_select() as usize
                            } else {
                                self.sound_control_high.get_dma_sound_b_timer_select() as usize
                            };
                            let fifo_len = if is_fifo_a { mem_map.mem_map.fifo_a.len() } else { mem_map.mem_map.fifo_b.len() };
                            if timer_overflows[timer] > 0 && fifo_len <= 4 {
                                self.dma_channels[i].refill_sound_fifo(mem_map, is_fifo_a);
                            }
                        }
                        if self.dma_channels[i].control.get_dma_repeat() == 0 {
                            self.dma_channels[i].control.set_dma_enable(0);
                        }
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
            vblanking: false,
            sound_control_high: SoundControlHigh::new(),
        }
    }
}
