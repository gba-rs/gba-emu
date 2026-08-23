use crate::memory::timer_registers::*;
use crate::interrupts::interrupts::Interrupts;
use std::cell::RefCell;
use std::rc::Rc;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Timer {
    pub timer: TimerDataRegister,
    pub controller: TimerControlRegister,
    pub initial_value: u16,
    pub cycles: usize,
    pub previously_disabled: bool
}

impl Timer {
    pub fn register(&mut self, mem: &Rc<RefCell<Vec<u8>>>) {
        self.timer.register(mem);
        self.controller.register(mem);
    }

    fn frequency(&self) -> usize {
        match self.controller.get_pre_scalar_selection() {
            0 => 1,
            1 => 64,
            2 => 256,
            3 => 1024,
            _ => panic!("Error in processing frequency")
        }
    }

    fn reload_data(&mut self) {
        self.timer.set_data(self.timer.get_reload());
    }

    // Cycles between overflows at the timer's current reload/prescaler, or 0
    // while disabled. Used by Apu to tick Direct Sound at cycle-accurate
    // granularity instead of a per-CPU-step overflow count.
    pub fn period_cycles(&self) -> usize {
        if self.controller.get_enable() == 0 {
            return 0;
        }
        self.frequency() * (0x10000 - self.timer.get_reload() as usize)
    }

    pub fn update(&mut self, current_cycles: usize, irq_ctrl: &mut Interrupts) -> usize {
        self.cycles += current_cycles;
        let mut overflows = 0;
        let freq = self.frequency();
        let mut timer_data = self.timer.get_data();

        while self.cycles >= freq {
            self.cycles -= freq;
            timer_data = timer_data.wrapping_add(1);
            if timer_data == 0 {
                match self.timer.index {
                    0 => irq_ctrl.if_interrupt.set_timer_zero_overflow(1),
                    1 => irq_ctrl.if_interrupt.set_timer_one_overflow(1),
                    2 => irq_ctrl.if_interrupt.set_timer_two_overflow(1),
                    3 => irq_ctrl.if_interrupt.set_timer_three_overflow(1),
                    _ => panic!("Error in processing timer")
                }

                timer_data = self.timer.get_reload();
                overflows+=1;
            }
        }
        self.timer.set_data(timer_data);
        overflows
    }

    pub fn update_overflow(&mut self, overflows: usize, irq_ctrl: &mut Interrupts) -> usize {
        let mut timer_data = self.timer.get_data();
        let mut new_overflows = 0;

        for _ in 0..overflows {
            timer_data = timer_data.wrapping_add(1);
            if timer_data == 0 {
                match self.timer.index {
                    0 => irq_ctrl.if_interrupt.set_timer_zero_overflow(1),
                    1 => irq_ctrl.if_interrupt.set_timer_one_overflow(1),
                    2 => irq_ctrl.if_interrupt.set_timer_two_overflow(1),
                    3 => irq_ctrl.if_interrupt.set_timer_three_overflow(1),
                    _ => panic!("Error in processing timer")
                }

                timer_data = self.initial_value;
                new_overflows += 1;
            }
        }

        self.timer.set_data(timer_data);
        new_overflows
    }
}

#[derive(Serialize, Deserialize)]
pub struct TimerHandler {
    pub timers: [Timer; 4],
    pub running_timers: u8
}

impl TimerHandler {
    pub fn new() -> TimerHandler {
        return TimerHandler {
            timers: [
                Timer {
                    timer: TimerDataRegister::new(0),
                    controller: TimerControlRegister::new(0),
                    initial_value: 0,
                    cycles: 0,
                    previously_disabled: true
                },
                Timer {
                    timer: TimerDataRegister::new(1),
                    controller: TimerControlRegister::new(1),
                    initial_value: 0,
                    cycles: 0,
                    previously_disabled: true
                },
                Timer {
                    timer: TimerDataRegister::new(2),
                    controller: TimerControlRegister::new(2),
                    initial_value: 0,
                    cycles: 0,
                    previously_disabled: true
                },
                Timer {
                    timer: TimerDataRegister::new(3),
                    controller: TimerControlRegister::new(3),
                    initial_value: 0,
                    cycles: 0,
                    previously_disabled: true
                },
            ],
            running_timers: 0
        }
    }


    pub fn register(&mut self, mem: &Rc<RefCell<Vec<u8>>>){
        for i in 0..4 {
            self.timers[i].register(mem);
        }
    }

    // Per-timer overflow counts this call, indexed 0-3 (used by Apu/DMAController for Direct Sound).
    pub fn update(&mut self, cycles: usize, irq_ctrl: &mut Interrupts) -> [usize; 4] {
        let mut overflows = 0usize;
        let mut per_timer_overflows = [0usize; 4];
        for id in 0..4 {
            let mut timer = &mut self.timers[id];
            if timer.controller.get_enable() == 1 {
                if timer.previously_disabled {
                    timer.reload_data();
                    timer.previously_disabled = false;
                }

                if timer.controller.get_cascade() == 0 {
                    // if we are not a cascade timer we dont care about the previous overflows
                    overflows = timer.update(cycles, irq_ctrl);
                } else {
                    // if we are a cascade timer we dont care about the cycles
                    if overflows > 0 {
                        overflows = timer.update_overflow(overflows, irq_ctrl);
                    } else {
                        overflows = 0;
                    }
                }
                per_timer_overflows[id] = overflows;
            } else if !self.timers[id].previously_disabled {
                self.timers[id].previously_disabled = true;
            }
        }
        per_timer_overflows
    }
}

#[cfg(test)]
mod tests {
    use crate::gba::GBA;

    #[test]
    fn period_cycles_matches_frequency_and_reload() {
        let mut gba = GBA::default();
        let timer = &mut gba.timer_handler.timers[0];
        timer.controller.set_pre_scalar_selection(0);
        timer.timer.write_reload(64472);
        timer.controller.set_enable(1);
        assert_eq!(gba.timer_handler.timers[0].period_cycles(), 1064);
    }

    #[test]
    fn period_cycles_is_zero_while_disabled() {
        let mut gba = GBA::default();
        let timer = &mut gba.timer_handler.timers[0];
        timer.controller.set_pre_scalar_selection(0);
        timer.timer.write_reload(64472);
        timer.controller.set_enable(0);
        assert_eq!(gba.timer_handler.timers[0].period_cycles(), 0);
    }

    #[test]
    fn period_cycles_applies_prescaler() {
        let mut gba = GBA::default();
        let timer = &mut gba.timer_handler.timers[0];
        timer.controller.set_pre_scalar_selection(2);
        timer.timer.write_reload(0xFF00);
        timer.controller.set_enable(1);
        assert_eq!(gba.timer_handler.timers[0].period_cycles(), 256 * 256);
    }
}
