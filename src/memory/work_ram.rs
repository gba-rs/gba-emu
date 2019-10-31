use std::cell::RefCell;
use std::rc::Rc;

pub struct WorkRam {
    pub memory: Rc<RefCell<Vec<u8>>>
}

// 256000
impl WorkRam {
    pub fn new(size: usize, init_value: u8) -> WorkRam {
        return WorkRam {
            memory: Rc::new(RefCell::new(vec![init_value; size]))
        }
    }
}