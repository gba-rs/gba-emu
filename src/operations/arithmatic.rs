use crate::{cpu::Flags};

fn _add(op1: u32, op2: u32, carry_in: bool) -> (u32, Flags) {
    let output: u64 = (op1 as u64) + (op2 as u64) + (carry_in as u64);
    let carryout: bool = (output >> 32) != 0;
    let real_output: u32 = (output & 0xFFFFFFFF) as u32;
    let op1_sign: bool = (op1 >> 31) != 0;
    let op2_sign: bool = (op2 >> 31) != 0;
    let output_sign: bool = ((output >> 31) & 0x01) != 0;

    return (real_output, Flags{
        negative: (output & (0x1 << 31)) != 0,
        zero: output == 0,
        carry: carryout,
        signed_overflow: (op1_sign == op2_sign) && (op1_sign != output_sign)
    });
}

fn _mul(op1: u32, op2: u32) -> (u32, Flags) {
    let result = op1.overflowing_mul(op2);
    let product = result.0;
    return (product, Flags{
        negative: (product << 31) != 0,
        zero: product == 0,
        carry: false,
        signed_overflow: result.1
    });
}

fn _mull(op1: u32, op2: u32) -> (u32, Flags) {
    //TODO implement
    return (product, Flags{
        //TODO set flags
    });
}

pub fn mla(op1: u32, op2: u32, op3: u32) -> (u32, Flags) {
    let product = _mul(op1, op2);
    //TODO figure out how to do
    return output;
}

pub fn mlal(op1: u32, op2: u32, op3: u32) -> (u32, Flags) {
    let product = _mull(op1, op2);
    let output = (add(product.0, op3).0, product.1);
    return output;
}


pub fn mul(op1: u32, op2: u32) -> (u32, Flags) { return _mul(op1, op2) }

pub fn mull(op1: u32, op2: u32) -> (u32, Flags) { return _mull(op1, op2) }

pub fn add(op1: u32, op2: u32) -> (u32, Flags) {
    return _add(op1, op2, false);
}

pub fn sub(op1: u32, op2: u32) -> (u32, Flags) {
    return _add(op1, !op2, true);
}

pub fn rsb(op1: u32, op2: u32) -> (u32, Flags) {
    return _add(!op1, op2, true);
}

pub fn sbc(op1: u32, op2: u32) -> (u32, Flags) {
    return _add(op1, !op2, false);
}

pub fn rsc(op1: u32, op2: u32) -> (u32, Flags) {
    return _add(!op1, op2, false);
}

pub fn adc(op1: u32, op2: u32) -> (u32, Flags) {
    return _add(op1, op2, true);
}

pub fn cmp(op1: u32, op2: u32) -> Flags {
    let (_, flags) = sub(op1, op2);
    return flags;
}

pub fn cmn(op1: u32, op2: u32) -> Flags {
    let (_, flags) = add(op1, op2);
    return flags;
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_add_normal() {
        let op1 = 0x0101;
        let op2 = 0x1010;
        let (value, flags) = add(op1, op2);
        assert_eq!(value, op1 + op2);
        assert_eq!(flags.negative, false);
        assert_eq!(flags.zero, false);
        assert_eq!(flags.carry, false);
        assert_eq!(flags.signed_overflow, false);
    }

    #[test]
    fn test_add_zero() {
        let (value, flags) = add(0, 0);
        assert_eq!(value, 0);
        assert_eq!(flags.negative, false);
        assert_eq!(flags.zero, true);
        assert_eq!(flags.carry, false);
        assert_eq!(flags.signed_overflow, false);
    }

    #[test]
    fn test_add_carry() {
        let (value, flags) = add(0xFFFFFFFF, 0xFFFFFFFF);
        assert_eq!(value, 0xFFFFFFFF - 1);
        assert_eq!(flags.negative, true);
        assert_eq!(flags.zero, false);
        assert_eq!(flags.carry, true);
        assert_eq!(flags.signed_overflow, false);
    }
}