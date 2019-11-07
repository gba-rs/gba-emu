pub fn apply_offset(base_value: u32, offset: u8, add: bool) -> u32 {
    if add {
        return base_value + (offset as u32);
    }
    let val = base_value - (offset as u32);
    return val;
}

pub fn is_word_aligned(memory_address: u32) -> bool {
    return (memory_address & 0x3) == 0; // mult of 4s
}

pub fn is_word_plus_1_aligned(memory_address: u32) -> bool {
    return (memory_address & 0x3) == 1; // 1 more than mult. of 4
}

pub fn is_halfword_aligned(memory_address: u32) -> bool {
    return (memory_address & 0x3) == 2; // 2 more than mult. of 4
}