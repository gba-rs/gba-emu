use std::cell::RefCell;
use std::rc::Rc;

pub struct WorkRam {
    pub memory: Rc<RefCell<Vec<u8>>>
}

impl WorkRam {
    pub fn new(init_value: u8) -> WorkRam {
        return WorkRam {
            memory: Rc::new(RefCell::new(vec![init_value; 256000]))
        }
    }
}