use std::rc::Rc;
use serde::{Serialize, Deserialize};
use crate::memory::{GbaMem, memory_bus::MemoryBus, sound_registers::{SoundChannelControlWaveLow, SoundChannelControlWaveHigh, SoundChannelControlWaveX}};

// Reads Wave RAM directly from 0x04000090-0x0400009F rather than the real
// dual-bank-switched register; diverges from hardware only for the rare
// bank-swap double-buffering trick some games use for streamed playback.
#[derive(Serialize, Deserialize)]
pub struct WaveChannel {
    low: SoundChannelControlWaveLow,
    high: SoundChannelControlWaveHigh,
    x: SoundChannelControlWaveX,

    enabled: bool,
    frequency_timer: i32,
    sample_position: u8,
    length_counter: u16,
}

impl WaveChannel {
    pub fn new() -> WaveChannel {
        WaveChannel {
            low: SoundChannelControlWaveLow::new(),
            high: SoundChannelControlWaveHigh::new(),
            x: SoundChannelControlWaveX::new(),
            enabled: false,
            frequency_timer: 1,
            sample_position: 0,
            length_counter: 0,
        }
    }

    pub fn register(&mut self, mem: &Rc<GbaMem>) {
        self.low.register(mem);
        self.high.register(mem);
        self.x.register(mem);
    }

    // GBATEK: 2097152/(2048-rate) Hz per sample step (already per-sample, not per-cycle).
    fn step_period(frequency: u16) -> i32 {
        8 * (2048 - frequency as i32)
    }

    // NR30 bit 7: 1 = DAC on. sound_registers.rs's field name reflects the
    // register's traditional name, not this bit's polarity.
    fn dac_enabled(&self) -> bool {
        self.low.get_sound_channel_3_off() != 0
    }

    pub fn on_trigger(&mut self) {
        self.enabled = self.dac_enabled();
        self.frequency_timer = Self::step_period(self.x.get_sample_rate());
        self.sample_position = 0;
        if self.length_counter == 0 {
            self.length_counter = 256 - self.high.get_sound_length() as u16;
        }
    }

    pub fn step(&mut self, cycles: i32) {
        if !self.enabled {
            return;
        }
        self.frequency_timer -= cycles;
        while self.frequency_timer <= 0 {
            self.frequency_timer += Self::step_period(self.x.get_sample_rate()).max(1);
            self.sample_position = (self.sample_position + 1) % 32;
        }
        if !self.dac_enabled() {
            self.enabled = false;
        }
    }

    pub fn clock_length(&mut self) {
        if self.x.get_length_flag() != 0 && self.length_counter > 0 {
            self.length_counter -= 1;
            if self.length_counter == 0 {
                self.enabled = false;
            }
        }
    }

    pub fn amplitude(&self, mem_bus: &MemoryBus) -> u8 {
        if !self.enabled || !self.dac_enabled() {
            return 0;
        }
        let byte_offset = (self.sample_position / 2) as u32;
        let byte = mem_bus.mem_map.memory[(0x0400_0090 + byte_offset) as usize].get();
        let raw = if self.sample_position % 2 == 0 { byte >> 4 } else { byte & 0xF };

        if self.high.get_force_volume() != 0 {
            (raw as u32 * 3 / 4) as u8
        } else {
            match self.high.get_sound_volume() {
                0 => 0,
                1 => raw,
                2 => raw >> 1,
                _ => raw >> 2,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gba::GBA;

    #[test]
    fn dac_off_silences_channel_even_when_triggered() {
        let mut gba = GBA::default();
        let ch = &mut gba.apu.wave;
        ch.low.set_sound_channel_3_off(0);
        ch.on_trigger();
        assert!(!ch.enabled);
        assert_eq!(ch.amplitude(&gba.memory_bus), 0);
    }

    #[test]
    fn amplitude_reads_nibble_from_wave_ram() {
        let mut gba = GBA::default();
        gba.memory_bus.mem_map.memory[0x0400_0090].set(0xA5);
        let ch = &mut gba.apu.wave;
        ch.low.set_sound_channel_3_off(1);
        ch.high.set_sound_volume(1);
        ch.on_trigger();
        assert_eq!(ch.amplitude(&gba.memory_bus), 0xA);
        ch.sample_position = 1;
        assert_eq!(ch.amplitude(&gba.memory_bus), 0x5);
    }

    #[test]
    fn length_expiry_disables_channel() {
        let mut gba = GBA::default();
        let ch = &mut gba.apu.wave;
        ch.low.set_sound_channel_3_off(1);
        ch.x.set_length_flag(1);
        ch.high.set_sound_length(255);
        ch.on_trigger();
        assert_eq!(ch.length_counter, 1);
        ch.clock_length();
        assert!(!ch.enabled);
    }
}
