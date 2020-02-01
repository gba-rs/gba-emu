pub fn sign_extend_u32(value: u32, sign_bit_index: u8) -> u32 {
    let bit_mask = 0x1 << sign_bit_index;
    let sign_bit = (value & bit_mask) >> sign_bit_index;
    let temp: u64 = 0x8000_0000_0000_0000;
    let mut sign_mask: i64 = temp as i64;
    if sign_bit == 1 {
        let offset = 64 - sign_bit_index;
        sign_mask = sign_mask >> offset;
        return value | (sign_mask as u32);
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

// pub fn set_bits_u32(old_value: u32, value: u32, start_bit: u8, num_bits: u8) -> u32 {
//     if value as u32 > (num_bits as u32).pow(2) {
//         panic!("Attempting to set number out of range of bit field");
//     }

//     let mut temp = old_value;
//     for i in start_bit..=(num_bits + start_bit) {
//         temp &= !(1 << i);
//     }

//     let shifted_val = (value << start_bit) as u32;
//     return temp | shifted_val;
// }

// pub fn set_bits_u16(old_value: u16, value: u16, start_bit: u8, num_bits: u8) -> u16 {
//     return set_bits_u32(old_value as u32, value as u32, start_bit, num_bits) as u16;
// }

// pub fn set_bits_u8(old_value: u16, value: u16, start_bit: u8, num_bits: u8) -> u8 {
//     return set_bits_u32(old_value as u32, value as u32, start_bit, num_bits) as u8;
// }