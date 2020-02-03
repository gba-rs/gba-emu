
/// Returns (n, z)
pub fn check_flags(value: u32) -> (bool, bool){
    let n = if (value as i32) < 0 { true } else { false };
    let z = if value == 0 { true } else { false };
    return (n, z);
}

/// Returns (value, (n, z))
pub fn and(op1: u32, op2: u32) -> (u32, (bool, bool)) {
    let value = op1 & op2;
    return (value, check_flags(value));
}

/// Returns (value, (n, z))
pub fn eor(op1: u32, op2: u32) -> (u32, (bool, bool)) {
    let value = op1 ^ op2;
    return (value, check_flags(value));
}

/// Returns (value, (n, z))
pub fn orr(op1: u32, op2: u32) -> (u32, (bool, bool)) {
    let value = op1 | op2;
    return (value, check_flags(value));
}

/// Returns (value, (n, z))
pub fn bic(op1: u32, op2: u32) -> (u32, (bool, bool)) {
    let value = op1 & !op2;
    return (value, check_flags(value));
}
