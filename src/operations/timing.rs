pub struct Clock {
    prev_address: u32
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum MemAccessSize {
    Mem8,
    Mem16,
    Mem32,
}
impl Clock {
    pub fn get_cycles(address:u32, access_size: MemAccessSize) {
        // TODO
    }
}


