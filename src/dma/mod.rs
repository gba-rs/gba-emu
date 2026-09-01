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
    pub previously_disabled: bool,
    #[serde(skip)]
    pub pending_immediate: bool,
    pub last_bus_value: u32,
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
            pending_immediate: false,
            last_bus_value: 0,
        }
    }

    fn is_dma_protected_region(address: u32) -> bool {
        address < 0x0200_0000
    }

    fn put_on_bus(&mut self, value: u32) {
        self.last_bus_value = value;
        crate::memory::memory_map::DMA_BUS_OVERRIDE.with(|d| d.set(Some(value)));
    }

    fn latched_read_u16(&mut self, mem_map: &mut MemoryBus, address: u32) -> u16 {
        if Self::is_dma_protected_region(address) {
            self.put_on_bus(self.last_bus_value);
            return self.last_bus_value as u16;
        }
        let value = mem_map.read_u16(address);
        self.put_on_bus(value as u32);
        value
    }

    fn latched_read_u32(&mut self, mem_map: &mut MemoryBus, address: u32) -> u32 {
        if Self::is_dma_protected_region(address) {
            self.put_on_bus(self.last_bus_value);
            return self.last_bus_value;
        }
        let value = mem_map.read_u32(address);
        self.put_on_bus(value);
        value
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
        // GBATEK: starting a DMA costs 2I; +2I more if source and destination are both gamepak memory.
        mem_map.cycle_clock.cycles += 2;
        let src_is_gamepak = (0x08..=0x0D).contains(&(self.internal_source_address >> 24));
        let dst_is_gamepak = (0x08..=0x0D).contains(&(self.internal_destination_address >> 24));
        if src_is_gamepak && dst_is_gamepak {
            mem_map.cycle_clock.cycles += 2;
        }

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
                    let value = self.latched_read_u16(mem_map, self.internal_source_address & !1);
                    mem_map.write_u16(self.internal_destination_address & !1, value);

                    self.update_source_address();
                    self.update_destination_address();
                }
            },
            1 => { // 32
                for _ in 0..self.internal_word_count {
                    let value = self.latched_read_u32(mem_map, self.internal_source_address & !3);
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
        // GBATEK: starting a DMA costs 2I (destination here is always the IO FIFO register,
        // never gamepak, so the "+2I both regions gamepak" case never applies).
        mem_map.cycle_clock.cycles += 2;

        for _ in 0..4 {
            let value = self.latched_read_u32(mem_map, self.internal_source_address & !3);
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
}

#[cfg(test)]
mod dma_channel_tests {
    use super::*;
    use crate::memory::memory_bus::MemoryBus;
    use crate::interrupts::interrupts::Interrupts;

    #[test]
    fn refill_sound_fifo_transfers_four_words() {
        let mut channel = DMAChannel::new(1);
        let mut bus = MemoryBus::new_stub();
        let base = 0x0200_0000u32;
        for i in 0..4u32 {
            bus.write_u32(base + i * 4, 0x1000_0000 * (i + 1));
        }
        channel.internal_source_address = base;

        channel.refill_sound_fifo(&mut bus, true);

        assert_eq!(bus.mem_map.fifo_a.len(), 16);
        assert_eq!(channel.internal_source_address, base + 16);
    }

    fn one_shot_word_transfer(channel: &mut DMAChannel) {
        channel.control.set_dma_transfer_type(1);
        channel.control.set_source_address_control(0);
        channel.control.set_destination_address_control(0);
        channel.control.set_dma_repeat(0);
        channel.internal_word_count = 1;
    }

    #[test]
    fn dma_reading_protected_region_returns_its_own_last_latched_value() {
        let mut channel = DMAChannel::new(0);
        let mut bus = MemoryBus::new_stub();
        channel.register(&bus.mem_map.memory);
        let mut irq = Interrupts::new();
        one_shot_word_transfer(&mut channel);

        let scratch = 0x0300_0000u32;
        bus.write_u32(scratch, 0xCAFEBABE);
        channel.internal_source_address = scratch;
        channel.internal_destination_address = 0x0300_1000;
        channel.transfer(&mut bus, &mut irq);

        one_shot_word_transfer(&mut channel);
        channel.internal_source_address = 0x0;
        channel.internal_destination_address = 0x0300_2000;
        channel.transfer(&mut bus, &mut irq);

        assert_eq!(bus.read_u32(0x0300_2000), 0xCAFEBABE);
    }

    #[test]
    fn each_dma_channel_has_an_independent_latch() {
        let mut bus = MemoryBus::new_stub();
        let mut irq = Interrupts::new();

        let mut channel0 = DMAChannel::new(0);
        channel0.register(&bus.mem_map.memory);
        one_shot_word_transfer(&mut channel0);
        bus.write_u32(0x0300_0000, 0x1BADF00D);
        channel0.internal_source_address = 0x0300_0000;
        channel0.internal_destination_address = 0x0300_1000;
        channel0.transfer(&mut bus, &mut irq);

        let mut channel1 = DMAChannel::new(1);
        channel1.register(&bus.mem_map.memory);
        one_shot_word_transfer(&mut channel1);
        bus.write_u32(0x0300_0004, 0x2BADCAFE);
        channel1.internal_source_address = 0x0300_0004;
        channel1.internal_destination_address = 0x0300_1004;
        channel1.transfer(&mut bus, &mut irq);

        one_shot_word_transfer(&mut channel1);
        channel1.internal_source_address = 0x0;
        channel1.internal_destination_address = 0x0300_2000;
        channel1.transfer(&mut bus, &mut irq);

        assert_eq!(bus.read_u32(0x0300_2000), 0x2BADCAFE);
    }

    #[test]
    fn a_dma_transfer_leaves_its_value_on_the_shared_bus() {
        let mut channel = DMAChannel::new(0);
        let mut bus = MemoryBus::new_stub();
        channel.register(&bus.mem_map.memory);
        let mut irq = Interrupts::new();
        one_shot_word_transfer(&mut channel);

        let scratch = 0x0300_0000u32;
        bus.write_u32(scratch, 0x1234_5678);
        channel.internal_source_address = scratch;
        channel.internal_destination_address = 0x0300_1000;
        channel.transfer(&mut bus, &mut irq);

        assert_eq!(bus.read_u32(0x0400_0FF0), 0x1234_5678);
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
        // Snapshot these so every channel independently sees the same HBlank/VBlank event
        // this update() call; consuming the flag on the first matching channel would
        // otherwise hide it from any other channel also configured for the same trigger.
        let vblanking = self.vblanking;
        let hblanking = self.hblanking;

        for i in 0..4 {
            if self.dma_channels[i].control.get_dma_enable() == 1 {
                if self.dma_channels[i].previously_disabled {
                    self.dma_channels[i].reload_data();
                    self.dma_channels[i].previously_disabled = false;
                }

                match self.dma_channels[i].control.get_dma_start_timing() {
                    0 => {
                        if self.dma_channels[i].pending_immediate {
                            self.dma_channels[i].pending_immediate = false;
                            self.dma_channels[i].transfer(mem_map, irq_ctl);
                        } else {
                            self.dma_channels[i].pending_immediate = true;
                        }
                    },
                    1 => {
                        // start at vblank
                        if vblanking {
                            self.dma_channels[i].transfer(mem_map, irq_ctl);
                        }
                    },
                    2 => {
                        // start at hblank
                        if hblanking {
                            self.dma_channels[i].transfer(mem_map, irq_ctl);
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
                            if timer_overflows[timer] > 0 && fifo_len <= 16 {
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
            } else {
                self.dma_channels[i].pending_immediate = false;
                if !self.dma_channels[i].previously_disabled {
                    self.dma_channels[i].previously_disabled = true;
                }
            }
        }

        self.vblanking = false;
        self.hblanking = false;
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
