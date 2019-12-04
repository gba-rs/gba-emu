use std::cell::RefCell;
use std::rc::Rc;

pub struct GamePackRom {
    pub memory: Rc<RefCell<Vec<u8>>>
}

impl GamePackRom {
    pub fn new(init_value: u8) -> GamePackRom {
        return GamePackRom {
            memory: Rc::new(RefCell::new(vec![init_value; 0x1FF_FFFF]))
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