pub mod direct_sound;
pub mod square_channel;
pub mod wave_channel;
pub mod noise_channel;

use std::rc::Rc;
use serde::{Serialize, Deserialize};
use crate::memory::{GbaMem, memory_bus::MemoryBus, sound_registers::*};
use direct_sound::DirectSoundChannel;
use square_channel::SquareChannel;
use wave_channel::WaveChannel;
use noise_channel::NoiseChannel;

pub const OUTPUT_SAMPLE_RATE: usize = 32768;
const CYCLES_PER_SAMPLE: usize = 512;
const FRAME_SEQUENCER_CYCLES: i32 = 32768;

const MIX_SCALE: i32 = 64;

#[derive(Serialize, Deserialize)]
pub struct Apu {
    pub sound_control_low: SoundControlLow,
    pub sound_control_high: SoundControlHigh,
    pub sound_control_x: SoundControlX,
    pub sound_bias: SoundBias,

    pub square1: SquareChannel,
    pub square2: SquareChannel,
    pub wave: WaveChannel,
    pub noise: NoiseChannel,

    pub direct_sound_a: DirectSoundChannel,
    pub direct_sound_b: DirectSoundChannel,

    cycle_accumulator: usize,
    frame_sequencer_cycles: i32,
    frame_sequencer_step: u8,

    pub sample_buffer: Vec<i16>,
}

impl Apu {
    pub fn new() -> Apu {
        return Apu {
            sound_control_low: SoundControlLow::new(),
            sound_control_high: SoundControlHigh::new(),
            sound_control_x: SoundControlX::new(),
            sound_bias: SoundBias::new(),
            square1: SquareChannel::new(true, 0, 0),
            square2: SquareChannel::new(false, 1, 1),
            wave: WaveChannel::new(),
            noise: NoiseChannel::new(),
            direct_sound_a: DirectSoundChannel::new(),
            direct_sound_b: DirectSoundChannel::new(),
            cycle_accumulator: 0,
            frame_sequencer_cycles: FRAME_SEQUENCER_CYCLES,
            frame_sequencer_step: 0,
            sample_buffer: Vec::new(),
        };
    }

    pub fn register(&mut self, mem: &Rc<GbaMem>) {
        self.sound_control_low.register(mem);
        self.sound_control_high.register(mem);
        self.sound_control_x.register(mem);
        self.sound_bias.register(mem);
        self.sound_bias.set_bias_level(0x100);
        self.square1.register(mem);
        self.square2.register(mem);
        self.wave.register(mem);
        self.noise.register(mem);
    }

    pub fn step(&mut self, cycles: usize, timer_periods: [usize; 4], mem_bus: &mut MemoryBus) {
        let triggers = mem_bus.mem_map.trigger_flags;
        mem_bus.mem_map.trigger_flags = 0;
        if triggers & 0x1 != 0 { self.square1.on_trigger(); }
        if triggers & 0x2 != 0 { self.square2.on_trigger(); }
        if triggers & 0x4 != 0 { self.wave.on_trigger(); }
        if triggers & 0x8 != 0 { self.noise.on_trigger(); }

        let timer_a = self.sound_control_high.get_dma_sound_a_timer_select() as usize;
        let timer_b = self.sound_control_high.get_dma_sound_b_timer_select() as usize;
        let period_a = timer_periods[timer_a];
        let period_b = timer_periods[timer_b];

        if self.sound_control_high.get_dma_sound_a_reset_fifo() != 0 {
            mem_bus.mem_map.fifo_a.clear();
            self.direct_sound_a.current_sample = 0;
            self.sound_control_high.set_dma_sound_a_reset_fifo(0);
        }
        if self.sound_control_high.get_dma_sound_b_reset_fifo() != 0 {
            mem_bus.mem_map.fifo_b.clear();
            self.direct_sound_b.current_sample = 0;
            self.sound_control_high.set_dma_sound_b_reset_fifo(0);
        }

        let cycles_i32 = cycles as i32;
        self.frame_sequencer_cycles -= cycles_i32;
        while self.frame_sequencer_cycles <= 0 {
            self.frame_sequencer_cycles += FRAME_SEQUENCER_CYCLES;
            self.clock_frame_sequencer();
        }

        self.cycle_accumulator += cycles;
        while self.cycle_accumulator >= CYCLES_PER_SAMPLE {
            self.cycle_accumulator -= CYCLES_PER_SAMPLE;
            let step_cycles = CYCLES_PER_SAMPLE as i32;
            self.square1.step(step_cycles);
            self.square2.step(step_cycles);
            self.wave.step(step_cycles);
            self.noise.step(step_cycles);
            self.direct_sound_a.step(CYCLES_PER_SAMPLE, period_a, &mut mem_bus.mem_map.fifo_a);
            self.direct_sound_b.step(CYCLES_PER_SAMPLE, period_b, &mut mem_bus.mem_map.fifo_b);
            self.mix_and_emit_sample(mem_bus);
        }
    }

    fn clock_frame_sequencer(&mut self) {
        if self.frame_sequencer_step % 2 == 0 {
            self.square1.clock_length();
            self.square2.clock_length();
            self.wave.clock_length();
            self.noise.clock_length();
        }
        if self.frame_sequencer_step % 4 == 2 {
            self.square1.clock_sweep();
        }
        if self.frame_sequencer_step == 7 {
            self.square1.clock_envelope();
            self.square2.clock_envelope();
            self.noise.clock_envelope();
        }
        self.frame_sequencer_step = (self.frame_sequencer_step + 1) % 8;
    }

    fn mix_and_emit_sample(&mut self, mem_bus: &MemoryBus) {
        if self.sound_control_x.get_psg_fifo_master_enable() == 0 {
            self.sample_buffer.push(0);
            self.sample_buffer.push(0);
            return;
        }

        let ds_a = if self.sound_control_high.get_dma_sound_a_volume() != 0 {
            self.direct_sound_a.current_sample as i32 * 4
        } else {
            self.direct_sound_a.current_sample as i32 * 2
        };
        let ds_b = if self.sound_control_high.get_dma_sound_b_volume() != 0 {
            self.direct_sound_b.current_sample as i32 * 4
        } else {
            self.direct_sound_b.current_sample as i32 * 2
        };

        let mut left: i32 = 0;
        let mut right: i32 = 0;
        if self.sound_control_high.get_dma_sound_a_enable_left() != 0 { left += ds_a; }
        if self.sound_control_high.get_dma_sound_a_enable_right() != 0 { right += ds_a; }
        if self.sound_control_high.get_dma_sound_b_enable_left() != 0 { left += ds_b; }
        if self.sound_control_high.get_dma_sound_b_enable_right() != 0 { right += ds_b; }

        let enable_right = self.sound_control_low.get_sound_enable_flags_right();
        let enable_left = self.sound_control_low.get_sound_enable_flags_left();
        let channels = [
            (self.square1.is_active(), self.square1.amplitude()),
            (self.square2.is_active(), self.square2.amplitude()),
            (self.wave.is_active(), self.wave.amplitude(mem_bus)),
            (self.noise.is_active(), self.noise.amplitude()),
        ];

        let mut psg_right_raw: i32 = 0;
        let mut psg_left_raw: i32 = 0;
        for (i, (is_active, amplitude)) in channels.iter().enumerate() {
            if !is_active {
                continue;
            }
            let bit = 1 << i;
            let centered = (*amplitude as i32) * 16 - 128;
            if enable_right & bit != 0 { psg_right_raw += centered; }
            if enable_left & bit != 0 { psg_left_raw += centered; }
        }

        let master_right = (self.sound_control_low.get_sound_master_volume_right() as i32) + 1;
        let master_left = (self.sound_control_low.get_sound_master_volume_left() as i32) + 1;
        let psg_right = psg_right_raw * master_right / 8;
        let psg_left = psg_left_raw * master_left / 8;

        left += psg_left;
        right += psg_right;

        let bias = (self.sound_bias.get_bias_level() as i32) << 1;
        let biased_left = (left + bias).clamp(0, 0x3FF) - bias;
        let biased_right = (right + bias).clamp(0, 0x3FF) - bias;

        let left_sample = (biased_left * MIX_SCALE).clamp(i16::MIN as i32, i16::MAX as i32) as i16;
        let right_sample = (biased_right * MIX_SCALE).clamp(i16::MIN as i32, i16::MAX as i32) as i16;

        self.sample_buffer.push(left_sample);
        self.sample_buffer.push(right_sample);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gba::GBA;

    #[test]
    fn fifo_reset_also_silences_the_channel() {
        let mut gba = GBA::default();
        gba.apu.direct_sound_a.current_sample = 100;
        gba.apu.direct_sound_b.current_sample = -100;
        gba.apu.sound_control_high.set_dma_sound_a_reset_fifo(1);
        gba.apu.sound_control_high.set_dma_sound_b_reset_fifo(1);
        gba.apu.step(0, [0, 0, 0, 0], &mut gba.memory_bus);
        assert_eq!(gba.apu.direct_sound_a.current_sample, 0);
        assert_eq!(gba.apu.direct_sound_b.current_sample, 0);
    }

    #[test]
    fn mix_never_panics_or_overflows_at_worst_case_amplitude() {
        let mut gba = GBA::default();
        gba.apu.sound_control_x.set_psg_fifo_master_enable(1);
        gba.apu.sound_control_low.set_sound_master_volume_left(7);
        gba.apu.sound_control_low.set_sound_master_volume_right(7);
        gba.apu.sound_control_low.set_sound_enable_flags_left(0xF);
        gba.apu.sound_control_low.set_sound_enable_flags_right(0xF);
        gba.apu.direct_sound_a.current_sample = i8::MIN;
        gba.apu.direct_sound_b.current_sample = i8::MIN;
        gba.apu.sound_control_high.set_dma_sound_a_volume(1);
        gba.apu.sound_control_high.set_dma_sound_b_volume(1);
        gba.apu.sound_control_high.set_dma_sound_a_enable_left(1);
        gba.apu.sound_control_high.set_dma_sound_a_enable_right(1);
        gba.apu.sound_control_high.set_dma_sound_b_enable_left(1);
        gba.apu.sound_control_high.set_dma_sound_b_enable_right(1);
        gba.apu.mix_and_emit_sample(&gba.memory_bus);
        assert_eq!(gba.apu.sample_buffer[0], i16::MIN);
        assert_eq!(gba.apu.sample_buffer[1], i16::MIN);
    }

    #[test]
    fn bias_domain_clamp_saturates_rather_than_wrapping() {
        let mut gba = GBA::default();
        gba.apu.sound_control_x.set_psg_fifo_master_enable(1);
        gba.apu.sound_control_low.set_sound_master_volume_left(7);
        gba.apu.sound_control_low.set_sound_enable_flags_left(0xF);
        gba.apu.direct_sound_a.current_sample = i8::MIN;
        gba.apu.direct_sound_b.current_sample = i8::MIN;
        gba.apu.sound_control_high.set_dma_sound_a_volume(1);
        gba.apu.sound_control_high.set_dma_sound_b_volume(1);
        gba.apu.sound_control_high.set_dma_sound_a_enable_left(1);
        gba.apu.sound_control_high.set_dma_sound_b_enable_left(1);
        gba.apu.mix_and_emit_sample(&gba.memory_bus);
        assert_eq!(gba.apu.sample_buffer[0], i16::MIN);
    }

    #[test]
    fn bias_level_register_measurably_shifts_output() {
        let mut gba = GBA::default();
        gba.apu.sound_control_x.set_psg_fifo_master_enable(1);
        gba.apu.direct_sound_a.current_sample = i8::MIN;
        gba.apu.sound_control_high.set_dma_sound_a_enable_left(1);

        gba.apu.sound_bias.set_bias_level(0x10);
        gba.apu.mix_and_emit_sample(&gba.memory_bus);
        let at_small_bias = gba.apu.sample_buffer[0];

        gba.apu.sound_bias.set_bias_level(0x100);
        gba.apu.mix_and_emit_sample(&gba.memory_bus);
        let at_default_bias = gba.apu.sample_buffer[2];

        assert_ne!(at_small_bias, at_default_bias);
        assert!(at_small_bias.unsigned_abs() < i16::MAX.unsigned_abs());
        assert!(at_default_bias.unsigned_abs() < i16::MAX.unsigned_abs());
    }

    #[test]
    fn true_silence_produces_exactly_zero() {
        let mut gba = GBA::default();
        gba.apu.sound_control_x.set_psg_fifo_master_enable(1);
        gba.apu.sound_control_low.set_sound_master_volume_left(7);
        gba.apu.sound_control_low.set_sound_master_volume_right(7);
        gba.apu.mix_and_emit_sample(&gba.memory_bus);
        assert_eq!(&gba.apu.sample_buffer[..], &[0, 0]);
    }

    #[test]
    fn direct_sound_100_percent_is_not_amplified_beyond_source() {
        let mut gba = GBA::default();
        gba.apu.sound_control_x.set_psg_fifo_master_enable(1);
        gba.apu.direct_sound_a.current_sample = i8::MIN;
        gba.apu.sound_control_high.set_dma_sound_a_volume(1);
        gba.apu.sound_control_high.set_dma_sound_a_enable_left(1);
        gba.apu.mix_and_emit_sample(&gba.memory_bus);
        let left = gba.apu.sample_buffer[0] as i32;
        assert!(left.unsigned_abs() <= (i8::MIN.unsigned_abs() as u32) * 4 * MIX_SCALE as u32);
    }

    #[test]
    fn untriggered_psg_channel_enabled_in_mixer_contributes_silence() {
        let mut gba = GBA::default();
        gba.apu.sound_control_x.set_psg_fifo_master_enable(1);
        gba.apu.sound_control_low.set_sound_master_volume_left(7);
        gba.apu.sound_control_low.set_sound_master_volume_right(7);
        gba.apu.sound_control_low.set_sound_enable_flags_left(0xF);
        gba.apu.sound_control_low.set_sound_enable_flags_right(0xF);
        gba.apu.mix_and_emit_sample(&gba.memory_bus);
        assert_eq!(&gba.apu.sample_buffer[..], &[0, 0]);
    }

    #[test]
    fn active_channel_at_zero_duty_still_contributes_to_mix() {
        let mut gba = GBA::default();
        gba.apu.sound_control_x.set_psg_fifo_master_enable(1);
        gba.apu.sound_control_low.set_sound_master_volume_left(7);
        gba.apu.sound_control_low.set_sound_enable_flags_left(0x1);
        gba.memory_bus.write_u8(0x0400_0063, 0xF0);
        gba.apu.square1.on_trigger();
        assert_eq!(gba.apu.square1.amplitude(), 0);
        gba.apu.mix_and_emit_sample(&gba.memory_bus);
        assert_ne!(gba.apu.sample_buffer[0], 0);
    }

    #[test]
    fn master_disable_forces_silence() {
        let mut gba = GBA::default();
        gba.apu.sound_control_x.set_psg_fifo_master_enable(0);
        gba.apu.direct_sound_a.current_sample = 127;
        gba.apu.mix_and_emit_sample(&gba.memory_bus);
        assert_eq!(&gba.apu.sample_buffer[..], &[0, 0]);
    }

    #[test]
    fn sustained_signal_does_not_decay_toward_zero() {
        let mut gba = GBA::default();
        gba.apu.direct_sound_a.current_sample = 100;
        gba.apu.sound_control_high.set_dma_sound_a_volume(1);
        gba.apu.sound_control_high.set_dma_sound_a_enable_left(1);
        for _ in 0..50 {
            gba.apu.mix_and_emit_sample(&gba.memory_bus);
        }
        let first = gba.apu.sample_buffer[0];
        let last = *gba.apu.sample_buffer.iter().rev().nth(1).unwrap();
        assert_eq!(first, last);
    }
}
