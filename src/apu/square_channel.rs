use std::rc::Rc;
use serde::{Serialize, Deserialize};
use crate::memory::{GbaMem, sound_registers::{SoundChannelControlSweep, SoundChannelControlDLE, SoundChannelControlFC}};

const DUTY_TABLE: [[u8; 8]; 4] = [
    [0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 1, 1, 1],
    [0, 1, 1, 1, 1, 1, 1, 0],
];

#[derive(Serialize, Deserialize)]
pub struct SquareChannel {
    has_sweep: bool,
    sweep: SoundChannelControlSweep,
    dle: SoundChannelControlDLE,
    fc: SoundChannelControlFC,

    enabled: bool,
    frequency_timer: i32,
    duty_position: u8,
    length_counter: u16,
    volume: u8,
    envelope_timer: u8,
    envelope_running: bool,

    shadow_frequency: u16,
    sweep_timer: u8,
    sweep_enabled: bool,
}

impl SquareChannel {
    pub fn new(has_sweep: bool, dle_index: usize, fc_index: usize) -> SquareChannel {
        SquareChannel {
            has_sweep,
            sweep: SoundChannelControlSweep::new(),
            dle: SoundChannelControlDLE::new(dle_index),
            fc: SoundChannelControlFC::new(fc_index),
            enabled: false,
            frequency_timer: 1,
            duty_position: 0,
            length_counter: 0,
            volume: 0,
            envelope_timer: 0,
            envelope_running: false,
            shadow_frequency: 0,
            sweep_timer: 0,
            sweep_enabled: false,
        }
    }

    pub fn register(&mut self, mem: &Rc<GbaMem>) {
        if self.has_sweep {
            self.sweep.register(mem);
        }
        self.dle.register(mem);
        self.fc.register(mem);
    }

    fn step_period(frequency: u16) -> i32 {
        16 * (2048 - frequency as i32)
    }

    fn trigger(&mut self) {
        self.enabled = true;
        if self.length_counter == 0 {
            self.length_counter = 64 - self.dle.get_sound_length() as u16;
        }
        self.frequency_timer = Self::step_period(self.fc.get_frequency());
        self.volume = self.dle.get_initial_volume_of_envelope();
        self.envelope_timer = self.dle.get_envelope_step_time();
        self.envelope_running = self.envelope_timer != 0;

        if self.has_sweep {
            self.shadow_frequency = self.fc.get_frequency();
            self.sweep_timer = self.sweep.get_sweep_time();
            if self.sweep_timer == 0 {
                self.sweep_timer = 8;
            }
            self.sweep_enabled = self.sweep_timer != 8 || self.sweep.get_number_of_sweep_shift() != 0;
            if self.sweep.get_number_of_sweep_shift() != 0 {
                self.calculate_sweep_frequency();
            }
        }

        if self.volume == 0 && self.dle.get_envelope_direction() == 0 {
            self.enabled = false;
        }
    }

    fn calculate_sweep_frequency(&mut self) -> u16 {
        let shift = self.sweep.get_number_of_sweep_shift();
        let delta = self.shadow_frequency >> shift;
        let new_frequency = if self.sweep.get_sweep_frequency_direction() != 0 {
            self.shadow_frequency.saturating_sub(delta)
        } else {
            self.shadow_frequency + delta
        };
        if new_frequency > 2047 {
            self.enabled = false;
        }
        new_frequency
    }

    pub fn on_trigger(&mut self) {
        self.trigger();
    }

    pub fn step(&mut self, cycles: i32) {
        self.frequency_timer -= cycles;
        while self.frequency_timer <= 0 {
            self.frequency_timer += Self::step_period(self.fc.get_frequency()).max(1);
            self.duty_position = (self.duty_position + 1) % 8;
        }
    }

    pub fn clock_length(&mut self) {
        if self.fc.get_length_flag() != 0 && self.length_counter > 0 {
            self.length_counter -= 1;
            if self.length_counter == 0 {
                self.enabled = false;
            }
        }
    }

    pub fn clock_sweep(&mut self) {
        if !self.has_sweep || !self.sweep_enabled {
            return;
        }
        if self.sweep_timer > 0 {
            self.sweep_timer -= 1;
        }
        if self.sweep_timer == 0 {
            self.sweep_timer = self.sweep.get_sweep_time();
            if self.sweep_timer == 0 {
                self.sweep_timer = 8;
            }
            if self.sweep.get_sweep_time() != 0 {
                let new_frequency = self.calculate_sweep_frequency();
                if new_frequency <= 2047 && self.sweep.get_number_of_sweep_shift() != 0 {
                    self.fc.set_frequency(new_frequency);
                    self.shadow_frequency = new_frequency;
                    self.calculate_sweep_frequency();
                }
            }
        }
    }

    pub fn clock_envelope(&mut self) {
        let period = self.dle.get_envelope_step_time();
        if period == 0 {
            return;
        }
        if self.envelope_timer > 0 {
            self.envelope_timer -= 1;
        }
        if self.envelope_timer == 0 {
            self.envelope_timer = period;
            if self.dle.get_envelope_direction() != 0 && self.volume < 15 {
                self.volume += 1;
            } else if self.dle.get_envelope_direction() == 0 && self.volume > 0 {
                self.volume -= 1;
            }
        }
    }

    pub fn amplitude(&self) -> u8 {
        if !self.enabled {
            return 0;
        }
        let duty = self.dle.get_wave_pattern_duty() as usize;
        if DUTY_TABLE[duty][self.duty_position as usize] != 0 {
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

    fn channel(gba: &mut GBA) -> &mut SquareChannel {
        &mut gba.apu.square1
    }

    #[test]
    fn high_frequency_output_is_independent_of_cpu_batch_size() {
        fn make_gba() -> GBA {
            let mut gba = GBA::default();
            {
                let ch = channel(&mut gba);
                ch.fc.set_frequency(2032);
                ch.dle.set_initial_volume_of_envelope(15);
                ch.dle.set_wave_pattern_duty(3);
                ch.on_trigger();
            }
            gba.apu.sound_control_low.set_sound_enable_flags_left(1);
            gba.apu.sound_control_low.set_sound_master_volume_left(7);
            gba.apu.sound_control_x.set_psg_fifo_master_enable(1);
            gba
        }

        let mut one_big_step = make_gba();
        one_big_step.apu.step(1232, [0, 0, 0, 0], &mut one_big_step.memory_bus);

        let mut many_small_steps = make_gba();
        let mut remaining = 1232usize;
        while remaining > 0 {
            let chunk = remaining.min(64);
            many_small_steps.apu.step(chunk, [0, 0, 0, 0], &mut many_small_steps.memory_bus);
            remaining -= chunk;
        }

        assert_eq!(one_big_step.apu.sample_buffer, many_small_steps.apu.sample_buffer);
        assert!(one_big_step.apu.sample_buffer.iter().any(|&s| s != 0), "expected the triggered channel to actually produce sound");
    }

    #[test]
    fn trigger_enables_and_loads_length() {
        let mut gba = GBA::default();
        let ch = channel(&mut gba);
        ch.dle.set_sound_length(20);
        ch.dle.set_initial_volume_of_envelope(15);
        ch.on_trigger();
        assert!(ch.enabled);
        assert_eq!(ch.length_counter, 64 - 20);
        assert_eq!(ch.volume, 15);
    }

    #[test]
    fn zero_volume_decreasing_envelope_disables_dac() {
        let mut gba = GBA::default();
        let ch = channel(&mut gba);
        ch.dle.set_initial_volume_of_envelope(0);
        ch.dle.set_envelope_direction(0);
        ch.on_trigger();
        assert!(!ch.enabled);
    }

    #[test]
    fn length_expiry_disables_channel() {
        let mut gba = GBA::default();
        let ch = channel(&mut gba);
        ch.dle.set_initial_volume_of_envelope(15);
        ch.fc.set_length_flag(1);
        ch.dle.set_sound_length(63);
        ch.on_trigger();
        assert_eq!(ch.length_counter, 1);
        ch.clock_length();
        assert!(!ch.enabled);
    }

    #[test]
    fn sweep_overflow_disables_channel() {
        let mut gba = GBA::default();
        let ch = channel(&mut gba);
        ch.dle.set_initial_volume_of_envelope(15);
        ch.fc.set_frequency(2000);
        ch.sweep.set_number_of_sweep_shift(1);
        ch.sweep.set_sweep_frequency_direction(0);
        ch.sweep.set_sweep_time(1);
        ch.on_trigger();
        ch.clock_sweep();
        assert!(!ch.enabled);
    }

    #[test]
    fn amplitude_follows_duty_table_and_volume() {
        let mut gba = GBA::default();
        let ch = channel(&mut gba);
        ch.dle.set_wave_pattern_duty(0);
        ch.dle.set_initial_volume_of_envelope(9);
        ch.on_trigger();
        assert_eq!(ch.amplitude(), 0);
        for _ in 0..7 {
            ch.step(SquareChannel::step_period(ch.fc.get_frequency()));
        }
        assert_eq!(ch.amplitude(), 9);
    }
}
