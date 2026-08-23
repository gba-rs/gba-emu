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

    // `period` is the driving timer's own overflow period in cycles, recomputed
    // every call from its live reload/prescaler so a mid-song rate change takes
    // effect immediately. Ticking this against `cycles` directly (rather than a
    // pre-aggregated overflow count from a much coarser CPU step) keeps FIFO pops
    // interleaved with sample emission at the same cycle-accurate granularity —
    // otherwise a large halted-CPU cycle jump pops several bytes at once before
    // any samples are emitted, and the intermediate bytes are never heard.
    pub fn step(&mut self, cycles: usize, period: usize, fifo: &mut VecDeque<u8>) {
        if period == 0 {
            return;
        }
        self.cycle_counter += cycles;
        while self.cycle_counter >= period {
            self.cycle_counter -= period;
            if let Some(byte) = fifo.pop_front() {
                self.current_sample = byte as i8;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pops_one_byte_per_elapsed_period_keeping_the_last() {
        let mut channel = DirectSoundChannel::new();
        let mut fifo: VecDeque<u8> = vec![10, 20, 30].into();
        channel.step(200, 100, &mut fifo);
        assert_eq!(channel.current_sample, 20);
        assert_eq!(fifo.len(), 1);
    }

    #[test]
    fn empty_fifo_repeats_last_sample() {
        let mut channel = DirectSoundChannel::new();
        channel.current_sample = 42;
        let mut fifo: VecDeque<u8> = VecDeque::new();
        channel.step(300, 100, &mut fifo);
        assert_eq!(channel.current_sample, 42);
    }

    #[test]
    fn zero_period_never_pops() {
        let mut channel = DirectSoundChannel::new();
        channel.current_sample = 5;
        let mut fifo: VecDeque<u8> = vec![99].into();
        channel.step(1000, 0, &mut fifo);
        assert_eq!(channel.current_sample, 5);
        assert_eq!(fifo.len(), 1);
    }

    #[test]
    fn accumulates_leftover_cycles_across_calls() {
        let mut channel = DirectSoundChannel::new();
        let mut fifo: VecDeque<u8> = vec![10, 20].into();
        channel.step(60, 100, &mut fifo);
        assert_eq!(fifo.len(), 2, "60 cycles shouldn't reach a 100-cycle period yet");
        channel.step(60, 100, &mut fifo);
        assert_eq!(channel.current_sample, 10);
        assert_eq!(fifo.len(), 1, "leftover cycles from the first call should carry over");
    }
}
