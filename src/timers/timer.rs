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
}

pub struct TimerHandler {
    pub timers: [Timer; 4],
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
            ]
        }
    }
    fn frequency(&self, timer_number: usize) -> usize {
        match self.timers[timer_number].controller.get_pre_scalar_selection() {
            0 => 1,
            1 => 64,
            2 => 256,
            3 => 1024,
            _ => panic!("Error in processing frequency")
        }
    }

    pub fn register(&mut self, mem: &Rc<RefCell<Vec<u8>>>){
        for i in 0..4 {
            self.timers[i].register(mem);
        }
    }

    pub fn write_to_register(&mut self, _timer_number: usize, initial_value: u16){
        self.timers[_timer_number].initial_value = initial_value;
    }

    pub fn update(&mut self, _current_cycles: usize, timer_number: usize ){
        let _overflows = 0;
        let _freq = self.frequency(timer_number);


    }
    
    pub fn set_enable(&mut self, _timer_number: usize){
        //TODO
    }

}
