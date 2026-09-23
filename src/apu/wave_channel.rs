use std::rc::Rc;
use serde::{Serialize, Deserialize};
use crate::memory::{GbaMem, memory_bus::MemoryBus, sound_registers::{SoundChannelControlWaveLow, SoundChannelControlWaveHigh, SoundChannelControlWaveX}};

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

    fn step_period(frequency: u16) -> i32 {
        8 * (2048 - frequency as i32)
    }

    fn dac_enabled(&self) -> bool {
        self.low.get_sound_channel_3_off() != 0
    }

    pub fn is_active(&self) -> bool {
        self.enabled && self.dac_enabled()
    }

    pub fn get_dimension(&self) -> u8 {
        self.low.get_wave_ram_dimension()
    }

    pub fn get_bank_number(&self) -> u8 {
        self.low.get_wave_ram_bank_number()
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
        let sample_count = if self.low.get_wave_ram_dimension() != 0 { 64 } else { 32 };
        self.frequency_timer -= cycles;
        while self.frequency_timer <= 0 {
            self.frequency_timer += Self::step_period(self.x.get_sample_rate()).max(1);
            self.sample_position = (self.sample_position + 1) % sample_count;
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
        let playback_bank = if self.sample_position < 32 {
            self.low.get_wave_ram_bank_number()
        } else {
            self.low.get_wave_ram_bank_number() ^ 1
        };
        let byte_offset = ((self.sample_position % 32) / 2) as u32;
        let byte = mem_bus.mem_map.read_wave_ram_byte(playback_bank, byte_offset);
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
        gba.apu.wave.low.set_wave_ram_bank_number(1);
        gba.memory_bus.write_u8(0x0400_0090, 0xA5);
        gba.apu.wave.low.set_wave_ram_bank_number(0);
        let ch = &mut gba.apu.wave;
        ch.low.set_sound_channel_3_off(1);
        ch.high.set_sound_volume(1);
        ch.on_trigger();
        assert_eq!(ch.amplitude(&gba.memory_bus), 0xA);
        ch.sample_position = 1;
        assert_eq!(ch.amplitude(&gba.memory_bus), 0x5);
    }

    #[test]
    fn bank_number_selects_which_bank_plays_back() {
        let mut gba = GBA::default();
        gba.apu.wave.low.set_wave_ram_bank_number(0);
        gba.memory_bus.write_u8(0x0400_0090, 0x30);
        gba.apu.wave.low.set_wave_ram_bank_number(1);
        gba.memory_bus.write_u8(0x0400_0090, 0x70);

        let ch = &mut gba.apu.wave;
        ch.low.set_sound_channel_3_off(1);
        ch.high.set_sound_volume(1);

        ch.low.set_wave_ram_bank_number(0);
        ch.on_trigger();
        assert_eq!(ch.amplitude(&gba.memory_bus), 0x7);

        ch.low.set_wave_ram_bank_number(1);
        ch.on_trigger();
        assert_eq!(ch.amplitude(&gba.memory_bus), 0x3);
    }

    #[test]
    fn dimension_one_plays_64_samples_crossing_both_banks() {
        let mut gba = GBA::default();
        gba.apu.wave.low.set_wave_ram_bank_number(1);
        gba.memory_bus.write_u8(0x0400_0090, 0x12);
        gba.apu.wave.low.set_wave_ram_bank_number(0);
        gba.memory_bus.write_u8(0x0400_0090, 0x34);

        let ch = &mut gba.apu.wave;
        ch.low.set_sound_channel_3_off(1);
        ch.high.set_sound_volume(1);
        ch.low.set_wave_ram_dimension(1);
        ch.low.set_wave_ram_bank_number(0);
        ch.on_trigger();

        assert_eq!(ch.amplitude(&gba.memory_bus), 0x1);
        ch.sample_position = 1;
        assert_eq!(ch.amplitude(&gba.memory_bus), 0x2);

        ch.sample_position = 32;
        assert_eq!(ch.amplitude(&gba.memory_bus), 0x3);
        ch.sample_position = 33;
        assert_eq!(ch.amplitude(&gba.memory_bus), 0x4);
    }

    #[test]
    fn dimension_one_step_wraps_sample_position_at_64_not_32() {
        let mut gba = GBA::default();
        let ch = &mut gba.apu.wave;
        ch.low.set_sound_channel_3_off(1);
        ch.low.set_wave_ram_dimension(1);
        ch.x.set_sample_rate(2047);
        ch.on_trigger();

        for _ in 0..32 {
            ch.step(8);
        }
        assert_eq!(ch.sample_position, 32);

        for _ in 0..32 {
            ch.step(8);
        }
        assert_eq!(ch.sample_position, 0);
    }

    #[test]
    fn dimension_zero_step_wraps_sample_position_at_32() {
        let mut gba = GBA::default();
        let ch = &mut gba.apu.wave;
        ch.low.set_sound_channel_3_off(1);
        ch.low.set_wave_ram_dimension(0);
        ch.x.set_sample_rate(2047);
        ch.on_trigger();

        for _ in 0..32 {
            ch.step(8);
        }
        assert_eq!(ch.sample_position, 0);
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
