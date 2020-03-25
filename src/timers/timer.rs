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
    fn frequency(&mut self) -> usize {
        match self.controller.get_pre_scalar_selection() {
            0 => 1,
            1 => 64,
            2 => 256,
            3 => 1024,
            _ => panic!("Error in processing frequency")
        }
    }
    pub fn update(&mut self, _current_cycles: usize, irq_ctrl: &mut Interrupts) -> u32 {
        self.cycles += _current_cycles;
        let mut _overflows = 0;
        let _freq = self.frequency();
        while self.cycles >= _freq {
            self.cycles -= _freq;
            self.timer.set_data(self.timer.get_data().wrapping_add(1));
            if self.timer.get_data() == 0 {
                match self.timer.index {
                    0 => irq_ctrl.if_interrupt.set_timer_zero_overflow(1),
                    1 => irq_ctrl.if_interrupt.set_timer_one_overflow(1),
                    2 => irq_ctrl.if_interrupt.set_timer_two_overflow(1),
                    3 => irq_ctrl.if_interrupt.set_timer_three_overflow(1),
                    _ => panic!("Error in processing timer")
                }
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

    pub fn write_to_control(&mut self, id: usize, value: u16){
        self.timers[id].controller.set_pre_scalar_selection((value & 3) as u8);
        self.timers[id].controller.set_count_up_enable(((value >> 2) & 1) as u8);
        self.timers[id].controller.set_timer_irq_enable(((value >> 6) &1) as u8);
        self.timers[id].controller.set_timer_start_stop(((value >> 7) & 1) as u8);
        let new_enabled = self.timers[id].controller.get_timer_start_stop() == 1;
        let cascade = self.timers[id].controller.get_count_up_enable() == 1;
        if new_enabled && !cascade {
            self.running_timers |= 1 << id;
        } else {
            self.running_timers &= !(1 << id);
        }
        
    }

    pub fn read_timer(&mut self, _id: usize) -> u16{
        self.timers[_id].timer.get_data()
    }

    pub fn update(&mut self, cycles: usize, irq_ctrl: &mut Interrupts){
        for id in 0..4 {
            if self.timers[id].controller.get_count_up_enable() == 1 {
                let _timer = &mut self.timers[id];
                let _overflows = _timer.update(cycles, irq_ctrl);
                if _overflows > 0 {
                    if id != 3 {
                        let _cascade_timer = &mut self.timers[id+1];
                        if _cascade_timer.controller.get_count_up_enable() == 1{
                            _cascade_timer.update(_overflows as usize, irq_ctrl);
                        }
                    }
                }
            }
        }
    }
}
