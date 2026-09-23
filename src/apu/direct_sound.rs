use std::collections::VecDeque;
use serde::{Serialize, Deserialize};

#[derive(Default, Clone, Copy, Serialize, Deserialize)]
pub struct DirectSoundChannel {
    pub current_sample: i8,
    cycle_counter: usize,
}

impl DirectSoundChannel {
    pub fn new() -> DirectSoundChannel {
        DirectSoundChannel { current_sample: 0, cycle_counter: 0 }
    }

    pub fn clock(&mut self, overflows: usize, fifo: &mut VecDeque<u8>) -> bool {
        if overflows == 0 { return false; }
        const PACKED_SHIFTER: usize = 1 << 31;
        let packed = if self.cycle_counter & PACKED_SHIFTER != 0 { self.cycle_counter } else { 0 };
        let mut remaining = packed & 3;
        let mut word = (packed >> 2) & 0xff_ffff;
        let mut request_dma = false;
        for _ in 0..overflows {
            request_dma |= fifo.len() < 16;
            if remaining == 0 {
                word = 0;
                for byte in 0..4 {
                    if let Some(value) = fifo.pop_front() {
                        word |= (value as usize) << (byte * 8);
                        remaining += 1;
                    }
                }
            }
            if remaining != 0 {
                self.current_sample = word as u8 as i8;
                word >>= 8;
                remaining -= 1;
            }
        }
        self.cycle_counter = PACKED_SHIFTER | (word << 2) | remaining;
        request_dma
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pops_one_byte_per_elapsed_period_keeping_the_last() {
        let mut channel = DirectSoundChannel::new();
        let mut fifo: VecDeque<u8> = vec![10, 20, 30].into();
        channel.clock(2, &mut fifo);
        assert_eq!(channel.current_sample, 20);
        assert!(fifo.is_empty());
        channel.clock(1, &mut fifo);
        assert_eq!(channel.current_sample, 30);
    }

    #[test]
    fn empty_fifo_repeats_last_sample() {
        let mut channel = DirectSoundChannel::new();
        channel.current_sample = 42;
        let mut fifo: VecDeque<u8> = VecDeque::new();
        channel.clock(3, &mut fifo);
        assert_eq!(channel.current_sample, 42);
    }

    #[test]
    fn zero_overflows_never_pops() {
        let mut channel = DirectSoundChannel::new();
        channel.current_sample = 5;
        let mut fifo: VecDeque<u8> = vec![99].into();
        channel.clock(0, &mut fifo);
        assert_eq!(channel.current_sample, 5);
        assert_eq!(fifo.len(), 1);
    }

    #[test]
    fn requests_dma_from_word_fifo_before_clocking_the_output_shifter() {
        let mut channel = DirectSoundChannel::new();
        let mut fifo: VecDeque<u8> = (0..32).collect();
        for sample in 0..17 {
            assert!(!channel.clock(1, &mut fifo));
            assert_eq!(channel.current_sample, sample);
        }
        assert_eq!(fifo.len(), 12);
        assert!(channel.clock(1, &mut fifo));
        assert_eq!(channel.current_sample, 17);
    }

    #[test]
    fn queued_word_survives_fifo_reset_and_save_state_roundtrip() {
        let mut channel = DirectSoundChannel::new();
        let mut fifo: VecDeque<u8> = [10, 20, 30, 40, 50].into();
        channel.clock(1, &mut fifo);
        fifo.clear();
        let bytes = bincode::serialize(&channel).unwrap();
        let mut restored: DirectSoundChannel = bincode::deserialize(&bytes).unwrap();
        for expected in [20, 30, 40] {
            restored.clock(1, &mut fifo);
            assert_eq!(restored.current_sample, expected);
        }
    }
}
