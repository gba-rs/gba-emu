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
    return sign_extend_u16(value as u16, sign_bit_index) as u8;
}