use std::rc::Rc;
use serde::{Serialize, Deserialize};
use crate::memory::{GbaMem, sound_registers::{SoundChannelControlNoiseLow, SoundChannelControlNoiseHigh}};

// GB divisor table (Pan Docs Channel 4), scaled x4 for the GBA's 4x clock.
const GB_DIVISOR: [i32; 8] = [8, 16, 32, 48, 64, 80, 96, 112];

#[derive(Serialize, Deserialize)]
pub struct NoiseChannel {
    low: SoundChannelControlNoiseLow,
    high: SoundChannelControlNoiseHigh,

    enabled: bool,
    frequency_timer: i32,
    lfsr: u16,
    length_counter: u16,
    volume: u8,
    envelope_timer: u8,
}

impl NoiseChannel {
    pub fn new() -> NoiseChannel {
        NoiseChannel {
            low: SoundChannelControlNoiseLow::new(),
            high: SoundChannelControlNoiseHigh::new(),
            enabled: false,
            frequency_timer: 1,
            lfsr: 0x7FFF,
            length_counter: 0,
            volume: 0,
            envelope_timer: 0,
        }
    }

    pub fn register(&mut self, mem: &Rc<GbaMem>) {
        self.low.register(mem);
        self.high.register(mem);
    }

    fn step_period(&self) -> i32 {
        let r = self.high.get_dividing_ratio_of_frequencies() as usize;
        let s = self.high.get_shift_clock_frequency() as u32;
        GB_DIVISOR[r] * 4 * (1i32 << s)
    }

    pub fn on_trigger(&mut self) {
        self.enabled = true;
        if self.length_counter == 0 {
            self.length_counter = 64 - self.low.get_sound_length() as u16;
        }
        self.frequency_timer = self.step_period();
        self.lfsr = 0x7FFF;
        self.volume = self.low.get_initial_volume_of_envelope();
        self.envelope_timer = self.low.get_envelope_step_time();

        if self.volume == 0 && self.low.get_envelope_direction() == 0 {
            self.enabled = false;
        }
    }

    pub fn step(&mut self, cycles: i32) {
        self.frequency_timer -= cycles;
        while self.frequency_timer <= 0 {
            self.frequency_timer += self.step_period().max(1);

            let xor_result = (self.lfsr & 1) ^ ((self.lfsr >> 1) & 1);
            self.lfsr >>= 1;
            self.lfsr |= xor_result << 14;
            if self.high.get_counter_step_width() != 0 {
                // Narrow/7-bit mode mirrors the feedback bit into bit 6 too.
                self.lfsr &= !(1 << 6);
                self.lfsr |= xor_result << 6;
            }
        }
    }

    pub fn clock_length(&mut self) {
        if self.high.get_length_flag() != 0 && self.length_counter > 0 {
            self.length_counter -= 1;
            if self.length_counter == 0 {
                self.enabled = false;
            }
        }
    }

    pub fn clock_envelope(&mut self) {
        let period = self.low.get_envelope_step_time();
        if period == 0 {
            return;
        }
        if self.envelope_timer > 0 {
            self.envelope_timer -= 1;
        }
        if self.envelope_timer == 0 {
            self.envelope_timer = period;
            if self.low.get_envelope_direction() != 0 && self.volume < 15 {
                self.volume += 1;
            } else if self.low.get_envelope_direction() == 0 && self.volume > 0 {
                self.volume -= 1;
            }
        }
    }

    pub fn amplitude(&self) -> u8 {
        if !self.enabled {
            return 0;
        }
        if self.lfsr & 1 == 0 {
            self.volume
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gba::GBA;

    #[test]
    fn trigger_resets_lfsr_and_loads_volume() {
        let mut gba = GBA::default();
        let ch = &mut gba.apu.noise;
        ch.low.set_initial_volume_of_envelope(12);
        ch.on_trigger();
        assert!(ch.enabled);
        assert_eq!(ch.lfsr, 0x7FFF);
        assert_eq!(ch.volume, 12);
    }

    #[test]
    fn zero_volume_decreasing_envelope_disables_dac() {
        let mut gba = GBA::default();
        let ch = &mut gba.apu.noise;
        ch.low.set_initial_volume_of_envelope(0);
        ch.low.set_envelope_direction(0);
        ch.on_trigger();
        assert!(!ch.enabled);
    }

    #[test]
    fn length_expiry_disables_channel() {
        let mut gba = GBA::default();
        let ch = &mut gba.apu.noise;
        ch.low.set_initial_volume_of_envelope(15);
        ch.high.set_length_flag(1);
        ch.low.set_sound_length(63);
        ch.on_trigger();
        assert_eq!(ch.length_counter, 1);
        ch.clock_length();
        assert!(!ch.enabled);
    }

    #[test]
    fn narrow_mode_mirrors_feedback_into_bit_6() {
        let mut gba = GBA::default();
        let ch = &mut gba.apu.noise;
        ch.low.set_initial_volume_of_envelope(15);
        ch.high.set_counter_step_width(1);
        ch.on_trigger();
        let period = ch.step_period();
        ch.step(period);
        let feedback = (ch.lfsr >> 14) & 1;
        assert_eq!((ch.lfsr >> 6) & 1, feedback);
    }
}
