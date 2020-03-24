use crate::memory::timer_registers::*;
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
    fn frequency(&mut self) -> usize {
        match self.controller.get_pre_scalar_selection() {
            0 => 1,
            1 => 64,
            2 => 256,
            3 => 1024,
            _ => panic!("Error in processing frequency")
        }
    }
    pub fn update(&mut self, _current_cycles: usize) -> u32 {
        self.cycles += _current_cycles;
        let mut _overflows = 0;
        let _freq = self.frequency();
        while self.cycles >= _freq {
            self.cycles -= _freq;
            self.timer.set_data(self.timer.get_data().wrapping_add(1));
            if self.timer.get_data() == 0 {
                //handle interrupt 
                self.timer.set_data(self.initial_value);
                _overflows+=1;
            }
        }
        _overflows
    }
    pub fn write(&mut self, updated_value: u16){
        self.initial_value = updated_value;
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

    pub fn set_irq_enable(&mut self, _timer_number: usize, _enable: u8){
        self.timers[_timer_number].controller.set_timer_irq_enable(_enable);
    }
    pub fn set_count_up_timing(&mut self, _timer_number: usize, _enable: u8){
        self.timers[_timer_number].controller.set_count_up_enable(_enable);
    }

}
