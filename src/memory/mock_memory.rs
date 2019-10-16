use std::cell::RefCell;
use std::rc::Rc;

pub struct MockMemory {
    pub memory: Rc<RefCell<Vec<u8>>>
}

impl MockMemory {
    pub fn new(init_value: u8) -> MockMemory {
        return MockMemory {
            memory: Rc::new(RefCell::new(vec![init_value; 256000]))
        }
    }

    pub fn get_mock_field(&self) -> u8 {
        return self.memory.borrow()[100];
    }

    pub fn set_mock_field(&mut self, value: u8) {
        self.memory.borrow_mut()[100] = value;
    }
}