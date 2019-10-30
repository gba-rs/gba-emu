use std::cell::RefCell;
use std::rc::Rc;

pub struct BiosRam {
    pub memory: Rc<RefCell<Vec<u8>>>
}

impl BiosRam {
    pub fn new(init_value: u8) -> BiosRam {
        return BiosRam {
            memory: Rc::new(RefCell::new(vec![init_value; 256000]))
        }
    }
}