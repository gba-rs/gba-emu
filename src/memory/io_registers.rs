use std::cell::RefCell;
use std::rc::Rc;

pub struct IORegisters {
    pub memory: Rc<RefCell<Vec<u8>>>
}

impl IORegisters {
    pub fn new(init_value: u8) -> IORegisters {
        return IORegisters {
            memory: Rc::new(RefCell::new(vec![init_value; 0x3FE]))
        }
    }
}