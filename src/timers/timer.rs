use crate::memory::timer_registers::*;
use crate::interrupts::interrupts::Interrupts;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Timer {
    pub timer: TimerDataRegister,
    pub controller: TimerControlRegister,
    pub initial_value: u16,
    pub cycles: usize
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

    pub fn update(&mut self, current_cycles: usize, irq_ctrl: &mut Interrupts) -> u32 {
        self.cycles += current_cycles;
        let mut overflows = 0;
        let freq = self.frequency();
        let mut timer_data = self.timer.get_data();
        let timer_reload = self.timer.get_reload();

        while self.cycles >= freq {
            self.cycles -= freq;
            // self.timer.set_data(self.timer.get_data().wrapping_add(1));
            timer_data = timer_data.wrapping_add(1);
            if timer_data == 0 {
                match self.timer.index {
                    0 => irq_ctrl.if_interrupt.set_timer_zero_overflow(1),
                    1 => irq_ctrl.if_interrupt.set_timer_one_overflow(1),
                    2 => irq_ctrl.if_interrupt.set_timer_two_overflow(1),
                    3 => irq_ctrl.if_interrupt.set_timer_three_overflow(1),
                    _ => panic!("Error in processing timer")
                }
                // self.timer.set_data(self.timer.get_reload());
                timer_data = timer_reload;
                overflows+=1;
            }
        }
        self.timer.set_data(timer_data);
        overflows
    }
}

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
                    cycles: 0
                },
                Timer {
                    timer: TimerDataRegister::new(1),
                    controller: TimerControlRegister::new(1),
                    initial_value: 0,
                    cycles: 0

                },
                Timer {
                    timer: TimerDataRegister::new(2),
                    controller: TimerControlRegister::new(2),
                    initial_value: 0,
                    cycles: 0
                },
                Timer {
                    timer: TimerDataRegister::new(3),
                    controller: TimerControlRegister::new(3),
                    initial_value: 0,
                    cycles: 0
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

    pub fn update(&mut self, cycles: usize, irq_ctrl: &mut Interrupts){
        for id in 0..4 {
            if self.timers[id].controller.get_enable() == 1 && self.timers[id].controller.get_cascade() == 0 {
                let timer = &mut self.timers[id];
                let overflows = timer.update(cycles, irq_ctrl);
                if overflows > 0 {
                    if id != 3 {
                        let cascade_timer = &mut self.timers[id+1];
                        if cascade_timer.controller.get_cascade() == 1{
                            cascade_timer.update(overflows as usize, irq_ctrl);
                        }
                    }
                }
            }
        }
    }
}
