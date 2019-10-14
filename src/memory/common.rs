use std::cell::RefCell;

struct Range {
    pub lower: u32,
    pub upper: u32
}

impl Range {
    pub fn new(lower: u32, upper: u32) -> Range {
        return Range {
            lower: lower,
            upper: upper
        };
    }
}

struct MemoryBlock {
    pub range: Range,
    pub memory: RefCell<Vec<u8>>
}

impl MemoryBlock {
    pub fn new(lower_address: u32, upper_address: u32, mem_ref: RefCell<Vec<u8>>) -> MemoryBlock {
        return MemoryBlock {
            range: Range::new(lower_address, upper_address),
            memory: mem_ref
        };
    }
}