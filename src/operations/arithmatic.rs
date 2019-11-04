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

pub fn mul(op1: u32, op2: u32) -> (u32, Flags) {
    let result = op1.overflowing_mul(op2);
    let product = result.0;

    return (product, Flags{
        negative: (product >> 31) != 0,
        zero: product == 0,
        carry: false,
        signed_overflow: result.1
    });
}

pub fn mull(op1: u32, op2: u32, unsigned: bool) -> (u32, u32, Flags) {
    if !unsigned {
        let mask = (0xFFFF_FFFF_0000_0000u64 as i64);
        let product = ((op1 as i64) | mask) * (op2 as i64);
        let rd_hi = ((product >> 32) as u32);
        let rd_lo = ((product & 0x0000_0000_FFFF_FFFF) as u32);
        return (rd_hi, rd_lo, Flags{
            negative: (rd_hi >> 31) != 0,
            zero: product == 0,
            carry: false,
            signed_overflow: false
        });
    } else{
        let product = (op1 as u64) * (op2 as u64);
        let rd_hi = ((product >> 32) as u32);
        let rd_lo = ((product & 0x0000_0000_FFFF_FFFF) as u32);
        return (rd_hi, rd_lo, Flags{
            negative: (rd_hi >> 31) != 0,
            zero: product == 0,
            carry: false,
            signed_overflow: false
        });
    }
}

pub fn u32_from_u64(num: u64) -> (u32, u32) {
    let rd_hi = ((product >> 32) as u32);
    let rd_lo = ((product & 0x0000_0000_FFFF_FFFF) as u32);
    return (rd_hi, rd_lo);
}

pub fn u32_from_i64(num: i64) -> (u32, u32) {
    let rd_hi = ((product >> 32) as u32);
    let rd_lo = ((product & 0x0000_0000_FFFF_FFFF) as u32);
    return (rd_hi, rd_lo);
}

pub fn u64_from_u32(rhi: u32, rlo: u32) -> u64 {
    return ((rhi as u64) << 32) + (rlo as u64);
}

pub fn i64_from_u32(rhi: u32, rlo: u32) -> i64 {
    return ((rhi as i64) << 32) + (rlo as i64);
}

pub fn mla(op1: u32, op2: u32, op3: u32) -> (u32, Flags) {
    let product = mul(op1, op2);
    let output = (add(product.0, op3).0, product.1);
    return output;
}

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