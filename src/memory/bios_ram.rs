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

    pub fn load(&mut self, bios: &Vec<u8>) {
        let mut index: usize = 0;
        for byte in bios {
            self.memory.borrow_mut()[index] = *byte;
            index += 1;
        }
    }
}