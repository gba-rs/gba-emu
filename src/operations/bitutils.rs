pub fn sign_extend_u32(value: u32, sign_bit_index: u8) -> u32 {
    let bit_mask = 0x1 << sign_bit_index;
    let sign_bit = (value & bit_mask) >> sign_bit_index;
    if sign_bit == 1 {
        let shift_amount = 31 - sign_bit_index;
        let temp: u32 = (((value << shift_amount) as i32) >> shift_amount) as u32;

        return temp;
    } else {
        return value;
    }
}

pub fn sign_extend_u16(value: u16, sign_bit_index: u8) -> u16 {
    return sign_extend_u32(value as u32, sign_bit_index) as u16;
}

pub fn sign_extend_u8(value: u8, sign_bit_index: u8) -> u8 {
    return sign_extend_u32(value as u32, sign_bit_index) as u8;
}

pub fn get_bits_u32(value: u32, start_bit: u8, num_bits: u8) -> u32 {
    return ((1 << num_bits) - 1) & (value >> (start_bit - 1)); 
}

pub fn get_bits_u16(value: u16, start_bit: u8, num_bits: u8) -> u16 {
    return get_bits_u32(value as u32, start_bit, num_bits) as u16;
}

pub fn get_bits_u8(value: u8, start_bit: u8, num_bits: u8) -> u8 {
    return get_bits_u32(value as u32, start_bit, num_bits) as u8;
}